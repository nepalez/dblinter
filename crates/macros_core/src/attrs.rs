use crate::client::Client;
use crate::flag::Flag;
use quote::ToTokens;
use syn::{
    parse::{Parse, ParseStream},
    parse2,
    punctuated::Punctuated,
    Error, ExprAssign, Token,
};

/// Parse the `#[problem(client="postgres", migration=false, rollback=false)]` attributes.
#[derive(Default)]
pub struct Attrs {
    client: Client,
    migration: Flag,
    rollback: Flag,
}

impl Parse for Attrs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut output = Self::default();
        for item in Punctuated::<ExprAssign, Token![,]>::parse_terminated(input)? {
            let key: String = item.left.to_token_stream().to_string();
            match key.as_str() {
                "client" => output.client = parse2(item.right.to_token_stream())?,
                "migration" => output.migration = parse2(item.right.to_token_stream())?,
                "rollback" => output.rollback = parse2(item.right.to_token_stream())?,
                _ => return Err(Error::new_spanned(&item, "Unknown attribute")),
            }
        }
        output.rollback = (output.rollback.into() && output.migration.into()).into();
        Ok(output)
    }
}

impl Attrs {
    pub fn client(&self) -> &'static str {
        self.client.into()
    }

    pub fn migration(&self) -> bool {
        self.migration.into()
    }

    pub fn rollback(&self) -> bool {
        self.rollback.into()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use syn::{parse2, parse_quote};

    #[test]
    fn default() {
        let input = parse_quote! {};
        let attrs = parse2::<Attrs>(input).unwrap();

        assert_eq!(attrs.client(), "PostgresClient");
        assert_eq!(attrs.migration(), true);
        assert_eq!(attrs.rollback(), true);
    }

    #[test]
    fn postgres_without_rollback() {
        let input = parse_quote! { client = "postgres", rollback = false };
        let attrs = parse2::<Attrs>(input).unwrap();

        assert_eq!(attrs.client(), "PostgresClient");
        assert_eq!(attrs.migration(), true);
        assert_eq!(attrs.rollback(), false);
    }

    #[test]
    fn postgres_without_migration() {
        let input = parse_quote! { client = "postgres", migration = false };
        let attrs = parse2::<Attrs>(input).unwrap();

        assert_eq!(attrs.client(), "PostgresClient");
        assert_eq!(attrs.migration(), false);
        assert_eq!(attrs.rollback(), false);
    }

    #[test]
    #[should_panic]
    fn unknown_attribute() {
        let input = parse_quote! { foo = "bar" };
        parse2::<Attrs>(input).unwrap();
    }

    #[test]
    #[should_panic]
    fn non_boolean_migration() {
        let input = parse_quote! { migration = "bar" };
        parse2::<Attrs>(input).unwrap();
    }

    #[test]
    #[should_panic]
    fn non_postgres_client() {
        let input = parse_quote! { client = "mysql" };
        parse2::<Attrs>(input).unwrap();
    }
}
