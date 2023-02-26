//!

use proc_macro::TokenStream;
use proc_macro2::{Span as Span2, TokenStream as TokenStream2};
use quote::quote;
use syn::{FnArg, Ident, ImplItem, ImplItemMethod, Item, Type};

#[proc_macro_attribute]
pub fn mine_methods(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut output = item.clone(); // Haven't consider attr on the impl
    let item: TokenStream2 = item.into();
    let parsed_item = syn::parse2::<Item>(item).expect("Failed to parse item");
    if let Item::Impl(item_impl) = parsed_item {
        assert!(
            item_impl.trait_.is_none(),
            "#[mine_methods] is only applicable to inherent impl blocks"
        );

        let self_ty = if let Type::Path(implementor) = &*item_impl.self_ty {
            implementor.path.get_ident().unwrap()
        } else {
            unimplemented!("implementor must be a single ident")
        };
        // struct name Foo -> mod prefix foo
        let mod_name = format!("{}_methods", self_ty.to_string().to_ascii_lowercase());
        let mod_name = Ident::new(&mod_name, Span2::call_site());

        // function pointer sig: fn(...) -> ...
        let mut fn_sig: Option<TokenStream2> = None;
        // function pointer: Foo::method
        let mut fn_pointers: Vec<TokenStream2> = vec![];
        // method name
        let mut methods: Vec<String> = vec![];
        for item in &item_impl.items {
            if let ImplItem::Method(ImplItemMethod { sig, .. }) = item {
                methods.push(format!("{}", quote! { #sig }));
                fn_sig.get_or_insert_with(|| fn_args(sig.inputs.iter(), &sig.output));
                fn_pointers.push(fn_pointer(self_ty, &sig.ident));
            }
        }

        println!(
            "{}\n{}",
            quote!( #fn_sig ),
            fn_pointers
                .iter()
                .map(|f| f.to_string())
                .collect::<Vec<_>>()
                .join("\t")
        );
        let quoted = quote! {
            pub mod #mod_name {
                use super::*;
                use std::collections::HashMap;
                type FnPointer = #fn_sig;
                lazy_static::lazy_static! {
                    pub static ref METHODS: Vec<&'static str> = vec![
                        #(#methods),*
                    ];
                    pub static ref FN_POINTERS: Vec<  FnPointer  > = vec![
                        #(#fn_pointers),*
                    ];
                    pub static ref FN_MAP: HashMap< &'static str, FnPointer > = {
                        METHODS.iter().zip(FN_POINTERS.iter()).map(|(s, f)| (*s, *f)).collect()
                    };
                }
            }
        };
        output.extend(TokenStream::from(quoted));
        output
    } else {
        panic!("#[mine_methods] is only applicable to inherent impl blocks")
    }
}

fn fn_pointer(self_ty: &Ident, method: &Ident) -> TokenStream2 {
    quote!(#self_ty :: #method)
}

fn fn_args<'a>(inputs: impl Iterator<Item = &'a FnArg>, output: &syn::ReturnType) -> TokenStream2 {
    let iter = inputs.filter_map(|i| {
        if let FnArg::Typed(arg) = i {
            Some(&*arg.ty)
        } else {
            None
        }
    });
    quote!( fn ( #(#iter),* ) #output )
}
