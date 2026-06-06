"""Byte-parity tests for the transpiled (`tools/gen_lexer.py`) lexers.

Each native Rust lexer in ``pygmentsrs/src/lexers/generated/`` must
produce a `(repr(ttype), value)` stream identical to the vendored
``pygments`` lexer it was generated from. Mirrors the strategy used by
``test_pygments_json_lexer.py`` and the ``code_block_python_*`` fixtures.

If a generated lexer ever drifts from upstream, this test fails and the
fix is to regenerate (or to mark the lexer bridge-only).
"""

from __future__ import annotations

import importlib

import pytest

pytest.importorskip("pygments")
import pygmentsrs


# alias -> (module, ClassName, [sample inputs])
GENERATED = {
    "ini": (
        "pygments.lexers.configs",
        "IniLexer",
        [
            "[section]\nkey = value\n",
            "; comment\n# also comment\n[s]\nk: v\n",
            'name = "quoted value"\n',
            "a = 1 \\\n    continued\n",
            "[empty]\n\nlonely\n",
        ],
    ),
    "properties": (
        "pygments.lexers.configs",
        "PropertiesLexer",
        [
            "a.b.c = value\n",
            "# comment\n! also comment\nkey:val\n",
            "escaped\\=key = v\n",
            "multi = one \\\n   two\n",
            "key value without separator\n",
        ],
    ),
    "toml": (
        "pygments.lexers.configs",
        "TOMLLexer",
        [
            '[table]\nkey = "value"\n',
            "n = 42\nf = 3.14\nb = true\n",
            "# comment\n[a.b.c]\nx = [1, 2, 3]\n",
            'date = 1979-05-27\nname = "x"\n',
            "arr = [\n  1,\n  2,\n]\n",
        ],
    ),
    "pot": (
        "pygments.lexers.textfmts",
        "GettextLexer",
        [
            'msgid "hello"\nmsgstr "bonjour"\n',
            '# translator comment\nmsgid ""\nmsgstr ""\n',
            '#: source.c:42\nmsgid "x"\nmsgstr "y"\n',
        ],
    ),
    "dpatch": (
        "pygments.lexers.diff",
        "DarcsPatchLexer",
        [
            "hunk ./file 1\n+added\n-removed\n",
            "addfile ./newfile\n",
        ],
    ),
    "vctreestatus": (
        "pygments.lexers.console",
        "VCTreeStatusLexer",
        [
            "? untracked\nA added\nM modified\nD deleted\n",
            "  C conflict\n",
        ],
    ),
    "groff": (
        "pygments.lexers.text",
        "GroffLexer",
        [
            ".TH TITLE 1\n.SH NAME\nfoo \\- bar\n",
            ".B bold\n.I italic\n\\fBmanual\\fP\n",
            ".\\\" a comment\nplain text\n",
        ],
    ),
    "bash": (
        "pygments.lexers.shell",
        "BashLexer",
        [
            "echo hi\nx=1\nif true; then echo $x; fi\n",
            # heredoc exercises the `\\2` backreference (fancy-regex).
            "cat <<EOF\nbody $v\nEOF\nrest\n",
            "for i in 1 2 3; do\n  echo $i\ndone\n",
            "func() {\n  local a=$1\n  return 0\n}\n",
            "x=${y:-default}\necho \"$x\"\n",
        ],
    ),
    "cmake": (
        "pygments.lexers.make",
        "CMakeLexer",
        [
            "set(X 1)\n# comment\nproject(Foo)\n",
            # bracket-argument exercises the `(?P=level)` named backref.
            "message([[\nmulti\n]])\n",
            "if(WIN32)\n  add_library(a)\nendif()\n",
        ],
    ),
    # --- Phase A: high-value doc/Sphinx languages (verified passing) ---
    "go": (
        "pygments.lexers.go",
        "GoLexer",
        [
            "package main\nimport \"fmt\"\nfunc main() {}\n",
            '// comment\nvar x int = 42\nconst s = "str"\n',
            "func (r *Receiver) Method() string {}\n",
            "defer close(ch)\nselect {}\n",
            "for i := 0; i < 10; i++ { }\n",
        ],
    ),
    "typescript": (
        "pygments.lexers.javascript",
        "TypeScriptLexer",
        [
            "interface Foo { bar: string; }\n",
            "function add(a: number, b: number): number { return a + b; }\n",
            "type ID = string | number;\n",
            "class C<T> { value: T; }\n",
            "async function f(): Promise<void> {}\n",
        ],
    ),
    "css": (
        "pygments.lexers.css",
        "CssLexer",
        [
            "body { margin: 0; padding: 10px; }\n",
            "/* comment */\n#id { color: red; }\n",
            ".class { display: flex; }\n",
            "@media (max-width: 600px) { }\n",
            ":hover { opacity: 0.8; }\n",
        ],
    ),
    "xml": (
        "pygments.lexers.html",
        "XmlLexer",
        [
            '<?xml version="1.0"?>\n<root><tag>text</tag></root>\n',
            '<!-- comment -->\n<elem attr="value"/>\n',
            '<tag a="1" b="2">content</tag>\n',
            '<?processing-instruction?>\n<cdata><![CDATA[data]]></cdata>\n',
            '<ns:element xmlns:ns="uri">mixed &amp; text</ns:element>\n',
        ],
    ),
    "sql": (
        "pygments.lexers.sql",
        "SqlLexer",
        [
            "SELECT * FROM users WHERE id = 1;\n",
            "-- comment\n/* block */\nINSERT INTO t VALUES (1, 'x');\n",
            "UPDATE t SET x = 1 WHERE y = 2;\n",
            'DELETE FROM t; CREATE TABLE x (id INT, name VARCHAR(50));\n',
            "SELECT COUNT(*) as cnt, name FROM users GROUP BY name;\n",
        ],
    ),
    "lua": (
        "pygments.lexers.scripting",
        "LuaLexer",
        [
            "function hello(x)\n  return x * 2\nend\n",
            "-- comment\nlocal t = {1, 2, 3}\n",
            "if x > 0 then\n  print(x)\nend\n",
            'for i = 1, 10 do\n  print(i)\nend\n',
            "t = {a = 1, b = 2}\nprint(t[\"a\"])\n",
        ],
    ),
    "r": (
        "pygments.lexers.r",
        "SLexer",
        [
            "x <- c(1, 2, 3)\ny <- x * 2\n",
            "# comment\nfunc <- function(x) { return(x + 1) }\n",
            "if (x > 0) { print(x) }\n",
            "for (i in 1:10) { print(i) }\n",
            'df <- data.frame(a = c(1, 2), b = c("x", "y"))\n',
        ],
    ),
    "matlab": (
        "pygments.lexers.matlab",
        "MatlabLexer",
        [
            "x = [1, 2, 3];\ny = x * 2;\n",
            "% comment\nfunction out = add(a, b)\n  out = a + b;\nend\n",
            "if x > 0\n  disp(x)\nend\n",
            "for i = 1:10\n  fprintf('%d\\n', i)\nend\n",
            "'string' \"char vector\"\n",
        ],
    ),
    "julia": (
        "pygments.lexers.julia",
        "JuliaLexer",
        [
            "function add(x::Int, y::Int)::Int\n  return x + y\nend\n",
            "# comment\nconst π = 3.14159\n",
            "for i in 1:10\n  println(i)\nend\n",
            "a = [1, 2, 3]\nb = a .* 2\n",
            'struct Point\n  x::Float64\n  y::Float64\nend\n',
        ],
    ),
    "clojure": (
        "pygments.lexers.jvm",
        "ClojureLexer",
        [
            "(defn add [x y] (+ x y))\n",
            "; comment\n[1 2 3]\n{:a 1 :b 2}\n",
            "(def x 42)\n(let [y (inc x)] y)\n",
            "(map #(* % 2) [1 2 3])\n",
            "(for [i (range 10)] i)\n",
        ],
    ),
    "erlang": (
        "pygments.lexers.erlang",
        "ErlangLexer",
        [
            "add(X, Y) -> X + Y.\n",
            "% comment\nfact(0) -> 1;\nfact(N) -> N * fact(N-1).\n",
            "hello() ->\n  io:fwrite(\"hello~n\").\n",
            "loop(N) when N > 0 -> loop(N-1).\n",
            "list_add(Lst, Val) -> [Val | Lst].\n",
        ],
    ),
    "nginx": (
        "pygments.lexers.configs",
        "NginxConfLexer",
        [
            "server {\n  listen 80;\n  server_name example.com;\n}\n",
            "# comment\nworker_processes auto;\n",
            "http {\n  upstream backend {\n    server 127.0.0.1:8080;\n  }\n}\n",
            "location / {\n  proxy_pass http://backend;\n}\n",
            "if ($request_method = POST) { return 405; }\n",
        ],
    ),
    "apache": (
        "pygments.lexers.configs",
        "ApacheConfLexer",
        [
            "<VirtualHost *:80>\n  ServerName example.com\n</VirtualHost>\n",
            "# comment\nListen 80\n",
            "<Directory /var/www>\n  Options Indexes\n</Directory>\n",
            "<If \"%{HTTP_HOST} == 'example.com'\">\n</If>\n",
            "LoadModule rewrite_module modules/mod_rewrite.so\n",
        ],
    ),
    "powershell": (
        "pygments.lexers.shell",
        "PowerShellLexer",
        [
            "Write-Host \"hello\"\n",
            "# comment\n$x = 42\nGet-Content file.txt\n",
            "function Foo { param($x) Write-Output $x }\n",
            "foreach ($i in 1..10) { Write-Host $i }\n",
            "$arr = @(1, 2, 3)\n$hash = @{a = 1; b = 2}\n",
        ],
    ),
    "tex": (
        "pygments.lexers.markup",
        "TexLexer",
        [
            "\\documentclass{article}\n\\begin{document}\nhello\n\\end{document}\n",
            "% comment\n\\section{Title}\n\\textbf{bold} \\textit{italic}\n",
            "\\usepackage{amssymb}\n$x^2 + y^2 = z^2$\n",
            "\\[\\sum_{i=1}^{n} i\\]\n",
            "\\def\\macro{expansion}\n\\macro\n",
        ],
    ),
    "graphql": (
        "pygments.lexers.graphql",
        "GraphQLLexer",
        [
            "query GetUser {\n  user(id: 1) {\n    name\n    email\n  }\n}\n",
            "# comment\nmutation CreateUser {\n  createUser(name: \"x\") {\n    id\n  }\n}\n",
            "type User {\n  id: ID!\n  name: String!\n}\n",
            "query {\n  users(limit: 10) @cached {\n    id\n  }\n}\n",
            "subscription OnUserUpdate {\n  userUpdated {\n    id\n  }\n}\n",
        ],
    ),
    "protobuf": (
        "pygments.lexers.dsls",
        "ProtoBufLexer",
        [
            "syntax = \"proto3\";\npackage foo;\n",
            "message User {\n  int32 id = 1;\n  string name = 2;\n}\n",
            "service UserService {\n  rpc GetUser(Id) returns (User);\n}\n",
            "// comment\nrequired bool flag = 1;\noptional string text = 2;\n",
            "enum Status {\n  UNKNOWN = 0;\n  ACTIVE = 1;\n}\n",
        ],
    ),
    "scala": (
        "pygments.lexers.jvm",
        "ScalaLexer",
        [
            "def add(x: Int, y: Int): Int = x + y\n",
            "// comment\nval x: List[Int] = List(1, 2, 3)\n",
            "class Point(val x: Double, val y: Double)\n",
            "for (i <- 1 to 10) println(i)\n",
            "case class User(name: String, age: Int)\n",
        ],
    ),
    # --- Phase B: config/markup/template formats ---
    "augeas": (
        "pygments.lexers.configs",
        "AugeasLexer",
        [
            "set /files/etc/passwd/*/uid 42\n",
            "# comment\nadd /files/etc/fstab/1 /dev/sda1\n",
            "rm /files/etc/hostname\n",
            "move /files/etc/shadow /files/etc/shadow.bak\n",
            "save\n",
        ],
    ),
    "bbcode": (
        "pygments.lexers.markup",
        "BBCodeLexer",
        [
            "[b]bold[/b] [i]italic[/i]\n",
            "[color=red]colored text[/color]\n",
            "[url=http://example.com]link[/url]\n",
            "[size=14]bigger[/size]\n",
            "[quote]quoted[/quote]\n",
        ],
    ),
    "cfengine3": (
        "pygments.lexers.configs",
        "Cfengine3Lexer",
        [
            "body common control {\n  bundlesequence => { \"main\" };\n}\n",
            "# comment\nbundle agent main {\n",
            "vars:\n  \"x\" string => \"value\";\n",
            "commands:\n  \"/bin/echo\" args => \"hello\";\n",
            "reports:\n  \"Report text\";\n",
        ],
    ),
    "cfs": (
        "pygments.lexers.templates",
        "ColdfusionLexer",
        [
            "<cfset x = 1>\n",
            "<cfif condition>yes</cfif>\n",
            "<cfloop from=\"1\" to=\"10\" index=\"i\">\n  #i#\n</cfloop>\n",
            "<cffunction name=\"foo\">\n</cffunction>\n",
            "<cfoutput>#variable#</cfoutput>\n",
        ],
    ),
    "debian.sources": (
        "pygments.lexers.installers",
        "DebianSourcesLexer",
        [
            "deb http://example.com/debian bullseye main\n",
            "# comment\ndeb-src http://example.com/debian bullseye main\n",
            "deb [signed-by=/usr/share/keyrings/archive.gpg] http://example.com\n",
            "Signed-By: /usr/share/keyrings/example.gpg\n",
        ],
    ),
    "desktop": (
        "pygments.lexers.configs",
        "DesktopLexer",
        [
            "[Desktop Entry]\nName=Foo\nExec=foo\n",
            "Type=Application\nIcon=foo\n",
            "Categories=Utility;\n",
            "Version=1.0\n",
            "Comment=A comment\n",
        ],
    ),
    "django": (
        "pygments.lexers.templates",
        "DjangoLexer",
        [
            "{{ variable }}\n",
            "{% if condition %}yes{% endif %}\n",
            "{% for item in list %}{{ item }}{% endfor %}\n",
            "{% load static %}\n",
            "{% block content %}{% endblock %}\n",
        ],
    ),
    "lighttpd": (
        "pygments.lexers.configs",
        "LighttpdConfLexer",
        [
            "server.port = 8080\n",
            "# comment\nserver.document-root = \"/var/www\"\n",
            "$HTTP[\"host\"] == \"example.com\" { server.document-root = \"/tmp\" }\n",
            "mimetype.assign = ( \".html\" => \"text/html\" )\n",
            "url.rewrite-once = ( \"^/(.*)$\" => \"/index.php?q=$1\" )\n",
        ],
    ),
    "mozhashpreproc": (
        "pygments.lexers.markup",
        "MozPreprocHashLexer",
        [
            "#define SYMBOL value\n",
            "# comment\n#ifdef SYMBOL\n",
            "#else\n#endif\n",
            "#include \"header.h\"\n",
            "#undef SYMBOL\n",
        ],
    ),
    "mozpercentpreproc": (
        "pygments.lexers.markup",
        "MozPreprocPercentLexer",
        [
            "%define SYMBOL value\n",
            "% comment\n%ifdef SYMBOL\n",
            "%else\n%endif\n",
            "%include \"file.inc\"\n",
            "%undef SYMBOL\n",
        ],
    ),
    "ng2": (
        "pygments.lexers.templates",
        "Angular2Lexer",
        [
            "<div>{{ variable }}</div>\n",
            "<button (click)=\"onClick()\">Click</button>\n",
            "<input [(ngModel)]=\"field\">\n",
            "<div *ngIf=\"condition\">show</div>\n",
            "<div [ngClass]=\"classes\"></div>\n",
        ],
    ),
    "pacmanconf": (
        "pygments.lexers.configs",
        "PacmanConfLexer",
        [
            "[options]\nHoldPkg = pacman glibc\n",
            "# comment\nCleanMethod = KeepInstalled\n",
            "[core]\nServer = https://mirrors.example.com/$repo/os/$arch\n",
            "DBPath = /var/lib/pacman/\n",
            "CacheDir = /var/cache/pacman/pkg/\n",
        ],
    ),
    "pkgconfig": (
        "pygments.lexers.configs",
        "PkgConfigLexer",
        [
            "prefix=/usr\nlibdir=${prefix}/lib\n",
            "# comment\nName: Foo\nVersion: 1.0\n",
            "Description: A library\nRequires: bar >= 1.0\n",
            "Libs: -L${libdir} -lfoo\n",
            "Cflags: -I${prefix}/include\n",
        ],
    ),
    "registry": (
        "pygments.lexers.configs",
        "RegeditLexer",
        [
            "[HKEY_LOCAL_MACHINE\\\\Software]\n\"Value\"=\"Data\"\n",
            "; comment\n[HKEY_CURRENT_USER\\\\Control Panel]\n",
            "\"string\"=\"data\"\n\"dword\"=dword:00000001\n",
            "\"hex\"=hex:01,02,03\n",
        ],
    ),
    # --- Phase B batch 2: more configs, installers, templates ---
    "debcontrol": (
        "pygments.lexers.installers",
        "DebianControlLexer",
        [
            "Package: foo\nVersion: 1.0-1\n",
            "Maintainer: John Doe <john@example.com>\n",
            "Architecture: amd64\nDepends: bar (>= 1.0)\n",
            "Description: A package\n A longer description\n",
            "# comment\nPriority: optional\n",
        ],
    ),
    "debsources": (
        "pygments.lexers.installers",
        "SourcesListLexer",
        [
            "deb http://example.com/debian bullseye main contrib\n",
            "# comment\ndeb-src http://example.com/debian bullseye main\n",
            "deb http://security.debian.org bullseye-security main\n",
            "deb [arch=amd64] http://example.com bullseye main\n",
        ],
    ),
    "kconfig": (
        "pygments.lexers.configs",
        "KconfigLexer",
        [
            "menuconfig FOO\n  bool \"Enable Foo\"\n",
            "# comment\nif FOO\n  config BAR\n    string \"Bar\"\nendif\n",
            "depends on FOO && BAR\n",
            "default y if X\n",
            "help\n  This is help text\n",
        ],
    ),
    "systemd": (
        "pygments.lexers.configs",
        "SystemdLexer",
        [
            "[Unit]\nDescription=Service\n",
            "[Service]\nType=simple\nExecStart=/usr/bin/foo\n",
            "Restart=always\nRestartSec=10\n",
            "[Install]\nWantedBy=multi-user.target\n",
            "# comment\nAfter=network.target\n",
        ],
    ),
    "termcap": (
        "pygments.lexers.configs",
        "TermcapLexer",
        [
            "vt100|xterm:\n  :do=^J:\n  :up=\\E[A:\n",
            "# comment\n:ce=\\E[K:\n",
            ":cm=\\E[%i%d;%dH:\n",
            ":cl=\\E[H\\E[2J:\n",
            ":bs:\n",
        ],
    ),
    "terminfo": (
        "pygments.lexers.configs",
        "TerminfoLexer",
        [
            "xterm|X11 terminal emulator\n",
            "\tcols#80, lines#24,\n",
            "\tcup=\\E[%i%p1%d;%p2%dH,\n",
            "# comment\t\n",
            "\tclear=\\E[H\\E[J,\n",
        ],
    ),
    "twig": (
        "pygments.lexers.templates",
        "TwigLexer",
        [
            "{{ variable }}\n",
            "{% if condition %}yes{% endif %}\n",
            "{% for item in items %}{{ item }}{% endfor %}\n",
            "{% block content %}default{% endblock %}\n",
            "{{ name | upper }}\n",
        ],
    ),
    "velocity": (
        "pygments.lexers.templates",
        "VelocityLexer",
        [
            "$variable\n",
            "#if($condition)yes#end\n",
            "#foreach($item in $items)$item#end\n",
            "#define($macro)content#end\n",
            "$!{name.toUpperCase()}\n",
        ],
    ),
    # --- Phase E transpilable batch (bitbake, cel, purescript) ---
    "bitbake": (
        "pygments.lexers.bitbake",
        "BitBakeLexer",
        [
            "DESCRIPTION = \"A simple recipe\"\nPV = \"1.0\"\nSRC_URI = \"file://main.c\"\n",
            "# comment\nPKGNAME := \"${PN}-extra\"\n",
            "FILES_${PN} += \"${bindir}/foo\"\n",
            "BBCLASSEXTEND = \"native nativesdk\"\n",
            "do_install() {\n    install -d ${D}${bindir}\n}\n",
        ],
    ),
    "cel": (
        "pygments.lexers.cel",
        "CELLexer",
        [
            "1 + 2\n",
            "x == 'hello' && y > 0\n",
            "request.auth.claims.email == 'user@example.com'\n",
            "size(list) > 0 ? list[0] : 'default'\n",
            "/* block comment */ // line comment\n42\n",
        ],
    ),
    "purescript": (
        "pygments.lexers.purescript",
        "PureScriptLexer",
        [
            "module Main where\nimport Prelude\nmain = log \"Hello\"\n",
            "data Maybe a = Nothing | Just a\n",
            "-- comment\nadd :: Int -> Int -> Int\nadd x y = x + y\n",
            "type alias Name = String\n",
            "foreign import f :: String -> Effect Unit\n",
        ],
    ),
    # --- adl / csharp (previously excluded, now fixed with NFA bound rewrite) ---
    "adl": (
        "pygments.lexers.archetype",
        "AdlLexer",
        [
            "archetype (adl_version=2.0.6; rm_release=1.0.3)\n    openEHR-EHR-OBSERVATION.blood_pressure.v1.0.0\n",
            "-- comment\nlanguage\n    original_language = <[ISO_639-1::en]>\n",
            "description\n    original_author = <\n        [\"name\"] = <\"Test\">\n    >\n",
        ],
    ),
    "csharp": (
        "pygments.lexers.dotnet",
        "CSharpLexer",
        [
            "using System;\nnamespace N { class C { static void Main() { Console.WriteLine(\"hi\"); } } }\n",
            "// comment\npublic async Task<int> Foo() { return await bar; }\n",
            "var x = new List<string> { \"a\", \"b\" };\n",
            "interface IFoo { string Name { get; set; } }\n",
            "record Point(int X, int Y);\n",
        ],
    ),
    # --- Phase E4: dispatch code-block lexers ---
    "markdown": (
        "pygments.lexers.markup",
        "MarkdownLexer",
        [
            "# Heading\n\nParagraph text.\n",
            "```python\nprint('hello')\n```\n",
            "**bold** and *italic*\n",
            "```rust\nfn main() {}\n```\n",
            "> blockquote\n\n- list item\n",
        ],
    ),
    "restructuredtext": (
        "pygments.lexers.markup",
        "RstLexer",
        [
            "Title\n=====\n\nParagraph text.\n",
            ".. code-block:: python\n\n   x = 1\n   y = 2\n",
            ".. note::\n\n   A note here.\n",
            ".. code-block:: rust\n\n   fn main() {}\n",
            "``inline code`` and **bold**.\n",
        ],
    ),
    "tid": (
        "pygments.lexers.markup",
        "TiddlyWiki5Lexer",
        [
            "```python\nprint('hello')\n```\n",
            "!Heading\nParagraph text.\n",
            "```javascript\nconsole.log('hi');\n```\n",
        ],
    ),
}


def _upstream(module: str, classname: str, src: str) -> list[tuple[str, str]]:
    try:
        mod = importlib.import_module(module)
    except ModuleNotFoundError:
        pytest.skip(f"Module {module} not available in installed pygments (vendored lexer)")
    cls = getattr(mod, classname)
    return [(repr(t), v) for _idx, t, v in cls().get_tokens_unprocessed(src)]


def _native(alias: str, src: str) -> list[tuple[str, str]]:
    pairs = pygmentsrs.lex(alias, src, backend="rust")
    assert pairs is not None, f"{alias} should be a native pygmentsrs alias"
    return [(repr_t, val) for repr_t, val in pairs]


def _cases() -> list[tuple[str, str, str, str]]:
    out = []
    for alias, (module, classname, samples) in GENERATED.items():
        for i, src in enumerate(samples):
            out.append((alias, module, classname, src, i))
    return out


@pytest.mark.parametrize(
    "alias,module,classname,src,idx",
    [(a, m, c, s, i) for (a, m, c, s, i) in _cases()],
    ids=[f"{a}-{i}" for (a, _m, _c, _s, i) in _cases()],
)
def test_generated_lexer_byte_parity(
    alias: str, module: str, classname: str, src: str, idx: int
) -> None:
    assert _native(alias, src) == _upstream(module, classname, src), (
        f"{alias} sample #{idx} diverged from upstream"
    )


@pytest.mark.parametrize("alias", sorted(GENERATED))
def test_generated_alias_is_native(alias: str) -> None:
    assert pygmentsrs.has_native_lexer(alias)
    assert alias in pygmentsrs.native_aliases()


def test_generated_routes_through_auto_backend() -> None:
    # Auto backend must reach the native lexer (not fall through to the
    # python bridge) for a generated alias.
    src = "[s]\nk = 1\n"
    assert pygmentsrs.lex("ini", src, backend="auto") == pygmentsrs.lex(
        "ini", src, backend="rust"
    )
