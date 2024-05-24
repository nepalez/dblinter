mod attrs;
mod client;
mod flag;

/// Provide structure for parsing problem definitions
pub use attrs::Attrs;

/// Reexport inventory crate that collects problem definitions.
pub use inventory;

#[repr(C)]
#[derive(Clone, Debug, PartialEq)]
pub struct Definition {
    pub client: &'static str,
    pub fields: &'static [Field],
    pub filters: &'static [TaggedField],
    pub limits: &'static [TaggedField],
    pub message: &'static str,
    pub migration: Option<&'static str>,
    pub name: &'static str,
    pub query: &'static str,
    pub rollback: Option<&'static str>,
}

#[repr(C)]
#[derive(Clone, Debug, PartialEq)]
pub struct Field {
    pub name: &'static str,
    pub ty: &'static str,
}

#[repr(C)]
#[derive(Clone, Debug, PartialEq)]
pub struct TaggedField {
    pub name: &'static str,
    pub ty: &'static str,
    pub desc: &'static str,
}

inventory::collect!(Definition);
