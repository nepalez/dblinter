use crate::field::Field;
use proc_macro2::TokenStream;
use quote::quote;
use syn::parse::{Parse, ParseStream};

/// Parse the struct definition of the problem with field attributes
/// `#[limit("description")]`,
/// `#[filter("description")]`
pub struct Item {
    pub name: String,
    pub fields: Vec<Field>,
}

impl Item {
    pub fn fields(&self) -> TokenStream {
        let list: TokenStream = self
            .fields
            .iter()
            .map(|Field { name, ty, .. }| quote! { Field { name: #name, ty: #ty }, })
            .collect();
        quote! { &[#list] }
    }

    pub fn filters(&self) -> TokenStream {
        let list: TokenStream = self
            .fields
            .iter()
            .filter(|f| f.is_filter())
            .map(|f| (&f.name, &f.optional_ty, f.kind.desc()))
            .map(|(name, ty, desc)| quote! { TaggedField { name: #name, ty: #ty, desc: #desc }, })
            .collect();
        quote! { &[#list] }
    }

    pub fn limits(&self) -> TokenStream {
        let list: TokenStream = self
            .fields
            .iter()
            .filter(|f| f.is_limit())
            .map(|f| (&f.name, &f.ty, f.kind.desc()))
            .map(|(name, ty, desc)| quote! { TaggedField { name: #name, ty: #ty, desc: #desc }, })
            .collect();
        quote! { &[#list] }
    }
}

impl Parse for Item {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let item = input.parse::<syn::ItemStruct>()?;
        let name = item.ident.to_string();
        let mut fields = Vec::with_capacity(item.fields.len());
        for (i, field) in item.fields.iter().enumerate() {
            fields.insert(i, field.try_into()?);
        }
        Ok(Self { name, fields })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use syn::{parse2, parse_quote};

    #[test]
    fn valid_item() {
        let input = parse_quote! {
            pub struct Foo {
                #[limit("the a message")]
                pub a: i32,
                #[filter("the b message")]
                pub b: String,
                #[filter("the c message")]
                pub(crate) c: Option<String>,
                d: bool,
            }
        };
        let output = parse2::<Item>(input).unwrap();

        assert_eq!(output.name, "Foo");
    }

    #[test]
    #[should_panic]
    fn not_a_struct() {
        let input = parse_quote! {
            pub enum Foo {
                Bar: i32,
                Baz: i32,
            }
        };
        parse2::<Item>(input).unwrap();
    }
}
