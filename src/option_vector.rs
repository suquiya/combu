pub struct OptionVector<T>(Option<Vec<T>>);

impl<T> Default for OptionVector<T> {
    fn default() -> Self {
        OptionVector(None)
    }
}

impl<T> OptionVector<T> {
    pub fn push(&mut self, push: T) {
        match self {
            OptionVector(None) => {
                *self = OptionVector(Some(vec![push]));
            }
            OptionVector(Some(ref mut v)) => (*v).push(push),
        }
    }
    pub fn none(self) -> bool {
        match self {
            OptionVector(None) => false,
            OptionVector(Some(_)) => true,
        }
    }
}
