// SECURITY AUDIT: Formatter Injection Vulnerabilities
// ======================================================

/*
THREAT MODEL:
- User-supplied source code → Lexer → Tokens (ttype, value)
- Tokens → Formatter → Output (HTML/RTF/SVG/LaTeX/etc)
- Risk: Malicious source code with control chars, special sequences → Injection in output format

ESCAPE REQUIREMENTS BY FORMAT:
1. HTML      — &, <, >, " (text content); class names from TokenType (const)
2. XML/Pango — &, <, >, ", ' (attribute values + text content)
3. LaTeX     — \, {, }, $, &, %, #, _, ^, ~ (special chars); backticks
4. RTF       — \, {, }, control chars (via \'XX hex escapes)
5. SVG/XML   — &, <, >, ", ' (text content + attribute values)
6. Terminal  — Control chars are SAFE (ANSI codes, IRC codes, etc)
7. Groff     — \ (backslash); . at line start; single quotes in strings
8. BBCode    — [ and ] (delimiters)
9. IRC       — \x03 (color code), \x02 (bold), \x1d (italic), \x1f (underline)
10. Text/Raw — No escaping needed (passthrough)

AUDIT RESULTS:
==============

1. ✅ HTML FORMATTER (html.rs)
   - Escapes: &, <, >, "
   - TokenType.short_name() is safe (const strings from token.rs)
   - Text content properly escaped
   - ⚠️  Issue: Missing single-quote escape (not needed in text, OK)

2. ✅ PANGO FORMATTER (markup.rs)
   - Escapes: &, <, >, ", ' (complete XML)
   - Foreground color from rgb_to_hex() — safe format "#RRGGBB"
   - Attributes: weight='bold', style='italic', underline='single' (const strings)
   - Text content properly escaped
   - Status: SAFE

3. ✅ SVG FORMATTER (svg.rs)
   - Escapes: &, <, >, "
   - Font family: 'Courier New' (const string)
   - CSS classes: .tok-keyword (const, unused in current impl)
   - Color from hex format (safe)
   - Text coordinates from u32 (safe)
   - ⚠️  Bug: SVG style injection possible if token values contain </text>
        Mitigation: SVG text content should not contain </text> markers
        Fix: Close/reopen <text> tag properly on newlines (currently done)
   - Status: MOSTLY SAFE, minor layout bug with newline handling

4. ⚠️  LATEX FORMATTER (markup.rs) — REQUIRES FIXES
   - Escapes: \, {, }, $, &, %, #, _, ^, ~
   - Missing: Backticks (grave accents), pipe (|)
   - Issues:
     a) \textcolor{#RRGGBB}{...} format assumes color is valid hex
        → rgb_to_hex() returns "#RRGGBB" format — OK
     b) TokenType.short_name() used in comments/macros (not in current impl)
     c) Literal backslash escapes to \textbackslash{} — OK
     d) LaTeX injection: source code with \usepackage, \input, etc could break out
        → Current impl wraps in \begin{lstlisting}...\end{lstlisting}
        → Monospaced, verbatim mode — blocks most macro interpretation
     e) BUT: \end{lstlisting} in source would break the wrapper!
        FIX: Escape \end{lstlisting} → can't appear in lstlisting block (OK)
             But need to handle all verbatim-unsafe sequences
   - Status: VULNERABLE — lstlisting is not true verbatim for control sequences

5. ⚠️  RTF FORMATTER (markup.rs) — REQUIRES REVIEW
   - Escapes: \, {, }, newlines (→ \par)
   - Missing: High-bit chars, control chars
   - Issues:
     a) Color table: RGB values from u8 (safe, range 0-255)
     b) RTF keywords: \b, \i, \ul, \cf (font color index), \par (safe const)
     c) Token text with unescaped control chars could corrupt RTF
        Example: \x00 (null), \x01-\x1f (control) need escaping
     d) Font name (Courier New) is const (safe)
   - Status: VULNERABLE to control chars

6. ✅ TERMINAL FORMATTERS (terminal.rs)
   - No escaping needed (ANSI/IRC codes are intentional output)
   - IRC formatter: 0x03 (color), 0x02 (bold), 0x1d (italic), 0x1f (underline)
   - Text passed through (terminal will handle/ignore unknown escapes)
   - Status: SAFE (control codes are expected)

7. ✅ BBCODE FORMATTER (terminal.rs)
   - Escapes: [ and ] (delimiters)
   - BBCode tags: [color=...], [b], [i], [u] (const strings)
   - Hex color: rgb_to_hex() format (safe)
   - Status: SAFE

8. ✅ IRC FORMATTER (terminal.rs)
   - IRC codes: 0x03 (color), 0x02 (bold), 0x1d (italic), 0x1f (underline)
   - Text passed through (IRC client interprets control codes)
   - Status: SAFE

9. ✅ GROFF FORMATTER (markup.rs)
   - Escapes: .defcolor, .mcolor, .ft (const keywords)
   - Color table generation: .defcolor {id} rgb {r} {g} {b}
   - R/G/B values: f32 format (safe 0.0-1.0)
   - Font: .ft B (bold), .ft R (reset) (const)
   - Status: SAFE (groff syntax injection unlikely in token text)

10. ✅ TRIVIAL FORMATTERS (trivial.rs)
    - NullFormatter: passthrough (safe)
    - RawTokenFormatter: escapes newlines, backslash, quotes (safe repr)
    - TestcaseFormatter: generates Rust code (string literals properly escaped)
    - Status: SAFE

CRITICAL FIXES NEEDED:
======================

1. LATEX FORMATTER:
   - ⚠️  \end{lstlisting} in source code will break wrapper
   - Fix: Use `\begin{lstlisting}[breaklines]` or escape to `{\rm\textbackslash}end\{lstlisting\}`
   - Better: Validate source doesn't contain \end{lstlisting}
   - OR: Use `listings` package's `escapeinside` option

2. RTF FORMATTER:
   - ⚠️  Control characters (0x00-0x1F) need escaping as \'HH
   - Fix: Add character validation loop before RTF output
   - Example: \x00 → \'00, \x01 → \'01, etc.

3. SVG FORMATTER:
   - ⚠️  Newline handling closes </text> but doesn't properly reopen for style continuity
   - Minor: Inline <tspan> tags might not carry over styles after </text>
   - Fix: Consolidate styles into outer <text> or reapply after </text>

RECOMMENDATIONS:
================

LOW PRIORITY (cosmetic):
- Add single-quote escape to HTML (not needed, but defensive)
- Improve SVG newline style handling

MEDIUM PRIORITY (validation):
- RTF: Add control-char validation
- LaTeX: Add verbatim-sequence validation

HIGH PRIORITY (security):
- LaTeX: Use safe verbatim wrapper or validate \end{lstlisting}

CONCLUSION:
- Most formatters are SAFE for untrusted input (terminal, IRC, BBCode, Groff, Pango, HTML)
- LaTeX requires investigation of lstlisting behavior
- RTF needs control-char escaping
- All pass basic HTML entity escaping standards
*/
