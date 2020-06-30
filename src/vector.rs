#[derive(Clone, Debug, PartialOrd, PartialEq)]
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

	pub fn append_vec(&mut self, mut other: Vec<T>) {
		match self {
			Vector(None) => {
				//let mut inner = vec![];
				//inner.append(other);
				//*self = Vector(Some(inner));
				*self = Vector(Some(other))
			}
			Vector(Some(ref mut vec)) => (*vec).append(&mut other),
		}
	}

	pub fn append(&mut self, mut other: Vector<T>) {
		match self {
			Vector(None) => {
				*self = other;
			}
			Vector(Some(ref mut vec)) => {
				if let Vector(Some(ref mut o_vec)) = other {
					//
					(*vec).append(o_vec);
				}
			}
		}
	}

	pub fn is_none(&self) -> bool {
		match self {
			Vector(None) => true,
			_ => false,
		}
	}

	pub fn has_inner_vec(&self) -> bool {
		match self {
			Vector(None) => false,
			_ => true,
		}
	}

	pub fn set_none(&mut self) {
		(*self) = Vector(None);
	}

	pub fn clear(&mut self) {
		match self {
			Vector(None) => {}
			Vector(Some(ref mut inner)) => (*inner).clear(),
		}
	}

	pub fn inner(&self) -> Option<&Vec<T>> {
		match &self {
			Vector(None) => None,
			Vector(Some(inner)) => Some(inner),
		}
	}

	pub fn inner_mut(&mut self) -> &mut Option<Vec<T>> {
		let Vector(inner) = self;
		inner
	}

	pub fn take(&mut self) -> Vector<T> {
		let Vector(inner) = self;
		Vector(inner.take())
	}

	pub fn inner_ref(self) -> Option<Vec<T>> {
		match self {
			Vector(None) => None,
			Vector(inner) => inner,
		}
	}
}

pub mod flag {
	use super::Vector;
	use crate::Flag;

	#[derive(Debug, Clone)]
	pub enum Found<T> {
		Name(T),
		Short(T),
		Long(T),
		None,
	}

	impl Vector<Flag> {
		pub fn find_long_flag(&self, name_or_alias: &str) -> Found<&Flag> {
			match &self {
				Vector(None) => Found::None,
				Vector(Some(flags)) => match flags.iter().find(|flag| flag.is(name_or_alias)) {
					None => match flags.iter().find(|flag| flag.is_long(name_or_alias)) {
						None => Found::None,
						Some(f) => Found::Long(f),
					},
					Some(f) => Found::Name(f),
				},
			}
		}

		pub fn find_short_flag(&self, short_alias: &char) -> Found<&Flag> {
			match &self {
				Vector(Some(flags)) => match flags.iter().find(|flag| flag.is_short(short_alias)) {
					None => Found::None,
					Some(f) => Found::Short(f),
				},
				Vector(None) => Found::None,
			}
		}
	}
}
