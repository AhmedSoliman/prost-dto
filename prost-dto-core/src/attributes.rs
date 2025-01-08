use std::fmt::{Display, Formatter};

use darling::util::SpannedValue;
use darling::{FromDeriveInput, FromField, FromVariant};
use syn::{Field, Variant};

#[derive(Debug, Clone)]
pub(crate) enum Direction<F, I> {
    FromProst(F),
    IntoProst(I),
}

impl<F, I> Direction<F, I>
where
    F: Clone + Clone,
    I: Clone + Clone,
{
    pub fn is_into(&self) -> bool {
        matches!(self, Direction::IntoProst(_))
    }

    pub fn is_from(&self) -> bool {
        matches!(self, Direction::FromProst(_))
    }

    pub fn with_variant(
        &self,
        variant: &Variant,
    ) -> darling::Result<Direction<FromProstVariantInfo, IntoProstVariantInfo>> {
        Ok(match self {
            Direction::FromProst(_) => {
                Direction::FromProst(FromProstVariantInfo::from_variant(variant)?)
            }
            Direction::IntoProst(_) => {
                Direction::IntoProst(IntoProstVariantInfo::from_variant(variant)?)
            }
        })
    }

    pub fn with_field(
        &self,
        field: &Field,
    ) -> darling::Result<Direction<FromProstFieldInfo, IntoProstFieldInfo>> {
        Ok(match self {
            Direction::FromProst(_) => Direction::FromProst(FromProstFieldInfo::from_field(field)?),
            Direction::IntoProst(_) => Direction::IntoProst(IntoProstFieldInfo::from_field(field)?),
        })
    }
}

impl<A, B> Display for Direction<A, B> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Direction::FromProst(_) => write!(f, "FromProst"),
            Direction::IntoProst(_) => write!(f, "IntoProst"),
        }
    }
}

// Attributes for struct/enum level #[prost(...)]
#[derive(Debug, Clone, FromDeriveInput)]
#[darling(attributes(prost), supports(struct_named, enum_newtype, enum_unit))]
pub(crate) struct ProstInfo {
    pub ident: syn::Ident,
    pub target: syn::Path,
    pub oneof: Option<syn::Ident>,
    #[darling(default)]
    // If true, the generated match will include a default arm.
    pub non_exhaustive: SpannedValue<bool>,
}

// Attributes for struct/enum level #[from_prost(...)]
#[derive(Debug, Clone, FromDeriveInput)]
#[darling(
    attributes(from_prost),
    supports(struct_named, enum_newtype, enum_unit)
)]
pub(crate) struct FromProstInfo {
    // Reserved for future use.
}

// Attributes for struct/enum level #[into_prost(...)]
#[derive(Debug, Clone, FromDeriveInput)]
#[darling(
    attributes(into_prost),
    supports(struct_named, enum_newtype, enum_unit)
)]
pub(crate) struct IntoProstInfo {
    // Reserved for future use.
}

// Attributes for enum-variant level #[prost(...)]
// This one is used for common attributes across from and into.
#[derive(Debug, Clone, FromVariant)]
#[darling(attributes(prost))]
pub(crate) struct ProstVariantInfo {
    // automatically populated by darling
    pub ident: syn::Ident,
    pub fields: darling::ast::Fields<ProstEnumFieldInfo>,
    // our prost variant attributes
    #[darling(default)]
    pub name: Option<syn::Ident>,
    #[darling(default)]
    pub skip: bool,
}
// Attributes for enum-variant level #[from_prost(...)]
#[derive(Debug, Clone, FromVariant)]
#[darling(attributes(from_prost))]
pub(crate) struct FromProstVariantInfo {
    // Reserved for future use.
}
// Attributes for enum-variant level #[into_prost(...)]
#[derive(Debug, Clone, FromVariant)]
#[darling(attributes(into_prost))]
pub(crate) struct IntoProstVariantInfo {
    // Reserved for future use.
}

#[derive(Debug, Clone, FromField)]
#[darling(attributes(prost))]
pub(crate) struct ProstEnumFieldInfo {
    // Reserved for future use.
}

// Attributes for struct-field level #[prost(...)]
#[derive(Debug, Clone, FromField)]
#[darling(attributes(prost))]
pub(crate) struct ProstFieldInfo {
    // automatically populated by darling
    pub ident: Option<syn::Ident>,
    pub ty: syn::Type,

    // our prost field attributes
    #[darling(default)]
    pub skip: bool,
    #[darling(default)]
    pub name: Option<syn::Ident>,
    #[darling(default)]
    pub required: bool,
}

impl ProstFieldInfo {
    // We only support structs with named fields (no tuples) so we can safely
    // unwrap ident.
    pub fn ident(&self) -> &syn::Ident {
        self.ident.as_ref().unwrap()
    }
}

// Attributes for struct-field level #[from_prost(...)]
#[derive(Debug, Clone, FromField)]
#[darling(attributes(from_prost))]
pub(crate) struct FromProstFieldInfo {
    #[darling(default)]
    // Always set the value to None (if must be Option<T>) in FromProst
    // conversion, effectively making this a read-only field.
    pub always_none: bool,

    #[darling(default)]
    pub map: Option<syn::Path>,
    #[darling(default)]
    pub map_by_ref: SpannedValue<bool>,
}

// Attributes for struct-field level #[into_prost(...)]
#[derive(Debug, Clone, FromField)]
#[darling(attributes(into_prost))]
pub(crate) struct IntoProstFieldInfo {
    #[darling(default)]
    pub map: Option<syn::Path>,
    #[darling(default)]
    pub map_by_ref: SpannedValue<bool>,
}

pub(crate) trait Skip {
    fn is_skipped(&self) -> bool;
}

impl Skip for ProstFieldInfo {
    fn is_skipped(&self) -> bool {
        self.skip
    }
}

impl Skip for ProstVariantInfo {
    fn is_skipped(&self) -> bool {
        self.skip
    }
}
