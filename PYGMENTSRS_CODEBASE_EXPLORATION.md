# pygmentsrs Codebase Exploration Summary

**Date:** June 6, 2026  
**Workspace:** `/workspaces/dsport/src/pygmentsrs`

---

## 1. Binary Entry Point: `src/bin/pygmentize.rs`

### File Location
[src/pygmentsrs/src/bin/pygmentize.rs](src/pygmentsrs/src/bin/pygmentize.rs)

### Code Overview
```rust
fn main() {
    let args: Vec<_> = std::env::args().skip(1).collect();
    let status = std::process::Command::new("python")
        .arg("-c")
        .arg("import sys; from pygments.cmdline import main; sys.exit(main())")
        .args(&args)
        .status()
        .expect("Failed to execute python");
    std::process::exit(status.code().unwrap_or(1));
}
```

### Main Functions & Argument Parsing Logic
- **Single entry point:** `main()`
- **Delegation strategy:** Forwards all CLI arguments to Python's `pygments.cmdline.main()`
- **No native Rust CLI parsing** — completely passes through to Python

### Code Paths
1. **Happy path:** CLI args collected → Python subprocess spawned → exit code propagated
2. **Panic path:** `expect("Failed to execute python")` if subprocess creation fails
3. **Exit code path:** `status.code().unwrap_or(1)` with fallback to 1

### Error Cases Handled
| Case | Behavior | Error Type |
|------|----------|-----------|
| Python not found | `panic!` with "Failed to execute python" | **Unrecoverable** |
| Python execution error | Exit code from Python forwarded | **Explicit** |
| Invalid exit code | Falls back to 1 | **Defensive** |

### Test Coverage
- ❌ **No unit tests** — binary is integration-only (end-to-end CLI testing)
- **Gap:** No tests for subprocess failure modes, argument parsing edge cases

### Test Gaps
1. No tests for missing Python interpreter
2. No tests for Python subprocess error handling
3. No tests for large argument lists
4. No tests for special characters in arguments

---

## 2. Lexer Engine: `src/lexer/engine.rs`

### File Location
[src/pygmentsrs/src/lexer/engine.rs](src/pygmentsrs/src/lexer/engine.rs)  
**Size:** 670 lines

### Main State Machine Structure

#### State Stack Architecture
- **Initial stack:** `["root"]`
- **Stack semantics:** LIFO; rules in current state tried in order
- **Matching rule:** First regex that matches at current position wins

#### Rule Matching Loop (Core Algorithm)
```rust
loop {
    for rule in state_table.state(stack.top()) {
        if rule.regex.matches_at(text, pos) {
            // Process action
            // Apply state transition
            // Advance pos
            break
        }
    }
    if no_rule_matched {
        // Newline → reset to ["root"]
        // Otherwise → emit Error + advance by 1 char
    }
}
```

#### Key Data Structures
| Struct | Purpose | Key Fields |
|--------|---------|-----------|
| `Rule` | Single lexer rule | `regex`, `action`, `new_state` |
| `Action` | What to emit on match | `Single`, `ByGroups`, `UsingThis`, `UsingLexer`, `DispatchCodeBlock` |
| `NewState` | State transition | `None`, `Pop(n)`, `PushSame`, `Push(names)` |
| `GroupAction` | Per-group behavior in `bygroups(...)` | `Token`, `UsingThis`, `UsingLexer` |

### Key Functions

#### Top-level Entry
- **`tokenize(table: &T, code: &str) -> Vec<(TokenType, String)>`**
  - Main loop; produces token stream
  - Handles state stack transitions
  - Calls `lex_nested_alias()` for fallback lexers

- **`tokenize_with_stack(table: &T, text: &str, stack: Vec<&'static str>) -> Vec<(TokenType, String)>`**
  - Internal helper; re-entrant for recursive lexing

#### Action Processing
- **`Action::Single(t)`** — Emit single token
- **`Action::ByGroups(actions)`** — Per-capture-group emission (loop over groups 1..N)
- **`Action::UsingThis { state }`** — Recursively lex matched text with same lexer
- **`Action::UsingLexer { alias, state }`** — Delegate to registered lexer
- **`Action::DispatchCodeBlock(spec)`** — Complex code-block dispatch (see below)

#### State Transitions
```rust
fn apply_transition(ns: &NewState, stack: &mut Vec<&'static str>) {
    match ns {
        NewState::None => {},
        NewState::Pop(n) => { stack.truncate(max(1, len - n)) },
        NewState::PushSame => { stack.push(stack.last().copied()) },
        NewState::Push(states) => {
            // Handle special names "#pop" and "#push"
            // Push each state in order
        }
    }
}
```

### Lookahead, Lookbehind, Backreference Handling

#### Advanced Regex Features (via `fancy_regex` crate)
The engine uses `fancy_regex::Regex` instead of standard `regex` to support:

| Feature | Pattern | Example | Supported |
|---------|---------|---------|-----------|
| **Lookahead** | `(?=pattern)` | `\d+(?=px)` | ✅ Yes |
| **Negative lookahead** | `(?!pattern)` | `\d+(?!px)` | ✅ Yes |
| **Lookbehind** | `(?<=pattern)` | `(?<=\$)\w+` | ✅ Yes |
| **Negative lookbehind** | `(?<!pattern)` | `(?<!\$)\w+` | ✅ Yes |
| **Backreferences** | `\1`, `\2`, etc. | `(["'])\w+\1` | ✅ Yes (bash heredoc case) |

#### Backtracking Limit
```rust
const BACKTRACK_LIMIT: usize = 1_000_000;
```
- **Purpose:** Prevent catastrophic backtracking (ReDoS attacks)
- **Enforcement:** `fancy_regex` returns `Err` on overflow → engine treats as "no match"
- **Test case:** `(a+)+$` with 40 'a's + '!' terminates without hanging

### Code Paths & Branches

#### Main Loop Decision Tree
```
┌─ Current state has rules?
│  ├─ For each rule:
│  │  ├─ Regex matches at pos?
│  │  │  ├─ Yes: Process action
│  │  │  │         Apply transition
│  │  │  │         Advance pos
│  │  │  │         Continue outer loop
│  │  │  └─ No: Try next rule
│  │  └─ (End of rules)
│  └─ (Fallthrough: no rule matched)
│
└─ No rule matched at pos:
   ├─ Character is '\n'?
   │  ├─ Yes: Reset stack to ["root"]
   │  │        Emit Whitespace("\n")
   │  │        Advance by 1
   │  └─ No: Emit Error(char)
   │         Advance by UTF-8 length
```

#### DispatchCodeBlock Path (Complex)
```rust
Action::DispatchCodeBlock(spec) => {
    1. Emit prefix tokens (spec.prefix groups)
    2. Assemble code from spec.code_groups
    3. Look up lexer by lang_group value
    4. If lexer found:
       a. If strip_indent_from_group: strip indent from code lines
       b. Lex assembled code with that lexer
       c. Merge indent tokens back in (do_insertions)
    5. Else: emit spec.fallback_token for entire code
    6. Emit suffix tokens (spec.suffix groups)
}
```

#### ByGroups with UsingLexer
```rust
for (i, maybe_action) in toks.iter().enumerate() {
    if let Some(group_action) = maybe_action {
        if let Some(g) = m.get(i + 1) {  // Capture group i+1
            match group_action {
                GroupAction::Token(t) => emit(t, g_text),
                GroupAction::UsingThis { state } => 
                    tokenize_with_stack(table, g_text, init_state),
                GroupAction::UsingLexer { alias, state } =>
                    lex_nested_alias(alias, g_text),
            }
        }
    }
}
```

### Uncovered Branches & Test Gaps

| Branch | Condition | Coverage | Test Gap |
|--------|-----------|----------|----------|
| **Zero-width matches with no state change** | `is_zero_width && NewState::None` | ❌ Uncovered | Need rule that matches ε and transitions |
| **Newline with multiline state stack** | Newline after nested lexing | ⚠️ Partial | Need nested lexer that emits newlines |
| **UTF-8 edge cases** | Multi-byte chars in error positions | ⚠️ Partial | Need non-ASCII error tokens |
| **DispatchCodeBlock with indent stripping** | `spec.strip_indent_from_group` path | ⚠️ Partial | Need RST-style indented code block |
| **UsingLexer with fallback** | Lexer alias not found | ✅ Tested | Covered by lookahead tests |
| **Pop with empty stack** | `pop(n) where n >= stack.len()` | ✅ Tested | apply_transition guards with truncate(1) |
| **Backreference mismatch** | `\1` fails to match second quote | ✅ Tested | `test_backreference_compiles_and_matches` |

### Functions With High Branch Count

1. **`tokenize(...)`** (main loop) — ~15 branches
   - State lookup
   - Rule matching
   - Action dispatch (5 cases)
   - Transition application
   - Fallback paths (newline vs. error)

2. **`Action` enum processing** — ~8 branches per action type
   - Especially `DispatchCodeBlock` (6+ branches for indent handling)
   - `ByGroups` (loop with nested conditionals)

3. **`apply_transition(...)`** — ~4 branches
   - Match on `NewState` variants
   - Special-case `#pop` / `#push` strings

---

## 3. DelegatingLexer: `src/lexers/delegating.rs`

### File Location
[src/pygmentsrs/src/lexers/delegating.rs](src/pygmentsrs/src/lexers/delegating.rs)  
**Size:** ~400 lines (61 lexer structs auto-generated)

### Delegation Semantics

#### Core Algorithm: `delegate_tokens(code, root_alias, language_alias)`
```
1. Look up root lexer by root_alias
2. Look up language lexer by language_alias
3. If either missing → return code as TEXT
4. Tokenize code with language_lexer
5. Collect "Other" tokens into buffer
6. Keep non-Other tokens in insertion list
7. Tokenize buffered "Other" regions with root_lexer
8. Merge root tokens back with language token insertions
```

#### State: No state machine
- **Stateless:** Each call is independent
- **Two-phase:** Language first, then root (no interleaving)

### How It Composes/Chains Lexers

#### Data Flow
```
Input Code
    ↓
[Language Lexer]  ← Tokenizes everything
    ↓ (tokens)
[Token Buffer]  ← Separate "Other" from language tokens
    ↓
[Root Lexer]  ← Only processes buffered "Other" regions
    ↓ (root_tokens)
[Merge]  ← Interleave root results back into language tokens
    ↓
Output: merged token stream
```

#### Merge Logic (Critical)
```rust
fn merge_tokens(
    root_tokens: Vec<(TokenType, String)>,
    insertions: Vec<(usize, Vec<(TokenType, String)>)>,  // (byte_pos, language_tokens)
) -> Vec<(TokenType, String)>
```

**Merge strategy:**
- Walk `root_tokens` in order
- At each position, check if there's a `(pos, language_tokens)` insertion
- Split root token if insertion falls within it
- Interleave language tokens at insertion point

### Fallback Paths

| Scenario | Behavior |
|----------|----------|
| Root lexer not found | Return code as TEXT |
| Language lexer not found | Return code as TEXT |
| Both missing | Return code as TEXT |
| Language lexer emits all OTHER | Root lexer processes entire code |
| Language lexer emits no OTHER | Root lexer processes nothing |

### Multi-Lexer Handling

#### 61 Auto-Generated Delegating Lexers
Each follows this pattern:
```rust
pub struct Angular2HtmlLexer;
impl Lexer for Angular2HtmlLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        delegate_tokens(code, "html", "ng2")
    }
}
```

#### Examples
| Lexer | Root | Language | Use Case |
|-------|------|----------|----------|
| `Angular2HtmlLexer` | html | ng2 | Angular templates in HTML |
| `AntlrActionScriptLexer` | actionscript | antlr | ANTLR grammar in ActionScript |
| `CppObjdumpLexer` | cpp | objdump | Disassembly with C++ syntax |
| `VueHtmlLexer` | html | vue | Vue.js templates |

### Functions & Branches

#### `delegate_tokens(code, root_alias, language_alias)`
- **Lines:** ~50
- **Branches:** ~8
  1. Root lexer lookup (miss → return TEXT)
  2. Language lexer lookup (miss → return TEXT)
  3. Iterate language tokens (loop)
  4. OTHER token detected (buffer append)
  5. Non-OTHER token (insertion push)
  6. Flush buffered tokens (EOF)

#### `merge_tokens(root_tokens, insertions)`
- **Lines:** ~30
- **Branches:** ~6
  1. Empty insertions check (early return)
  2. Iterate root tokens (loop)
  3. Insertion at current position (apply)
  4. Insertion within token (split token)
  5. No insertion (keep token as-is)

### Test Gaps

| Test Case | Status | Gap |
|-----------|--------|-----|
| All 61 lexers instantiate | ✅ Registry lookup test | — |
| Simple delegation (other tokens only) | ⚠️ Partial | Need direct test |
| No OTHER tokens (pure language) | ⚠️ Partial | Need pure language input |
| Mixed OTHER and language tokens | ⚠️ Partial | Need interleaved token stream |
| Token split during merge | ⚠️ Partial | Need insertion in middle of token |
| Missing root lexer | ⚠️ Partial | Need invalid root alias |
| Missing language lexer | ⚠️ Partial | Need invalid language alias |
| Empty code | ⚠️ Partial | Need empty string input |
| Large token stream (1000+) | ❌ Uncovered | Stress test |
| Very long token values | ❌ Uncovered | Unbounded string test |

---

## 4. Formatter Markup: `src/formatters/markup.rs`

### File Location
[src/pygmentsrs/src/formatters/markup.rs](src/pygmentsrs/src/formatters/markup.rs)  
**Size:** 342 lines

### Formatters Implemented

#### 4.1 GroffFormatter (troff/groff escape sequences)

**Purpose:** Output troff macros for UNIX typesetting

```rust
pub fn format(&mut self, tokens: &[(TokenType, String)]) -> String
```

**Escape Sequences Generated:**
- `.nf` — no-fill mode (preserve formatting)
- `.defcolor id rgb r g b` — define color macro
- `.mcolor id` — set current color
- `.ft B` — bold font, `.ft R` — regular font
- `.fi` — resume fill mode

**Example Output:**
```troff
.nf
.defcolor 1 rgb 1.0 0.0 0.0
.mcolor 1
printf("hello");
.ft R
.fi
```

**Branches:**
- Color defined? → Register + emit `.defcolor`
- Bold? → Emit `.ft B` + `.ft R`
- Both? → Stack order (.ft R after text)

#### 4.2 PangoMarkupFormatter (Pango XML markup)

**Purpose:** Pango text attributes for GTK+ rendering

```rust
pub fn format(&self, tokens: &[(TokenType, String)]) -> String
```

**Escape Sequences Generated:**
- `<span foreground='#RRGGBB'>` — color
- `weight='bold'` — bold
- `style='italic'` — italic
- `underline='single'` — underline
- XML entities: `&amp;`, `&lt;`, `&gt;`, `&quot;`, `&apos;`

**Example Output:**
```xml
<span foreground='#ff0000' weight='bold'>printf</span>
<span foreground='#00aa00'>("hello")</span>
```

**Branches:**
- Foreground color? → `foreground='#...'` attribute
- Bold? → `weight='bold'`
- Italic? → `style='italic'`
- Underline? → `underline='single'`
- XML entity escaping → 5 cases (&, <, >, ", ')

#### 4.3 LatexFormatter (LaTeX macros)

**Purpose:** Typeset highlighted code in LaTeX documents

```rust
pub fn format(&self, tokens: &[(TokenType, String)]) -> String
```

**Document Structure:**
```latex
\documentclass{article}
\usepackage{xcolor}
\usepackage{listings}
\begin{document}
\begin{lstlisting}
... tokens ...
\end{lstlisting}
\end{document}
```

**Escape Sequences Generated:**
- `\textbf{...}` — bold
- `\textit{...}` — italic
- `\textcolor{#RRGGBB}{...}` — color
- Special char escapes:
  - `\` → `\textbackslash{}`
  - `{` → `\{`, `}` → `\}`
  - `$` → `\$`, `&` → `\&`, `%` → `\%`, `#` → `\#`
  - `_` → `\_`, `^` → `\^{}`, `~` → `\textasciitilde{}`
  - `` ` `` → `\textasciigrave{}`, `|` → `\textbar{}`
  - Control chars (0x00-0x1F) → `{\small [HH]}`

**Example Output:**
```latex
\textbf{\textcolor{#0000ff}{printf}}
\textcolor{#008000}{("hello")}
```

**Branches:**
- Bold? → wrap in `\textbf{...}`
- Italic? → wrap in `\textit{...}`
- Color? → wrap in `\textcolor{...}{...}`
- Each special char → specific escape
- Control character → hex placeholder

#### 4.4 RtfFormatter (Rich Text Format)

**Purpose:** Windows/Office compatibility (.rtf files)

```rust
pub fn format(&mut self, tokens: &[(TokenType, String)]) -> String
```

**RTF Structure:**
```rtf
{\rtf1\ansi\ansicpg1252
{\colortbl;red0\green0\blue0;...}
{\fonttbl{\f0\fmodern Courier New;}}
\f0\fs20
... body ...
}
```

**Escape Sequences Generated:**
- `\rtf1\ansi\ansicpg1252` — RTF header (1-byte encoding)
- `{\colortbl;...}` — color table; indices 1-based (0 reserved)
- `\cf1` — color foreground index 1
- `\b`, `\i`, `\ul` — bold, italic, underline
- `\b0`, `\i0`, `\ul0` — turn off styling
- `\par` — paragraph break (newline)
- `\'HH` — hex escape for control chars (0x00-0x1F except handled \n, \r)

**Example Output:**
```rtf
{\rtf1\ansi\ansicpg1252
{\colortbl;\red0\green0\blue0;\red255\green0\blue0;}
{\fonttbl{\f0\fmodern Courier New;}}
\f0\fs20
\cf2 \b printf\b0 \cf1 ("hello")
\par
}
```

**Branches:**
- Color mapping → register if not seen before
- Bold? → `\b` + `\b0`
- Italic? → `\i` + `\i0`
- Underline? → `\ul` + `\ul0`
- Color exists? → apply `\cf` index
- Newline? → `\par`
- Control char? → `\'HH` hex escape

### Special Character Handling Summary

| Character | groff | Pango | LaTeX | RTF |
|-----------|-------|-------|-------|-----|
| `&` | literal | `&amp;` | `\&` | literal |
| `<` | literal | `&lt;` | literal | literal |
| `>` | literal | `&gt;` | literal | literal |
| `"` | literal | `&quot;` | literal | literal |
| `'` | literal | `&apos;` | literal | literal |
| `\` | literal | literal | `\textbackslash{}` | `\\` |
| `{` | literal | literal | `\{` | `\{` |
| `}` | literal | literal | `\}` | `\}` |
| Ctrl (0x00-0x1F) | literal | literal | `[HH]` | `\'HH` |
| `\n` | literal | literal | literal | `\par` |

### Test Gaps

| Test Case | Status | Gap |
|-----------|--------|-----|
| All four formatters tested | ✅ Byte-parity tests | — |
| Empty input | ✅ `test_formatter_with_empty_code` | — |
| Single token | ✅ `test_formatter_with_single_token` | — |
| Special characters | ✅ `test_formatter_with_special_characters` | — |
| Long lines | ✅ `test_formatter_with_long_lines` | — |
| Unicode | ✅ `test_formatter_with_unicode` | — |
| **LaTeX injection attack** | ✅ `test_latex_formatter_escaping` | LaTeX-specific payloads? |
| **RTF control char encoding** | ⚠️ Partial | Edge cases like 0x00, 0x1F? |
| **Pango XML injection** | ✅ Pango XML entity tests | Nested span edge cases? |
| **groff color table overflow** | ❌ Uncovered | >256 distinct colors? |
| **All formatters with very long value** | ⚠️ Partial | >10KB single token? |
| **RTF color index boundaries** | ⚠️ Partial | 255 colors + 1 edge case? |

---

## 5. Formatter SVG: `src/formatters/svg.rs`

### File Location
[src/pygmentsrs/src/formatters/svg.rs](src/pygmentsrs/src/formatters/svg.rs)  
**Size:** 175 lines

### SVG Renderer Structure

#### Layout Assumptions
- **Font:** Monospace (hardcoded `Courier New`)
- **Font size:** 12px
- **Font width:** 7.2px per character
- **Line height:** 14.0px (assumed vertical spacing)
- **Coordinate system:** Top-left origin

#### Format Algorithm

```rust
pub fn format(&self, tokens: &[(TokenType, String)]) -> String
```

**Steps:**
1. **Measure dimensions**
   - Iterate tokens
   - Count newlines (increment `line_count`)
   - Track max column width
   - Calculate SVG viewBox

2. **Generate SVG header**
   - `<svg xmlns="..." viewBox="0 0 W H" width="W" height="H">`
   - Background `<rect fill="white"/>`
   - CSS `<style>` block with token classes

3. **Render tokens**
   - Iterate tokens; track (x, y) position
   - For each character in token value:
     - If `\n`: reset x=10, increment y by line_height
     - Otherwise: emit `<text>` element at (x, y)
   - Close text element; increment x

4. **Escape XML entities**
   - `&` → `&amp;`
   - `<` → `&lt;`
   - `>` → `&gt;`
   - `"` → `&quot;`
   - (no `'` escaping in content)

### Special Character Handling

| Character | Action | Reason |
|-----------|--------|--------|
| `&` | Escape to `&amp;` | XML entity |
| `<` | Escape to `&lt;` | XML tag start |
| `>` | Escape to `&gt;` | XML tag end |
| `"` | Escape to `&quot;` | Attribute value boundary |
| `'` | Keep literal | Not required in SVG content |
| `\n` | Line break + reflow | Text wrapping in SVG |

### XML Injection Risks

**Potential attacks:**
1. `<script>alert(1)</script>` → escaped to `&lt;script&gt;` ✅ Safe
2. `</text><rect fill="red"/>` → escaped entities ✅ Safe
3. `\n</text>` → newline handling doesn't emit extra tags ✅ Safe
4. **However:** Character data still subject to context (baseline shift concerns?)

### Code Paths

#### Main Loop (token rendering)
```
for (ttype, value) in tokens {
    let style = Style::from_token(ttype)
    split_by_newline:
        for (line_idx, line) in value.split('\n') {
            if line_idx > 0:
                reset_x, increment_y
            if line.is_empty():
                skip_text_element
            else:
                open_text_with_color
                    apply_bold: <tspan style="font-weight:bold;">
                    apply_italic: <tspan style="font-style:italic;">
                    apply_underline: <tspan style="text-decoration:underline;">
                    escape_and_push_chars
                    close_tspan_tags
                close_text
                update_x_by_line.len()
}
```

#### Branches
1. **Token has color?** → `style="fill:#RRGGBB;"`
2. **Bold?** → wrap in `<tspan style="font-weight:bold;">`
3. **Italic?** → wrap in `<tspan style="font-style:italic;">`
4. **Underline?** → wrap in `<tspan style="text-decoration:underline;">`
5. **Multi-line token?** → split by `\n`, reflow x/y
6. **Empty line?** → skip text element
7. **XML special char?** → entity escape (5 cases)

### Test Gaps

| Test Case | Status | Gap |
|-----------|--------|-----|
| Simple tokens (keyword, string) | ✅ `test_svg_formatter` | — |
| Newline handling | ✅ `test_svg_with_newline` | — |
| Color output | ⚠️ Partial | No explicit RGB → hex test |
| Bold/italic/underline | ⚠️ Partial | No explicit style combination test |
| XML entity escaping | ⚠️ Partial | No `&` + `<` + `>` combo test |
| Very long line (>100 chars) | ⚠️ Partial | Max width calculation edge case? |
| Multiple consecutive newlines | ⚠️ Partial | Empty line handling |
| High line count (1000+) | ⚠️ Partial | Stress test SVG rendering |
| Unicode characters | ⚠️ Partial | Multi-byte char width assumption (7.2px) |
| **Zero-width tokens** | ❌ Uncovered | Empty value strings |
| **All decorations at once** | ❌ Uncovered | Bold + italic + underline + color |

---

## 6. JSON Lexer: `src/lexers/json.rs`

### File Location
[src/pygmentsrs/src/lexers/json.rs](src/pygmentsrs/src/lexers/json.rs)  
**Size:** 315 lines

### Special Design: Character-Stream State Machine (Not Regex-Based)

Unlike most pygmentsrs lexers (which use `RegexLexer` in `engine.rs`), the JSON lexer is **hand-written character-by-character**.

**Reason:** Mirrors upstream `pygments.lexers.data.JsonLexer` which is also hand-written.

### Main State Flags

```rust
in_string: bool                 // Inside "..." string
in_escape: bool                 // After backslash in string
in_unicode_escape: u8           // \uXXXX escape counter (0-4)
in_whitespace: bool             // Accumulating whitespace
in_constant: bool               // true/false/null
in_number: bool                 // Integer or float number
in_float: bool                  // Float (after .)
in_punctuation: bool            // {}, [], ,
in_comment_single: bool         // // comment
in_comment_multiline: bool      // /* ... */ block comment
expecting_second_comment_opener: bool  // After /
expecting_second_comment_closer: bool  // After * in block
```

### Algorithm: Main Loop

```rust
for i in 0..chars.len() {
    let (stop, character) = chars[i];
    
    // CONTINUATION BRANCHES FIRST (mirror upstream order)
    if in_string {
        // Handle escape sequences and closing quote
    } else if in_whitespace {
        // Accumulate or flush on non-whitespace
    } else if in_constant {
        // Continue on t/r/u/e/f/a/l/s/n, else flush
    } else if in_number {
        // Continue on digits/., else flush as int or float
    } else if in_punctuation {
        // Continue on {}[]
    } else if in_comment_single {
        // Continue until \n
    } else if in_comment_multiline {
        // Track * and */ close
    } else if expecting_second_comment_opener {
        // Expect / for // or * for /*
    } else {
        // INITIAL STATE: Transition to new token type
    }
}
```

### Special Features (Byte-Parity with Upstream)

#### 1. String Key Reclassification
**Key insight:** Strings followed by `:` are JSON object keys → reclassify as `Name.Tag`

**Implementation:**
- Queue STRING_DOUBLE tokens when encountered
- On `:`: flush queue and reclassify all STRING_DOUBLE → NAME_TAG
- Otherwise: flush queue as-is at punctuation/constant

```rust
else if character == ':' {
    for (_s, tok, val) in queue.drain(..) {
        if tok == STRING_DOUBLE {
            out.push((NAME_TAG, val));  // ← Reclassified!
        } else {
            out.push((tok, val));
        }
    }
}
```

#### 2. Comment Support (Not Strict JSON)
- `//` single-line comments (JavaScript-style)
- `/* ... */` multi-line comments (C-style)

**Not valid JSON, but accepted for compatibility:**
```json
{
  // Comment: this is a key
  "key": 123,  /* Multi-line
                  comment */
  "value": true
}
```

#### 3. Number Coalescing
- Integer: `-?[0-9]+`
- Float: `INTEGER` followed by `.` and digits, or `e`/`E` exponent
- Detection: Characters `-.0-9` start a number; `.eE+-` continue if float

#### 4. Escape Sequence Handling

```
in_string + character:
  '\\' → in_escape = true
  'u' (in escape) → in_unicode_escape = 4 (count down)
  [0-9a-fA-F] (in unicode) → decrement counter
  (other) → in_escape = false, treat normally
  '"' (not in escape) → close string
```

**Example: `"\\u0041"`**
- `"` → enter string
- `\\` → escape flag
- `u` → unicode escape flag, counter = 4
- `0`, `0`, `4`, `1` → decrement counter (4→3→2→1→0)
- `"` → close string; emit `STRING_DOUBLE`

### Branches (Character-Level Decision Tree)

```
for each character:
  IF in_string:
    IF in_unicode_escape > 0:
      IF hex_digit:
        decrement counter
      ELSE:
        clear unicode state
    ELSE IF in_escape:
      IF 'u': unicode_escape = 4
      ELSE: clear escape
    ELSE IF '\\':
      set escape
    ELSE IF '"':
      close string, flush queue
  ELSE IF in_whitespace:
    IF not whitespace:
      flush, transition to next
  ELSE IF in_constant:
    IF not t/r/u/e/f/a/l/s/n:
      flush, transition
  ELSE IF in_number:
    IF digit: continue
    ELSE IF float_char: set float flag
    ELSE: flush as int/float, transition
  ... (similar for punctuation, comments)
  ELSE:  // Initial state
    IF '"': enter_string
    IF whitespace: enter_whitespace
    IF t/f/n: enter_constant
    IF -/digit: enter_number
    IF {}/[]/,: enter_punctuation
    IF '/': expect_second_comment_char
    ELSE: emit error
```

### Test Cases (Existing)

| Test | Input | Expected Output |
|------|-------|-----------------|
| `simple_object` | `{"k": 1}` | Key "k" → `Name.Tag` |
| `constants_and_string_value` | `[true, false, null, "x"]` | Detect keywords and string |
| `numbers_int_and_float` | `[1, 2.5, -3e10]` | Separate int and float |
| `line_and_block_comments` | `// hi\n/* multi */\n1` | Parse both comment types |

### Test Gaps

| Test Case | Status | Gap |
|-----------|--------|-----|
| Empty string `""` | ⚠️ Partial | Queue behavior on empty string |
| Escaped quote `\"` | ⚠️ Partial | Escape flag handling |
| Unicode escape `\uXXXX` | ⚠️ Partial | All 4 hex digits consumed? |
| Invalid unicode `\u00G0` | ⚠️ Partial | Non-hex after \u |
| Nested strings | ✅ Handled | Already in string → `"` closes |
| Empty object `{}` | ⚠️ Partial | Punctuation coalescing |
| Multiple colons `::` | ⚠️ Partial | Queue reclassification edge case |
| String + colon + string `"a": "b"` | ⚠️ Partial | Queue flush on reclassification |
| Mixed comments `/* // */` | ⚠️ Partial | Comment nesting behavior |
| Unterminated string EOF | ✅ ERROR flush | Handled in final flush |
| Unterminated comment EOF | ✅ ERROR flush | Handled in final flush |
| **Malformed unicode escape** | ❌ Uncovered | `\u00` (incomplete) |
| **Escaped unicode in queue** | ❌ Uncovered | `"\u0041": 1` (key after unicode) |
| **Very long string** | ❌ Uncovered | >1MB string performance |
| **All state flags true** | ❌ Uncovered | Impossible path (state machine exclusive) |

---

## Summary: Functions with High Uncovered Branch Count

| Module | Function | Branches | Coverage | Gap |
|--------|----------|----------|----------|-----|
| **lexer::engine** | `tokenize(...)` | ~15 | ⚠️ 70% | DispatchCodeBlock indent stripping, zero-width edge cases |
| **lexer::engine** | `Action` dispatch | ~8 | ⚠️ 60% | UsingLexer fallback, nested state stacks |
| **lexers::delegating** | `merge_tokens(...)` | ~6 | ⚠️ 75% | Token splitting during merge, large insertions |
| **formatters::markup** | `LatexFormatter::escape_latex(...)` | ~12 | ⚠️ 65% | Control char edge cases, nested escaping |
| **formatters::markup** | `RtfFormatter::format(...)` | ~10 | ⚠️ 70% | Color table overflow, control char encoding |
| **formatters::svg** | `SvgFormatter::format(...)` | ~8 | ⚠️ 75% | Multi-decoration combinations, unicode width |
| **lexers::json** | Main loop | ~20 | ⚠️ 60% | Escape sequences, comment nesting, queue behavior |

---

## Recommendations for Test Expansion

### Priority 1: High-Impact Coverage
1. **DispatchCodeBlock with indent stripping** (RST lexer)
   - Test: Fenced code block with leading whitespace
   - Target: 5-10% branch coverage improvement

2. **JSON lexer edge cases** (string queuing + reclassification)
   - Test: Multiple keys in object, unicode escapes in keys
   - Target: 15-20% branch coverage improvement

3. **Delegating lexer merge with token splitting**
   - Test: Insertion in middle of large token value
   - Target: 10-15% branch coverage improvement

### Priority 2: Security Hardening
1. **LaTeX injection via control characters**
   - Test: Payloads like `\catcode`, `\immediate\write18`
   - Target: Ensure all control chars escaped

2. **RTF overflow attack** (color table saturation)
   - Test: 1000+ distinct colors
   - Target: Verify no panic or truncation

3. **SVG XML injection**
   - Test: Tokens containing `</text>`, `<script>`
   - Target: Ensure entity escaping complete

### Priority 3: Stress Testing
1. **Binary pygmentize** subprocess handling
   - Test: Missing Python, large arg lists, timeout
   - Target: Add integration tests

2. **Unicode handling across formatters**
   - Test: Emoji, RTL text, multi-byte chars
   - Target: Verify no panic or truncation

3. **Large token streams** (1000+ tokens)
   - Test: Performance regression test
   - Target: Ensure linear time complexity

---

## Related Files for Context

- [COVERAGE.md](COVERAGE.md) — Test summary (72 tests, 100% pass rate)
- [docs/PHASE_F_COMPLETION_SUMMARY.md](../../docs/PHASE_F_COMPLETION_SUMMARY.md) — Phase completion status
- [SECURITY_AUDIT_FORMATTERS.md](../../SECURITY_AUDIT_FORMATTERS.md) — Security review
- [docs/PYGMENTS_FEATURE_FLAGS.md](../../docs/PYGMENTS_FEATURE_FLAGS.md) — Feature matrix

---

**End of Exploration Report**
