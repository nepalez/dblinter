use core::*;

mod custom {
    use super::*;

    #[derive(Debug, Deserialize, FromRow)]
    pub struct ColumnLimitMissed {
        pub scope_name: String,
        pub table_name: String,
        pub column_name: String,
        pub limit: i32,
    }

    impl From<&ColumnLimitMissed> for Context {
        fn from(value: &ColumnLimitMissed) -> Self {
            let mut context = Self::new();
            context.insert("scope_name", &value.scope_name);
            context.insert("table_name", &value.table_name);
            context.insert("column_name", &value.column_name);
            context.insert("limit", &value.limit);
            context
        }
    }
    impl CustomProblem for ColumnLimitMissed {
        type Client = PostgresClient;

        fn kind_() -> &'static str {
            "ColumnLimitMissed"
        }
        fn message_() -> &'static str {
            "The column {{ scope_name }}.{{ table_name }} ({{ column_name }}) \
        is not limited to {{ limit }} chars"
        }
        fn migration_() -> Option<&'static str> {
            Some(
                "ALTER TABLE {{ scope_name }}.{{ table_name }} \
                ADD CONSTRAINT {{ table_name }}_{{ column_name }}_limit \
                CHECK (LENGTH({{ column_name }}) <= {{ limit }});",
            )
        }
        fn rollback_() -> Option<&'static str> {
            Some(
                "ALTER TABLE {{ scope_name }}.{{ table_name }} \
                DROP CONSTRAINT {{ table_name }}_{{ column_name }}_limit;",
            )
        }
    }

    #[derive(Debug)]
    pub enum TestProblem {
        ColumnLimitMissed(ColumnLimitMissed),
    }
    impl Problem for TestProblem {
        type Client = PostgresClient;

        fn kind(&self) -> &'static str {
            match self {
                Self::ColumnLimitMissed(p) => p.kind(),
            }
        }
        fn message(&self) -> Result<String> {
            match self {
                Self::ColumnLimitMissed(p) => p.message(),
            }
        }
        fn migration(&self) -> Option<Result<String>> {
            match self {
                Self::ColumnLimitMissed(p) => p.migration(),
            }
        }
        fn rollback(&self) -> Option<Result<String>> {
            match self {
                Self::ColumnLimitMissed(p) => p.rollback(),
            }
        }
    }

    #[derive(Debug, Deserialize, Serialize)]
    pub struct ColumnLimitMissedFilter {
        scope_name: String,
        table_name: String,
    }
    #[derive(Debug, Deserialize, Serialize)]
    pub struct ColumnLimitMissedInspector {
        limit: i32,
        only: Option<Vec<ColumnLimitMissedFilter>>,
        except: Option<Vec<ColumnLimitMissedFilter>>,
    }
    impl From<&ColumnLimitMissedInspector> for Context {
        fn from(value: &ColumnLimitMissedInspector) -> Self {
            let mut context = Self::new();
            context.insert("limit", &value.limit);
            context
        }
    }
    impl CustomInspector for ColumnLimitMissedInspector {
        type Problem = ColumnLimitMissed;

        fn query_() -> &'static str {
            "SELECT * FROM table WHERE limit = {{ limit }};"
        }
    }

    #[derive(Debug)]
    pub enum TestInspector {
        ColumnLimitMissed(ColumnLimitMissedInspector),
    }
    impl Inspector for TestInspector {
        type Problem = TestProblem;

        fn build(key: &str, value: &str) -> Result<Self> {
            match key {
                "ColumnLimitMissed" => Ok(Self::ColumnLimitMissed(
                    ColumnLimitMissedInspector::build(key, value)?,
                )),
                _ => Err(key.to_string().into()),
            }
        }
        fn query(&self) -> Result<String> {
            match self {
                Self::ColumnLimitMissed(i) => i.query(),
            }
        }
        fn parse(
            &self,
            row: <<Self::Problem as Problem>::Client as Client>::Row,
        ) -> Result<Self::Problem> {
            match self {
                Self::ColumnLimitMissed(i) => i.parse(row).map(TestProblem::ColumnLimitMissed),
            }
        }
    }
}

#[test]
fn test_problem() {
    let problem = custom::ColumnLimitMissed {
        scope_name: "public".to_string(),
        table_name: "users".to_string(),
        column_name: "email".to_string(),
        limit: 40,
    };

    assert_eq!(problem.kind(), "ColumnLimitMissed");
    assert_eq!(
        problem.message().unwrap(),
        "The column public.users (email) is not limited to 40 chars"
    );
    assert_eq!(
        problem.migration().unwrap().unwrap(),
        "ALTER TABLE public.users ADD CONSTRAINT users_email_limit CHECK (LENGTH(email) <= 40);"
    );
    assert_eq!(
        problem.rollback().unwrap().unwrap(),
        "ALTER TABLE public.users DROP CONSTRAINT users_email_limit;"
    );
}
