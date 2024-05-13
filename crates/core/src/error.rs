use std::error::Error as StdError;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::result::Result as StdResult;

#[allow(dead_code)]
pub type Result<T> = StdResult<T, Error>;

#[derive(Debug)]
pub enum Error {}

impl Display for Error {
    fn fmt(&self, _f: &mut Formatter) -> FmtResult {
        Err(core::fmt::Error)
    }
}

impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        None
    }
}
