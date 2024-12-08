use codama_errors::{CodamaError, CodamaResult};
use codama_syn_helpers::syn_traits::*;

#[derive(Debug, PartialEq)]
pub struct DeriveAttribute<'a> {
    pub ast: &'a syn::Attribute,
    pub derives: Vec<syn::Path>,
}

impl<'a> DeriveAttribute<'a> {
    pub fn parse<T: TryInto<Self, Error = CodamaError>>(attr: T) -> CodamaResult<Self> {
        attr.try_into()
    }
}

impl<'a> TryFrom<&'a syn::Attribute> for DeriveAttribute<'a> {
    type Error = CodamaError;

    fn try_from(ast: &'a syn::Attribute) -> CodamaResult<Self> {
        // Check if the attribute is feature-gated.
        let unfeatured = ast.unfeatured();
        let attr = unfeatured.as_ref().unwrap_or(ast);

        // Check if the attribute is a #[derive(...)] attribute.
        let list = attr.meta.require_list()?;
        if !list.path.is_strict("derive") {
            return Err(syn::Error::new_spanned(&list.path, "expected #[derive(...)]").into());
        };

        // Parse the list of derives.
        let derives = attr.parse_comma_args::<syn::Path>()?;
        Ok(Self { ast, derives })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use codama_syn_helpers::syn_build;
    use quote::quote;

    #[test]
    fn test_derive_attribute() {
        let ast = syn_build::attribute(quote! { #[derive(Debug, PartialEq)] });
        let attribute = DeriveAttribute::parse(&ast).unwrap();

        assert_eq!(attribute.ast, &ast);
        assert_eq!(
            attribute.derives,
            [
                syn_build::parse(quote! { Debug }),
                syn_build::parse(quote! { PartialEq }),
            ]
        );
    }

    #[test]
    fn test_feature_gated_derive_attribute() {
        let ast = syn_build::attribute(
            quote! { #[cfg_attr(feature = "some_feature", derive(Debug, PartialEq))] },
        );
        let attribute = DeriveAttribute::parse(&ast).unwrap();

        assert_eq!(attribute.ast, &ast);
        assert_eq!(
            attribute.derives,
            [
                syn_build::parse(quote! { Debug }),
                syn_build::parse(quote! { PartialEq }),
            ]
        );
    }
}