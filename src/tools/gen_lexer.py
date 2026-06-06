#!/usr/bin/env python3
"""Transpile a vendored Pygments ``RegexLexer`` into a native Rust lexer.

The Rust ``pygmentsrs`` crate ports the Pygments ``RegexLexer`` engine
(state stack, ``bygroups``, ``default``, ``#pop``/``#push``) but writing
each lexer's state table by hand is slow and error-prone. Pygments,
however, *already* stores every lexer as structured data: after
``process_tokendef`` each state is a list of
``(compiled_pattern.match, action, new_state)`` triples, with
``include()`` / ``words()`` / ``combined()`` expanded. This tool walks
that processed table and emits an equivalent
``pygmentsrs/src/lexers/generated/<name>.rs``.

Transpilable actions:

* a ``_TokenType``          -> ``Rule::token`` / ``Rule::token_to``
* ``bygroups(t1, t2, ...)`` -> ``Rule::bygroups`` / ``Rule::bygroups_to``
  (every group must itself be a ``_TokenType`` or ``None``)
* ``default(state)``        -> ``Rule::default`` (zero-width, action ``None``)

Non-transpilable actions (the lexer is reported as bridge-only and no
file is written):

* ``using(OtherLexer)`` / ``this`` cross-lexer delegation
* arbitrary Python callback actions
* ``bygroups`` whose groups contain a nested callback

Usage::

    # generate one or more lexers (each arg is module:ClassName:rust_name)
    python tools/gen_lexer.py \
        pygments.lexers.configs:IniLexer:ini \
        pygments.lexers.shell:BashLexer:bash

    # inventory every un-ported lexer by transpilability category;
    # the `transpilable` rows are ready-to-paste generate specs
    python tools/gen_lexer.py --classify
    python tools/gen_lexer.py --classify transpilable   # only that bucket

    # print the registry.rs / generated/mod.rs wiring snippet for specs
    python tools/gen_lexer.py --registry \
        pygments.lexers.shell:BashLexer:bash

``rust_name`` becomes the generated file stem; the Rust struct is
``<RustName>Lexer`` (each ``_``-separated part capitalized).
"""

from __future__ import annotations

import importlib
import re
import sys
from pathlib import Path

REPO_ROOT = Path(__file__).resolve().parents[1]
TOKEN_RS = REPO_ROOT / "pygmentsrs" / "src" / "token.rs"
OUT_DIR = REPO_ROOT / "pygmentsrs" / "src" / "lexers" / "generated"

# Python `re` flag bits -> fancy-regex inline flag chars.
FLAG_CHARS = [
    (re.IGNORECASE, "i"),
    (re.MULTILINE, "m"),
    (re.DOTALL, "s"),
    (re.VERBOSE, "x"),
]


class NotTranspilable(Exception):
    """Raised when a lexer uses an action the transpiler cannot emit."""


def load_token_map() -> dict[tuple[str, ...], str]:
    """Parse ``token.rs`` -> {("Literal","String","Escape"): "STRING_ESCAPE"}."""
    text = TOKEN_RS.read_text(encoding="utf-8")
    pat = re.compile(
        r"pub const ([A-Z0-9_]+): TokenType = TokenType::new\(&\[([^\]]*)\]\);"
    )
    out: dict[tuple[str, ...], str] = {}
    for m in pat.finditer(text):
        const = m.group(1)
        seg_src = m.group(2).strip()
        if seg_src:
            segs = tuple(s.strip().strip('"') for s in seg_src.split(","))
        else:
            segs = ()
        out[segs] = const
    return out


TOKEN_MAP = load_token_map()


def token_expr(ttype) -> str:
    """Map a pygments ``_TokenType`` to a Rust ``token::`` expression.

    Prefers a named const (e.g. ``STRING_ESCAPE``) when one exists in
    ``token.rs`` — keeps generated code readable and lets snapshot
    diffs stay stable. Falls back to a structural
    ``TokenType::new(&["Name", "Variable", "Anonymous"])`` for the many
    ad-hoc custom subtypes individual lexers invent, so the transpiler
    never blocks on a missing const. ``TokenType::new`` is a ``const fn``
    over a ``'static`` slice, so the inline form is zero-cost.
    """
    segs = tuple(str(s) for s in ttype)  # _TokenType iterates its segments
    named = TOKEN_MAP.get(segs)
    if named is not None:
        return named
    if not segs:
        return "TOKEN"
    seg_lits = ", ".join(f'"{s}"' for s in segs)
    return f"TokenType::new(&[{seg_lits}])"


def is_tokentype(action) -> bool:
    # pygments _TokenType is a tuple subclass with a `.split` method and
    # repr starting with "Token".
    return action.__class__.__name__ == "_TokenType"


def bygroups_args(action):
    """Recover the (t1, t2, ...) args of a ``bygroups`` callback, or None."""
    if getattr(action, "__qualname__", "") != "bygroups.<locals>.callback":
        return None
    freevars = action.__code__.co_freevars
    if "args" not in freevars:
        return None
    cell = action.__closure__[freevars.index("args")]
    return cell.cell_contents


def flag_prefix(flags: int) -> str:
    chars = "".join(c for bit, c in FLAG_CHARS if flags & bit)
    # Pygments lexers default to re.MULTILINE; the engine relies on `m`
    # so `^`/`$` are line-anchored. Always include it.
    if "m" not in chars:
        chars += "m"
    chars = "".join(sorted(set(chars)))
    return f"(?{chars})"


def rust_raw_string(s: str) -> str:
    """Emit ``s`` as a Rust raw string literal with enough ``#`` hashes."""
    n = 0
    while f'"{"#" * n}' in s or s.endswith('"' + "#" * n):
        n += 1
    hashes = "#" * n
    return f'r{hashes}"{s}"{hashes}'


def new_state_expr(new_state) -> str:
    """Translate a processed pygments newstate into a ``NewState`` expr."""
    if new_state is None:
        return "NewState::None"
    if isinstance(new_state, int):
        # -1 == #pop, -N == pop N.
        return f"NewState::Pop({abs(new_state)})"
    if isinstance(new_state, str):
        if new_state == "#pop":
            return "NewState::Pop(1)"
        if new_state == "#push":
            return "NewState::PushSame"
        return f'NewState::Push(vec![{rust_state_name(new_state)}])'
    if isinstance(new_state, tuple):
        if len(new_state) == 1 and new_state[0] == "#push":
            return "NewState::PushSame"
        items = ", ".join(rust_state_name(s) for s in new_state)
        return f"NewState::Push(vec![{items}])"
    raise NotTranspilable(f"unhandled newstate {new_state!r}")


def rust_state_name(s: str) -> str:
    # State names can be temp names like "Foo-1" from combined(); they're
    # only used as map keys, so any string is fine.
    return rust_raw_string(s)


def rule_expr(pattern: str, action, new_state, flags: int) -> str:
    ns = new_state_expr(new_state)
    has_ns = ns != "NewState::None"

    # default(...) -> zero-width, action is None.
    if action is None:
        return f"Rule::default({ns})"

    full_pat = flag_prefix(flags) + pattern
    pat_lit = rust_raw_string(full_pat)

    if is_tokentype(action):
        const = token_expr(action)
        if has_ns:
            return f"Rule::token_to({pat_lit}, {const}, {ns})"
        return f"Rule::token({pat_lit}, {const})"

    args = bygroups_args(action)
    if args is not None:
        groups = []
        for a in args:
            if a is None:
                groups.append("None")
            elif is_tokentype(a):
                groups.append(f"Some({token_expr(a)})")
            else:
                raise NotTranspilable(
                    f"bygroups group is not a token: {a!r}"
                )
        groups_lit = "vec![" + ", ".join(groups) + "]"
        if has_ns:
            return f"Rule::bygroups_to({pat_lit}, {groups_lit}, {ns})"
        return f"Rule::bygroups({pat_lit}, {groups_lit})"

    raise NotTranspilable(
        f"action {getattr(action, '__qualname__', action)!r} is not "
        f"a token / bygroups / default (likely using()/callback)"
    )


def transpile(module: str, classname: str, rust_name: str) -> str:
    mod = importlib.import_module(module)
    cls = getattr(mod, classname)
    # Instantiate and read `_tokens` from the *instance*: token-variant
    # lexers (e.g. `CSharpLexer`, `NemerleLexer`) set `self._tokens` in
    # `__init__` and never populate the class attribute.
    inst = cls()  # triggers RegexLexerMeta.process_tokendef
    flags = cls.flags
    states = inst._tokens

    struct = "".join(p.capitalize() for p in rust_name.split("_")) + "Lexer"
    aliases = list(cls.aliases)

    # Pre-render every rule so we can tell whether `NewState` is used
    # (it isn't for single-state token-only lexers) and avoid emitting
    # an unused import.
    rendered: dict[str, list[str]] = {}
    uses_new_state = False
    for state, rules in states.items():
        exprs = []
        for matchfn, action, new_state in rules:
            pattern = matchfn.__self__.pattern
            expr = rule_expr(pattern, action, new_state, flags)
            if "NewState::" in expr:
                uses_new_state = True
            exprs.append(expr)
        rendered[state] = exprs

    engine_imports = "NewState, Rule, StateTable, tokenize" if uses_new_state else "Rule, StateTable, tokenize"

    lines: list[str] = []
    lines.append(f"//! AUTO-GENERATED from `pygments.{module}:{classname}`.")
    lines.append("//!")
    lines.append("//! Do not edit by hand. Regenerate with:")
    lines.append(f"//!   python tools/gen_lexer.py {module}:{classname}:{rust_name}")
    lines.append("")
    lines.append("use std::collections::HashMap;")
    lines.append("use std::sync::OnceLock;")
    lines.append("")
    lines.append("use crate::lexer::Lexer;")
    lines.append(f"use crate::lexer::engine::{{{engine_imports}}};")
    lines.append("use crate::token::*;")
    lines.append("")
    lines.append(f"/// Aliases: {', '.join(aliases)}")
    lines.append(f"pub struct {struct};")
    lines.append("")
    lines.append("struct Table(HashMap<&'static str, Vec<Rule>>);")
    lines.append("")
    lines.append("impl StateTable for Table {")
    lines.append("    fn state(&self, name: &str) -> Option<&[Rule]> {")
    lines.append("        self.0.get(name).map(Vec::as_slice)")
    lines.append("    }")
    lines.append("}")
    lines.append("")
    lines.append("static TABLE: OnceLock<Table> = OnceLock::new();")
    lines.append("")
    lines.append("fn build_table() -> Table {")
    lines.append("    let mut m: HashMap<&'static str, Vec<Rule>> = HashMap::new();")
    for state, exprs in rendered.items():
        lines.append(f"    m.insert({rust_state_name(state)}, vec![")
        for expr in exprs:
            lines.append(f"        {expr},")
        lines.append("    ]);")
    lines.append("    Table(m)")
    lines.append("}")
    lines.append("")
    lines.append(f"impl Lexer for {struct} {{")
    lines.append("    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {")
    lines.append("        let table = TABLE.get_or_init(build_table);")
    lines.append("        tokenize(table, code)")
    lines.append("    }")
    lines.append("}")
    lines.append("")
    return "\n".join(lines)


def classify_lexer(module: str, classname: str) -> tuple[str, str]:
    """Classify a lexer without writing anything.

    Returns ``(category, detail)`` where category is one of:

    * ``transpilable``    — ready to generate (token/bygroups/default only)
    * ``bridge_using``    — uses ``using()`` / ``this`` (keep on PyO3 bridge)
    * ``bridge_callback`` — arbitrary Python callback action (bridge)
    * ``non_regex``       — not a ``RegexLexer`` subclass (bridge)
    * ``error``           — failed to import / process_tokendef
    """
    from pygments.lexer import RegexLexer

    try:
        mod = importlib.import_module(module)
        cls = getattr(mod, classname)
    except Exception as exc:  # noqa: BLE001
        return ("error", f"import: {exc}")
    if not isinstance(cls, type) or not issubclass(cls, RegexLexer):
        base = cls.__bases__[0].__name__ if isinstance(cls, type) else type(cls).__name__
        return ("non_regex", base)
    try:
        inst = cls()  # trigger process_tokendef
        states = inst._tokens
    except Exception as exc:  # noqa: BLE001
        return ("error", f"tokendef: {str(exc)[:60]}")
    for _state, rules in states.items():
        for _matchfn, action, _ns in rules:
            if action is None:
                continue
            qn = getattr(action, "__qualname__", "")
            if is_tokentype(action):
                continue
            if qn == "bygroups.<locals>.callback":
                if bygroups_args(action) is None:
                    return ("bridge_callback", "bygroups w/ nested callback")
                if any(
                    a is not None and not is_tokentype(a)
                    for a in bygroups_args(action)
                ):
                    return ("bridge_callback", "bygroups w/ nested callback")
                continue
            if qn == "using.<locals>.callback":
                return ("bridge_using", "using()/this delegation")
            return ("bridge_callback", qn or repr(action)[:40])
    return ("transpilable", f"{len(states)} states")


def cmd_classify(filter_cat: str | None) -> int:
    """Inventory every lexer in the vendored registry by transpilability."""
    from pygments.lexers._mapping import LEXERS

    # Already-native aliases, so we can skip covered lexers.
    try:
        import pygmentsrs  # type: ignore

        native = set(pygmentsrs.native_aliases())
    except Exception:  # noqa: BLE001
        native = set()

    buckets: dict[str, list[tuple[str, str, str, str]]] = {}
    for classname, (module, _ln, aliases, _fns, _mimes) in LEXERS.items():
        if any(a in native for a in aliases):
            continue
        cat, detail = classify_lexer(module, classname)
        primary = aliases[0] if aliases else ""
        buckets.setdefault(cat, []).append((primary, classname, module, detail))

    order = [
        "transpilable",
        "bridge_using",
        "bridge_callback",
        "non_regex",
        "error",
    ]
    total = sum(len(v) for v in buckets.values())
    print(f"# {total} un-ported lexers (native aliases excluded)\n")
    for cat in order:
        rows = sorted(buckets.get(cat, []))
        print(f"## {cat}: {len(rows)}")
        if filter_cat and cat != filter_cat:
            print()
            continue
        for primary, classname, module, detail in rows:
            # Emit a ready-to-use generate spec for transpilable rows.
            if cat == "transpilable":
                rust = primary.replace("-", "_").replace("+", "p") or classname.lower()
                print(f"  {module}:{classname}:{rust}    # {detail}")
            else:
                print(f"  {primary:18} {classname:28} {detail}")
        print()
    return 0


def cmd_registry(specs: list[str]) -> int:
    """Print the registry wiring snippet for the given generated specs.

    Emits the `get_lexer_by_name` match arms and `native_aliases`
    entries to paste into `src/pygmentsrs/src/lexers/registry.rs` and
    the `pub mod` lines for `generated/mod.rs`.
    """
    arms, alias_lines, mods = [], [], []
    for spec in specs:
        try:
            module, classname, rust_name = spec.split(":")
        except ValueError:
            print(f"// SKIP {spec!r}: expected module:Class:rust_name")
            continue
        mod = importlib.import_module(module)
        cls = getattr(mod, classname)
        aliases = list(cls.aliases)
        struct = "".join(p.capitalize() for p in rust_name.split("_")) + "Lexer"
        pat = " | ".join(f'"{a}"' for a in aliases)
        arms.append(
            f"        {pat} => Some(Box::new(generated::{rust_name}::{struct})),"
        )
        for a in aliases:
            alias_lines.append(f'        "{a}",')
        mods.append(f"pub mod {rust_name};")

    print("// --- generated/mod.rs ---")
    print("\n".join(sorted(mods)))
    print("\n// --- registry.rs: get_lexer_by_name arms ---")
    print("\n".join(arms))
    print("\n// --- registry.rs: native_aliases entries ---")
    print("\n".join(alias_lines))
    return 0


def cmd_generate(specs: list[str]) -> int:
    OUT_DIR.mkdir(parents=True, exist_ok=True)
    written, skipped = [], []
    for spec in specs:
        try:
            module, classname, rust_name = spec.split(":")
        except ValueError:
            print(f"SKIP {spec!r}: expected module:Class:rust_name")
            skipped.append(spec)
            continue
        try:
            src = transpile(module, classname, rust_name)
        except NotTranspilable as exc:
            print(f"SKIP {classname} ({rust_name}): {exc}")
            skipped.append(spec)
            continue
        out = OUT_DIR / f"{rust_name}.rs"
        out.write_text(src, encoding="utf-8")
        print(f"WROTE {out.relative_to(REPO_ROOT)}")
        written.append((rust_name, classname))
    print(f"\n{len(written)} written, {len(skipped)} skipped")
    return 0 if not skipped else 1


def main(argv: list[str]) -> int:
    if not argv or argv[0] in ("-h", "--help"):
        print(__doc__)
        return 0 if argv else 2
    if argv[0] == "--classify":
        cat = argv[1] if len(argv) > 1 else None
        return cmd_classify(cat)
    if argv[0] == "--registry":
        return cmd_registry(argv[1:])
    return cmd_generate(argv)


if __name__ == "__main__":
    raise SystemExit(main(sys.argv[1:]))
