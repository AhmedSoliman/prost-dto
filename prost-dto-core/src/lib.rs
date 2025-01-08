use darling::FromDeriveInput;
use proc_macro2::TokenStream;
use syn::DeriveInput;
mod attributes;
mod enum_codegen;
mod proto_conv;
mod struct_codegen;
mod utils;

use self::attributes::{Direction, FromProstInfo, IntoProstInfo, ProstInfo};

pub fn derive_into_prost(input: DeriveInput) -> TokenStream {
    let into_info = match IntoProstInfo::from_derive_input(&input) {
        Ok(info) => info,
        Err(e) => {
            return e.write_errors();
        }
    };
    derive_prost(Direction::IntoProst(into_info), input)
}

pub fn derive_from_prost(input: DeriveInput) -> TokenStream {
    let from_info = match FromProstInfo::from_derive_input(&input) {
        Ok(info) => info,
        Err(e) => {
            return e.write_errors();
        }
    };
    derive_prost(Direction::FromProst(from_info), input)
}

fn derive_prost(
    direction: Direction<FromProstInfo, IntoProstInfo>,
    input: DeriveInput,
) -> TokenStream {
    let tokens = ProstInfo::from_derive_input(&input)
        .and_then(|info| proto_conv::expand_proto_conv(direction, info, input));

    match tokens {
        Ok(tokens) => tokens,
        Err(e) => e.write_errors(),
    }
}
