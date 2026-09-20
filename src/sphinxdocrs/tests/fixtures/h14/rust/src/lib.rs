//! H14 Rust fixture crate.

#![doc = "Fixture crate documentation."]

pub mod api;
pub use api::Component as PublicComponent;

#[doc(hidden)]
pub fn hidden_item() {}

/// Use answer instead.
#[deprecated(note = "use api::answer")]
pub fn old_answer() -> i32 { 0 }

/// Private implementation detail.
fn private_item() {}

#[macro_export]
macro_rules! answer_macro {
    () => { 42 };
}
