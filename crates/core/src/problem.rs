use tera::{Context, Tera};

use crate::client::{Client, TryFromRow};
use crate::error::Result;

/// A problem in the database that is reportable in the form of message and optional fixes.
/// Its interface uses `Result<String>` to support templates whose rendering may fail.
pub trait Problem: Sized {
    type Client: Client;

    /// The kind of the problem
    fn kind(&self) -> &'static str;
    /// The message, describing the problem.
    fn message(&self) -> Result<String>;
    /// The migration to fix the problem.
    fn migration(&self) -> Option<Result<String>>;
    /// The rollback of the migration.
    fn rollback(&self) -> Option<Result<String>>;

    /// A helper method to implement Ord and Eq for problems
    fn id(&self) -> Result<String> {
        Ok(format!(
            "{}{}{}{}",
            self.kind(),
            self.message()?,
            self.migration().unwrap_or(Ok("".into()))?,
            self.migration().unwrap_or(Ok("".into()))?,
        ))
    }
}

/// A specific problem has some structure bound to the rendered templates.
pub trait CustomProblem
where
    Self: Sized,
    for<'a> &'a Self: Into<Context>,
    Self::Client: Client,
    Self: TryFromRow<<Self::Client as Client>::Row>,
{
    type Client;

    /// The kind of the problem
    fn kind_() -> &'static str;
    /// The template for the message describing the problem
    fn message_() -> &'static str;
    /// The (optional) template for the migration to fix the problem
    fn migration_() -> Option<&'static str> {
        None
    }
    /// The (optional) template for the rollback of the migration
    fn rollback_() -> Option<&'static str> {
        None
    }
    // Helper method, not a part of public interface
    #[doc(hidden)]
    fn __render_template(&self, template: &'static str) -> Result<String> {
        let mut tera = Tera::default();
        let kind = Self::kind_();
        let context = self.into();
        tera.add_raw_template(kind, template)
            .and_then(|_| tera.render(kind, &context))
            .map_err(|err| (kind, err).into())
    }
}

impl<P: CustomProblem> Problem for P
where
    Self: Sized,
    for<'a> &'a Self: Into<Context>,
{
    type Client = <P as CustomProblem>::Client;

    fn kind(&self) -> &'static str {
        P::kind_()
    }
    fn message(&self) -> Result<String> {
        self.__render_template(P::message_())
    }
    fn migration(&self) -> Option<Result<String>> {
        P::migration_().map(|t| self.__render_template(t))
    }
    fn rollback(&self) -> Option<Result<String>> {
        if P::migration_().is_some() {
            P::rollback_().map(|t| self.__render_template(t))
        } else {
            None
        }
    }
}
