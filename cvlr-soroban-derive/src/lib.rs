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

// A dummy field attribute to return the original input.
#[proc_macro_attribute]
pub fn topic(_attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}

// A dummy macro for `#[contractevent]`.
// It removes `#[topic]` and returns the struct without any changes.
#[proc_macro_attribute]
pub fn contractevent(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(item as syn::ItemStruct);
    let mut output = input.clone();

    // Removes #[topic] attributes from fields
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

    let vis = &output.vis;
    let ident = &output.ident;
    let fields = &output.fields;

    // Return the struct without `#[topic]`
    let expanded = quote::quote! {
        #vis struct #ident #fields
    };

    expanded.into()
}