use std::error::Error as StdError;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::result::Result as StdResult;

use crate::client::{EstablishConnectionError, ExecuteQueryError, ParseRowError};
use crate::to_sql::Error as ToSqlError;

pub type Result<T> = StdResult<T, Error>;

#[derive(Debug)]
pub enum Error {
    EstablishConnection(EstablishConnectionError),
    ExecuteQuery(ExecuteQueryError),
    ParseRow(ParseRowError),
    RenderSql(ToSqlError),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        match self {
            Self::EstablishConnection(err) => write!(f, "Failed to establish connection: {}", err),
            Self::ExecuteQuery(err) => write!(f, "Failed to execute query: {}", err),
            Self::ParseRow(err) => write!(f, "Failed to parse row: {}", err),
            Self::RenderSql(err) => write!(f, "Failed to render SQL WHERE clause: {}", err),
        }
    }
}

impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            Self::EstablishConnection(err) => Some(err),
            Self::ExecuteQuery(err) => Some(err),
            Self::ParseRow(err) => Some(err),
            Self::RenderSql(err) => Some(err),
        }
    }
}

impl From<ToSqlError> for Error {
    fn from(err: ToSqlError) -> Self {
        Self::RenderSql(err)
    }
}

impl From<EstablishConnectionError> for Error {
    fn from(err: EstablishConnectionError) -> Self {
        Self::EstablishConnection(err)
    }
}

impl From<ExecuteQueryError> for Error {
    fn from(err: ExecuteQueryError) -> Self {
        Self::ExecuteQuery(err)
    }
}

impl From<ParseRowError> for Error {
    fn from(err: ParseRowError) -> Self {
        Self::ParseRow(err)
    }
}
