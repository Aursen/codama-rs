use crate::CodamaError;

pub trait CombineErrors {
    fn combine(&mut self, other: Self);
}

impl CombineErrors for CodamaError {
    fn combine(&mut self, other: Self) {
        if let (CodamaError::Compilation(this), CodamaError::Compilation(that)) = (self, other) {
            this.combine(that)
        }
    }
}

impl CombineErrors for syn::Error {
    fn combine(&mut self, other: Self) {
        syn::Error::combine(self, other)
    }
}

pub trait IteratorCombineErrors<T, E>: Iterator<Item = Result<T, E>>
where
    E: std::error::Error + CombineErrors,
{
    fn collect_and_combine_errors(self) -> Result<Vec<T>, E>
    where
        Self: Sized,
    {
        self.fold(Ok(Vec::new()), |acc, result| match (acc, result) {
            (Ok(mut acc_vec), Ok(parsed)) => {
                acc_vec.push(parsed);
                Ok(acc_vec)
            }
            (Err(mut acc_err), Err(err)) => {
                acc_err.combine(err);
                Err(acc_err)
            }
            (Err(acc_err), _) => Err(acc_err),
            (_, Err(err)) => Err(err),
        })
    }
}

impl<I, T, E> IteratorCombineErrors<T, E> for I
where
    I: Iterator<Item = Result<T, E>>,
    E: std::error::Error + CombineErrors,
{
}

/// Combine multiple results into a single result by combining errors.
/// Note we could use recursion here but the tuple would be nested.
/// E.g. (a, (b, c)) instead of (a, b, c).
#[macro_export]
macro_rules! combine_errors {
    // Base case: 1 result.
    ($result:expr) => {
        $result
    };

    // 2 results.
    ($first:expr, $second:expr $(,)?) => {{
        match ($first, $second) {
            (Ok(value1), Ok(value2)) => Ok((value1, value2)),
            (Err(err1), Err(err2)) => {
                let mut combined = err1;
                codama_errors::CombineErrors::combine(&mut combined, err2);
                Err(combined)
            }
            (Err(err), _) => Err(err),
            (_, Err(err)) => Err(err),
        }
    }};

    // 3 results.
    ($first:expr, $second:expr, $third:expr $(,)?) => {{
        match ($first, combine_errors!($second, $third)) {
            (Ok(value1), Ok((value2, value3))) => Ok((value1, value2, value3)),
            (Err(err1), Err(err2)) => {
                let mut combined = err1;
                codama_errors::CombineErrors::combine(&mut combined, err2);
                Err(combined)
            }
            (Err(err), _) => Err(err),
            (_, Err(err)) => Err(err),
        }
    }};
}
