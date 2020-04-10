#[derive(Clone, Debug)]
pub struct Vector<T>(Option<Vec<T>>);

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

impl<T> From<Option<Vec<T>>> for Vector<T> {
    fn from(val: Option<Vec<T>>) -> Self {
        Vector(val)
    }
}

impl<T: Clone> From<&Vec<T>> for Vector<T> {
    fn from(vec: &Vec<T>) -> Self {
        Vector(Some(vec.to_vec()))
    }
}

impl<T> Vector<T> {
    pub fn new() -> Self {
        Vector::default()
    }

    pub fn init(&mut self, value: Option<Vec<T>>) {
        *self = Vector(value);
    }

    pub fn push(&mut self, push: T) {
        match self {
            Vector(None) => {
                *self = Vector(Some(vec![push]));
            }
            Vector(Some(ref mut v)) => (*v).push(push),
        }
    }

    pub fn append(&mut self, other: &mut Vec<T>) {
        match self {
            Vector(None) => {
                let mut inner = vec![];
                inner.append(other);
            }
            Vector(Some(ref mut vec)) => {
                (*vec).append(other);
            }
        }
    }

    pub fn is_none(&self) -> bool {
        match self {
            Vector(None) => true,
            _ => false,
        }
    }

    pub fn none(&mut self) {
        (*self) = Vector(None);
    }
}
