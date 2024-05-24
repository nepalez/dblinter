mod expand;
mod field;
mod item;

use expand::expand;
use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn problem(attr: TokenStream, item: TokenStream) -> TokenStream {
    expand(attr.into(), item.into()).into()
}
