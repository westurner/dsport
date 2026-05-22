//! Line-based preprocessing that lowers MyST-only constructs into shapes
//! `pulldown-cmark` understands natively.
//!
//! Currently:
//! * `:::name …\n:::` colon fences become ` ```{name} …\n``` ` fenced code
//!   blocks. The renderer recognises `{name}` info strings and emits the
//!   `myst-directive` markup.
//! * `$$ … $$` block-level math becomes a fenced code block with the info
//!   string `math` (rendered as `<div class="math">`).
//!
//! Inline `$…$` and inline roles `` {name}`…` `` are handled in the
//! renderer/event filter, not here.

/// Apply MyST line-level rewrites and return the transformed source.
pub fn preprocess(input: &str) -> String {
    let mut out = String::with_capacity(input.len());
    let mut colon_stack: Vec<usize> = Vec::new();
    let mut in_dollar_block = false;

    for line in input.split_inclusive('\n') {
        let body = line.trim_end_matches(['\r', '\n']);
        let leading = body.len() - body.trim_start().len();
        let trimmed = &body[leading..];

        // Block math fence: a line that is exactly "$$" toggles a fenced math
        // block. We don't try to capture inline-trailing content here — that is
        // the dollarmath inline rule's job.
        if trimmed == "$$" {
            if in_dollar_block {
                out.push_str("```\n");
                in_dollar_block = false;
            } else {
                out.push_str("```math\n");
                in_dollar_block = true;
            }
            continue;
        }

        if in_dollar_block {
            out.push_str(line);
            continue;
        }

        // Colon fence open: ":::name" or ":::name args"
        if let Some(rest) = trimmed.strip_prefix(":::") {
            if rest.is_empty() {
                // close — only treat as close if we have a matching open at
                // this indent. Otherwise pass through unchanged.
                if colon_stack.pop().is_some() {
                    out.push_str(&" ".repeat(leading));
                    out.push_str("```\n");
                    continue;
                }
            } else if !rest.starts_with(':') {
                // open
                let name = rest.split_whitespace().next().unwrap_or("");
                let extra = rest[name.len()..].trim();
                colon_stack.push(leading);
                out.push_str(&" ".repeat(leading));
                if extra.is_empty() {
                    out.push_str(&format!("```{{{name}}}\n"));
                } else {
                    out.push_str(&format!("```{{{name}}} {extra}\n"));
                }
                continue;
            }
        }

        out.push_str(line);
    }

    // Close any still-open structures defensively so cmark doesn't get
    // confused mid-document.
    if in_dollar_block {
        out.push_str("```\n");
    }
    for _ in colon_stack.drain(..) {
        out.push_str("```\n");
    }

    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn colon_fence_basic() {
        let src = ":::note\nhello\n:::\n";
        let out = preprocess(src);
        assert_eq!(out, "```{note}\nhello\n```\n");
    }

    #[test]
    fn dollar_block_math() {
        let src = "$$\nx = 1\n$$\n";
        let out = preprocess(src);
        assert_eq!(out, "```math\nx = 1\n```\n");
    }

    #[test]
    fn passthrough_when_no_constructs() {
        let src = "hello *world*\n";
        assert_eq!(preprocess(src), src);
    }
}
