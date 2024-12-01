use codama_errors::CodamaResult;

pub trait Expr {
    fn get_self(&self) -> &syn::Expr;

    /// Returns the integer value of the expression if it is a literal integer.
    fn as_literal_integer<T>(&self) -> CodamaResult<T>
    where
        T: std::str::FromStr,
        T::Err: std::fmt::Display,
    {
        let this = self.get_self();
        match this {
            syn::Expr::Lit(syn::ExprLit {
                lit: syn::Lit::Int(value),
                ..
            }) => value.base10_parse::<T>().map_err(Into::into),
            _ => Err(syn::Error::new_spanned(
                this,
                "Could not evaluate expression as a literal integer",
            )
            .into()),
        }
    }
}

impl Expr for syn::Expr {
    fn get_self(&self) -> &syn::Expr {
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::syn_build;
    use codama_errors::CodamaError;
    use quote::quote;

    #[test]
    fn as_literal_integer_ok() {
        let expr: syn::Expr = syn_build::parse(quote! { 42 });
        let result = expr.as_literal_integer::<usize>();
        assert!(matches!(result, Ok(42usize)));
    }

    #[test]
    fn as_literal_integer_err() {
        let expr: syn::Expr = syn_build::parse(quote! { 40 + 2 });
        let result = expr.as_literal_integer::<usize>();
        assert!(matches!(result, Err(CodamaError::Compilation(_))));
    }
}