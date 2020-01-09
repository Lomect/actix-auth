extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use quote::quote;
use syn::{
    parse_macro_input, AttributeArgs, Data, DataStruct, DeriveInput, Fields, FnArg, ItemFn, Meta,
    NestedMeta, Pat,
};

#[proc_macro_attribute]
pub fn login_required(_: TokenStream, func: TokenStream) -> TokenStream {
    let func = parse_macro_input!(func as ItemFn);
    let func_vis = &func.vis;
    let func_block = &func.block;

    let login_ident = Ident::new("_id", Span::call_site());

    let func_sig = &func.sig;
    let func_name = &func_sig.ident;
    let func_generics = &func_sig.generics;
    let func_inputs = &func_sig.inputs;
    let func_output = &func_sig.output;
    let func_async = &func_sig.asyncness.unwrap();

    let identity_vec: Vec<_> = func_inputs
        .iter()
        .filter(|s| match s {
            FnArg::Typed(ref arg_type) => {
                if let Pat::Ident(ref id) = *arg_type.pat {
                    id.ident.eq(&login_ident)
                } else {
                    false
                }
            }
            _ => unreachable!("This not gonna happend!"),
        })
        .map(|s| match s {
            FnArg::Typed(ref arg) => &arg.pat,
            _ => unreachable!("This not gonna happend!"),
        })
        .collect();

    let identity = *identity_vec.get(0).unwrap();

    let expended = quote! {
        #func_vis #func_async fn #func_name #func_generics(#func_inputs) #func_output {
            if let Some(ident) = #identity.identity() {
                #func_block
            } else {
                actix_web::HttpResponse::Unauthorized().json("Unauthorized")
            }
        }
    };
    expended.into()
}
