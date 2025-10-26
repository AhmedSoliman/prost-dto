use proc_macro::TokenStream;
use syn::{DeriveInput, parse_macro_input};

#[proc_macro_derive(IntoProst, attributes(prost, into_prost))]
pub fn derive_into_prost(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    prost_dto_core::derive_into_prost(input).into()
}

#[proc_macro_derive(FromProst, attributes(prost, from_prost))]
pub fn derive_from_prost(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    prost_dto_core::derive_from_prost(input).into()
}
