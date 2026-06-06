# Phase E Implementation Strategy

## Current State (after E4 completion)
- **Phase E4**: ✅ **COMPLETE** — 3 regex-based dispatch lexers (markdown, restructuredtext, tid)
- **JSON-LD + YAML-LD**: ✅ **COMPLETE** — 2 new hand-crafted lexers with full functionality
- **Phase E5**: Analyzed and planned (26 custom Lexer subclasses), not started
- **Phase E1-E3**: Not started

## Strategic Recommendation: E1 First (Higher ROI)

**Phase E1 (DelegatingLexer wiring)** unlocks **40–50 lexers** with **minimal effort per lexer**.
This should be prioritized over E5 for maximum impact in minimum time.

### Comparison: E1 vs E5

| Metric | E1 | E5a | E5b |
| --- | --- | --- | --- |
| Lexers unlocked | 40–50 | 5 | 21 |
| Code per lexer | ~5 LOC | ~500 LOC | ~300 LOC |
| Total LOC | ~250 | ~2500 | ~6300 |
| Effort | Low | High | Med–High |
| Time per lexer | 5 min | 1–2h | 30–60min |
| **ROI** | **50 lexers / 250 LOC = 0.2 LOC/lexer** | 5 / 2500 = 0.002 | 21 / 6300 = 0.003 |

**E1 is ~100x better ROI than E5.**

---

## Phase E1 — DelegatingLexer Wiring (40–50 immediate, 80 total)

**Why First**: Unlocks ~50 lexers with minimal code per lexer (5 lines of Rust each)
- Both component lexers are already native
- Auto-generation in `gen_lexer.py`
- Thin wrapper struct pattern

**Requirements**:
1. Add `--delegating` sub-command to `gen_lexer.py`
2. Identify all 80 `DelegatingLexer` subclasses
3. Check if both components are in `native_aliases()`
4. For those 40–50 with both components native: auto-generate wrappers
5. Wire into `registry.rs`

**Expected output**: ~250 LOC of new tool code + auto-generated wrappers unlocks 40–50 lexers immediately

---

### Phase E1 — DelegatingLexer Wiring (40-50 immediate, 80 total)
**Why First**: Highest ROI — unlocks ~50 lexers with minimal code per lexer
- Tool effort: Add `--delegating` mode to `gen_lexer.py`
- Per-lexer effort: ~5 lines of Rust code (thin wrapper)
- **Estimated impact**: +50 native lexers for ~100 LOC new tool code

**Requirements**:
1. Identify all 80 DelegatingLexer subclasses
2. Check both components are native
3. Auto-generate thin wrapper structs
4. Wire into registry

**Expected result**: 40-50 immediately available, 30-40 blocked on their component lexers

---

### Phase E2 — Indentation Tracking (5 lexers)
**Why Second**: Medium effort, enables 5 lexers (haml, pug, sass, scaml, slim)
- Engine effort: Add `Rule::indent_sensitive` + indent stack
- Per-lexer effort: Transpile from Python with new action
- **Estimated impact**: +5 native lexers for ~200 LOC engine code

**Requirements**:
1. Add indent_stack to lexer engine state
2. Implement indent_sensitive rule action
3. Regenerate these 5 lexers with gen_lexer.py

---

### Phase E3 — Heredoc Callbacks (3 lexers)
**Why Third**: High complexity, enables 3 valuable lexers
- Engine effort: Named-capture forwarding in Rule::using_this
- Per-lexer effort: Custom state setup
- **Estimated impact**: +3 native lexers for ~300 LOC engine code

---

### Phase E5 — Custom Lexer Subclasses (26 total)
**Why Last**: Highest per-lexer effort, lower overall impact

**Sub-phases**:
1. **E5a** (5 callback-based structured-text): Simplified implementations, medium effort each
   - http, mime, bibtex, notmuch, wikitext
   - Can provide basic syntax highlighting without full dispatch
   
2. **E5b** (21 other custom Lexer): Varying complexity
   - MakefileLexer, SqliteConsoleLexer, etc.
   - Hand-port one per cycle

---

## Next Action

**Recommend starting with Phase E1**: Tool mode + auto-generation gives maximum lexers with minimum effort. This can be completed in one focused session.

Would you like to:
1. **Proceed with E1** (DelegatingLexer wiring) — highest ROI, ~100-200 LOC
2. **Skip to E5** (custom Lexer subclasses) — full hand-porting approach
3. **Do both** — E1 first, then E5 (sequential)

