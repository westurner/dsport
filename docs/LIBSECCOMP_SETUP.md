# libseccomp Installation Guide

This document provides instructions for installing `libseccomp` on different Linux distributions. `libseccomp` is required to build jinja2rs with the `seccomp` feature enabled.

## Overview

- **libseccomp**: Core C library for seccomp filtering (runtime library)
- **libseccomp-devel** (or `-dev`): Development headers and static/shared libraries (required for building)

The `seccomp` feature in jinja2rs uses the Rust bindings (`libseccomp` crate) to enforce syscall sandboxing in `SandboxedEnvironment`.

### Why `-devel` is required

The Rust `libseccomp` crate bindings need both:
1. **C header files** (`seccomp.h`) — required by bindgen to generate FFI bindings
2. **Library files** (static or shared) — required for linking

The `-devel` / `-dev` package provides both. The `-static` package alone is **insufficient** because it lacks the headers. Installing only `-static` will result in a build error: `seccomp.h: No such file or directory`.

**Do not attempt to use `-static` alone.** Always install the `-devel` / `-dev` package, which includes headers and can link against either static or dynamic libraries as needed.

## Debian / Ubuntu

### Install libseccomp

```bash
# Update package lists
sudo apt update

# Install runtime library and development headers
sudo apt install -y libseccomp2 libseccomp-dev
```

### Verify Installation

```bash
# Check runtime library
dpkg -l | grep libseccomp

# Check pkg-config
pkg-config --list-all | grep libseccomp

# Verify header files
ls -la /usr/include/seccomp.h
```

### Build with seccomp feature

```bash
cd src/jinja2rs
cargo test --features sandbox,seccomp,resource-limits
```

## Fedora / RHEL / CentOS

### Install libseccomp

```bash
# Install runtime library and development headers
sudo dnf install -y libseccomp libseccomp-devel
```

Or with yum (older systems):

```bash
sudo yum install -y libseccomp libseccomp-devel
```

### Verify Installation

```bash
# Check installed packages
dnf list installed | grep libseccomp

# Check pkg-config
pkg-config --list-all | grep libseccomp

# Verify header files
ls -la /usr/include/seccomp.h
```

### Build with seccomp feature

```bash
cd src/jinja2rs
cargo test --features sandbox,seccomp,resource-limits
```

## Arch Linux

### Install libseccomp

```bash
sudo pacman -S libseccomp
```

### Build with seccomp feature

```bash
cd src/jinja2rs
cargo test --features sandbox,seccomp,resource-limits
```

## Alpine Linux

### Install libseccomp

```bash
apk add libseccomp libseccomp-dev
```

### Build with seccomp feature

```bash
cd src/jinja2rs
cargo test --features sandbox,seccomp,resource-limits
```

## Troubleshooting

### "error: linking with `cc` failed: unable to find library -lseccomp"

The system libseccomp library is not installed or not found by pkg-config.

**Solution:**

1. Verify installation:
   ```bash
   pkg-config --modversion libseccomp
   ```

2. If pkg-config doesn't find it, check the library path:
   ```bash
   # Debian/Ubuntu
   ldconfig -p | grep libseccomp
   
   # Fedora
   ldconfig -p | grep libseccomp
   ```

3. If still not found, reinstall:
   ```bash
   # Debian/Ubuntu
   sudo apt reinstall libseccomp-dev
   
   # Fedora
   sudo dnf reinstall libseccomp-devel
   ```

### "seccomp.h: No such file or directory"

The development headers are missing.

**Solution:**

Ensure the `-devel` or `-dev` package is installed:

```bash
# Debian/Ubuntu
sudo apt install libseccomp-dev

# Fedora
sudo dnf install libseccomp-devel
```

### "feature `seccomp` requires kernel >= 3.17"

Runtime error: your kernel version doesn't support seccomp filtering. This is unlikely on modern systems (Linux kernel 4.0+), but can occur on very old systems.

**Solution:**

Either:
1. Build without the `seccomp` feature (sandbox mode will still work, just without syscall filtering):
   ```bash
   cargo test --features sandbox,resource-limits
   ```

2. Update your kernel to >= 3.17.

## Building without seccomp

If libseccomp is not available or not needed, jinja2rs works without the `seccomp` feature:

```bash
# Build and test without seccomp
cd src/jinja2rs
cargo test --features sandbox

# Or with resource limits but no seccomp:
cargo test --features sandbox,resource-limits
```

When the `seccomp` feature is not enabled, `SandboxedEnvironment` will still enforce sandbox restrictions (path sandboxing, attribute/method denials, undefined reference strictness) — it just won't use Linux syscall filtering.

## Feature Summary

| Feature | Purpose | Dependencies |
|---------|---------|--------------|
| `sandbox` | Path/attribute/method sandboxing | none |
| `seccomp` | Linux syscall filtering for sandboxing | libseccomp-dev |
| `resource-limits` | Memory/CPU limits (RLIMIT_AS, RLIMIT_CPU) | nix crate (via Cargo) |
| `python-callable-warnings` | Warn when Python callables appear in context | tracing crate (via Cargo) |

All features are optional. The base sandbox mode works without any external dependencies.

## Testing with Features

To run the full test suite with all features enabled:

```bash
cd src/jinja2rs

# Test with all features
cargo test --features sandbox,seccomp,resource-limits,python-callable-warnings

# Or test each feature individually
cargo test --features sandbox
cargo test --features sandbox,seccomp
cargo test --features sandbox,resource-limits
cargo test --features sandbox,python-callable-warnings
```

See [test_sandbox_security.rs](../src/jinja2rs/tests/test_sandbox_security.rs) for sandbox-specific tests.
