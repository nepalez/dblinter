// Adds definitions for `TestColumnLimitMissed`
// and `TestPrimaryKeyMissed` problems,
// annotated with the `#[problem]` attribute.
use macros_fixture::*;

#[test]
fn expand_problem() {
    let mut definitions: Vec<_> = inventory::iter::<Definition>().map(|p| p.clone()).collect();
    definitions.sort_by_key(|def| def.name);

    assert_eq!(
        definitions,
        vec![
            Definition {
                client: "PostgresClient",
                name: "TestColumnLimitMissed",
                fields: &[
                    Field {
                        name: "scope_name",
                        ty: "String"
                    },
                    Field {
                        name: "table_name",
                        ty: "String"
                    },
                    Field {
                        name: "column_name",
                        ty: "String"
                    },
                    Field {
                        name: "limit",
                        ty: "u32"
                    },
                ],
                filters: &[
                    TaggedField {
                        name: "scope_name",
                        ty: "Option < String >",
                        desc: "The scope of the database table"
                    },
                    TaggedField {
                        name: "table_name",
                        ty: "Option < String >",
                        desc: "The name of the table"
                    },
                ],
                limits: &[TaggedField {
                    name: "limit",
                    ty: "u32",
                    desc: "The max number of chars allowed in the column"
                },],
                query: "SELECT \
                            t.relnamespace::regnamespace AS scope_name, \
                            a.attrelid::regclass AS table_name, \
                            a.attname AS column_name, \
                            {{ limit }} AS limit \
                        FROM pg_attribute a \
                            INNER JOIN pg_class t \
                                ON a.attrelid = t.oid \
                            LEFT OUTER JOIN pg_constraint c \
                                ON c.conrelid = a.attrelid \
                                AND c.conkey = a.attnum \
                                AND c.contype = 'p' \
                            WHERE c.contype IS NULL;",
                message: "The size of the {{ scope_name }}.{{ table_name }}.{{ column_name }} \
                          is not restricted to {{ limit }} chars.",
                migration: Some(
                    "ALTER TABLE {{ scope_name }}.{{ table_name }} \
                    ADD CONSTRAINT {{ column_name }}_limit \
                    CHECK (lenght({{ column_name }}) <= {{ limit }});",
                ),
                rollback: Some(
                    "ALTER TABLE {{ scope_name }}.{{ table_name }} \
                    DROP CONSTRAINT {{ column_name }}_limit;",
                ),
            },
            Definition {
                client: "PostgresClient",
                name: "TestPrimaryKeyMissed",
                fields: &[
                    Field {
                        name: "scope_name",
                        ty: "String"
                    },
                    Field {
                        name: "table_name",
                        ty: "String"
                    },
                ],
                filters: &[
                    TaggedField {
                        name: "scope_name",
                        ty: "Option < String >",
                        desc: "The scope of the database table"
                    },
                    TaggedField {
                        name: "table_name",
                        ty: "Option < String >",
                        desc: "The name of the table"
                    },
                ],
                limits: &[],
                query: "SELECT c.relnamespace::regnamespace AS scope_name, \
                            c.relname AS table_name \
                        FROM pg_catalog.pg_class c \
                           LEFT OUTER JOIN pg_catalog.pg_index i \
                               ON c.oid = i.indrelid AND i.indisprimary \
                        WHERE i.indkey IS NULL;",
                message: "Index {{ scope_name }}.{{ table_name }} is missed.",
                migration: None,
                rollback: None,
            },
        ],
    );
}
