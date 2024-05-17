use std::fmt::Debug;

use crate::error::Result;
use crate::problem::Problem;

/// Collection of problems found in the database.
#[repr(C)]
#[derive(Clone, Debug)]
pub struct Report<P: Problem> {
    problems: Vec<P>,
}

impl<P: Problem> Default for Report<P> {
    fn default() -> Self {
        Self { problems: vec![] }
    }
}

impl<P: Problem> Report<P> {
    pub fn iter(&self) -> Iter<P> {
        Iter {
            report: self,
            index: 0,
        }
    }

    pub fn insert(&mut self, problem: P) {
        self.problems.push(problem);
    }

    pub fn compact(mut self) -> Self {
        self.problems.sort_by_key(|a| a.id().unwrap());
        self.problems.dedup_by_key(|a| a.id().unwrap());
        self
    }

    pub fn message(&self) -> Result<String> {
        let mut output = String::new();
        for problem in self.iter() {
            if !output.is_empty() {
                output.push('\n');
            }
            output.push_str(problem.message()?.as_str());
        }
        Ok(output)
    }

    pub fn migration(&self) -> Result<String> {
        let mut output = String::new();
        for problem in self.iter() {
            if let Some(migration) = problem.migration() {
                if !output.is_empty() {
                    output.push('\n');
                }
                output.push_str(migration?.as_str());
            }
        }
        Ok(output)
    }

    pub fn rollback(&self) -> Result<String> {
        let mut output = String::new();
        for problem in self.iter() {
            if let Some(rollback) = problem.rollback() {
                if !output.is_empty() {
                    output.push('\n');
                }
                output.push_str(rollback?.as_str());
            }
        }
        Ok(output)
    }

    pub fn count(&self) -> usize {
        self.problems.len()
    }

    pub fn is_empty(&self) -> bool {
        self.problems.is_empty()
    }

    pub fn count_migrations(&self) -> usize {
        self.problems
            .iter()
            .filter(|p| p.migration().is_some())
            .count()
    }
}

#[repr(C)]
#[derive(Clone, Debug)]
pub struct Iter<'a, P: Problem> {
    report: &'a Report<P>,
    index: usize,
}

impl<'a, P: Problem> Iterator for Iter<'a, P> {
    type Item = &'a P;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.report.problems.len() {
            let problem = &self.report.problems[self.index];
            self.index += 1;
            Some(problem)
        } else {
            None
        }
    }
}

#[cfg(all(test, feature = "postgres"))]
mod test {
    use super::*;
    use crate::client::PostgresClient;
    use crate::error::Result;
    use postgres_from_row::FromRow;

    #[repr(C)]
    #[derive(Debug, FromRow)]
    struct Item {
        kind: &'static str,
        message: String,
        migration: Option<String>,
        rollback: Option<String>,
    }
    impl Problem for Item {
        type Client = PostgresClient;

        fn kind(&self) -> &'static str {
            &self.kind
        }
        fn message(&self) -> Result<String> {
            Ok(self.message.clone())
        }
        fn migration(&self) -> Option<Result<String>> {
            self.migration.as_ref().map(|m| Ok(m.clone()))
        }
        fn rollback(&self) -> Option<Result<String>> {
            self.rollback.as_ref().map(|r| Ok(r.clone()))
        }
    }

    #[test]
    fn iter() {
        let report = Report {
            problems: vec![
                Item {
                    kind: "foo",
                    message: "some foo".to_string(),
                    migration: None,
                    rollback: None,
                },
                Item {
                    kind: "bar",
                    message: "some bar".to_string(),
                    migration: None,
                    rollback: None,
                },
            ],
        };

        let mut iter = report.iter();
        assert_eq!(iter.next().unwrap().kind, "foo".to_string());
        assert_eq!(iter.next().unwrap().kind, "bar".to_string());
        assert!(iter.next().is_none());
    }

    #[test]
    fn into_iter() {
        let report = Report {
            problems: vec![
                Item {
                    kind: "foo",
                    message: "some foo".to_string(),
                    migration: None,
                    rollback: None,
                },
                Item {
                    kind: "bar",
                    message: "some bar".to_string(),
                    migration: None,
                    rollback: None,
                },
            ],
        };

        let mut iter = report.iter();
        assert_eq!(iter.next().unwrap().kind, "foo".to_string());
        assert_eq!(iter.next().unwrap().kind, "bar".to_string());
        assert!(iter.next().is_none());
    }

    #[test]
    fn compact() {
        let report = Report {
            problems: vec![
                Item {
                    kind: "foo",
                    message: "some foo".to_string(),
                    migration: None,
                    rollback: None,
                },
                Item {
                    kind: "bar",
                    message: "some bar".to_string(),
                    migration: None,
                    rollback: None,
                },
                Item {
                    kind: "foo",
                    message: "some foo".to_string(),
                    migration: None,
                    rollback: None,
                },
            ],
        };

        let report = report.compact();

        let mut iter = report.iter();
        assert_eq!(iter.next().unwrap().kind, "bar".to_string());
        assert_eq!(iter.next().unwrap().kind, "foo".to_string());
        assert!(iter.next().is_none());
    }

    #[test]
    fn insert() {
        let mut report = Report {
            problems: vec![Item {
                kind: "foo",
                message: "some foo".to_string(),
                migration: None,
                rollback: None,
            }],
        };

        let item = Item {
            kind: "bar",
            message: "some bar".to_string(),
            migration: None,
            rollback: None,
        };

        report.insert(item);

        let mut iter = report.iter();
        assert_eq!(iter.next().unwrap().kind, "foo".to_string());
        assert_eq!(iter.next().unwrap().kind, "bar".to_string());
        assert!(iter.next().is_none());
    }

    #[test]
    fn message() {
        let report = Report {
            problems: vec![
                Item {
                    kind: "foo",
                    message: "some foo".to_string(),
                    migration: None,
                    rollback: None,
                },
                Item {
                    kind: "bar",
                    message: "some bar".to_string(),
                    migration: None,
                    rollback: None,
                },
            ],
        };

        assert_eq!(report.message().unwrap(), "some foo\nsome bar");
    }

    #[test]
    fn migration() {
        let report = Report {
            problems: vec![
                Item {
                    kind: "foo",
                    message: "some foo".to_string(),
                    migration: Some("foo migration".to_string()),
                    rollback: Some("foo rollback".to_string()),
                },
                Item {
                    kind: "bar",
                    message: "some bar".to_string(),
                    migration: None,
                    rollback: None,
                },
                Item {
                    kind: "baz",
                    message: "some baz".to_string(),
                    migration: Some("baz migration".to_string()),
                    rollback: None,
                },
                Item {
                    kind: "qux",
                    message: "some qux".to_string(),
                    migration: Some("qux migration".to_string()),
                    rollback: Some("qux rollback".to_string()),
                },
            ],
        };

        assert_eq!(
            report.migration().unwrap(),
            "foo migration\nbaz migration\nqux migration"
        );
    }

    #[test]
    fn rollback() {
        let report = Report {
            problems: vec![
                Item {
                    kind: "foo",
                    message: "some foo".to_string(),
                    migration: Some("foo migration".to_string()),
                    rollback: Some("foo rollback".to_string()),
                },
                Item {
                    kind: "bar",
                    message: "some bar".to_string(),
                    migration: None,
                    rollback: None,
                },
                Item {
                    kind: "baz",
                    message: "some baz".to_string(),
                    migration: Some("baz migration".to_string()),
                    rollback: None,
                },
                Item {
                    kind: "qux",
                    message: "some qux".to_string(),
                    migration: Some("qux migration".to_string()),
                    rollback: Some("qux rollback".to_string()),
                },
            ],
        };

        assert_eq!(report.rollback().unwrap(), "foo rollback\nqux rollback");
    }

    #[test]
    fn counters() {
        let report = Report {
            problems: vec![
                Item {
                    kind: "foo",
                    message: "some foo".to_string(),
                    migration: Some("foo migration".to_string()),
                    rollback: Some("foo rollback".to_string()),
                },
                Item {
                    kind: "bar",
                    message: "some bar".to_string(),
                    migration: None,
                    rollback: None,
                },
                Item {
                    kind: "baz",
                    message: "some baz".to_string(),
                    migration: Some("baz migration".to_string()),
                    rollback: None,
                },
                Item {
                    kind: "qux",
                    message: "some qux".to_string(),
                    migration: Some("qux migration".to_string()),
                    rollback: Some("qux rollback".to_string()),
                },
            ],
        };

        assert_eq!(report.is_empty(), false);
        assert_eq!(report.count(), 4);
        assert_eq!(report.count_migrations(), 3);
    }
}
