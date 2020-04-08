#[derive(Clone, Debug)]
pub struct Vector<T>(pub Option<Vec<T>>);

impl<T> Default for Vector<T> {
    fn default() -> Self {
        Vector(None)
    }
}

impl<T> From<Vec<T>> for Vector<T> {
    fn from(vec: Vec<T>) -> Self {
        Vector(Some(vec))
    }
}

impl<T> Vector<T> {
    pub fn new() -> Self {
        Vector::default()
    }
    pub fn init(value: Option<Vec<T>>) -> Self {
        Vector(value)
    }
    pub fn push(&mut self, push: T) {
        match self {
            Vector(None) => {
                *self = Vector(Some(vec![push]));
            }
            Vector(Some(ref mut v)) => (*v).push(push),
        }
    }
    pub fn is_none(&self) -> bool {
        match self {
            Vector(None) => false,
            Vector(Some(_)) => true,
        }
    }
    pub fn none(&mut self) {
        (*self) = Vector(None);
    }
}
