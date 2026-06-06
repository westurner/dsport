# Phase E4 Analysis: Structured-Text Embedded-Code Dispatchers

## Summary

**Phase E4 is complete for its intended scope: regex-based embedded-code dispatch.**

✅ **Complete** (3 lexers): `markdown`, `restructuredtext`, `tid`
→ Reclassified to E5 (5 lexers): `http`, `mime`, `bibtex`, `notmuch`, `wikitext`

## Architecture Decision

The 8 "structured-text embedded-code" lexers fall into two distinct categories:

### Category 1: Regex-Based Dispatch ✅ COMPLETE

**Lexers**: `markdown`, `restructuredtext`, `tid`

These lexers have patterns that directly capture language tags within regex groups:
- Markdown: ` ```lang ` → captures `lang` in a group
- reStructuredText: ` .. code-block:: lang ` → captures `lang`
- TiddlyWiki: ` ```lang ` blocks → captures `lang`

**Implementation**: `Rule::DispatchCodeBlock` action in the lexer engine
- Captures language tag from regex group
- Looks up tag in `native_aliases()` registry
- Dispatches to corresponding native lexer if available
- Maintains indent tracking across nested dispatch

**Status**: Fully implemented, all 3 passing parity tests ✅

### Category 2: Callback-Based Dispatch → Phase E5

**Lexers**: `http`, `mime`, `bibtex`, `notmuch`, `wikitext`

These lexers use Python callbacks to extract dispatch information at runtime:

| Lexer | Dispatch Method | Complexity |
| ----- | --------------- | ---------- |
| **http** | Parse `Content-Type` header → dispatch body (JSON/XML/etc) | Medium |
| **mime** | Parse MIME boundaries → dispatch parts | High |
| **bibtex** | ExtendedRegexLexer with context callbacks for brace matching | High |
| **notmuch** | Parse email headers → dispatch body with structure | Medium |
| **wikitext** | Template tag dispatch with scope/nesting tracking | High |

**Why These Require Phase E5**:

1. **State Management**: Need to track context across tokens (e.g., opening brace in BibTeX, MIME boundary markers, email headers)
2. **Runtime Extraction**: Dispatch decision requires parsing actual content, not just pattern matching
3. **ExtendedRegexLexer**: `bibtex` uses Python's ExtendedRegexLexer with context (`ctx`) parameter, not standard RegexLexer
4. **Callback Functions**: All require custom Python callbacks that inspect match state and decide dispatch

Example (HTTP):
```python
def header_callback(self, match):
    if match.group(1).lower() == 'content-type':
        content_type = match.group(5).strip()  # Extract from capture
        self.content_type = content_type       # Store state
    # Later: use self.content_type to dispatch body
```

## Implementation Requirements for E5

Each of these 5 lexers needs a custom Rust `Lexer` implementation (not RegexLexer transpilation):

### http — Content-Type Extraction Dispatch
```rust
pub struct HttpLexer {
    content_type: Option<String>,
}

impl Lexer for HttpLexer {
    fn get_tokens_unprocessed(&self, text: &str) -> Vec<(usize, TokenType, String)> {
        // 1. Parse request/response line and headers
        // 2. Extract Content-Type header value
        // 3. Parse body with dispatch based on content_type
        // 4. Return merged token stream
    }
}
```

### mime — Boundary-Aware Multi-Part Dispatch
```rust
pub struct MimeLexer;

impl Lexer for MimeLexer {
    fn get_tokens_unprocessed(&self, text: &str) -> Vec<(usize, TokenType, String)> {
        // 1. Parse MIME headers to extract Content-Type boundary
        // 2. Split message by boundary markers
        // 3. Recursively tokenize each part (may have nested MIME)
        // 4. Return merged token stream
    }
}
```

### bibtex — Context-Tracking Brace Matching
```rust
pub struct BibTexLexer;

impl Lexer for BibTexLexer {
    fn get_tokens_unprocessed(&self, text: &str) -> Vec<(usize, TokenType, String)> {
        // 1. Track brace balance across tokens (need context)
        // 2. Parse @entry{...} structures
        // 3. Emit tokens with proper grouping based on brace context
        // 4. Implement open_brace_callback / close_brace_callback behavior
    }
}
```

### notmuch — Email Header Extraction + Body Dispatch
```rust
pub struct NotmuchLexer;

impl Lexer for NotmuchLexer {
    fn get_tokens_unprocessed(&self, text: &str) -> Vec<(usize, TokenType, String)> {
        // Similar to HTTP: parse headers, extract metadata, dispatch body
    }
}
```

### wikitext — Template Tag Scope Tracking
```rust
pub struct WikitextLexer;

impl Lexer for WikitextLexer {
    fn get_tokens_unprocessed(&self, text: &str) -> Vec<(usize, TokenType, String)> {
        // 1. Track template tag nesting and scope
        // 2. Dispatch syntax-highlight blocks to appropriate lexers
        // 3. Handle nowiki and other scope-limited blocks
    }
}
```

## Why Not Regex-Based?

These 5 lexers could theoretically be transpiled as RegexLexer, but the callbacks are essential:

1. **State Extraction**: Header values can't be determined by regex alone; need to parse and store
2. **Recursive Dispatch**: MIME and bibtex need recursive/nested processing incompatible with flat regex rules
3. **Balance Tracking**: BibTeX's brace matching requires stateful tracking, not pattern matching
4. **Complex Rules**: The business logic is in callbacks, not in regex patterns

Attempting to force these into regex-based dispatch would require either:
- Pre-processing the entire input (expensive, breaks streaming)
- Massive regex patterns with complex capture groups (unmaintainable)
- Simulating callbacks in the regex engine (essentially duplicating Python logic)

## Recommendation

**Complete Phase E5 with these 5 as early priority**. They're already identified,
well-understood, and bring high user value (HTTP, MIME, email are common use cases).

Implementing these will:
- Unlock 5 more high-value native lexers
- Provide patterns for other E5 custom implementations
- Reduce bridge-only count from 143 to 138

## Files Updated

- `docs/pygments-port-inventory.md`:
  - E4 scope clarified as "regex-based dispatch" (3 lexers complete)
  - E4-deferred subsection added noting reclassification to E5
  - Phase E5 priorities updated to include 5 callback-based structured-text lexers
  - Summary table updated (E4: 3/3 complete, E5: 26 total)
