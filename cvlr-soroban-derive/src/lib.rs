use proc_macro::TokenStream;

mod mock_client;
mod rule;

#[proc_macro_attribute]
pub fn cvlr_mock_client(attr: TokenStream, item: TokenStream) -> TokenStream {
    mock_client::cvlr_mock_client(attr, item)
}

#[proc_macro]
pub fn declare_rule(input: TokenStream) -> TokenStream {
    rule::declare_rule(input)
}

#[proc_macro_attribute]
pub fn rule(attr: TokenStream, input: TokenStream) -> TokenStream {
    rule::rule(attr, input)
}

/// A compatibility stub for Soroban's `#[contractevent]`.
/// In CVLR builds we don't emit event metadata, but we still want event
/// structs to compile. We strip `#[topic]` so the attribute doesn't linger
/// as an unused field attribute.
/// # Example
/// ```
/// #[contractevent]
/// #[derive(Clone, Debug, Eq, PartialEq)]
/// pub struct RoleGranted {
///     #[topic]
///     pub role: Symbol,
///     #[topic]
///     pub account: Address,
///     pub caller: Address,
/// }
/// ```
#[proc_macro_attribute]
pub fn contractevent(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(item as syn::ItemStruct);
    let mut output = input.clone();

    // Remove #[topic] attributes from fields
    if let syn::Fields::Named(ref mut fields) = output.fields {
        for field in &mut fields.named {
            field.attrs = field
                .attrs
                .iter()
                .filter(|a| !a.path().is_ident("topic"))
                .cloned()
                .collect::<Vec<syn::Attribute>>();
        }
    }

    let ident = &output.ident;
    let generics = &output.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    quote::quote! {
        #output

        // stub out `publish`
        impl #impl_generics #ident #ty_generics #where_clause {
            #[inline(always)]
            pub fn publish<E>(&self, _env: &E) {}
        }
    }
    .into()
}

/// A no-op attribute so `#[topic]` doesn't cause errors outside of
/// `#[contractevent]` contexts.
#[proc_macro_attribute]
pub fn topic(_attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}
