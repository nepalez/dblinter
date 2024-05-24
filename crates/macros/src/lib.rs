/// This module only reexports macros:
///
/// * `#[problem]` (with feature `problem`)
/// * `#[linter]` (with default feature `linter`)
///
/// Because linter uses problems defined by upstream crates,
/// it is not possible to use both features at the same time.
pub use macros_core::{inventory, Definition, Field, TaggedField};
/// Annotate problem definition with `#[problem(client="postgres", migration=false, rollback=false)]`.
///
/// By default (when used as `#[problem]`), the client is set to "postgres",
/// and both migration and rollback are enabled.
/// If a migration is skipped (`#[problem(migration = false)]`), the rollback is also disabled,
/// but a migration can be used without a rollback (`#[problem(rollback = false)]`).
///
/// Fields of the structure can be optionally annotated with
/// either `#[limit("description")]` or `#[filter("description")]` (but not both!)
/// For annotated fields you should provide descriptions to be used
/// in the generated config file like:
///
/// ```rust
/// # use macros::*;
/// #[problem(migration = false)]
/// pub struct TestColumnLimitMissed {
///     #[filter("The name of the table")]
///     pub scope_name: String,
///     #[filter("The name of the column")]
///     table_name: String,
///     column_name: String,
///     #[limit("The limit to be added to the column")]
///     pub limit: u32,
/// }
/// ```
///
/// ```yaml
/// ---
/// # Ensure column lengths are limited
/// Test:
///   # Required params:
///   limit: # The limit to be added to the column
///   # The optional whitelist of problems to check
///   only:
///     - table_name: # The name of the table
///       column_name: # The name of the column
///   # The optional blacklist of problems to be ignored
///   except:
///     - table_name: # The name of the table
///       column_name: # The name of the column
/// ```
#[cfg(feature = "problem")]
pub use macros_problem::problem;
