#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct VariantToken<'a, T> {
    pub variant: T,
    pub trivia: &'a str,
}

impl<'a, T> Default for VariantToken<'a, T>
where
    T: Default,
{
    fn default() -> Self {
        VariantToken {
            variant: T::default(),
            trivia: "",
        }
    }
}
