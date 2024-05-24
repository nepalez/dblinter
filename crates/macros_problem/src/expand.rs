use crate::item::Item;
use convert_case::{Case, Casing};
use macros_core::Attrs;
use proc_macro2::TokenStream;
use quote::quote;
use regex::Regex;
use std::env::current_dir;
use std::fs::read_to_string;
use syn::parse2;

pub fn expand(attrs: TokenStream, item: TokenStream) -> TokenStream {
    let attrs: Attrs = parse2(attrs).unwrap();
    let client = attrs.client();

    let item: Item = parse2(item).unwrap();
    let name = &item.name;
    let fields = item.fields();
    let limits = item.limits();
    let filters = item.filters();

    let message = read_file(name, "message.txt");
    let query = read_file(name, "query.sql");
    let mut migration = quote! { None };
    let mut rollback = quote! { None };
    if attrs.migration() {
        let data = read_file(name, "migration.sql");
        migration = quote! { Some(#data) };
    }
    if attrs.rollback() {
        let data = read_file(name, "rollback.sql");
        rollback = quote! { Some(#data) };
    }

    quote! {
        inventory::submit! {
            Definition {
                client: #client,
                fields: #fields,
                filters: #filters,
                limits: #limits,
                message: #message,
                migration: #migration,
                name: #name,
                query: #query,
                rollback: #rollback,
            }
        }
    }
}

fn read_file(problem: &str, filename: &'static str) -> String {
    let path = current_dir()
        .unwrap()
        .join("templates")
        .join(problem.to_case(Case::Snake))
        .join(filename);
    let re = Regex::new(r"[\s\n]+").unwrap();
    let line = read_to_string(&path)
        .map_err(|err| format!("Cannot read file {:?}: {}", path, err))
        .unwrap();
    re.replace_all(line.trim(), " ").to_string()
}

#[cfg(test)]
mod test {
    use super::*;
    use syn::parse_quote;

    fn item() -> TokenStream {
        parse_quote! {
            pub struct Test {
                #[filter("Table name")]
                pub table_name: String,
                pub column_name: String,
                #[limit("Max size of the column")]
                pub max_size: i32,
            }
        }
    }

    #[test]
    fn default() {
        let attrs = quote! {};
        let output = expand(attrs.into(), item().into());
        let target = quote! {
            inventory::submit! {
                Definition {
                    client: "PostgresClient",
                    fields: &[
                        Field { name: "table_name", ty: "String" },
                        Field { name: "column_name", ty: "String" },
                        Field { name: "max_size", ty: "i32" },
                    ],
                    filters: &[
                        TaggedField {
                            name: "table_name",
                            ty: "Option < String >",
                            desc: "Table name"
                        },
                    ],
                    limits: &[
                        TaggedField {
                            name: "max_size",
                            ty: "i32",
                            desc: "Max size of the column"
                        },
                    ],
                    message: "./message.txt",
                    migration: Some("./migration.sql"),
                    name: "Test",
                    query: "./query.sql",
                    rollback: Some("./rollback.sql"),
                }
            }
        };
        assert_eq!(output.to_string(), target.to_string());
    }

    #[test]
    fn without_rollback() {
        let attrs = quote! { rollback = false };
        let output = expand(attrs.into(), item().into());
        let target = quote! {
            inventory::submit! {
                Definition {
                    client: "PostgresClient",
                    fields: &[
                        Field { name: "table_name", ty: "String" },
                        Field { name: "column_name", ty: "String" },
                        Field { name: "max_size", ty: "i32" },
                    ],
                    filters: &[
                        TaggedField {
                            name: "table_name",
                            ty: "Option < String >",
                            desc: "Table name"
                        },
                    ],
                    limits: &[
                        TaggedField {
                            name: "max_size",
                            ty: "i32",
                            desc: "Max size of the column"
                        },
                    ],
                    message: "./message.txt",
                    migration: Some("./migration.sql"),
                    name: "Test",
                    query: "./query.sql",
                    rollback: None,
                }
            }
        };
        assert_eq!(output.to_string(), target.to_string());
    }

    #[test]
    fn without_migration() {
        let attrs = quote! { migration = false };
        let output = expand(attrs.into(), item().into());
        let target = quote! {
            inventory::submit! {
                Definition {
                    client: "PostgresClient",
                    fields: &[
                        Field { name: "table_name", ty: "String" },
                        Field { name: "column_name", ty: "String" },
                        Field { name: "max_size", ty: "i32" },
                    ],
                    filters: &[
                        TaggedField {
                            name: "table_name",
                            ty: "Option < String >",
                            desc: "Table name"
                        },
                    ],
                    limits: &[
                        TaggedField {
                            name: "max_size",
                            ty: "i32",
                            desc: "Max size of the column"
                        },
                    ],
                    message: "./message.txt",
                    migration: None,
                    name: "Test",
                    query: "./query.sql",
                    rollback: None,
                }
            }
        };
        assert_eq!(output.to_string(), target.to_string());
    }
}
