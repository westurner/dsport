//! AUTO-GENERATED from `pygments.pygments.lexers.tcl:TclLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.tcl:TclLexer:tcl

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{NewState, Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: tcl
pub struct TclLexer;

struct Table(HashMap<&'static str, Vec<Rule>>);

impl StateTable for Table {
    fn state(&self, name: &str) -> Option<&[Rule]> {
        self.0.get(name).map(Vec::as_slice)
    }
}

static TABLE: OnceLock<Table> = OnceLock::new();

fn build_table() -> Table {
    let mut m: HashMap<&'static str, Vec<Rule>> = HashMap::new();
    m.insert(r"root", vec![
        Rule::token_to(r"(?m)\b(a(?:fter|(?:ppl|rra)y)|break|c(?:atch|ontinue)|e(?:lse(?:(?:if)?)|rror|val|xpr)|for(?:(?:each)?)|global|if|namespace|proc|re(?:name|turn)|s(?:et|witch)|t(?:hen|race)|u(?:nset|p(?:date|level|var))|v(?:ariable|wait)|while)\b", KEYWORD, NewState::Push(vec![r"params"])),
        Rule::token_to(r"(?m)\b(append|b(?:gerror|inary)|c(?:d|han|lo(?:ck|se)|oncat)|d(?:de|ict)|e(?:ncoding|of|x(?:ec|it))|f(?:blocked|co(?:nfigure|py)|ile(?:(?:event)?)|lush|ormat)|g(?:ets|lob)|h(?:istory|ttp)|in(?:cr|fo|terp)|join|l(?:a(?:ppend|ssign)|i(?:n(?:dex|sert)|st)|length|oad(?:(?:Tk)?)|r(?:ange|e(?:p(?:eat|lace)|verse))|s(?:e(?:arch|t)|ort))|m(?:ath(?:func|op)|emory|sgcat)|open|p(?:ackage|id|kg(?:::create|_mkIndex)|latform(?:(?:::shell)?)|uts|wd)|re(?:_syntax|ad|fchan|g(?:exp|istry|sub))|s(?:can|eek|o(?:cket|urce)|plit|tring|ubst)|t(?:ell|ime|m)|un(?:known|load))\b", NAME_BUILTIN, NewState::Push(vec![r"params"])),
        Rule::token_to(r"(?m)([\w.-]+)", NAME_VARIABLE, NewState::Push(vec![r"params"])),
        Rule::token_to(r"(?m)#", COMMENT, NewState::Push(vec![r"comment"])),
        Rule::token_to(r"(?m)\(", KEYWORD, NewState::Push(vec![r"paren"])),
        Rule::token_to(r"(?m)\[", KEYWORD, NewState::Push(vec![r"bracket"])),
        Rule::token_to(r"(?m)\{", KEYWORD, NewState::Push(vec![r"brace"])),
        Rule::token_to(r#"(?m)""#, STRING_DOUBLE, NewState::Push(vec![r"string"])),
        Rule::token(r"(?m)(eq|ne|in|ni)\b", OPERATOR_WORD),
        Rule::token(r"(?m)!=|==|<<|>>|<=|>=|&&|\|\||\*\*|[-+~!*/%<>&^|?:]", OPERATOR),
        Rule::token(r"(?m)\s+", WHITESPACE),
        Rule::token(r"(?m)0x[a-fA-F0-9]+", NUMBER_HEX),
        Rule::token(r"(?m)0[0-7]+", NUMBER_OCT),
        Rule::token(r"(?m)\d+\.\d+", NUMBER_FLOAT),
        Rule::token(r"(?m)\d+", NUMBER_INTEGER),
        Rule::token(r"(?m)\$[\w.:-]+", NAME_VARIABLE),
        Rule::token(r"(?m)\$\{[\w.:-]+\}", NAME_VARIABLE),
        Rule::token(r"(?m)[\w.,@:-]+", TEXT),
        Rule::token(r"(?m)\}", KEYWORD),
    ]);
    m.insert(r"command", vec![
        Rule::token_to(r"(?m)\b(a(?:fter|(?:ppl|rra)y)|break|c(?:atch|ontinue)|e(?:lse(?:(?:if)?)|rror|val|xpr)|for(?:(?:each)?)|global|if|namespace|proc|re(?:name|turn)|s(?:et|witch)|t(?:hen|race)|u(?:nset|p(?:date|level|var))|v(?:ariable|wait)|while)\b", KEYWORD, NewState::Push(vec![r"params"])),
        Rule::token_to(r"(?m)\b(append|b(?:gerror|inary)|c(?:d|han|lo(?:ck|se)|oncat)|d(?:de|ict)|e(?:ncoding|of|x(?:ec|it))|f(?:blocked|co(?:nfigure|py)|ile(?:(?:event)?)|lush|ormat)|g(?:ets|lob)|h(?:istory|ttp)|in(?:cr|fo|terp)|join|l(?:a(?:ppend|ssign)|i(?:n(?:dex|sert)|st)|length|oad(?:(?:Tk)?)|r(?:ange|e(?:p(?:eat|lace)|verse))|s(?:e(?:arch|t)|ort))|m(?:ath(?:func|op)|emory|sgcat)|open|p(?:ackage|id|kg(?:::create|_mkIndex)|latform(?:(?:::shell)?)|uts|wd)|re(?:_syntax|ad|fchan|g(?:exp|istry|sub))|s(?:can|eek|o(?:cket|urce)|plit|tring|ubst)|t(?:ell|ime|m)|un(?:known|load))\b", NAME_BUILTIN, NewState::Push(vec![r"params"])),
        Rule::token_to(r"(?m)([\w.-]+)", NAME_VARIABLE, NewState::Push(vec![r"params"])),
        Rule::token_to(r"(?m)#", COMMENT, NewState::Push(vec![r"comment"])),
    ]);
    m.insert(r"basic", vec![
        Rule::token_to(r"(?m)\(", KEYWORD, NewState::Push(vec![r"paren"])),
        Rule::token_to(r"(?m)\[", KEYWORD, NewState::Push(vec![r"bracket"])),
        Rule::token_to(r"(?m)\{", KEYWORD, NewState::Push(vec![r"brace"])),
        Rule::token_to(r#"(?m)""#, STRING_DOUBLE, NewState::Push(vec![r"string"])),
        Rule::token(r"(?m)(eq|ne|in|ni)\b", OPERATOR_WORD),
        Rule::token(r"(?m)!=|==|<<|>>|<=|>=|&&|\|\||\*\*|[-+~!*/%<>&^|?:]", OPERATOR),
    ]);
    m.insert(r"data", vec![
        Rule::token(r"(?m)\s+", WHITESPACE),
        Rule::token(r"(?m)0x[a-fA-F0-9]+", NUMBER_HEX),
        Rule::token(r"(?m)0[0-7]+", NUMBER_OCT),
        Rule::token(r"(?m)\d+\.\d+", NUMBER_FLOAT),
        Rule::token(r"(?m)\d+", NUMBER_INTEGER),
        Rule::token(r"(?m)\$[\w.:-]+", NAME_VARIABLE),
        Rule::token(r"(?m)\$\{[\w.:-]+\}", NAME_VARIABLE),
        Rule::token(r"(?m)[\w.,@:-]+", TEXT),
    ]);
    m.insert(r"command-in-brace", vec![
        Rule::token_to(r"(?m)\b(a(?:fter|(?:ppl|rra)y)|break|c(?:atch|ontinue)|e(?:lse(?:(?:if)?)|rror|val|xpr)|for(?:(?:each)?)|global|if|namespace|proc|re(?:name|turn)|s(?:et|witch)|t(?:hen|race)|u(?:nset|p(?:date|level|var))|v(?:ariable|wait)|while)\b", KEYWORD, NewState::Push(vec![r"params-in-brace"])),
        Rule::token_to(r"(?m)\b(append|b(?:gerror|inary)|c(?:d|han|lo(?:ck|se)|oncat)|d(?:de|ict)|e(?:ncoding|of|x(?:ec|it))|f(?:blocked|co(?:nfigure|py)|ile(?:(?:event)?)|lush|ormat)|g(?:ets|lob)|h(?:istory|ttp)|in(?:cr|fo|terp)|join|l(?:a(?:ppend|ssign)|i(?:n(?:dex|sert)|st)|length|oad(?:(?:Tk)?)|r(?:ange|e(?:p(?:eat|lace)|verse))|s(?:e(?:arch|t)|ort))|m(?:ath(?:func|op)|emory|sgcat)|open|p(?:ackage|id|kg(?:::create|_mkIndex)|latform(?:(?:::shell)?)|uts|wd)|re(?:_syntax|ad|fchan|g(?:exp|istry|sub))|s(?:can|eek|o(?:cket|urce)|plit|tring|ubst)|t(?:ell|ime|m)|un(?:known|load))\b", NAME_BUILTIN, NewState::Push(vec![r"params-in-brace"])),
        Rule::token_to(r"(?m)([\w.-]+)", NAME_VARIABLE, NewState::Push(vec![r"params-in-brace"])),
        Rule::token_to(r"(?m)#", COMMENT, NewState::Push(vec![r"comment"])),
    ]);
    m.insert(r"command-in-bracket", vec![
        Rule::token_to(r"(?m)\b(a(?:fter|(?:ppl|rra)y)|break|c(?:atch|ontinue)|e(?:lse(?:(?:if)?)|rror|val|xpr)|for(?:(?:each)?)|global|if|namespace|proc|re(?:name|turn)|s(?:et|witch)|t(?:hen|race)|u(?:nset|p(?:date|level|var))|v(?:ariable|wait)|while)\b", KEYWORD, NewState::Push(vec![r"params-in-bracket"])),
        Rule::token_to(r"(?m)\b(append|b(?:gerror|inary)|c(?:d|han|lo(?:ck|se)|oncat)|d(?:de|ict)|e(?:ncoding|of|x(?:ec|it))|f(?:blocked|co(?:nfigure|py)|ile(?:(?:event)?)|lush|ormat)|g(?:ets|lob)|h(?:istory|ttp)|in(?:cr|fo|terp)|join|l(?:a(?:ppend|ssign)|i(?:n(?:dex|sert)|st)|length|oad(?:(?:Tk)?)|r(?:ange|e(?:p(?:eat|lace)|verse))|s(?:e(?:arch|t)|ort))|m(?:ath(?:func|op)|emory|sgcat)|open|p(?:ackage|id|kg(?:::create|_mkIndex)|latform(?:(?:::shell)?)|uts|wd)|re(?:_syntax|ad|fchan|g(?:exp|istry|sub))|s(?:can|eek|o(?:cket|urce)|plit|tring|ubst)|t(?:ell|ime|m)|un(?:known|load))\b", NAME_BUILTIN, NewState::Push(vec![r"params-in-bracket"])),
        Rule::token_to(r"(?m)([\w.-]+)", NAME_VARIABLE, NewState::Push(vec![r"params-in-bracket"])),
        Rule::token_to(r"(?m)#", COMMENT, NewState::Push(vec![r"comment"])),
    ]);
    m.insert(r"command-in-paren", vec![
        Rule::token_to(r"(?m)\b(a(?:fter|(?:ppl|rra)y)|break|c(?:atch|ontinue)|e(?:lse(?:(?:if)?)|rror|val|xpr)|for(?:(?:each)?)|global|if|namespace|proc|re(?:name|turn)|s(?:et|witch)|t(?:hen|race)|u(?:nset|p(?:date|level|var))|v(?:ariable|wait)|while)\b", KEYWORD, NewState::Push(vec![r"params-in-paren"])),
        Rule::token_to(r"(?m)\b(append|b(?:gerror|inary)|c(?:d|han|lo(?:ck|se)|oncat)|d(?:de|ict)|e(?:ncoding|of|x(?:ec|it))|f(?:blocked|co(?:nfigure|py)|ile(?:(?:event)?)|lush|ormat)|g(?:ets|lob)|h(?:istory|ttp)|in(?:cr|fo|terp)|join|l(?:a(?:ppend|ssign)|i(?:n(?:dex|sert)|st)|length|oad(?:(?:Tk)?)|r(?:ange|e(?:p(?:eat|lace)|verse))|s(?:e(?:arch|t)|ort))|m(?:ath(?:func|op)|emory|sgcat)|open|p(?:ackage|id|kg(?:::create|_mkIndex)|latform(?:(?:::shell)?)|uts|wd)|re(?:_syntax|ad|fchan|g(?:exp|istry|sub))|s(?:can|eek|o(?:cket|urce)|plit|tring|ubst)|t(?:ell|ime|m)|un(?:known|load))\b", NAME_BUILTIN, NewState::Push(vec![r"params-in-paren"])),
        Rule::token_to(r"(?m)([\w.-]+)", NAME_VARIABLE, NewState::Push(vec![r"params-in-paren"])),
        Rule::token_to(r"(?m)#", COMMENT, NewState::Push(vec![r"comment"])),
    ]);
    m.insert(r"params", vec![
        Rule::token_to(r"(?m);", KEYWORD, NewState::Pop(1)),
        Rule::token_to(r"(?m)\n", TEXT, NewState::Pop(1)),
        Rule::token(r"(?m)(else|elseif|then)\b", KEYWORD),
        Rule::token_to(r"(?m)\(", KEYWORD, NewState::Push(vec![r"paren"])),
        Rule::token_to(r"(?m)\[", KEYWORD, NewState::Push(vec![r"bracket"])),
        Rule::token_to(r"(?m)\{", KEYWORD, NewState::Push(vec![r"brace"])),
        Rule::token_to(r#"(?m)""#, STRING_DOUBLE, NewState::Push(vec![r"string"])),
        Rule::token(r"(?m)(eq|ne|in|ni)\b", OPERATOR_WORD),
        Rule::token(r"(?m)!=|==|<<|>>|<=|>=|&&|\|\||\*\*|[-+~!*/%<>&^|?:]", OPERATOR),
        Rule::token(r"(?m)\s+", WHITESPACE),
        Rule::token(r"(?m)0x[a-fA-F0-9]+", NUMBER_HEX),
        Rule::token(r"(?m)0[0-7]+", NUMBER_OCT),
        Rule::token(r"(?m)\d+\.\d+", NUMBER_FLOAT),
        Rule::token(r"(?m)\d+", NUMBER_INTEGER),
        Rule::token(r"(?m)\$[\w.:-]+", NAME_VARIABLE),
        Rule::token(r"(?m)\$\{[\w.:-]+\}", NAME_VARIABLE),
        Rule::token(r"(?m)[\w.,@:-]+", TEXT),
    ]);
    m.insert(r"params-in-brace", vec![
        Rule::token_to(r"(?m)\}", KEYWORD, NewState::Push(vec![r"#pop", r"#pop"])),
        Rule::token_to(r"(?m);", KEYWORD, NewState::Pop(1)),
        Rule::token_to(r"(?m)\n", TEXT, NewState::Pop(1)),
        Rule::token(r"(?m)(else|elseif|then)\b", KEYWORD),
        Rule::token_to(r"(?m)\(", KEYWORD, NewState::Push(vec![r"paren"])),
        Rule::token_to(r"(?m)\[", KEYWORD, NewState::Push(vec![r"bracket"])),
        Rule::token_to(r"(?m)\{", KEYWORD, NewState::Push(vec![r"brace"])),
        Rule::token_to(r#"(?m)""#, STRING_DOUBLE, NewState::Push(vec![r"string"])),
        Rule::token(r"(?m)(eq|ne|in|ni)\b", OPERATOR_WORD),
        Rule::token(r"(?m)!=|==|<<|>>|<=|>=|&&|\|\||\*\*|[-+~!*/%<>&^|?:]", OPERATOR),
        Rule::token(r"(?m)\s+", WHITESPACE),
        Rule::token(r"(?m)0x[a-fA-F0-9]+", NUMBER_HEX),
        Rule::token(r"(?m)0[0-7]+", NUMBER_OCT),
        Rule::token(r"(?m)\d+\.\d+", NUMBER_FLOAT),
        Rule::token(r"(?m)\d+", NUMBER_INTEGER),
        Rule::token(r"(?m)\$[\w.:-]+", NAME_VARIABLE),
        Rule::token(r"(?m)\$\{[\w.:-]+\}", NAME_VARIABLE),
        Rule::token(r"(?m)[\w.,@:-]+", TEXT),
    ]);
    m.insert(r"params-in-paren", vec![
        Rule::token_to(r"(?m)\)", KEYWORD, NewState::Push(vec![r"#pop", r"#pop"])),
        Rule::token_to(r"(?m);", KEYWORD, NewState::Pop(1)),
        Rule::token_to(r"(?m)\n", TEXT, NewState::Pop(1)),
        Rule::token(r"(?m)(else|elseif|then)\b", KEYWORD),
        Rule::token_to(r"(?m)\(", KEYWORD, NewState::Push(vec![r"paren"])),
        Rule::token_to(r"(?m)\[", KEYWORD, NewState::Push(vec![r"bracket"])),
        Rule::token_to(r"(?m)\{", KEYWORD, NewState::Push(vec![r"brace"])),
        Rule::token_to(r#"(?m)""#, STRING_DOUBLE, NewState::Push(vec![r"string"])),
        Rule::token(r"(?m)(eq|ne|in|ni)\b", OPERATOR_WORD),
        Rule::token(r"(?m)!=|==|<<|>>|<=|>=|&&|\|\||\*\*|[-+~!*/%<>&^|?:]", OPERATOR),
        Rule::token(r"(?m)\s+", WHITESPACE),
        Rule::token(r"(?m)0x[a-fA-F0-9]+", NUMBER_HEX),
        Rule::token(r"(?m)0[0-7]+", NUMBER_OCT),
        Rule::token(r"(?m)\d+\.\d+", NUMBER_FLOAT),
        Rule::token(r"(?m)\d+", NUMBER_INTEGER),
        Rule::token(r"(?m)\$[\w.:-]+", NAME_VARIABLE),
        Rule::token(r"(?m)\$\{[\w.:-]+\}", NAME_VARIABLE),
        Rule::token(r"(?m)[\w.,@:-]+", TEXT),
    ]);
    m.insert(r"params-in-bracket", vec![
        Rule::token_to(r"(?m)\]", KEYWORD, NewState::Push(vec![r"#pop", r"#pop"])),
        Rule::token_to(r"(?m);", KEYWORD, NewState::Pop(1)),
        Rule::token_to(r"(?m)\n", TEXT, NewState::Pop(1)),
        Rule::token(r"(?m)(else|elseif|then)\b", KEYWORD),
        Rule::token_to(r"(?m)\(", KEYWORD, NewState::Push(vec![r"paren"])),
        Rule::token_to(r"(?m)\[", KEYWORD, NewState::Push(vec![r"bracket"])),
        Rule::token_to(r"(?m)\{", KEYWORD, NewState::Push(vec![r"brace"])),
        Rule::token_to(r#"(?m)""#, STRING_DOUBLE, NewState::Push(vec![r"string"])),
        Rule::token(r"(?m)(eq|ne|in|ni)\b", OPERATOR_WORD),
        Rule::token(r"(?m)!=|==|<<|>>|<=|>=|&&|\|\||\*\*|[-+~!*/%<>&^|?:]", OPERATOR),
        Rule::token(r"(?m)\s+", WHITESPACE),
        Rule::token(r"(?m)0x[a-fA-F0-9]+", NUMBER_HEX),
        Rule::token(r"(?m)0[0-7]+", NUMBER_OCT),
        Rule::token(r"(?m)\d+\.\d+", NUMBER_FLOAT),
        Rule::token(r"(?m)\d+", NUMBER_INTEGER),
        Rule::token(r"(?m)\$[\w.:-]+", NAME_VARIABLE),
        Rule::token(r"(?m)\$\{[\w.:-]+\}", NAME_VARIABLE),
        Rule::token(r"(?m)[\w.,@:-]+", TEXT),
    ]);
    m.insert(r"string", vec![
        Rule::token_to(r"(?m)\[", STRING_DOUBLE, NewState::Push(vec![r"string-square"])),
        Rule::token(r#"(?m)(?s)(\\\\|\\[0-7]+|\\.|[^"\\])"#, STRING_DOUBLE),
        Rule::token_to(r#"(?m)""#, STRING_DOUBLE, NewState::Pop(1)),
    ]);
    m.insert(r"string-square", vec![
        Rule::token_to(r"(?m)\[", STRING_DOUBLE, NewState::Push(vec![r"string-square"])),
        Rule::token(r"(?m)(?s)(\\\\|\\[0-7]+|\\.|\\\n|[^\]\\])", STRING_DOUBLE),
        Rule::token_to(r"(?m)\]", STRING_DOUBLE, NewState::Pop(1)),
    ]);
    m.insert(r"brace", vec![
        Rule::token_to(r"(?m)\}", KEYWORD, NewState::Pop(1)),
        Rule::token_to(r"(?m)\b(a(?:fter|(?:ppl|rra)y)|break|c(?:atch|ontinue)|e(?:lse(?:(?:if)?)|rror|val|xpr)|for(?:(?:each)?)|global|if|namespace|proc|re(?:name|turn)|s(?:et|witch)|t(?:hen|race)|u(?:nset|p(?:date|level|var))|v(?:ariable|wait)|while)\b", KEYWORD, NewState::Push(vec![r"params-in-brace"])),
        Rule::token_to(r"(?m)\b(append|b(?:gerror|inary)|c(?:d|han|lo(?:ck|se)|oncat)|d(?:de|ict)|e(?:ncoding|of|x(?:ec|it))|f(?:blocked|co(?:nfigure|py)|ile(?:(?:event)?)|lush|ormat)|g(?:ets|lob)|h(?:istory|ttp)|in(?:cr|fo|terp)|join|l(?:a(?:ppend|ssign)|i(?:n(?:dex|sert)|st)|length|oad(?:(?:Tk)?)|r(?:ange|e(?:p(?:eat|lace)|verse))|s(?:e(?:arch|t)|ort))|m(?:ath(?:func|op)|emory|sgcat)|open|p(?:ackage|id|kg(?:::create|_mkIndex)|latform(?:(?:::shell)?)|uts|wd)|re(?:_syntax|ad|fchan|g(?:exp|istry|sub))|s(?:can|eek|o(?:cket|urce)|plit|tring|ubst)|t(?:ell|ime|m)|un(?:known|load))\b", NAME_BUILTIN, NewState::Push(vec![r"params-in-brace"])),
        Rule::token_to(r"(?m)([\w.-]+)", NAME_VARIABLE, NewState::Push(vec![r"params-in-brace"])),
        Rule::token_to(r"(?m)#", COMMENT, NewState::Push(vec![r"comment"])),
        Rule::token_to(r"(?m)\(", KEYWORD, NewState::Push(vec![r"paren"])),
        Rule::token_to(r"(?m)\[", KEYWORD, NewState::Push(vec![r"bracket"])),
        Rule::token_to(r"(?m)\{", KEYWORD, NewState::Push(vec![r"brace"])),
        Rule::token_to(r#"(?m)""#, STRING_DOUBLE, NewState::Push(vec![r"string"])),
        Rule::token(r"(?m)(eq|ne|in|ni)\b", OPERATOR_WORD),
        Rule::token(r"(?m)!=|==|<<|>>|<=|>=|&&|\|\||\*\*|[-+~!*/%<>&^|?:]", OPERATOR),
        Rule::token(r"(?m)\s+", WHITESPACE),
        Rule::token(r"(?m)0x[a-fA-F0-9]+", NUMBER_HEX),
        Rule::token(r"(?m)0[0-7]+", NUMBER_OCT),
        Rule::token(r"(?m)\d+\.\d+", NUMBER_FLOAT),
        Rule::token(r"(?m)\d+", NUMBER_INTEGER),
        Rule::token(r"(?m)\$[\w.:-]+", NAME_VARIABLE),
        Rule::token(r"(?m)\$\{[\w.:-]+\}", NAME_VARIABLE),
        Rule::token(r"(?m)[\w.,@:-]+", TEXT),
    ]);
    m.insert(r"paren", vec![
        Rule::token_to(r"(?m)\)", KEYWORD, NewState::Pop(1)),
        Rule::token_to(r"(?m)\b(a(?:fter|(?:ppl|rra)y)|break|c(?:atch|ontinue)|e(?:lse(?:(?:if)?)|rror|val|xpr)|for(?:(?:each)?)|global|if|namespace|proc|re(?:name|turn)|s(?:et|witch)|t(?:hen|race)|u(?:nset|p(?:date|level|var))|v(?:ariable|wait)|while)\b", KEYWORD, NewState::Push(vec![r"params-in-paren"])),
        Rule::token_to(r"(?m)\b(append|b(?:gerror|inary)|c(?:d|han|lo(?:ck|se)|oncat)|d(?:de|ict)|e(?:ncoding|of|x(?:ec|it))|f(?:blocked|co(?:nfigure|py)|ile(?:(?:event)?)|lush|ormat)|g(?:ets|lob)|h(?:istory|ttp)|in(?:cr|fo|terp)|join|l(?:a(?:ppend|ssign)|i(?:n(?:dex|sert)|st)|length|oad(?:(?:Tk)?)|r(?:ange|e(?:p(?:eat|lace)|verse))|s(?:e(?:arch|t)|ort))|m(?:ath(?:func|op)|emory|sgcat)|open|p(?:ackage|id|kg(?:::create|_mkIndex)|latform(?:(?:::shell)?)|uts|wd)|re(?:_syntax|ad|fchan|g(?:exp|istry|sub))|s(?:can|eek|o(?:cket|urce)|plit|tring|ubst)|t(?:ell|ime|m)|un(?:known|load))\b", NAME_BUILTIN, NewState::Push(vec![r"params-in-paren"])),
        Rule::token_to(r"(?m)([\w.-]+)", NAME_VARIABLE, NewState::Push(vec![r"params-in-paren"])),
        Rule::token_to(r"(?m)#", COMMENT, NewState::Push(vec![r"comment"])),
        Rule::token_to(r"(?m)\(", KEYWORD, NewState::Push(vec![r"paren"])),
        Rule::token_to(r"(?m)\[", KEYWORD, NewState::Push(vec![r"bracket"])),
        Rule::token_to(r"(?m)\{", KEYWORD, NewState::Push(vec![r"brace"])),
        Rule::token_to(r#"(?m)""#, STRING_DOUBLE, NewState::Push(vec![r"string"])),
        Rule::token(r"(?m)(eq|ne|in|ni)\b", OPERATOR_WORD),
        Rule::token(r"(?m)!=|==|<<|>>|<=|>=|&&|\|\||\*\*|[-+~!*/%<>&^|?:]", OPERATOR),
        Rule::token(r"(?m)\s+", WHITESPACE),
        Rule::token(r"(?m)0x[a-fA-F0-9]+", NUMBER_HEX),
        Rule::token(r"(?m)0[0-7]+", NUMBER_OCT),
        Rule::token(r"(?m)\d+\.\d+", NUMBER_FLOAT),
        Rule::token(r"(?m)\d+", NUMBER_INTEGER),
        Rule::token(r"(?m)\$[\w.:-]+", NAME_VARIABLE),
        Rule::token(r"(?m)\$\{[\w.:-]+\}", NAME_VARIABLE),
        Rule::token(r"(?m)[\w.,@:-]+", TEXT),
    ]);
    m.insert(r"bracket", vec![
        Rule::token_to(r"(?m)\]", KEYWORD, NewState::Pop(1)),
        Rule::token_to(r"(?m)\b(a(?:fter|(?:ppl|rra)y)|break|c(?:atch|ontinue)|e(?:lse(?:(?:if)?)|rror|val|xpr)|for(?:(?:each)?)|global|if|namespace|proc|re(?:name|turn)|s(?:et|witch)|t(?:hen|race)|u(?:nset|p(?:date|level|var))|v(?:ariable|wait)|while)\b", KEYWORD, NewState::Push(vec![r"params-in-bracket"])),
        Rule::token_to(r"(?m)\b(append|b(?:gerror|inary)|c(?:d|han|lo(?:ck|se)|oncat)|d(?:de|ict)|e(?:ncoding|of|x(?:ec|it))|f(?:blocked|co(?:nfigure|py)|ile(?:(?:event)?)|lush|ormat)|g(?:ets|lob)|h(?:istory|ttp)|in(?:cr|fo|terp)|join|l(?:a(?:ppend|ssign)|i(?:n(?:dex|sert)|st)|length|oad(?:(?:Tk)?)|r(?:ange|e(?:p(?:eat|lace)|verse))|s(?:e(?:arch|t)|ort))|m(?:ath(?:func|op)|emory|sgcat)|open|p(?:ackage|id|kg(?:::create|_mkIndex)|latform(?:(?:::shell)?)|uts|wd)|re(?:_syntax|ad|fchan|g(?:exp|istry|sub))|s(?:can|eek|o(?:cket|urce)|plit|tring|ubst)|t(?:ell|ime|m)|un(?:known|load))\b", NAME_BUILTIN, NewState::Push(vec![r"params-in-bracket"])),
        Rule::token_to(r"(?m)([\w.-]+)", NAME_VARIABLE, NewState::Push(vec![r"params-in-bracket"])),
        Rule::token_to(r"(?m)#", COMMENT, NewState::Push(vec![r"comment"])),
        Rule::token_to(r"(?m)\(", KEYWORD, NewState::Push(vec![r"paren"])),
        Rule::token_to(r"(?m)\[", KEYWORD, NewState::Push(vec![r"bracket"])),
        Rule::token_to(r"(?m)\{", KEYWORD, NewState::Push(vec![r"brace"])),
        Rule::token_to(r#"(?m)""#, STRING_DOUBLE, NewState::Push(vec![r"string"])),
        Rule::token(r"(?m)(eq|ne|in|ni)\b", OPERATOR_WORD),
        Rule::token(r"(?m)!=|==|<<|>>|<=|>=|&&|\|\||\*\*|[-+~!*/%<>&^|?:]", OPERATOR),
        Rule::token(r"(?m)\s+", WHITESPACE),
        Rule::token(r"(?m)0x[a-fA-F0-9]+", NUMBER_HEX),
        Rule::token(r"(?m)0[0-7]+", NUMBER_OCT),
        Rule::token(r"(?m)\d+\.\d+", NUMBER_FLOAT),
        Rule::token(r"(?m)\d+", NUMBER_INTEGER),
        Rule::token(r"(?m)\$[\w.:-]+", NAME_VARIABLE),
        Rule::token(r"(?m)\$\{[\w.:-]+\}", NAME_VARIABLE),
        Rule::token(r"(?m)[\w.,@:-]+", TEXT),
    ]);
    m.insert(r"comment", vec![
        Rule::token_to(r"(?m).*[^\\]\n", COMMENT, NewState::Pop(1)),
        Rule::token(r"(?m).*\\\n", COMMENT),
    ]);
    Table(m)
}

impl Lexer for TclLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
