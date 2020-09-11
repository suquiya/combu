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

	pub fn with_inner(inner_vec: Vec<T>) -> Self {
		Vector(Some(inner_vec))
	}

	pub fn init(&mut self, value: Option<Vec<T>>) {
		*self = Vector(value);
	}

	pub fn with_first_elem(elem: T) -> Self {
		Vector(Some(vec![elem]))
	}

	pub fn push(&mut self, push: T) {
		match self {
			Vector(None) => {
				*self = Vector(Some(vec![push]));
			}
			Vector(Some(ref mut v)) => (*v).push(push),
		}
	}

	pub fn len(&self) -> usize {
		match self {
			Vector(None) => 0,
			Vector(Some(inner)) => inner.len(),
		}
	}

	pub fn append_vec(&mut self, mut other: Vec<T>) {
		match self {
			Vector(None) => {
				//let mut inner = vec![];
				//inner.append(other);
				// self = Vector(Some(inner));
				*self = Vector(Some(other))
			}
			Vector(Some(ref mut vec)) => (*vec).append(&mut other),
		}
	}

	pub fn insert(&mut self, index: usize, insert: T) {
		match self {
			Vector(None) => *self = Vector(Some(vec![insert])),
			Vector(Some(ref mut vec)) => (*vec).insert(index, insert),
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

	pub fn get(&self, index: usize) -> Option<&T> {
		match self {
			Vector(None) => None,
			Vector(Some(inner)) => inner.get(index),
		}
	}
}

impl<T> From<Vector<T>> for Vector<Vector<T>> {
	fn from(f: Vector<T>) -> Vector<Vector<T>> {
		Vector(Some(vec![f]))
	}
}

impl From<String> for Vector<String> {
	fn from(f: String) -> Vector<String> {
		Vector::with_first_elem(f)
	}
}

impl From<&str> for Vector<String> {
	fn from(str: &str) -> Self {
		Vector::with_first_elem(str.into())
	}
}

impl From<Option<String>> for Vector<String> {
	fn from(t: Option<String>) -> Self {
		match t {
			None => Vector(None),
			Some(t) => Vector::with_first_elem(t),
		}
	}
}

impl<T> Vector<Vector<T>> {
	pub fn sum_of_length(&self) -> usize {
		match self {
			Vector(None) => 0,
			Vector(Some(list)) => list.iter().fold(0, |acc, item| acc + item.len()),
		}
	}

	pub fn length_of_last(&self) -> usize {
		match self {
			Vector(None) => 0,
			Vector(Some(list)) => {
				if let Some(last) = list.last() {
					last.len()
				} else {
					0
				}
			}
		}
	}
}

/// Inner module of Vector. This module about Vector and Flag.
pub mod flag {
	use super::Vector;
	use crate::Flag;

	#[derive(Debug, Clone)]
	pub enum LongFound<T> {
		Name(T),
		Long(T),
		None,
	}

	pub trait FlagSearch {
		fn find_long_flag(&self, name_or_alias: &str) -> LongFound<&Flag>;
		fn find_short_flag(&self, name_or_alias: &char) -> Option<&Flag>;
		fn find(&self, name_or_alias: &str) -> Option<&Flag>;
	}

	impl FlagSearch for Vector<Flag> {
		fn find_long_flag(&self, name_or_alias: &str) -> LongFound<&Flag> {
			match &self {
				Vector(None) => LongFound::None,
				Vector(Some(flags)) => match flags.iter().find(|flag| flag.is(name_or_alias)) {
					None => match flags.iter().find(|flag| flag.is_long(name_or_alias)) {
						None => LongFound::None,
						Some(f) => LongFound::Long(f),
					},
					Some(f) => LongFound::Name(f),
				},
			}
		}

		fn find_short_flag(&self, short_alias: &char) -> Option<&Flag> {
			match &self {
				Vector(Some(flags)) => flags.iter().find(|flag| flag.is_short(short_alias)),
				Vector(None) => None,
			}
		}

		fn find(&self, flag_name: &str) -> Option<&Flag> {
			match &self {
				Vector(Some(flags)) => match flags.iter().find(|flag| flag.is(flag_name)) {
					Some(f) => Some(f),
					None => None,
				},
				Vector(None) => None,
			}
		}
	}

	impl<T: FlagSearch> FlagSearch for Vector<T> {
		fn find_long_flag(&self, name_or_alias: &str) -> LongFound<&Flag> {
			match &self {
				Vector(None) => LongFound::None,
				Vector(Some(flags_list)) => {
					let mut iter = flags_list.iter();
					return loop {
						let flags = iter.next_back();
						if let Some(flags) = flags {
							match flags.find_long_flag(name_or_alias) {
								LongFound::None => {}
								val => {
									break val;
								}
							}
						} else {
							break LongFound::None;
						}
					};
				}
			}
		}

		fn find_short_flag(&self, name_or_alias: &char) -> Option<&Flag> {
			match &self {
				Vector(None) => None,
				Vector(Some(flags_list)) => {
					let mut iter = flags_list.iter();
					return loop {
						let flags = iter.next_back();
						if let Some(flags) = flags {
							match flags.find_short_flag(name_or_alias) {
								None => {}
								val => {
									break val;
								}
							}
						} else {
							break None;
						}
					};
				}
			}
		}

		fn find(&self, name_or_alias: &str) -> Option<&Flag> {
			match &self {
				Vector(None) => None,
				Vector(Some(flags_list)) => {
					let mut iter = flags_list.iter();
					return loop {
						let flags = iter.next_back();
						if let Some(flags) = flags {
							match flags.find(name_or_alias) {
								None => {}
								val => {
									break val;
								}
							}
						} else {
							break None;
						}
					};
				}
			}
		}
	}
}
