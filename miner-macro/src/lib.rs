//!

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
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
        let mod_name = quote::format_ident!("{}_methods", self_ty.to_string().to_ascii_lowercase());

        // function pointer sig: fn(...) -> ...
        let mut fn_sig: Option<TokenStream2> = None;
        // function pointer: Foo::method
        let mut fn_pointers: Vec<TokenStream2> = vec![];
        // method name
        let mut methods: Vec<String> = vec![];
        for item in &item_impl.items {
            if let ImplItem::Method(ImplItemMethod { sig, .. }) = item {
                methods.push(format!("{}", quote! { #sig }));
                fn_sig.get_or_insert_with(|| fn_args(self_ty, sig.inputs.iter(), &sig.output));
                fn_pointers.push(fn_pointer(self_ty, &sig.ident));
            }
        }

        let quoted = quote! {
            pub mod #mod_name {
                use super::*;
                type __FnPointer = #fn_sig;
                lazy_static::lazy_static! {
                    pub static ref METHODS: ::std::vec::Vec<&'static ::std::primitive::str> = ::std::vec![
                        #(#methods),*
                    ];
                    pub static ref FN_POINTERS: ::std::vec::Vec<  __FnPointer  > = ::std::vec![
                        #(#fn_pointers),*
                    ];
                    pub static ref FN_MAP: ::std::collections::HashMap< &'static ::std::primitive::str, __FnPointer > = {
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

fn fn_args<'a>(
    self_ty: &Ident,
    inputs: impl Iterator<Item = &'a FnArg>,
    output: &syn::ReturnType,
) -> TokenStream2 {
    let iter = inputs.map(|i| {
        match i {
            FnArg::Receiver(syn::Receiver {
                reference,
                mutability,
                ..
            }) => {
                if reference.is_none() {
                    // implementor is self
                    quote!( #self_ty )
                } else if mutability.is_none() {
                    // &self
                    quote!( & #self_ty )
                } else {
                    // &mut self
                    quote!( &mut #self_ty )
                }
            }
            FnArg::Typed(arg) => {
                let ty = &arg.ty;
                quote!( #ty )
            }
        }
    });
    quote!( fn ( #(#iter),* ) #output )
}
