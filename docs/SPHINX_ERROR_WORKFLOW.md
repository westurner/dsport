# Sphinx Error Workflow

sphinxdocrs can record native build output and review diagnostics from the
command line. The storage layer is also available to Rust callers as
`sphinxdocrs::error_log`.

## Record a Native Build

Pass `--error-db` to a native `sphinx-build-rs` invocation:

```text
sphinx-build-rs --error-db .sphinx-errors.sqlite -b html docs build/html
```

Each invocation creates one `SphinxBuild` row. Its `command`, `logpath`, and
`logcontent` preserve the invocation metadata and the diagnostics collected by
the native build. Warnings and errors become `ErrorMessage` rows.

## Import Existing Logs

Text logs from either Sphinx or sphinxdocrs are accepted. Sphinx locations such
as `/path/index.rst:12:4: WARNING: message` and sphinxdocrs lines such as
`WARNING: guide: message` are both recognized:

```text
sphinx-errors-rs --format text --db .sphinx-errors.sqlite sphinx.log
```

JSON may be an array, a single object, a wrapper with an `errors`, `warnings`,
`messages`, or `diagnostics` array, or JSON Lines. Records accept `message` or
`msg` and location fields such as `path`, `filepath`, `line`, `column`, and
`location`:

```text
sphinx-errors-rs --format json --db .sphinx-errors.sqlite diagnostics.json
```

## Review Messages

List pending messages from a database:

```text
sphinx-errors-rs .sphinx-errors.sqlite
```

Run the reusable interactive workflow with `-i` or `--interactive`:

```text
sphinx-errors-rs --interactive .sphinx-errors.sqlite
sphinx-errors-rs -i --build-id 3 .sphinx-errors.sqlite
```

The per-message commands are:

- `e` opens the source path in `VISUAL` or `EDITOR` and marks the message
  `edited` after the editor exits successfully.
- `s` marks the message `skipped`.
- `n` and `p` move to the next and previous message without changing status.
- `q` exits while retaining all statuses already written.

By default, only messages whose `status` is the empty string are shown. Use
`--all` to review or list messages with an existing status. The database adds
indexes for build, status, path, and build/status lookups.
