use std::error::Error as StdError;
use std::fmt::{Display, Formatter, Result as FmtResult};

#[cfg(feature = "postgres")]
use postgres::{Error as PostgresError, Row as PostgresRow};
#[cfg(feature = "postgres")]
#[allow(unused_imports)]
use postgres_from_row::FromRow;

pub trait TryFromRow<Row>
where
    Self: Sized,
{
    fn try_from_row(row: Row) -> Result<Self, ParseRowError>;
}

#[cfg(feature = "postgres")]
impl<T: FromRow> TryFromRow<PostgresRow> for T {
    fn try_from_row(row: PostgresRow) -> Result<Self, ParseRowError> {
        <T as FromRow>::try_from_row(&row).map_err(ParseRowError::Postgres)
    }
}

/// Interface to interact with a database
pub trait Client {
    type Row;
    fn query(&mut self, query: &str) -> Result<Vec<Self::Row>, ExecuteQueryError>;
}

#[derive(Debug)]
pub enum EstablishConnectionError {
    #[cfg(feature = "postgres")]
    Postgres(PostgresError),
}

impl Display for EstablishConnectionError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        match self {
            #[cfg(feature = "postgres")]
            Self::Postgres(err) => write!(f, "{}", err),
        }
    }
}

impl StdError for EstablishConnectionError {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            #[cfg(feature = "postgres")]
            Self::Postgres(err) => Some(err),
        }
    }
}

#[derive(Debug)]
pub enum ExecuteQueryError {
    #[cfg(feature = "postgres")]
    Postgres(PostgresError),
}

impl Display for ExecuteQueryError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        match self {
            #[cfg(feature = "postgres")]
            Self::Postgres(err) => write!(f, "{}", err),
        }
    }
}

impl StdError for ExecuteQueryError {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            #[cfg(feature = "postgres")]
            Self::Postgres(err) => Some(err),
        }
    }
}

#[derive(Debug)]
pub enum ParseRowError {
    #[cfg(feature = "postgres")]
    Postgres(PostgresError),
}

impl Display for ParseRowError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        match self {
            #[cfg(feature = "postgres")]
            Self::Postgres(err) => write!(f, "{}", err),
        }
    }
}

impl StdError for ParseRowError {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            #[cfg(feature = "postgres")]
            Self::Postgres(err) => Some(err),
        }
    }
}

#[cfg(feature = "postgres")]
#[repr(C)]
pub struct PostgresClient {
    conn: postgres::Client,
}

#[cfg(feature = "postgres")]
impl PostgresClient {
    pub fn connect(url: &str) -> Result<Self, EstablishConnectionError> {
        postgres::Client::connect(url, postgres::NoTls)
            .map_err(EstablishConnectionError::Postgres)
            .map(|conn| Self { conn })
    }
}

#[cfg(feature = "postgres")]
impl Client for PostgresClient {
    type Row = PostgresRow;

    fn query(&mut self, query: &str) -> Result<Vec<Self::Row>, ExecuteQueryError> {
        self.conn
            .query(query, &[])
            .map_err(ExecuteQueryError::Postgres)
    }
}
