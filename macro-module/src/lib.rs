#[macro_use]
extern crate syn;
extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;

/// ```
/// use macro_module::add_answer;
///
/// #[add_answer]
/// mod a {
///     pub fn original1() -> u32 { 21 }
///     pub fn original2() -> u32 { 42 }
/// }
///
/// assert_eq!(a::original1(), 21);
/// assert_eq!(a::original2(), 42);
/// assert_eq!(a::answer(), 42);
/// ```
#[proc_macro_attribute]
pub fn add_answer(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let module = parse_macro_input!(item as syn::ItemMod);
    let name = module.ident;
    let body = match module.content {
        None => vec![],
        Some((_, items)) => items,
    };

    quote! {
        mod #name {
            #(#body)*

            pub fn answer() -> u32 { 42 }
        }
    }
    .into()
}
