use darling::ast::Style;
use proc_macro2::TokenStream;
use quote::quote_spanned;

use crate::attributes::{
    Direction, FromProstVariantInfo, IntoProstVariantInfo, ProstVariantInfo, Skip,
};

impl ProstVariantInfo {
    pub(crate) fn gen_tokens(
        self,
        direction: Direction<FromProstVariantInfo, IntoProstVariantInfo>,
        source_type: &TokenStream,
        target_type: &TokenStream,
    ) -> darling::Result<TokenStream> {
        let span = self.ident.span();
        let variant_name = &self.ident;
        let target_variant_name = self.name.as_ref().unwrap_or(variant_name);

        if self.is_skipped() {
            return Ok(TokenStream::new());
        }

        let (s_variant, t_variant) = match direction {
            Direction::IntoProst(_) => (variant_name, target_variant_name),
            Direction::FromProst(_) => (target_variant_name, variant_name),
        };

        match self.fields.style {
            Style::Unit => Ok(quote_spanned! { span =>
                #source_type::#s_variant => #target_type::#t_variant,
            }),
            Style::Tuple => Ok(quote_spanned! { span =>
                #source_type::#s_variant(v) => #target_type::#t_variant(v.into()),
            }),
            Style::Struct => Err(darling::Error::unsupported_shape(
                "Enums with struct-like fields are not supported",
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use darling::error::Accumulator;
    use darling::FromVariant;
    use pretty_assertions::assert_eq;
    use quote::quote;
    use syn::parse_quote;

    use super::*;

    #[track_caller]
    fn gen_tokens_test_helper(
        variant: ProstVariantInfo,
        direction: Direction<FromProstVariantInfo, IntoProstVariantInfo>,
        source_type: &TokenStream,
        target_type: &TokenStream,
        expected: TokenStream,
    ) -> darling::Result<()> {
        let mut acc = Accumulator::default();
        let actual = acc.handle(variant.gen_tokens(direction, source_type, target_type));
        acc.finish()?;
        let actual = actual.unwrap();
        assert_eq!(expected.to_string(), actual.to_string());
        Ok(())
    }

    #[test]
    fn gen_tokens_shape() -> darling::Result<()> {
        // We don't support struct-style enums.
        let variant: syn::Variant = parse_quote! { Something {x: i32, y: String} };
        let variant_info = ProstVariantInfo::from_variant(&variant)?;

        let direction: Direction<FromProstVariantInfo, IntoProstVariantInfo> =
            Direction::IntoProst(IntoProstVariantInfo::from_variant(&variant)?);

        let source_type = &parse_quote! { Foo };
        let target_type = &parse_quote! { Bar };

        let maybe_tokens =
            gen_tokens_test_helper(variant_info, direction, source_type, target_type, quote! {});

        assert!(maybe_tokens.is_err());

        assert_eq!(
            maybe_tokens.unwrap_err().to_string(),
            "Unsupported shape `Enums with struct-like fields are not \
             supported`"
        );

        Ok(())
    }

    #[test]
    fn gen_tokens_skipped() -> darling::Result<()> {
        // Normal
        let variant: syn::Variant = parse_quote! { Something };
        let variant_info = ProstVariantInfo::from_variant(&variant)?;
        assert!(!variant_info.is_skipped());

        // Skipped
        let variant: syn::Variant = parse_quote! { #[prost(skip)] Something };
        let variant_info = ProstVariantInfo::from_variant(&variant)?;
        assert!(variant_info.is_skipped());
        Ok(())
    }

    #[test]
    fn gen_tokens_unit() -> darling::Result<()> {
        // unit
        {
            let variant: syn::Variant = parse_quote! { Something  };

            let variant_info = ProstVariantInfo::from_variant(&variant)?;

            let direction: Direction<FromProstVariantInfo, IntoProstVariantInfo> =
                Direction::IntoProst(IntoProstVariantInfo::from_variant(&variant)?);
            let source_type = &parse_quote! { Foo };
            let target_type = &parse_quote! { Bar };

            gen_tokens_test_helper(
                variant_info.clone(),
                direction,
                source_type,
                target_type,
                quote! {
                    Foo::Something => Bar::Something,
                },
            )?;

            let direction: Direction<FromProstVariantInfo, IntoProstVariantInfo> =
                Direction::FromProst(FromProstVariantInfo::from_variant(&variant)?);

            gen_tokens_test_helper(
                variant_info,
                direction,
                source_type,
                target_type,
                quote! {
                    Foo::Something => Bar::Something,
                },
            )?;
        }

        // unit - renamed
        {
            let variant: &syn::Variant =
                &parse_quote! { #[prost(name = "AnotherThing")] Something };

            let variant_info = ProstVariantInfo::from_variant(variant)?;

            let direction: Direction<FromProstVariantInfo, IntoProstVariantInfo> =
                Direction::IntoProst(IntoProstVariantInfo::from_variant(variant)?);
            let source_type = &parse_quote! { Foo };
            let target_type = &parse_quote! { Bar };

            gen_tokens_test_helper(
                variant_info.clone(),
                direction,
                source_type,
                target_type,
                quote! {
                    Foo::Something => Bar::AnotherThing,
                },
            )?;

            let direction: Direction<FromProstVariantInfo, IntoProstVariantInfo> =
                Direction::FromProst(FromProstVariantInfo::from_variant(variant)?);

            gen_tokens_test_helper(
                variant_info,
                direction,
                source_type,
                target_type,
                quote! {
                    Foo::AnotherThing => Bar::Something,
                },
            )?;
        }
        Ok(())
    }

    #[test]
    fn gen_tokens_non_unit() -> darling::Result<()> {
        {
            let variant: &syn::Variant = &parse_quote! { Something(BigObject) };

            let variant_info = ProstVariantInfo::from_variant(variant)?;

            let direction: Direction<FromProstVariantInfo, IntoProstVariantInfo> =
                Direction::IntoProst(IntoProstVariantInfo::from_variant(variant)?);
            let source_type = &parse_quote! { Foo };
            let target_type = &parse_quote! { Bar };

            gen_tokens_test_helper(
                variant_info.clone(),
                direction,
                source_type,
                target_type,
                quote! {
                    Foo::Something(v) => Bar::Something(v.into()),
                },
            )?;

            let direction: Direction<FromProstVariantInfo, IntoProstVariantInfo> =
                Direction::FromProst(FromProstVariantInfo::from_variant(variant)?);

            gen_tokens_test_helper(
                variant_info,
                direction,
                source_type,
                target_type,
                quote! {
                    Foo::Something(v) => Bar::Something(v.into()),
                },
            )?;
        }

        // renamed
        {
            let variant: &syn::Variant = &parse_quote! {
                #[prost(name = "AnotherThing")]
                Something(BigObject)
            };
            let variant_info = ProstVariantInfo::from_variant(variant)?;

            let direction: Direction<FromProstVariantInfo, IntoProstVariantInfo> =
                Direction::IntoProst(IntoProstVariantInfo::from_variant(variant)?);
            let source_type = &parse_quote! { Foo };
            let target_type = &parse_quote! { Bar };

            gen_tokens_test_helper(
                variant_info.clone(),
                direction,
                source_type,
                target_type,
                quote! {
                    Foo::Something(v) => Bar::AnotherThing(v.into()),
                },
            )?;

            let direction: Direction<FromProstVariantInfo, IntoProstVariantInfo> =
                Direction::FromProst(FromProstVariantInfo::from_variant(variant)?);

            gen_tokens_test_helper(
                variant_info,
                direction,
                source_type,
                target_type,
                quote! {
                    Foo::AnotherThing(v) => Bar::Something(v.into()),
                },
            )?;
        }
        Ok(())
    }
}
