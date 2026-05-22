"""Tests for the Python directive plugin bridge."""

import docutilsrs


def setup_function(_):
    docutilsrs.clear_directives()


def teardown_function(_):
    docutilsrs.clear_directives()


def test_register_and_invoke_directive():
    docutilsrs.register_directive("upper", lambda args, body: body.upper())
    out = docutilsrs.parse_to_pseudoxml(".. upper:: x\n\n   hello world\n")
    assert "HELLO WORLD" in out


def test_plugin_can_return_rst_markup():
    docutilsrs.register_directive(
        "wrap", lambda args, body: "**bold** " + body
    )
    out = docutilsrs.parse_to_pseudoxml(".. wrap:: x\n\n   inner\n")
    assert "<strong>" in out
    assert "bold" in out
    assert "inner" in out


def test_registered_directives_listing():
    docutilsrs.register_directive("a", lambda args, body: body)
    docutilsrs.register_directive("b", lambda args, body: body)
    names = docutilsrs.registered_directives()
    assert sorted(names) == ["a", "b"]


def test_unregister_directive():
    docutilsrs.register_directive("a", lambda args, body: body)
    assert docutilsrs.unregister_directive("a") is True
    assert docutilsrs.unregister_directive("a") is False
    assert "a" not in docutilsrs.registered_directives()


def test_plugin_failure_falls_back_to_comment():
    def boom(args, body):
        raise RuntimeError("nope")

    docutilsrs.register_directive("boom", boom)
    out = docutilsrs.parse_to_pseudoxml(".. boom:: x\n\n   ignored\n")
    # Comment fallback: the body text appears in a <comment>, not as a paragraph.
    assert "<comment" in out
    assert "ignored" in out


def test_unknown_directive_without_plugin_is_comment():
    out = docutilsrs.parse_to_pseudoxml(".. nosuch:: x\n\n   body\n")
    assert "<comment" in out


def test_plugin_feature_advertised():
    assert docutilsrs.supports("plugin:python_directives") is True
