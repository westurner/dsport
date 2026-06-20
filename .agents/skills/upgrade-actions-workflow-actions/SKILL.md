---
name: upgrade-actions-workflow-actions
description: >-
  Upgrade GitHub Actions workflow `uses:` references to the latest available
  floating major tags (for example `@v7`, `@v6`, `@v2`) while preserving
  non-semver action references such as `dtolnay/rust-toolchain@stable` or
  explicit channels when required.
---

# Upgrade Workflow Actions To Floating Major Tags

Use this skill when a user asks to update GitHub workflow action versions in
`.github/workflows/*.yml` and prefers floating major tags instead of pinned
patch versions.

## Goal

- Keep action references on the newest major line.
- Use floating major tags only: `@vN` (not `@vN.x.y`).
- Preserve behavior for actions that intentionally use non-version channels.

## Scope

- Primary target files: `.github/workflows/*.yml` and `.github/workflows/*.yaml`.
- Update only `uses:` entries.
- Do not refactor unrelated workflow logic unless needed for compatibility.

## Rules

1. Prefer floating major tags.
- Convert `owner/action@v5.2.1` -> `owner/action@v5`.
- Convert `owner/action@v5` -> newer major only when available (for example `@v7`).
- Do not pin patch tags unless the user explicitly asks.

2. Keep non-semver channels when that is the action's intended interface.
- Example: `dtolnay/rust-toolchain@stable` is valid and often preferred.
- If switching `dtolnay/rust-toolchain` to `@v1`, add `with: toolchain: stable`
  unless the workflow already specifies a different toolchain.

3. Preserve existing semantics.
- Keep `with:` blocks and options unless required by a version migration.
- Keep job order, step order, and conditions (`if:`) unchanged.

4. Favor smallest diff.
- Only touch action versions and migration-required keys.

## Procedure

1. Discover all workflow action references.

```sh
rg "^\s*-\s*uses:" .github/workflows -n
```

2. Check latest available tags per action repo.

```sh
git ls-remote --tags "https://github.com/<owner>/<action>" 'v*'
```

3. Choose target version form.
- If action supports semver tags, use latest major as `@vN`.
- If action is channel-based, keep channel usage unless user asks otherwise.

4. Apply edits in workflow files.
- Update only `uses:` lines and any minimum migration tweaks.

5. Validate syntax quickly.
- Re-open edited files and verify YAML indentation and structure.
- Optionally run a local lint/checker if available.

## Common Mappings

These are examples, not hardcoded rules:
- `actions/checkout@v5` -> `actions/checkout@v7`
- `actions/setup-python@v5` -> `actions/setup-python@v6`
- `actions/upload-artifact@v4` -> `actions/upload-artifact@v7`
- `Swatinem/rust-cache@v2` -> keep `@v2` if no newer major exists

## Output Expectations

When done, report:
- Which workflow files changed.
- Which actions were upgraded and to what floating major tags.
- Any actions intentionally left unchanged and why.

## Safety Notes

- Do not upgrade to prerelease-only lines unless user asks.
- Avoid replacing a stable channel (`@stable`) with a version tag unless
  behavior is preserved and explained.
- If a major upgrade requires broader workflow changes, call that out clearly.
