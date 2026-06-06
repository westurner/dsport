//! Lexer alias → lexer instance resolution.
//!
//! Mirrors `pygments.lexers.get_lexer_by_name`. New lexers register
//! their aliases by extending both [`get_lexer_by_name`] and
//! [`native_aliases`] (the latter advertises which aliases the
//! native path handles, so callers can decide whether to short-circuit
//! a PyO3 hop).

use crate::lexer::Lexer;
use crate::lexers::diff::DiffLexer;
use crate::lexers::generated;
use crate::lexers::json::JsonLexer;
use crate::lexers::python::PythonLexer;
use crate::lexers::text::TextLexer;

pub fn get_lexer_by_name(alias: &str) -> Option<Box<dyn Lexer>> {
    match alias {
        "text" | "plain" | "" => Some(Box::new(TextLexer)),
        "python" | "py" | "python3" | "py3" => Some(Box::new(PythonLexer)),
        "json" | "json-object" => Some(Box::new(JsonLexer)),
        "diff" | "udiff" => Some(Box::new(DiffLexer)),
        // --- transpiled (tools/gen_lexer.py) ---
        "ini" | "cfg" | "dosini" => Some(Box::new(generated::ini::IniLexer)),
        "properties" | "jproperties" => {
            Some(Box::new(generated::properties::PropertiesLexer))
        }
        "toml" => Some(Box::new(generated::toml::TomlLexer)),
        "pot" | "po" => Some(Box::new(generated::gettext::GettextLexer)),
        "dpatch" => Some(Box::new(generated::darcs::DarcsLexer)),
        "vctreestatus" => Some(Box::new(generated::vctreestatus::VctreestatusLexer)),
        "groff" | "nroff" | "man" => Some(Box::new(generated::groff::GroffLexer)),
        "bash" | "sh" | "ksh" | "zsh" | "shell" | "openrc" => {
            Some(Box::new(generated::bash::BashLexer))
        }
        "cmake" => Some(Box::new(generated::cmake::CmakeLexer)),
        // --- Phase B: config/markup/template formats ---
        "augeas" => Some(Box::new(generated::augeas::AugeasLexer)),
        "bbcode" => Some(Box::new(generated::bbcode::BbcodeLexer)),
        "desktop" => Some(Box::new(generated::desktop::DesktopLexer)),
        "lighttpd" | "lighty" => Some(Box::new(generated::lighttpd::LighttpdLexer)),
        "pacmanconf" => Some(Box::new(generated::pacmanconf::PacmanconfLexer)),
        "pkgconfig" => Some(Box::new(generated::pkgconfig::PkgconfigLexer)),
        "registry" => Some(Box::new(generated::registry::RegistryLexer)),
        "mozhashpreproc" => Some(Box::new(generated::moz_hashpreproc::MozHashpreprocLexer)),
        "mozpercentpreproc" => Some(Box::new(generated::moz_percentpreproc::MozPercentpreprocLexer)),
        "debian.sources" => Some(Box::new(generated::debian_sources::DebianSourcesLexer)),
        "django" | "jinja" => Some(Box::new(generated::django::DjangoLexer)),
        "ng2" => Some(Box::new(generated::ng2::Ng2Lexer)),
        "cfengine3" | "cf3" => Some(Box::new(generated::cfengine3::Cfengine3Lexer)),
        "cfs" => Some(Box::new(generated::cfs::CfsLexer)),
        // --- Phase A: high-value doc/Sphinx languages ---
        "rust" | "rs" => Some(Box::new(generated::rust::RustLexer)),
        "go" | "golang" => Some(Box::new(generated::go::GoLexer)),
        "javascript" | "js" => Some(Box::new(generated::javascript::JavascriptLexer)),
        "swift" => Some(Box::new(generated::swift::SwiftLexer)),
        "perl" | "pl" => Some(Box::new(generated::perl::PerlLexer)),
        "haskell" | "hs" => Some(Box::new(generated::haskell::HaskellLexer)),
        "elixir" | "ex" | "exs" => Some(Box::new(generated::elixir::ElixirLexer)),
        "typescript" | "ts" => Some(Box::new(generated::typescript::TypescriptLexer)),
        "css" => Some(Box::new(generated::css::CssLexer)),
        "xml" => Some(Box::new(generated::xml::XmlLexer)),
        "sql" => Some(Box::new(generated::sql::SqlLexer)),
        "lua" => Some(Box::new(generated::lua::LuaLexer)),
        "splus" | "s" | "r" => Some(Box::new(generated::r::RLexer)),
        "matlab" => Some(Box::new(generated::matlab::MatlabLexer)),
        "julia" | "jl" => Some(Box::new(generated::julia::JuliaLexer)),
        "clojure" | "clj" => Some(Box::new(generated::clojure::ClojureLexer)),
        "erlang" => Some(Box::new(generated::erlang::ErlangLexer)),
        "nginx" => Some(Box::new(generated::nginx::NginxLexer)),
        "apacheconf" | "aconf" | "apache" => Some(Box::new(generated::apache::ApacheLexer)),
        "powershell" | "pwsh" | "posh" | "ps1" | "psm1" => Some(Box::new(generated::powershell::PowershellLexer)),
        "tex" | "latex" => Some(Box::new(generated::tex::TexLexer)),
        "graphql" => Some(Box::new(generated::graphql::GraphqlLexer)),
        "protobuf" | "proto" => Some(Box::new(generated::protobuf::ProtobufLexer)),
        "scala" => Some(Box::new(generated::scala::ScalaLexer)),
        _ => None,
    }
}

/// Aliases for which a native Rust lexer exists. Sorted by primary
/// name first so the list is stable for snapshotting.
pub fn native_aliases() -> &'static [&'static str] {
    &[
        "text",
        "plain",
        "",
        "python",
        "py",
        "python3",
        "py3",
        "json",
        "json-object",
        "diff",
        "udiff",
        // --- transpiled ---
        "ini",
        "cfg",
        "dosini",
        "properties",
        "jproperties",
        "toml",
        "pot",
        "po",
        "dpatch",
        "vctreestatus",
        "groff",
        "nroff",
        "man",
        "bash",
        "sh",
        "ksh",
        "zsh",
        "shell",
        "openrc",
        "cmake",
        // --- Phase B: config/markup/template formats ---
        "augeas",
        "bbcode",
        "desktop",
        "lighttpd",
        "lighty",
        "pacmanconf",
        "pkgconfig",
        "registry",
        "mozhashpreproc",
        "mozpercentpreproc",
        "debian.sources",
        "django",
        "jinja",
        "ng2",
        "cfengine3",
        "cf3",
        "cfs",
        // --- Phase A ---
        "rust",
        "rs",
        "go",
        "golang",
        "javascript",
        "js",
        "swift",
        "perl",
        "pl",
        "haskell",
        "hs",
        "elixir",
        "ex",
        "exs",
        "typescript",
        "ts",
        "css",
        "xml",
        "sql",
        "lua",
        "splus",
        "s",
        "r",
        "matlab",
        "julia",
        "jl",
        "clojure",
        "clj",
        "erlang",
        "nginx",
        "apacheconf",
        "aconf",
        "apache",
        "powershell",
        "pwsh",
        "posh",
        "ps1",
        "psm1",
        "tex",
        "latex",
        "graphql",
        "protobuf",
        "proto",
        "scala",
    ]
}
