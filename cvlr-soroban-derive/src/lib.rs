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
