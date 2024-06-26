use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(IntoProto, attributes(proto, into_proto))]
pub fn derive_into_proto(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    prost_dto_core::derive_into_proto(input).into()
}

#[proc_macro_derive(FromProto, attributes(proto, from_proto))]
pub fn derive_from_proto(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    prost_dto_core::derive_from_proto(input).into()
}
