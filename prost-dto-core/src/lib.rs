use darling::FromDeriveInput;
use proc_macro2::TokenStream;
use syn::DeriveInput;
mod attributes;
mod enum_codegen;
mod proto_conv;
mod struct_codegen;
mod utils;

use self::attributes::{Direction, FromProtoInfo, IntoProtoInfo, ProtoInfo};

pub fn derive_into_proto(input: DeriveInput) -> TokenStream {
    let into_info = match IntoProtoInfo::from_derive_input(&input) {
        Ok(info) => info,
        Err(e) => {
            return e.write_errors();
        }
    };
    derive_proto(Direction::IntoProto(into_info), input)
}

pub fn derive_from_proto(input: DeriveInput) -> TokenStream {
    let from_info = match FromProtoInfo::from_derive_input(&input) {
        Ok(info) => info,
        Err(e) => {
            return e.write_errors();
        }
    };
    derive_proto(Direction::FromProto(from_info), input)
}

fn derive_proto(
    direction: Direction<FromProtoInfo, IntoProtoInfo>,
    input: DeriveInput,
) -> TokenStream {
    let tokens = ProtoInfo::from_derive_input(&input)
        .and_then(|info| proto_conv::expand_proto_conv(direction, info, input));

    match tokens {
        Ok(tokens) => tokens,
        Err(e) => e.write_errors(),
    }
}
