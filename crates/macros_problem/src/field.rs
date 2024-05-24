use quote::quote;
use syn::{Attribute, Error, Expr, ExprLit, Lit};

/// Convert a `syn::Field` into a `Field` struct
/// accepting nor more than one of the following attributes:
/// `#[limit("description")]`,
/// `#[filter("description")]`.
#[derive(Debug, PartialEq)]
pub struct Field {
    pub kind: Kind,
    pub name: String,
    pub optional_ty: String,
    pub ty: String,
}

impl Field {
    pub fn is_limit(&self) -> bool {
        matches!(self.kind, Kind::Limit(_))
    }

    pub fn is_filter(&self) -> bool {
        matches!(self.kind, Kind::Filter(_))
    }
}

impl TryFrom<&syn::Field> for Field {
    type Error = Error;

    fn try_from(value: &syn::Field) -> Result<Self, Self::Error> {
        let kind = value.try_into()?;
        let Name(name) = value.try_into()?;
        let Type(ty) = value.try_into()?;
        let OptionalType(optional_ty) = value.try_into()?;
        Ok(Self {
            kind,
            name,
            ty,
            optional_ty,
        })
    }
}

// Extract the kind of the field along with its description
#[derive(Debug, PartialEq)]
pub enum Kind {
    Filter(String),
    Limit(String),
    Plain,
}

impl Kind {
    pub fn desc(&self) -> &str {
        match self {
            Kind::Filter(desc) | Kind::Limit(desc) => desc,
            Kind::Plain => "",
        }
    }
}

impl TryFrom<&syn::Field> for Kind {
    type Error = Error;

    fn try_from(value: &syn::Field) -> Result<Self, Self::Error> {
        let mut kind = Kind::Plain;
        for a in value.attrs.iter() {
            let k = Kind::try_from(a)?;
            match (&kind, &k) {
                (&Kind::Plain, _) => kind = k,
                (_, &Kind::Plain) => (),
                _ => {
                    return Err(Error::new_spanned(
                        value,
                        "multiple attributes not supported",
                    ))
                }
            }
        }
        Ok(kind)
    }
}

impl TryFrom<&Attribute> for Kind {
    type Error = Error;

    fn try_from(value: &Attribute) -> Result<Self, Self::Error> {
        match value.path().get_ident() {
            Some(ident) if ident == "limit" => Ok(Self::Limit(desc(value)?)),
            Some(ident) if ident == "filter" => Ok(Self::Filter(desc(value)?)),
            Some(_) => Err(Error::new_spanned(value, "unknown attribute")),
            None => Ok(Self::Plain),
        }
    }
}

// Extract the name of the field
struct Name(String);

impl TryFrom<&syn::Field> for Name {
    type Error = Error;

    fn try_from(value: &syn::Field) -> Result<Self, Self::Error> {
        value
            .ident
            .as_ref()
            .map(|i| Self(i.to_string()))
            .ok_or(Error::new_spanned(value, "field name missed"))
    }
}

// Extract the type of the field
// For now the implementation is trivial, but later we can want to add some checks.
struct Type(String);

impl TryFrom<&syn::Field> for Type {
    type Error = Error;

    fn try_from(value: &syn::Field) -> Result<Self, Self::Error> {
        let ty = &value.ty;
        Ok(Self(quote! { #ty }.to_string()))
    }
}

// Convert the type into its optional part
struct OptionalType(String);

impl TryFrom<&syn::Field> for OptionalType {
    type Error = Error;

    fn try_from(value: &syn::Field) -> Result<Self, Self::Error> {
        let ty = &value.ty;
        if let syn::Type::Path(syn::TypePath { path, .. }) = &ty {
            if path.segments.first().unwrap().ident == "Option" {
                Ok(Self((quote! { #ty }).to_string()))
            } else {
                Ok(Self((quote! { Option<#ty> }).to_string()))
            }
        } else {
            Err(Error::new_spanned(value, "unknown type"))
        }
    }
}

fn desc(value: &Attribute) -> Result<String, Error> {
    if let Expr::Lit(ExprLit {
        lit: Lit::Str(s), ..
    }) = &value.parse_args()?
    {
        Ok(s.value())
    } else {
        Err(Error::new_spanned(value, "a description missed"))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use proc_macro2::TokenStream;
    use syn::ItemStruct;

    fn field(input: TokenStream) -> syn::Field {
        let item = syn::parse2::<ItemStruct>(quote! {
            struct Foo {
                #input
            }
        });
        item.unwrap().fields.into_iter().next().unwrap()
    }

    #[test]
    fn plain() {
        let input = field(quote! { pub name: String, });
        let output: Field = (&input).try_into().unwrap();

        assert_eq!(
            output,
            Field {
                kind: Kind::Plain,
                name: "name".to_string(),
                optional_ty: "Option < String >".to_string(),
                ty: "String".to_string(),
            }
        );
    }

    #[test]
    fn optional() {
        let input = field(quote! { pub name: Option<String>, });
        let output: Field = (&input).try_into().unwrap();

        assert_eq!(
            output,
            Field {
                kind: Kind::Plain,
                name: "name".to_string(),
                optional_ty: "Option < String >".to_string(),
                ty: "Option < String >".to_string(),
            }
        );
    }

    #[test]
    fn valid_filter() {
        let input = field(quote! {
            #[filter("name description")]
            pub name: String,
        });
        let output: Field = (&input).try_into().unwrap();

        assert_eq!(
            output,
            Field {
                kind: Kind::Filter("name description".to_string()),
                name: "name".to_string(),
                optional_ty: "Option < String >".to_string(),
                ty: "String".to_string(),
            },
        );
    }

    #[test]
    fn valid_limit() {
        let input = field(quote! {
            #[limit("name description")]
            pub name: String,
        });
        let output: Field = (&input).try_into().unwrap();

        assert_eq!(
            output,
            Field {
                kind: Kind::Limit("name description".to_string()),
                name: "name".to_string(),
                optional_ty: "Option < String >".to_string(),
                ty: "String".to_string(),
            },
        );
    }

    #[test]
    #[should_panic]
    fn limit_without_description() {
        let input = field(quote! {
            #[limit]
            pub name: String,
        });
        let _: Field = (&input).try_into().unwrap();
    }

    #[test]
    #[should_panic]
    fn filter_without_description() {
        let input = field(quote! {
            #[filter]
            pub name: String,
        });
        let _: Field = (&input).try_into().unwrap();
    }

    #[test]
    #[should_panic]
    fn double_filter() {
        let input = field(quote! {
            #[filter("name description")]
            #[filter("another description")]
            pub name: String,
        });
        let _: Field = (&input).try_into().unwrap();
    }

    #[test]
    #[should_panic]
    fn double_limit() {
        let input = field(quote! {
            #[limit("name description")]
            #[limit("another description")]
            pub name: String,
        });
        let _: Field = (&input).try_into().unwrap();
    }

    #[test]
    #[should_panic]
    fn filter_and_limit() {
        let input = field(quote! {
            #[filter("name description")]
            #[limit("another description")]
            pub name: String,
        });
        let _: Field = (&input).try_into().unwrap();
    }

    #[test]
    #[should_panic]
    fn unknown_attribute() {
        let input = field(quote! {
            #[foo]
            pub name: String,
        });
        let _: Field = (&input).try_into().unwrap();
    }
}
