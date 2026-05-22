# Vendored upstream test data

Files in this directory are copied (not symlinked) from upstream
repositories so the Rust test suite does not depend on path-traversing
into other workspace folders.

| file | source | upstream commit |
| --- | --- | --- |
| `commonmark.json` | `MyST-Parser/tests/test_commonmark/commonmark.json` (originally from `commonmark/CommonMark` tag `0.29`, dumped via `test/spec_tests.py --dump-tests`) | `aa273e3fe6d1242c686b27c55e23f85370606bdd` |
| `option_parsing.yaml` | `MyST-Parser/tests/test_renderers/fixtures/option_parsing.yaml` | `aa273e3fe6d1242c686b27c55e23f85370606bdd` |
| `option_parsing_errors.yaml` | `MyST-Parser/tests/test_renderers/fixtures/option_parsing_errors.yaml` | `aa273e3fe6d1242c686b27c55e23f85370606bdd` |
| `directive_parsing.txt` | `MyST-Parser/tests/test_renderers/fixtures/directive_parsing.txt` | `aa273e3fe6d1242c686b27c55e23f85370606bdd` |

Refresh procedure: re-run the upstream `spec.sh` against the desired
CommonMark version, copy the resulting JSON here, and update the
commit hash above.
