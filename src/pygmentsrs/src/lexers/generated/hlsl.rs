#![allow(clippy::invisible_characters)]

//! AUTO-GENERATED from `pygments.pygments.lexers.graphics:HLSLShaderLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.graphics:HLSLShaderLexer:hlsl

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{NewState, Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: hlsl
pub struct HlslLexer;

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
        Rule::token(r"(?m)#(?:.*\\\n)*.*$", COMMENT_PREPROC),
        Rule::token(r"(?m)//.*$", COMMENT_SINGLE),
        Rule::token(r"(?m)/(\\\n)?[*](.|\n)*?[*](\\\n)?/", COMMENT_MULTILINE),
        Rule::token(r"(?m)\+|-|~|!=?|\*|/|%|<<|>>|<=?|>=?|==?|&&?|\^|\|\|?", OPERATOR),
        Rule::token(r"(?m)[?:]", OPERATOR),
        Rule::token(r"(?m)\bdefined\b", OPERATOR),
        Rule::token(r"(?m)[;{}(),.\[\]]", PUNCTUATION),
        Rule::token(r"(?m)[+-]?\d*\.\d+([eE][-+]?\d+)?f?", NUMBER_FLOAT),
        Rule::token(r"(?m)[+-]?\d+\.\d*([eE][-+]?\d+)?f?", NUMBER_FLOAT),
        Rule::token(r"(?m)0[xX][0-9a-fA-F]*", NUMBER_HEX),
        Rule::token(r"(?m)0[0-7]*", NUMBER_OCT),
        Rule::token(r"(?m)[1-9][0-9]*", NUMBER_INTEGER),
        Rule::token_to(r#"(?m)""#, STRING, NewState::Push(vec![r"string"])),
        Rule::token(r"(?m)\b(NULL|asm(?:(?:_fragment)?)|break|c(?:ase|buffer|entroid|lass|o(?:lumn_major|mpile(?:(?:_fragment)?)|n(?:st|tinue)))|d(?:efault|iscard|o)|e(?:lse|x(?:port|tern))|f(?:or|xgroup)|g(?:loballycoherent|roupshared)|i(?:n(?:line|out|terface)|[fn])|line(?:(?:a(?:dj|r))?)|n(?:amespace|o(?:interpolation|perspective))|out|p(?:a(?:ckoffset|ss)|ixelfragment|oint|recise)|r(?:e(?:gister|turn)|ow_major)|s(?:ample(?:(?:r)?)|hared|t(?:at(?:eblock(?:(?:_state)?)|ic)|ruct)|witch)|t(?:buffer|e(?:chnique(?:(?:1(?:[01]))?)|xture)|riangle(?:(?:adj)?)|ypedef)|uniform|v(?:ertexfragment|olatile)|while)\b", KEYWORD),
        Rule::token(r"(?m)\b((?:fals|tru)e)\b", KEYWORD_CONSTANT),
        Rule::token(r"(?m)\b(auto|c(?:atch|har|onst_cast)|d(?:elete|ynamic_cast)|e(?:num|xplicit)|friend|goto|long|mutable|new|operator|p(?:r(?:ivate|otected)|ublic)|reinterpret_cast|s(?:hort|i(?:gned|zeof)|tatic_cast)|t(?:emplate|h(?:is|row)|ry|ypename)|u(?:n(?:ion|signed)|sing)|virtual)\b", KEYWORD_RESERVED),
        Rule::token(r"(?m)\b(B(?:lendState|(?:(?:yteAddressB)?)uffer)|ComputeShader|D(?:epthStencil(?:State|View)|omainShader)|GeometryShader|HullShader|InputPatch|LineStream|OutputPatch|P(?:ixelShader|ointStream)|R(?:W(?:B(?:(?:(?:yteAddressB)?)uffer)|StructuredBuffer|Texture(?:1D(?:(?:Array)?)|2D(?:(?:Array)?)|3D))|asterizer(?:Ordered(?:B(?:(?:(?:yteAddressB)?)uffer)|StructuredBuffer|Texture(?:1D(?:(?:Array)?)|2D(?:(?:Array)?)|3D))|State)|enderTargetView)|S(?:ampler(?:(?:(?:Comparison)?)State)|tructuredBuffer)|T(?:exture(?:1D(?:(?:Array)?)|2D(?:(?:Array|MS(?:(?:Array)?))?)|3D|Cube(?:(?:Array)?))|riangleStream)|VertexShader|dword|matrix|s(?:norm|tring)|un(?:orm|signed)|v(?:ector|oid))\b", KEYWORD_TYPE),
        Rule::token(r"(?m)\b(bool|double|float|half|(?:in|min1(?:0floa|2in|6(?:floa|(?:(?:u)?)in))|uin)t)([1-4](x[1-4])?)?\b", KEYWORD_TYPE),
        Rule::token(r"(?m)\b(A(?:llMemoryBarrier(?:(?:WithGroupSync)?)|ppendStructuredBuffer)|C(?:heckAccessFullyMapped|o(?:(?:mpileShad|nsumeStructuredBuff)er))|D(?:3DCOLORtoUBYTE4|eviceMemoryBarrier(?:(?:WithGroupSync)?))|EvaluateAttribute(?:At(?:Centroid|Sample)|Snapped)|G(?:etRenderTargetSample(?:Count|Position)|lobalOrderedCountIncrement|roupMemoryBarrier(?:(?:WithGroupSync)?))|Interlocked(?:A(?:(?:[dn])d)|Compare(?:(?:Exchang|Stor)e)|Exchange|M(?:ax|in)|(?:O|Xo)r)|Process(?:2DQuadTessFactors(?:Avg|M(?:ax|in))|IsolineTessFactors|QuadTessFactors(?:Avg|M(?:ax|in))|TriTessFactors(?:Avg|M(?:ax|in)))|Quad(?:ReadLaneAt|Swap(?:[XY]))|Wave(?:A(?:ll(?:Bit(?:And|(?:O|Xo)r)|Equal|M(?:ax|in)|Product|Sum|True)|nyTrue)|Ballot|Get(?:Lane(?:Count|Index)|OrderedIndex)|IsHelperLane|Once|Prefix(?:Product|Sum)|Read(?:FirstLane|LaneAt))|a(?:b(?:ort|s)|cos|ll|ny|s(?:double|float|in(?:(?:t)?)|uint(?:(?:)?))|tan(?:(?:2)?))|c(?:eil|l(?:(?:am|i)p)|o(?:s(?:(?:h)?)|untbits)|ross)|d(?:d(?:(?:x_(?:coars|fin)|y_(?:coars|fin))e|[xy])|e(?:grees|terminant)|istance|(?:[os])t)|e(?:rrorf|xp(?:(?:2)?))|f(?:16tof32|32tof16|aceforward|irstbit(?:high|low)|loor|m(?:a|od)|r(?:ac|exp)|width)|is(?:finite|inf|nan)|l(?:dexp|e(?:ngth|rp)|it|og(?:(?:10|2)?))|m(?:a(?:[dx])|in|odf|sad4|ul)|no(?:(?:is|rmaliz)e)|p(?:ow|rintf)|r(?:adians|cp|e(?:f(?:(?:le|ra)ct)|versebits)|ound|sqrt)|s(?:aturate|i(?:gn|n(?:(?:cos|h)?))|moothstep|qrt|tep)|t(?:an(?:(?:h)?)|ex(?:1D(?:(?:(?:bias|grad|lod|proj)?)?)|2D(?:(?:(?:bias|grad|lod|proj)?)?)|3D(?:(?:(?:bias|grad|lod|proj)?)?)|CUBE(?:(?:(?:bias|grad|lod|proj)?)?))|r(?:anspose|unc)))\b", NAME_BUILTIN),
        Rule::token(r"(?m)\b(SV_(?:C(?:lipDistance(?:(?:[01])?)|overage|ull(?:Distance(?:[01])|distance))|D(?:epth(?:(?:(?:Greater|Less)Equal)?)|ispatchThreadID|omainLocation)|G(?:SInstanceID|roup(?:I(?:D|ndex)|ThreadID))|I(?:n(?:nerCoverage|s(?:ideTessFactor|tanceID))|sFrontFace)|OutputControlPointID|P(?:osition|rimitiveID)|RenderTargetArrayIndex|S(?:ampleIndex|tencilRef)|TessFactor|V(?:ertexID|iewportArrayIndex)))\b", NAME_DECORATOR),
        Rule::token(r"(?m)\bSV_Target[0-7]?\b", NAME_DECORATOR),
        Rule::token(r"(?m)\b(allow_uav_condition|branch|call|domain|earlydepthstencil|f(?:astopt|latten|orcecase)|instance|loop|maxtessfactor|numthreads|output(?:controlpoints|topology)|pa(?:rtitioning|tchconstantfunc)|unroll)\b", NAME_DECORATOR),
        Rule::token(r"(?m)[a-zA-Z_]\w*", NAME),
        Rule::token(r"(?m)\\$", COMMENT_PREPROC),
        Rule::token(r"(?m)\s+", WHITESPACE),
    ]);
    m.insert(r"string", vec![
        Rule::token_to(r#"(?m)""#, STRING, NewState::Pop(1)),
        Rule::token(r#"(?m)\\([\\abfnrtv"\']|x[a-fA-F0-9]{2,4}|u[a-fA-F0-9]{4}|U[a-fA-F0-9]{8}|[0-7]{1,3})"#, STRING_ESCAPE),
        Rule::token(r#"(?m)[^\\"\n]+"#, STRING),
        Rule::token(r"(?m)\\\n", STRING),
        Rule::token(r"(?m)\\", STRING),
    ]);
    Table(m)
}

impl Lexer for HlslLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
