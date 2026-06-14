# VS Code Workspace Setup

This repository includes a multi-root VS Code workspace configuration that allows you to work with all Rust crates as separate repositories while keeping them organized within the monorepo.

## Opening the Workspace

To open this workspace in VS Code, open the `dsport.code-workspace` file:

```bash
code dsport.code-workspace
```

Or, from within VS Code:
1. File > Open Workspace from File
2. Select `dsport.code-workspace`

## Workspace Folders

The workspace includes the following root folders:

- **dsport (root)** - The main monorepo root
- **jinja2rs** - Jinja2 Rust port
- **docutilsrs** - Docutils Rust port  
- **sphinxdocrs** - Sphinx Rust port
- **pygmentsrs** - Pygments Rust port

Each folder appears as a separate repository in VS Code's file explorer and source control view, making it easier to navigate and work with individual crates.

## View Repositories in VS Code

To view all repositories in the workspace:

1. Open the **Source Control** view (Ctrl+Shift+G / Cmd+Shift+G)
2. Each workspace folder with a `.git` directory will appear as a separate repository
3. Click on any repository to view its commits, branches, and changes

## Features

- **Multi-root Explorer** - View all crates side-by-side in the file explorer
- **Per-workspace Settings** - Each folder can have its own `.vscode/settings.json`
- **Per-workspace Extensions** - Extensions can be enabled/disabled per folder
- **Unified Source Control** - All git repositories visible in one view

## Development Loop

From the workspace root, you can run:

```bash
cd src
cargo fmt --all --check
cargo clippy --workspace --all-targets
cargo test --workspace
```

Or develop within individual crates:

```bash
cd src/jinja2rs
cargo test
cargo build --release
```

## Adding New Folders

To add additional Rust crates to the workspace:

1. Edit `dsport.code-workspace`
2. Add a new entry to the `folders` array:
   ```json
   {
     "path": "src/mynewcrate",
     "name": "mynewcrate"
   }
   ```
3. Save the file (VS Code will prompt you to reload)

## See Also

- [README.md](./README.md) - Main project documentation
- [AGENTS.md](./AGENTS.md) - Repository guidelines for coding agents
