#!/usr/bin/env python3
"""
Generate reference formatter output from upstream Pygments for byte-parity testing.

This script creates fixtures for each formatter to test Rust implementations
for byte-for-byte compatibility with Pygments.
"""

import json
import sys
from pathlib import Path

# Add Pygments to path
sys.path.insert(0, str(Path(__file__).parent.parent / "pygments"))

from pygments import highlight
from pygments.lexers import PythonLexer
from pygments.formatters import (
    HtmlFormatter, TerminalFormatter, Terminal256Formatter,
    TerminalTrueColorFormatter, LatexFormatter, RtfFormatter,
    GroffFormatter, SvgFormatter,
    get_formatter_by_name,
)
from pygments.token import Token

# Test code samples
TEST_SAMPLES = {
    "simple": "x = 42",
    "string": 'text = "hello world"',
    "comment": "# comment",
    "complex": 'def foo(x):\n    return x + 1  # adds one',
}

def get_tokens_for_sample(code):
    """Get token stream from Python lexer."""
    lexer = PythonLexer()
    tokens = list(lexer.get_tokens(code))
    # Convert tokens to JSON-serializable format
    return [(str(ttype), value) for ttype, value in tokens]

def generate_formatter_output():
    """Generate reference output for all formatters."""
    results = {}
    lexer = PythonLexer()
    
    formatters_to_test = {
        "html": HtmlFormatter(style="default", full=False),
        "terminal": TerminalFormatter(),
        "terminal256": Terminal256Formatter(),
        "terminal16m": TerminalTrueColorFormatter(),
        "latex": LatexFormatter(),
        "rtf": RtfFormatter(),
        "groff": GroffFormatter(),
        "svg": SvgFormatter(),
        "bbcode": get_formatter_by_name("bbcode"),
    }
    
    for sample_name, code in TEST_SAMPLES.items():
        results[sample_name] = {
            "code": code,
            "tokens": get_tokens_for_sample(code),
            "output": {},
        }
        
        for fmt_name, formatter in formatters_to_test.items():
            try:
                output = highlight(code, lexer, formatter)
                results[sample_name]["output"][fmt_name] = output
            except Exception as e:
                results[sample_name]["output"][fmt_name] = f"ERROR: {e}"
    
    return results

def main():
    output = generate_formatter_output()
    
    # Write to JSON
    test_dir = Path(__file__).parent.parent / "tests"
    test_dir.mkdir(exist_ok=True)
    
    output_file = test_dir / "pygments_formatter_fixtures.json"
    with open(output_file, "w") as f:
        json.dump(output, f, indent=2)
    
    print(f"Generated fixtures to {output_file}")
    
    # Print summary
    for sample_name, data in output.items():
        print(f"\n{sample_name}:")
        print(f"  Code: {data['code']!r}")
        print(f"  Tokens: {len(data['tokens'])} tokens")
        for fmt_name, output_text in data["output"].items():
            if output_text.startswith("ERROR"):
                print(f"  {fmt_name}: {output_text}")
            else:
                preview = output_text[:50].replace("\n", "\\n")
                print(f"  {fmt_name}: {len(output_text)} chars → {preview}...")

if __name__ == "__main__":
    main()
