use regex::Regex;
use serde::de::DeserializeOwned;
use serde::Serialize;
use tera::Context;

use crate::client::{Client, TryFromRow};
use crate::error::{Error, Result};
use crate::to_sql::ToSql;
use crate::{CustomProblem, Problem};

/// Inspector produces a query to find problems in the database.
pub trait Inspector: Sized {
    type Problem: Problem;

    fn build(key: &str, value: &str) -> Result<Self>;
    fn query(&self) -> Result<String>;
    fn parse(
        &self,
        row: <<Self::Problem as Problem>::Client as Client>::Row,
    ) -> Result<Self::Problem>;
}

/// The implementation of an inspector based on a query template,
/// updated with the WHERE SQL clause extracted from the current instance.
pub trait CustomInspector: Sized + Serialize + DeserializeOwned
where
    Context: for<'a> From<&'a <Self as CustomInspector>::Problem>,
    Context: for<'a> From<&'a Self>,
{
    type Problem: CustomProblem;

    fn query_() -> &'static str;
    fn __query(&self) -> Result<String> {
        let compact = Regex::new(r"\s\n+").unwrap();
        let strip = Regex::new(r"^ | *(;.*)?$").unwrap();
        let query = Self::query_();
        let query = compact.replace(query, " ");
        let query = strip.replace(&query, "").to_string();
        let context = Context::from(self);
        tera::Tera::one_off(&query, &context, false).map_err(|e| ("query", e).into())
    }
}

impl<I: CustomInspector> ToSql for I
where
    Context: for<'a> From<&'a <Self as CustomInspector>::Problem>,
    Context: for<'a> From<&'a Self>,
{
}

impl<I: CustomInspector> Inspector for I
where
    Context: for<'a> From<&'a <I as CustomInspector>::Problem>,
    Context: for<'a> From<&'a Self>,
{
    type Problem = <I as CustomInspector>::Problem;

    fn build(_key: &str, value: &str) -> Result<Self> {
        serde_json::from_str(value).map_err(Error::ParseConfig)
    }
    fn query(&self) -> Result<String> {
        Ok(format!("{}{};", self.__query()?, self.to_sql()?))
    }
    fn parse(
        &self,
        row: <<Self::Problem as Problem>::Client as Client>::Row,
    ) -> Result<Self::Problem> {
        Ok(<Self::Problem as TryFromRow<
            <<Self::Problem as Problem>::Client as Client>::Row,
        >>::try_from_row(row)?)
    }
}
