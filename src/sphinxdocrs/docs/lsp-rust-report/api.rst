LSP-discovered Rust API
=======================

Declarations below were returned by rust-analyzer for the local fixture crate using protected LSP with a private PID namespace and no child ``/proc`` mount.

.. rust:struct:: BuildReport

.. rust:field:: BuildReport::declarations

.. rust:field:: BuildReport::diagnostics

.. rust:function:: build_report fn(declarations: usize, diagnostics: usize) -> BuildReport

.. rust:function:: render_report fn(report: &BuildReport) -> String
