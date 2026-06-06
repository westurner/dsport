//! Auto-generated DelegatingLexer wrappers (Phase E1)
//! 61 lexers where both root and language lexers are native.
//! Generated from pygments.lexers DelegatingLexer subclasses.

use crate::lexer::Lexer;
use crate::token::{TokenType, TEXT, OTHER};

/// Implement DelegatingLexer semantics:
/// 1. Tokenize with language_lexer
/// 2. Collect "Other" tokens into a buffer
/// 3. Pass buffer through root_lexer
/// 4. Merge results back
pub fn delegate_tokens(
    code: &str,
    root_alias: &str,
    language_alias: &str,
) -> Vec<(TokenType, String)> {
    let root_lex = match crate::lexers::registry::get_lexer_by_name(root_alias) {
        Some(lex) => lex,
        None => return vec![(TEXT, code.to_string())],
    };

    let lang_lex = match crate::lexers::registry::get_lexer_by_name(language_alias) {
        Some(lex) => lex,
        None => return vec![(TEXT, code.to_string())],
    };

    let language_tokens = lang_lex.get_tokens(code);
    let mut buffered = String::new();
    let mut insertions: Vec<(usize, Vec<(TokenType, String)>)> = vec![];
    let mut language_buffer = vec![];

    // Collect Other tokens, keep language tokens
    for (token_type, value) in language_tokens {
        if token_type == OTHER {
            if !language_buffer.is_empty() {
                insertions.push((buffered.len(), language_buffer.clone()));
                language_buffer.clear();
            }
            buffered.push_str(&value);
        } else {
            language_buffer.push((token_type, value));
        }
    }

    if !language_buffer.is_empty() {
        insertions.push((buffered.len(), language_buffer));
    }

    // Tokenize buffered "Other" regions with root lexer
    let root_tokens = root_lex.get_tokens(&buffered);
    merge_tokens(root_tokens, insertions)
}

/// Merge root tokens with language token insertions
fn merge_tokens(
    root_tokens: Vec<(TokenType, String)>,
    insertions: Vec<(usize, Vec<(TokenType, String)>)>,
) -> Vec<(TokenType, String)> {
    if insertions.is_empty() {
        return root_tokens;
    }

    let mut result = vec![];
    let mut pos = 0;
    let mut insertion_idx = 0;

    for (token_type, value) in root_tokens {
        let token_len = value.len();

        // Apply insertions before this token
        while insertion_idx < insertions.len() && insertions[insertion_idx].0 <= pos {
            for lang_token in &insertions[insertion_idx].1 {
                result.push(lang_token.clone());
            }
            insertion_idx += 1;
        }

        // Check for insertions within token
        if insertion_idx < insertions.len() && insertions[insertion_idx].0 < pos + token_len {
            let insert_offset = insertions[insertion_idx].0 - pos;
            if insert_offset > 0 {
                result.push((token_type.clone(), value[..insert_offset].to_string()));
            }
            for lang_token in &insertions[insertion_idx].1 {
                result.push(lang_token.clone());
            }
            if insert_offset < value.len() {
                result.push((token_type, value[insert_offset..].to_string()));
            }
            insertion_idx += 1;
        } else {
            result.push((token_type, value));
        }

        pos += token_len;
    }

    // Add remaining insertions
    while insertion_idx < insertions.len() {
        for lang_token in &insertions[insertion_idx].1 {
            result.push(lang_token.clone());
        }
        insertion_idx += 1;
    }

    result
}


// ===== Struct definitions (61 lexers) =====

pub struct Angular2HtmlLexer;
impl Lexer for Angular2HtmlLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        delegate_tokens(code, "html", "ng2")
    }
}

pub struct AntlrActionScriptLexer;
impl Lexer for AntlrActionScriptLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        delegate_tokens(code, "actionscript", "antlr")
    }
}

pub struct AntlrCSharpLexer;
impl Lexer for AntlrCSharpLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        delegate_tokens(code, "csharp", "antlr")
    }
}

pub struct AntlrCppLexer;
impl Lexer for AntlrCppLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        delegate_tokens(code, "cpp", "antlr")
    }
}

pub struct AntlrJavaLexer;
impl Lexer for AntlrJavaLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        delegate_tokens(code, "java", "antlr")
    }
}

pub struct AntlrObjectiveCLexer;
impl Lexer for AntlrObjectiveCLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        delegate_tokens(code, "objective-c", "antlr")
    }
}

pub struct AntlrPerlLexer;
impl Lexer for AntlrPerlLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        delegate_tokens(code, "perl", "antlr")
    }
}

pub struct AntlrPythonLexer;
impl Lexer for AntlrPythonLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        delegate_tokens(code, "python", "antlr")
    }
}

pub struct CObjdumpLexer;
impl Lexer for CObjdumpLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        delegate_tokens(code, "c", "objdump")
    }
}

pub struct CSSUL4Lexer;
impl Lexer for CSSUL4Lexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        delegate_tokens(code, "css", "ul4")
    }
}

pub struct CheetahHtmlLexer;
impl Lexer for CheetahHtmlLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        delegate_tokens(code, "html", "cheetah")
    }
}

pub struct CheetahJavascriptLexer;
impl Lexer for CheetahJavascriptLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        delegate_tokens(code, "javascript", "cheetah")
    }
}

pub struct CheetahXmlLexer;
impl Lexer for CheetahXmlLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        delegate_tokens(code, "xml", "cheetah")
    }
}

pub struct CppObjdumpLexer;
impl Lexer for CppObjdumpLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        delegate_tokens(code, "cpp", "objdump")
    }
}

pub struct CssDjangoLexer;
impl Lexer for CssDjangoLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        delegate_tokens(code, "css", "django")
    }
}

pub struct CssGenshiLexer;
impl Lexer for CssGenshiLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        delegate_tokens(code, "css", "genshitext")
    }
}

pub struct CssPhpLexer;
impl Lexer for CssPhpLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        delegate_tokens(code, "css", "php")
    }
}

pub struct CssSmartyLexer;
impl Lexer for CssSmartyLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        delegate_tokens(code, "css", "smarty")
    }
}

pub struct DObjdumpLexer;
impl Lexer for DObjdumpLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        delegate_tokens(code, "d", "objdump")
    }
}

pub struct EvoqueHtmlLexer;
impl Lexer for EvoqueHtmlLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        delegate_tokens(code, "html", "evoque")
    }
}

pub struct EvoqueXmlLexer;
impl Lexer for EvoqueXmlLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        delegate_tokens(code, "xml", "evoque")
    }
}

pub struct HTMLUL4Lexer;
impl Lexer for HTMLUL4Lexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        delegate_tokens(code, "html", "ul4")
    }
}

pub struct HandlebarsHtmlLexer;
impl Lexer for HandlebarsHtmlLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        delegate_tokens(code, "html", "handlebars")
    }
}

pub struct HtmlDjangoLexer;
impl Lexer for HtmlDjangoLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        delegate_tokens(code, "html", "django")
    }
}

pub struct HtmlPhpLexer;
impl Lexer for HtmlPhpLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        delegate_tokens(code, "html", "php")
    }
}

pub struct HtmlSmartyLexer;
impl Lexer for HtmlSmartyLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        delegate_tokens(code, "html", "smarty")
    }
}

pub struct JavascriptDjangoLexer;
impl Lexer for JavascriptDjangoLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        delegate_tokens(code, "javascript", "django")
    }
}

pub struct JavascriptGenshiLexer;
impl Lexer for JavascriptGenshiLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        delegate_tokens(code, "javascript", "genshitext")
    }
}

pub struct JavascriptPhpLexer;
impl Lexer for JavascriptPhpLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        delegate_tokens(code, "javascript", "php")
    }
}

pub struct JavascriptSmartyLexer;
impl Lexer for JavascriptSmartyLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        delegate_tokens(code, "javascript", "smarty")
    }
}

pub struct JavascriptUL4Lexer;
impl Lexer for JavascriptUL4Lexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        delegate_tokens(code, "javascript", "ul4")
    }
}

pub struct LassoCssLexer;
impl Lexer for LassoCssLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        delegate_tokens(code, "css", "lasso")
    }
}

pub struct LassoHtmlLexer;
impl Lexer for LassoHtmlLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        delegate_tokens(code, "html", "lasso")
    }
}

pub struct LassoJavascriptLexer;
impl Lexer for LassoJavascriptLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        delegate_tokens(code, "javascript", "lasso")
    }
}

pub struct LassoXmlLexer;
impl Lexer for LassoXmlLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        delegate_tokens(code, "xml", "lasso")
    }
}

pub struct MakoCssLexer;
impl Lexer for MakoCssLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        delegate_tokens(code, "css", "mako")
    }
}

pub struct MakoHtmlLexer;
impl Lexer for MakoHtmlLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        delegate_tokens(code, "html", "mako")
    }
}

pub struct MakoJavascriptLexer;
impl Lexer for MakoJavascriptLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        delegate_tokens(code, "javascript", "mako")
    }
}

pub struct MakoXmlLexer;
impl Lexer for MakoXmlLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        delegate_tokens(code, "xml", "mako")
    }
}

pub struct MozPreprocCssLexer;
impl Lexer for MozPreprocCssLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        delegate_tokens(code, "css", "mozpercentpreproc")
    }
}

pub struct MozPreprocJavascriptLexer;
impl Lexer for MozPreprocJavascriptLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        delegate_tokens(code, "javascript", "mozhashpreproc")
    }
}

pub struct MozPreprocXulLexer;
impl Lexer for MozPreprocXulLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        delegate_tokens(code, "xml", "mozhashpreproc")
    }
}

pub struct MyghtyCssLexer;
impl Lexer for MyghtyCssLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        delegate_tokens(code, "css", "myghty")
    }
}

pub struct MyghtyHtmlLexer;
impl Lexer for MyghtyHtmlLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        delegate_tokens(code, "html", "myghty")
    }
}

pub struct MyghtyJavascriptLexer;
impl Lexer for MyghtyJavascriptLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        delegate_tokens(code, "javascript", "myghty")
    }
}

pub struct MyghtyXmlLexer;
impl Lexer for MyghtyXmlLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        delegate_tokens(code, "xml", "myghty")
    }
}

pub struct PythonUL4Lexer;
impl Lexer for PythonUL4Lexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        delegate_tokens(code, "python", "ul4")
    }
}

pub struct RagelCLexer;
impl Lexer for RagelCLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        delegate_tokens(code, "c", "ragel-em")
    }
}

pub struct RagelCppLexer;
impl Lexer for RagelCppLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        delegate_tokens(code, "cpp", "ragel-em")
    }
}

pub struct RagelDLexer;
impl Lexer for RagelDLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        delegate_tokens(code, "d", "ragel-em")
    }
}

pub struct RagelJavaLexer;
impl Lexer for RagelJavaLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        delegate_tokens(code, "java", "ragel-em")
    }
}

pub struct RagelObjectiveCLexer;
impl Lexer for RagelObjectiveCLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        delegate_tokens(code, "objective-c", "ragel-em")
    }
}

pub struct SqlJinjaLexer;
impl Lexer for SqlJinjaLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        delegate_tokens(code, "sql", "django")
    }
}

pub struct TwigHtmlLexer;
impl Lexer for TwigHtmlLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        delegate_tokens(code, "html", "twig")
    }
}

pub struct VelocityHtmlLexer;
impl Lexer for VelocityHtmlLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        delegate_tokens(code, "html", "velocity")
    }
}

pub struct VelocityXmlLexer;
impl Lexer for VelocityXmlLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        delegate_tokens(code, "xml", "velocity")
    }
}

pub struct XMLUL4Lexer;
impl Lexer for XMLUL4Lexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        delegate_tokens(code, "xml", "ul4")
    }
}

pub struct XmlDjangoLexer;
impl Lexer for XmlDjangoLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        delegate_tokens(code, "xml", "django")
    }
}

pub struct XmlPhpLexer;
impl Lexer for XmlPhpLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        delegate_tokens(code, "xml", "php")
    }
}

pub struct XmlSmartyLexer;
impl Lexer for XmlSmartyLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        delegate_tokens(code, "xml", "smarty")
    }
}

pub struct YamlJinjaLexer;
impl Lexer for YamlJinjaLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        delegate_tokens(code, "yaml", "django")
    }
}
