use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input,
    punctuated::Punctuated,
    token::Comma,
    Ident, Token,
};
use uuid::Uuid;

struct Idents {
    idents: Punctuated<Ident, Comma>,
}

impl Parse for Idents {
    fn parse(input: ParseStream) -> syn::parse::Result<Self> {
        Ok(Idents {
            idents: input.parse_terminated(Ident::parse, Token![,])?,
        })
    }
}

pub fn declare_rule(input: TokenStream) -> TokenStream {
    let x = parse_macro_input!(input as Idents);
    let rules = x
        .idents
        .iter()
        .flat_map(|i| format!("{}\0", i).into_bytes())
        .collect::<Vec<u8>>();
    let rule_lit = proc_macro2::Literal::byte_string(rules.as_slice());
    let rule_size: usize = rules.len();
    let rule_set_name = format_ident!("{}", format!("RULES_{}", Uuid::new_v4().simple()));

    quote! {
        #[cfg_attr(target_family = "wasm", link_section = "certora_rules")]
        pub static #rule_set_name: [u8; #rule_size] = *#rule_lit;
    }
    .into()
}

/// The rule, exported under its own name and listed in `certora_rules`. Its body ends with
/// `cvlr_vacuity_check!()`, as in `cvlr`'s `#[rule]`; an early `return` skips it.
pub fn rule(_attr: TokenStream, input: TokenStream) -> TokenStream {
    extern crate self as us;
    let mut fn_item = parse_macro_input!(input as syn::ItemFn);
    let rule_name = format_ident!("{}", &fn_item.sig.ident);
    let block = &fn_item.block;
    fn_item.block = syn::parse_quote!({
        let cvlr_rule_result = #block;
        ::cvlr_soroban::rule::cvlr_vacuity_check!();
        cvlr_rule_result
    });

    quote! {
        cvlr_soroban_derive::declare_rule!(#rule_name);
        #[no_mangle]
        #fn_item
    }
    .into()
}
