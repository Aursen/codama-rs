use codama_errors::CodamaResult;

pub trait Fields {
    fn get_self(&self) -> &syn::Fields;

    fn single_unnamed_field(&self) -> CodamaResult<&syn::Field> {
        let this = self.get_self();
        match this {
            syn::Fields::Unnamed(fields) if fields.unnamed.len() == 1 => Ok(&fields.unnamed[0]),
            _ => Err(syn::Error::new_spanned(
                this,
                "expected a single unnamed field in the variant",
            )
            .into()),
        }
    }
}

impl Fields for syn::Fields {
    fn get_self(&self) -> &syn::Fields {
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::syn_build;
    use quote::quote;

    #[test]
    fn single_unnamed_field_ok() {
        let r#struct: syn::ItemStruct = syn_build::parse(quote! { struct Foo(u32); });
        assert!(matches!(r#struct.fields.single_unnamed_field(), Ok(_)));
    }

    #[test]
    fn single_unnamed_field_err() {
        let r#struct: syn::ItemStruct = syn_build::parse(quote! { struct Foo(u32, u64); });
        assert!(matches!(r#struct.fields.single_unnamed_field(), Err(_)));
    }
}