use proc_macro2::{Ident, Literal};
use syn::{
    parse::{Parse, ParseStream},
    Result,
};

/// Parses and validates a client attribute
/// `"postgres"` -> `Client(PostgresClient)`
#[derive(Copy, Clone)]
pub(crate) struct Client(&'static str);

impl Default for Client {
    fn default() -> Self {
        Self("PostgresClient")
    }
}

impl Parse for Client {
    fn parse(input: ParseStream) -> Result<Self> {
        let key = input
            .parse::<Literal>()
            .map(|x| x.to_string().replace("\"", ""))
            .or_else(|_| input.parse::<Ident>().map(|x| x.to_string()))?;

        match key {
            key if key.as_str() == "postgres" => Ok(Self::default()),
            _ => Err(syn::Error::new_spanned(key, "Unknown client")),
        }
    }
}

impl From<Client> for &'static str {
    fn from(client: Client) -> Self {
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
        let client: &str = Client::default().into();

        assert_eq!(client, "PostgresClient");
    }

    #[test]
    fn postgres() {
        let input = quote! { "postgres" };
        let client: &str = parse2::<Client>(input).unwrap().into();

        assert_eq!(client, "PostgresClient");
    }

    #[test]
    fn unquoted_postgres() {
        let input = quote! { postgres };
        let client: &str = parse2::<Client>(input).unwrap().into();

        assert_eq!(client, "PostgresClient");
    }

    #[test]
    #[should_panic]
    fn unknown() {
        let input = quote! { "unknown" };
        parse2::<Client>(input).unwrap();
    }
}
