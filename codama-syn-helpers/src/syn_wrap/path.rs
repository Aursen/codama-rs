use super::GenericArguments;

pub struct Path<'a>(pub &'a syn::Path);

impl Path<'_> {
    /// Returns all segment idents joined by "::" except the last one.
    /// E.g. for `a::b<B>::c::Option<T>` it returns `a::b::c`.
    pub fn prefix(&self) -> String {
        self.0
            .segments
            .iter()
            .map(|segment| segment.ident.to_string())
            .collect::<Vec<_>>()[..self.0.segments.len() - 1]
            .join("::")
    }

    /// Returns the last segment.
    pub fn last(&self) -> &syn::PathSegment {
        self.0.segments.last().unwrap()
    }

    /// Returns the ident of the last segment as a string.
    pub fn last_indent(&self) -> String {
        self.last().ident.to_string()
    }

    /// Returns true if the path is equal to the given path including or excluding the prefix.
    pub fn is(&self, path: &str) -> bool {
        let mut segments = path.split("::").collect::<Vec<_>>();
        let last = segments.pop().unwrap();
        let prefix = segments.join("::");
        (prefix == self.prefix() || prefix == "") && last == self.last_indent()
    }

    /// Returns true if the path is equal to the given path including the prefix.
    pub fn is_strict(&self, path: &str) -> bool {
        let mut segments = path.split("::").collect::<Vec<_>>();
        let last = segments.pop().unwrap();
        let prefix = segments.join("::");
        prefix == self.prefix() && last == self.last_indent()
    }

    /// Returns the generic arguments of the last segment.
    /// E.g. for `a::b::c::Option<'a, T, U>` it returns `GenericArguments(Some(['a, T, U]))`.
    /// E.g. for `a::b::c::u32` it returns `GenericArguments(None)`.
    pub fn generic_arguments(&self) -> GenericArguments {
        match &self.last().arguments {
            syn::PathArguments::AngleBracketed(syn::AngleBracketedGenericArguments {
                args,
                ..
            }) => GenericArguments(Some(args)),
            _ => GenericArguments(None),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::syn_build;
    use quote::quote;

    #[test]
    fn prefix() {
        let path = syn_build::parse(quote! { std::option::Option<String> });
        let result = Path(&path).prefix();
        assert_eq!(result, "std::option");
    }

    #[test]
    fn prefix_with_inner_generics() {
        let path = syn_build::parse(quote! { a<A>::b<B>::c::Final });
        let result = Path(&path).prefix();
        assert_eq!(result, "a::b::c");
    }

    #[test]
    fn prefix_empty() {
        let path = syn_build::parse(quote! { Foo });
        let result = Path(&path).prefix();
        assert_eq!(result, "");
    }

    #[test]
    fn is() {
        let path = syn_build::parse(quote! { prefix::Foo<'a, T> });
        assert_eq!(Path(&path).is("prefix::Foo"), true);
        assert_eq!(Path(&path).is("Foo"), true);
        assert_eq!(Path(&path).is("wrong::prefix::Foo"), false);
        assert_eq!(Path(&path).is("Bar"), false);

        let path = syn_build::parse(quote! { Foo<T> });
        assert_eq!(Path(&path).is("Foo"), true);
        assert_eq!(Path(&path).is("prefix::Foo"), false);
        assert_eq!(Path(&path).is("Bar"), false);
    }

    #[test]
    fn is_strict() {
        let path = syn_build::parse(quote! { prefix::Foo<'a, T> });
        assert_eq!(Path(&path).is_strict("prefix::Foo"), true);
        assert_eq!(Path(&path).is_strict("Foo"), false);
        assert_eq!(Path(&path).is_strict("wrong::prefix::Foo"), false);
        assert_eq!(Path(&path).is_strict("Bar"), false);

        let path = syn_build::parse(quote! { Foo<T> });
        assert_eq!(Path(&path).is_strict("Foo"), true);
        assert_eq!(Path(&path).is_strict("prefix::Foo"), false);
        assert_eq!(Path(&path).is_strict("Bar"), false);
    }

    #[test]
    fn generic_arguments() {
        let path = syn_build::parse(quote! { prefix::Foo<'a, T, U> });
        let path = Path(&path);
        let result = path.generic_arguments();
        assert!(matches!(result, GenericArguments(Some(_))));
    }
}
