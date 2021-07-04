/// struct Vector is a new type pattern of Option<Vec<T>>
#[derive(Clone, Debug, PartialOrd, PartialEq)]
pub struct Vector<T>(pub Option<Vec<T>>);

#[macro_export]
/// Creates new Vector.
macro_rules! v {
	() => {
		Vector(None)
	};
	($elem:expr; $n:expr)=>{
		Vector(Some(vec![$elem,$n]))
	};
	($($x:expr),+ $(,)?)=>{
		Vector(Some(vec![$($x),+]))
	}
}

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

impl<T> From<std::collections::VecDeque<T>> for Vector<T> {
	fn from(vec_deque: std::collections::VecDeque<T>) -> Self {
		let vec: Vec<T> = vec_deque.into();
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
	/// Creates a new Vector.
	pub fn new() -> Self {
		Vector::default()
	}

	/// Creates a new Vetocr with inner.
	pub fn with_inner(inner_vec: Vec<T>) -> Self {
		Vector(Some(inner_vec))
	}

	/// Initialize this. After initialization, this inner Option<Vec<T>> becomes value.
	pub fn init(&mut self, value: Option<Vec<T>>) {
		*self = Vector(value);
	}

	/// Creates a new Vector with first element.
	pub fn with_first_elem(elem: T) -> Self {
		Vector(Some(vec![elem]))
	}

	/// Pushes push:T to this.
	pub fn push(&mut self, push: T) {
		match self {
			Vector(None) => {
				*self = Vector(Some(vec![push]));
			}
			Vector(Some(ref mut v)) => (*v).push(push),
		}
	}

	/// Returns this Vector.
	pub fn len(&self) -> usize {
		match self {
			Vector(None) => 0,
			Vector(Some(inner)) => inner.len(),
		}
	}

	/// Append other(Vec<T>) to this inner.
	pub fn append_vec(&mut self, mut other: Vec<T>) {
		match self {
			Vector(None) => *self = Vector(Some(other)),
			Vector(Some(ref mut vec)) => (*vec).append(&mut other),
		}
	}

	/// Prepend other(Vec<T>) to this inner.
	pub fn prepend_vec(&mut self, mut other: Vec<T>) {
		match self {
			Vector(None) => *self = Vector(Some(other)),
			Vector(Some(ref mut vec)) => {
				other.append(vec);
				*vec = other;
			}
		}
	}

	/// Insert T into this inner.
	pub fn insert(&mut self, index: usize, insert: T) {
		match self {
			Vector(None) => *self = Vector(Some(vec![insert])),
			Vector(Some(ref mut vec)) => (*vec).insert(index, insert),
		}
	}

	/// Appends other to this.
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

	/// Returns true if this inner is None.
	pub fn is_none(&self) -> bool {
		match self {
			Vector(None) => true,
			_ => false,
		}
	}

	/// Returns true if this has inner vec(as Some(Vec<T>)).
	pub fn has_inner_vec(&self) -> bool {
		match self {
			Vector(None) => false,
			_ => true,
		}
	}

	/// Sets None as inner.
	pub fn set_none(&mut self) {
		(*self) = Vector(None);
	}

	/// Clears inner vec. If inner is None, does nothing.
	pub fn clear(&mut self) {
		match self {
			Vector(None) => {}
			Vector(Some(ref mut inner)) => (*inner).clear(),
		}
	}

	/// Gets inner.
	pub fn inner(&self) -> Option<&Vec<T>> {
		match &self {
			Vector(None) => None,
			Vector(Some(inner)) => Some(inner),
		}
	}

	/// Gets inner as mutable form.
	pub fn inner_mut(&mut self) -> &mut Option<Vec<T>> {
		let Vector(inner) = self;
		inner
	}

	/// Takes inner and returns Vector that stores the inner.
	pub fn take(&mut self) -> Vector<T> {
		let Vector(inner) = self;
		Vector(inner.take())
	}

	/// Gets inner reference.
	pub fn inner_ref(self) -> Option<Vec<T>> {
		match self {
			Vector(None) => None,
			Vector(inner) => inner,
		}
	}

	/// Returns a reference to an element has index(usize).
	pub fn get(&self, index: usize) -> Option<&T> {
		match self {
			Vector(None) => None,
			Vector(Some(inner)) => inner.get(index),
		}
	}

	/// Removes last element of inner vec and returns it. If inner vec is empty or None, returns None.
	pub fn pop(&mut self) -> Option<T> {
		match self {
			Vector(None) => None,
			Vector(Some(ref mut inner)) => inner.pop(),
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
	/// Returns the sum of inner Vectors' len.
	pub fn sum_of_length(&self) -> usize {
		match self {
			Vector(None) => 0,
			Vector(Some(list)) => list.iter().fold(0, |acc, item| acc + item.len()),
		}
	}

	/// Returns the len fo last of inner Vectors.
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

	/// Enum for result of search long flag.
	#[derive(Debug, Clone)]
	pub enum LongFound<T> {
		/// Shows hit inner long flag's name.
		Name(T),
		/// Shows hit inner long flag's long alias.
		Long(T),
		/// Shows hit no long flag.
		None,
	}

	/// A trait for the ability to search flags for three forms.
	pub trait FlagSearch {
		/// Finds long frag.
		fn find_long_flag(&self, name_or_alias: &str) -> LongFound<&Flag>;
		/// Finds short flag.
		fn find_short_flag(&self, short_alias: &char) -> Option<&Flag>;
		/// Finds flag that has specidied name.
		fn find(&self, name: &str) -> Option<&Flag>;
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

#[cfg(test)]
mod tests {
	use super::Vector;

	#[test]
	fn prepend_test() {
		let mut main: Vector<String> = vec!["a".to_string(), "b".to_string(), "c".to_string()].into();
		let other = vec!["test".to_string(), "test2".to_string()];
		main.prepend_vec(other);
		assert_eq!(
			Vector::from(vec![
				"test".to_string(),
				"test2".to_string(),
				"a".to_string(),
				"b".to_string(),
				"c".to_string()
			]),
			main
		);
	}
}
