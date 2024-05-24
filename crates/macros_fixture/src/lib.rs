/// The crate contains definitions of problems for testing only
/// to check results of macro expansion in a downstream crate.
pub use macros::*;

/// See the templates at the root `./templates` folder.

#[cfg(feature = "problem")]
#[problem]
pub struct TestColumnLimitMissed {
    #[filter("The scope of the database table")]
    pub scope_name: String,
    #[filter("The name of the table")]
    pub table_name: String,
    pub column_name: String,
    #[limit("The max number of chars allowed in the column")]
    pub limit: u32,
}

#[cfg(feature = "problem")]
#[problem(migration = false)]
pub struct TestPrimaryKeyMissed {
    #[filter("The scope of the database table")]
    pub scope_name: String,
    #[filter("The name of the table")]
    pub table_name: String,
}
