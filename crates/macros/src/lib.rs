/// This module only reexports macros:
///
/// * `#[problem]` (with feature `problem`)
/// * `#[linter]` (with default feature `linter`)
///
/// Because linter uses problems defined by upstream crates,
/// it is not possible to use both features at the same time.
pub use macros_core::{inventory, Definition, Field, TaggedField};
