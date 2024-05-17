use crate::client::Client;
use crate::error::Result;
use crate::inspector::Inspector;
use crate::problem::Problem;
use crate::report::Report;

use serde_json::value::RawValue;
use std::collections::HashMap;

/// Linter is a thin wrapper around the Inspector that binds things together.
///
/// The primary reason for adding it (instead of just adding the `run` method to
/// the Inspector) is that the enum inspector is built by macro expansion,
/// and we need some macro to provide it. To do this the `derive(Linter)`
/// macro is used.
pub trait Linter {
    type Inspector: Inspector;

    fn run(
        config: &str,
        client: &mut <<Self::Inspector as Inspector>::Problem as Problem>::Client,
    ) -> Result<Report<<Self::Inspector as Inspector>::Problem>> {
        let mut report = Report::default();
        let data: HashMap<String, Box<RawValue>> = serde_json::from_str(config)?;
        for (key, val) in data {
            let inspector = Self::Inspector::build(&key, &val.to_string())?;
            let query = inspector.query()?;
            let rows = client.query(&query)?;
            for row in rows {
                let problem = inspector.parse(row)?;
                report.insert(problem);
            }
        }
        Ok(report)
    }
}
