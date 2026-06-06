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


def get_using_info(action):
    """Inspect a ``using(...)`` callback's closure and return ``(target, stack)``.

    ``target`` is either the string ``"this"`` (for ``using(this, ...)`` /
    ``using(this, state='foo')``) or a primary lexer alias string (for
    ``using(SomeLexer)``).

    ``stack`` is a list of state names (the ``state=`` argument converted to
    a stack tuple by pygments) or ``None`` when no state override was given.

    Returns ``None`` if ``action`` is not a ``using(...)`` callback.
    """
    if getattr(action, "__qualname__", "") != "using.<locals>.callback":
        return None
    freevars = action.__code__.co_freevars
    try:
        gt_kwargs = action.__closure__[freevars.index("gt_kwargs")].cell_contents
    except (ValueError, IndexError):
        gt_kwargs = {}
    stack = gt_kwargs.get("stack", None)  # tuple like ('root', 'string') or None
    if "_other" not in freevars:
        # using(this, ...) — self-recursive
        return ("this", list(stack) if stack else None)
    try:
        other_cls = action.__closure__[freevars.index("_other")].cell_contents
    except (ValueError, IndexError):
        return None
    aliases = getattr(other_cls, "aliases", None)
    alias = aliases[0] if aliases else other_cls.__name__.lower().replace("lexer", "")
    return (alias, list(stack) if stack else None)


def state_vec_literal(stack) -> str:
    """Render an optional state list as a Rust ``Option<Vec<&'static str>>``."""
    if not stack:
        return "None"
    items = ", ".join(f'"{s}"' for s in stack)
    return f"Some(vec![{items}])"


def group_action_expr(a) -> str:
    """Render a single bygroups group entry as a ``GroupAction`` variant."""
    if a is None:
        return "None"
    if is_tokentype(a):
        return f"Some(GroupAction::Token({token_expr(a)}))"
    info = get_using_info(a)
    if info is not None:
        target, stack = info
        sv = state_vec_literal(stack)
        if target == "this":
            return f"Some(GroupAction::UsingThis {{ state: {sv} }})"
        else:
            return f'Some(GroupAction::UsingLexer {{ alias: "{target}", state: {sv} }})'
    raise NotTranspilable(f"bygroups group is not a token or using(): {a!r}")


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


def fix_py_regex_for_rust(pattern: str) -> str:
    """Fix Python regex character-class syntax incompatible with fancy-regex.

    Two cases handled:

    1. Unescaped ``[`` inside ``[...]`` or ``[^...]``:
       Python's ``re`` treats a bare ``[`` as a literal inside a character
       class; fancy-regex's parser rejects it.  Fix: escape to ``\\[``.

    2. Leading ``]`` in a character class (``[][...]`` or ``[^][...]``):
       Python treats a ``]`` immediately after the opening ``[`` (or ``[^``)
       as a literal ``]``; fancy-regex does not.  Fix: escape to ``\\]``.
    """
    result: list[str] = []
    i = 0
    n = len(pattern)
    while i < n:
        c = pattern[i]
        if c == "\\" and i + 1 < n:
            # Copy escape sequence verbatim.
            result.append(c)
            result.append(pattern[i + 1])
            i += 2
            continue
        if c != "[":
            result.append(c)
            i += 1
            continue
        # Opening bracket — enter character class.
        result.append("[")
        i += 1
        # Optional negation.
        if i < n and pattern[i] == "^":
            result.append("^")
            i += 1
        # Leading `]` is literal in Python but fancy-regex needs it escaped.
        if i < n and pattern[i] == "]":
            result.append("\\]")
            i += 1
        # Scan body of character class until unescaped closing `]`.
        while i < n:
            c = pattern[i]
            if c == "\\" and i + 1 < n:
                result.append(c)
                result.append(pattern[i + 1])
                i += 2
                continue
            if c == "]":
                result.append("]")
                i += 1
                break
            if c == "[":
                # Bare `[` inside character class — escape it.
                result.append("\\[")
                i += 1
                continue
            result.append(c)
            i += 1
    return "".join(result)


def detect_name_remap(cls) -> list[tuple[tuple[str, ...], object]]:
    """Detect if *cls* overrides ``get_tokens_unprocessed`` with a simple
    ``Token.Name → Token.Xxx`` remapping driven by word-sets stored as
    class attributes.  Returns ``[(words_tuple, target_tokentype), ...]``
    in the order the checks appear in the override (earlier = higher
    priority), or ``[]`` if not detected / too complex.

    Example pattern handled (Elixir, Erlang)::

        def get_tokens_unprocessed(self, text):
            for index, token, value in RegexLexer.get_tokens_unprocessed(self, text):
                if token is Name:
                    if value in self.KEYWORD:
                        yield index, Keyword, value
                    elif value in self.BUILTIN_DECLARATION:
                        yield index, Keyword.Declaration, value
                    ...
                else:
                    yield index, token, value
    """
    import inspect
    import re as _re

    if "get_tokens_unprocessed" not in cls.__dict__:
        return []
    try:
        src = inspect.getsource(cls.__dict__["get_tokens_unprocessed"])
    except Exception:
        return []
    if "token is Name" not in src:
        return []

    # Extract `elif value in self.ATTR:\n...: yield ..., TOKENTYPE, value` pairs.
    #   Group 1: attribute name (e.g. KEYWORD, BUILTIN_DECLARATION)
    #   Group 2: token type expression (e.g. Keyword, Keyword.Declaration)
    _pair_re = _re.compile(
        r"(?:if|elif)\s+value\s+in\s+self\.(\w+)"
        r".*?yield[^,\n]+,\s*([\w.]+)\s*,\s*value",
        _re.DOTALL,
    )

    from pygments import token as T
    _tok_names = {
        "Keyword": T.Keyword,
        "Keyword.Declaration": T.Keyword.Declaration,
        "Keyword.Namespace": T.Keyword.Namespace,
        "Keyword.Reserved": T.Keyword.Reserved,
        "Keyword.Type": T.Keyword.Type,
        "Keyword.Constant": T.Keyword.Constant,
        "Operator": T.Operator,
        "Operator.Word": T.Operator.Word,
        "Name": T.Name,
        "Name.Builtin": T.Name.Builtin,
        "Name.Builtin.Pseudo": T.Name.Builtin.Pseudo,
        "Name.Class": T.Name.Class,
        "Name.Constant": T.Name.Constant,
        "Name.Function": T.Name.Function,
        "Name.Variable": T.Name.Variable,
        "Punctuation": T.Punctuation,
    }

    result: list[tuple[tuple[str, ...], object]] = []
    for m in _pair_re.finditer(src):
        attr_name, tok_expr = m.group(1).strip(), m.group(2).strip()
        tok = _tok_names.get(tok_expr)
        if tok is None:
            continue
        words_attr = getattr(cls, attr_name, None)
        if not words_attr or not isinstance(words_attr, (tuple, frozenset, list)):
            continue
        words = tuple(str(w) for w in words_attr)
        if not words:
            continue
        result.append((words, tok))
    return result


def detect_postproc_remap(cls) -> list[tuple[list[object], frozenset[str], object]]:
    """Detect ``get_tokens_unprocessed`` overrides that remap token types
    *after* the regex engine runs (e.g. Swift's Cocoa-builtin remapping).

    Returns a list of ``(trigger_tokens, words, target_token)`` entries, or
    ``[]`` if the override is not detected / too complex.

    Each entry means: for any token whose type is in *trigger_tokens* and
    whose string value is in *words*, replace the token type with
    *target_token*.  Entries are ordered highest-priority first.

    Handles two styles of word sources:
    * ``self.ATTR``                   — class-level tuple/frozenset attribute
    * ``from X.Y import A, B, C``    — imports inside the method body
    """
    import inspect
    import re as _re

    if "get_tokens_unprocessed" not in cls.__dict__:
        return []
    try:
        src = inspect.getsource(cls.__dict__["get_tokens_unprocessed"])
    except Exception:
        return []

    # Must have a `token is <type>` guard; if not, too complex.
    if "token is" not in src:
        return []

    # Resolve any `from X.Y import A, B, C` inside the method (handles
    # continuation lines joined with backslash).
    # Join continuation lines first so the names span doesn't stop at $.
    src_joined = src.replace("\\\n", " ")
    import_re = _re.compile(
        r"from\s+([\w.]+)\s+import\s+([\w,\s]+)"
    )
    local_names: dict[str, object] = {}
    for m in import_re.finditer(src_joined):
        module_path = m.group(1).strip()
        names = [n.strip() for n in m.group(2).split(",")]
        try:
            mod = importlib.import_module(module_path)
            for name in names:
                name = name.strip()
                if name:
                    val = getattr(mod, name, None)
                    if val is not None:
                        local_names[name] = val
        except Exception:
            pass

    # Parse the token-remap body: find blocks of the form
    #   if token is A or token is B:
    #       if value in X or value in Y:
    #           token = TARGET
    from pygments import token as T
    _tok_names = {
        "Name": T.Name,
        "Name.Class": T.Name.Class,
        "Name.Builtin": T.Name.Builtin,
        "Name.Builtin.Pseudo": T.Name.Builtin.Pseudo,
        "Name.Constant": T.Name.Constant,
        "Keyword": T.Keyword,
        "Keyword.Declaration": T.Keyword.Declaration,
        "Keyword.Namespace": T.Keyword.Namespace,
        "Operator.Word": T.Operator.Word,
    }

    # Find `if token is X (or token is Y)*:` triggers.
    trigger_re = _re.compile(r"if\s+((?:token\s+is\s+[\w.]+\s*(?:or\s+)?)+):")
    # Find `if value in VAR (or value in VAR2)*:` word-set references.
    varlist_re = _re.compile(r"value\s+in\s+([\w.]+)")
    # Find `token\s*=\s*(\w[\w.]*)` (assignment line, the target type).
    assign_re = _re.compile(r"token\s*=\s*([\w.]+)")

    result: list[tuple[list[object], frozenset[str], object]] = []

    for blk in trigger_re.finditer(src):
        trigger_src = blk.group(1)
        triggers: list[object] = []
        for tok_name in _re.findall(r"token\s+is\s+([\w.]+)", trigger_src):
            if tok_name in _tok_names:
                triggers.append(_tok_names[tok_name])
        if not triggers:
            continue

        # Scan forward from this block for value-in checks and a token= assign.
        rest = src[blk.end():]
        # Only look at the immediately following indented block (≤ ~500 chars).
        snippet = rest[:500]

        vars_found = varlist_re.findall(snippet)
        assign_found = assign_re.search(snippet)
        if not vars_found or not assign_found:
            continue
        target_name = assign_found.group(1).strip()
        target_tok = _tok_names.get(target_name)
        if target_tok is None:
            continue

        # Collect all words from the referenced sets.
        all_words: list[str] = []
        for var in vars_found:
            # Try self.VAR first, then local imports.
            words_col = getattr(cls, var, None) or local_names.get(var)
            if words_col and isinstance(words_col, (tuple, frozenset, list, set)):
                all_words.extend(str(w) for w in words_col)
        if not all_words:
            continue

        result.append((triggers, frozenset(all_words), target_tok))

    return result


def _words_pattern(words: tuple[str, ...]) -> str:
    """Build a word-boundary alternation regex for the given keyword set."""
    alts = "|".join(_re_escape_for_rust(w) for w in sorted(words))
    return f"(?:{alts})(?![a-zA-Z0-9_!?])"


def _re_escape_for_rust(s: str) -> str:
    """Escape special regex chars in a literal string for use in a pattern."""
    import re as _re
    return _re.escape(s)


def rule_expr(pattern: str, action, new_state, flags: int) -> str:
    ns = new_state_expr(new_state)
    has_ns = ns != "NewState::None"

    # default(...) -> zero-width, action is None.
    if action is None:
        return f"Rule::default({ns})"

    full_pat = flag_prefix(flags) + fix_py_regex_for_rust(pattern)
    pat_lit = rust_raw_string(full_pat)

    if is_tokentype(action):
        const = token_expr(action)
        if has_ns:
            return f"Rule::token_to({pat_lit}, {const}, {ns})"
        return f"Rule::token({pat_lit}, {const})"

    # using(this, ...) or using(OtherLexer, ...) at the top-level rule.
    using_info = get_using_info(action)
    if using_info is not None:
        target, stack = using_info
        sv = state_vec_literal(stack)
        if target == "this":
            if has_ns:
                return f"Rule::using_this_to({pat_lit}, {sv}, {ns})"
            return f"Rule::using_this({pat_lit}, {sv})"
        else:
            if has_ns:
                return f'Rule::using_lexer_to({pat_lit}, "{target}", {sv}, {ns})'
            return f'Rule::using_lexer({pat_lit}, "{target}", {sv})'

    args = bygroups_args(action)
    if args is not None:
        # Decide whether any group needs GroupAction (i.e. contains using()).
        has_using = any(
            a is not None and not is_tokentype(a) and get_using_info(a) is not None
            for a in args
        )
        if has_using:
            # Emit bygroups_g / bygroups_g_to with full GroupAction groups.
            groups = [group_action_expr(a) for a in args]
            groups_lit = "vec![" + ", ".join(groups) + "]"
            if has_ns:
                return f"Rule::bygroups_g_to({pat_lit}, {groups_lit}, {ns})"
            return f"Rule::bygroups_g({pat_lit}, {groups_lit})"
        else:
            # Plain token-only bygroups — keep the simple Vec<Option<TokenType>> form.
            groups = []
            for a in args:
                if a is None:
                    groups.append("None")
                elif is_tokentype(a):
                    groups.append(f"Some({token_expr(a)})")
                else:
                    raise NotTranspilable(
                        f"bygroups group is not a token or using(): {a!r}"
                    )
            groups_lit = "vec![" + ", ".join(groups) + "]"
            if has_ns:
                return f"Rule::bygroups_to({pat_lit}, {groups_lit}, {ns})"
            return f"Rule::bygroups({pat_lit}, {groups_lit})"

    raise NotTranspilable(
        f"action {getattr(action, '__qualname__', action)!r} is not "
        f"a token / bygroups / default / using() (arbitrary callback)"
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

    # Detect if the lexer overrides get_tokens_unprocessed with a simple
    # Name→Token remapping driven by class-attribute word-sets (e.g. Elixir).
    # If so, pre-generate keyword rules that fire before each generic NAME rule
    # so the Rust engine matches them correctly without needing the Python hook.
    name_remap = detect_name_remap(cls)
    # Build the keyword-rule expressions to inject (highest priority first).
    # Each entry is (rule_expr_str, target_token_const_str).
    keyword_injections: list[str] = []
    if name_remap:
        for words, tok in name_remap:
            pat = flag_prefix(flags) + _words_pattern(words)
            const = token_expr(tok)
            keyword_injections.append(f"Rule::token({rust_raw_string(pat)}, {const})")

    # Pre-render every rule so we can tell whether `NewState` is used
    # (it isn't for single-state token-only lexers) and avoid emitting
    # an unused import.
    #
    # NAME_CONST is the Rust identifier for Token.Name — used to find
    # where to inject keyword rules.
    from pygments import token as T
    _NAME_CONST = token_expr(T.Name)

    rendered: dict[str, list[str]] = {}
    uses_new_state = False
    uses_group_action = False
    for state, rules in states.items():
        exprs: list[str] = []
        for matchfn, action, new_state in rules:
            pattern = matchfn.__self__.pattern
            expr = rule_expr(pattern, action, new_state, flags)
            if "NewState::" in expr:
                uses_new_state = True
            if "GroupAction::" in expr or "Rule::using_" in expr or "bygroups_g" in expr:
                uses_group_action = True
            # Inject keyword alternatives before the first plain-NAME rule.
            # Only inject once per state (track via a flag).
            if (
                keyword_injections
                and not any(ki in exprs for ki in keyword_injections)
                and f", {_NAME_CONST})" in expr
                and "bygroups" not in expr
            ):
                exprs.extend(keyword_injections)
            exprs.append(expr)
        rendered[state] = exprs

    engine_parts = ["Rule", "StateTable", "tokenize"]
    if uses_new_state:
        engine_parts.insert(0, "NewState")
    if uses_group_action:
        engine_parts.insert(0, "GroupAction")
    engine_imports = ", ".join(sorted(engine_parts))

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

    # Detect post-processing token remaps (e.g. Swift Cocoa builtins).
    postproc = detect_postproc_remap(cls)

    if postproc:
        # Emit a static word-set and a builder per remap entry.
        for idx, (trigger_toks, words, target_tok) in enumerate(postproc):
            set_name = f"REMAP_WORDS_{idx}"
            build_fn = f"build_remap_{idx}"
            # Sorted for deterministic output.
            word_lits = ", ".join(f'"{w}"' for w in sorted(words))
            lines.append(f"static {set_name}: OnceLock<std::collections::HashSet<&'static str>> = OnceLock::new();")
            lines.append(f"fn {build_fn}() -> std::collections::HashSet<&'static str> {{")
            lines.append(f"    [{word_lits}].into_iter().collect()")
            lines.append("}")
            lines.append("")

        # Build trigger constants for comparison.
        def tok_const_list(tok_list):
            return ", ".join(token_expr(t) for t in tok_list)

        lines.append(f"impl Lexer for {struct} {{")
        lines.append("    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {")
        lines.append("        let table = TABLE.get_or_init(build_table);")
        lines.append("        let mut tokens = tokenize(table, code);")
        for idx, (trigger_toks, _words, target_tok) in enumerate(postproc):
            set_name = f"REMAP_WORDS_{idx}"
            build_fn = f"build_remap_{idx}"
            target_const = token_expr(target_tok)
            triggers_check = " || ".join(f"*t == {token_expr(tt)}" for tt in trigger_toks)
            lines.append(f"        let remap_{idx} = {set_name}.get_or_init({build_fn});")
            lines.append(f"        for (t, v) in &mut tokens {{")
            lines.append(f"            if ({triggers_check}) && remap_{idx}.contains(v.as_str()) {{")
            lines.append(f"                *t = {target_const};")
            lines.append(f"            }}")
            lines.append(f"        }}")
        lines.append("        tokens")
        lines.append("    }")
        lines.append("}")
    else:
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

    * ``transpilable``    — ready to generate (token/bygroups/default/using() only)
    * ``bridge_callback`` — arbitrary Python callback action (bridge-only)
    * ``non_regex``       — not a ``RegexLexer`` subclass (bridge-only)
    * ``error``           — failed to import / process_tokendef

    ``using(this, ...)`` and ``using(OtherLexer)`` are now transpilable
    (native recursive / delegating dispatch).  The old ``bridge_using``
    bucket is removed — all such lexers now land in ``transpilable``.
    The only remaining bridge cases are truly arbitrary Python callbacks
    (i.e. free functions or closures that are neither a ``_TokenType``,
    a ``bygroups``, nor a ``using``).
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
            if qn == "using.<locals>.callback":
                # using(this) and using(OtherLexer) are now transpilable.
                continue
            if qn == "bygroups.<locals>.callback":
                if bygroups_args(action) is None:
                    return ("bridge_callback", "bygroups w/ unrecoverable args")
                for a in bygroups_args(action):
                    if a is None or is_tokentype(a):
                        continue
                    if get_using_info(a) is not None:
                        continue  # using(this/Other) inside bygroups: ok
                    return ("bridge_callback", f"bygroups group is callback: {getattr(a, '__qualname__', a)!r:.40}")
                continue
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
