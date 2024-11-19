#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum IsAccountSigner {
    True,
    False,
    Either,
}

impl From<bool> for IsAccountSigner {
    fn from(value: bool) -> Self {
        match value {
            true => Self::True,
            false => Self::False,
        }
    }
}
