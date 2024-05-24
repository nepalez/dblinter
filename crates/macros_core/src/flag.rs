use proc_macro2::Literal;
use syn::{
    parse::{Parse, ParseStream},
    LitBool, Result,
};

/// Parses and validates a client attribute
/// `"postgres"` -> `Client(PostgresClient)`
#[derive(Copy, Clone)]
pub(crate) struct Flag(bool);

impl Default for Flag {
    fn default() -> Self {
        Self(true)
    }
}

impl Parse for Flag {
    fn parse(input: ParseStream) -> Result<Self> {
        let key = input
            .parse::<Literal>()
            .map(|x| x.to_string().replace("\"", ""))
            .or_else(|_| input.parse::<LitBool>().map(|x| x.token().to_string()))?;

        match key {
            key if key.as_str() == "true" => Ok(Self(true)),
            key if key.as_str() == "false" => Ok(Self(false)),
            _ => Err(syn::Error::new_spanned(key, "Boolean flag expected")),
        }
    }
}

impl From<bool> for Flag {
    fn from(value: bool) -> Self {
        Self(value)
    }
}

impl From<Flag> for bool {
    fn from(client: Flag) -> Self {
        client.0
    }
}

#[cfg(test)]
mod test_client {
    use super::*;
    use quote::quote;
    use syn::parse2;

    #[test]
    fn default() {
        let client: bool = Flag::default().into();

        assert_eq!(client, true);
    }

    #[test]
    fn quoted_true() {
        let input = quote! { "true" };
        let flag: bool = parse2::<Flag>(input).unwrap().into();

        assert_eq!(flag, true);
    }

    #[test]
    fn unquoted_true() {
        let input = quote! { true };
        let flag: bool = parse2::<Flag>(input).unwrap().into();

        assert_eq!(flag, true);
    }

    #[test]
    fn quoted_false() {
        let input = quote! { "false" };
        let flag: bool = parse2::<Flag>(input).unwrap().into();

        assert_eq!(flag, false);
    }

    #[test]
    fn unquoted_false() {
        let input = quote! { false };
        let flag: bool = parse2::<Flag>(input).unwrap().into();

        assert_eq!(flag, false);
    }

    #[test]
    #[should_panic]
    fn unknown() {
        let input = quote! { "unknown" };
        parse2::<Flag>(input).unwrap();
    }
}
