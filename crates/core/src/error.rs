use std::error::Error as StdError;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::result::Result as StdResult;

use crate::to_sql::Error as ToSqlError;

pub type Result<T> = StdResult<T, Error>;

#[derive(Debug)]
pub enum Error {
    RenderSql(ToSqlError),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        match self {
            Self::RenderSql(err) => write!(f, "Failed to render SQL WHERE clause: {}", err),
        }
    }
}

impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            Self::RenderSql(err) => Some(err),
        }
    }
}

impl From<ToSqlError> for Error {
    fn from(err: ToSqlError) -> Self {
        Self::RenderSql(err)
    }
}
