//!

use proc_macro::TokenStream;
use proc_macro2::{Span as Span2, TokenStream as TokenStream2};
use syn::{Ident, Item, ImplItem, ImplItemMethod};
use quote::quote;

#[proc_macro_attribute]
pub fn mine_methods(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let item: TokenStream2 = item.into();
    let parsed_item = syn::parse2::<Item>(item).expect("Failed to parse item");
    if let Item::Impl(item_impl) = parsed_item {
        assert!(
            item_impl.trait_.is_none(),
            "#[mine_methods] is only applicable to inherent impl blocks"
        );

        let mod_name = format!("foo_methods"); // TODO change to e.g. <TYPE>_methods
        let mod_name = Ident::new(&mod_name, Span2::call_site());

        let mut methods: Vec<String> = vec![];
        for item in &item_impl.items {
            if let ImplItem::Method(ImplItemMethod { sig, .. }) = item {
                methods.push(format!("{}", quote! { #sig }));
            }
        }

        let quoted = quote! {
            pub mod #mod_name {
                lazy_static::lazy_static! {
                    pub static ref METHODS: Vec<&'static str> = vec![
                        #(#methods),*
                    ];
                }
            }
        };
        quoted.into()

    } else {
        panic!("#[mine_methods] is only applicable to inherent impl blocks")
    }

}
