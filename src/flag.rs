use crate::Vector;

#[derive(Clone, Debug)]
pub struct Flag {
	pub name: String,
	pub usage: String,
	pub short_alias: Vector<char>,
	pub long_alias: Vector<String>,
	pub default_value: FlagValue,
	pub flag_type: FlagType,
}

#[derive(PartialOrd, PartialEq, Clone, Debug)]
pub enum FlagType {
	Bool,
	String,
	Int,
	Float,
	//Unknown,
}

impl FlagType {
	pub fn name<'a>(&self) -> &'a str {
		match self {
			FlagType::Bool => "Bool",
			FlagType::String => "String",
			FlagType::Int => "Int",
			FlagType::Float => "Float",
			//_ => "Unknown",
		}
	}
	pub fn default_flag_value(&self) -> FlagValue {
		match self {
			FlagType::Bool => FlagValue::Bool(bool::default()),
			FlagType::String => FlagValue::String(String::default()),
			FlagType::Int => FlagValue::Int(isize::default()),
			FlagType::Float => FlagValue::Float(f64::default()),
		}
	}
	pub fn is_type_of(&self, val: &FlagValue) -> bool {
		Some(self) == val.get_type()
	}

	pub fn get_value_from_string(&self, val: String) -> FlagValue {
		match self {
			FlagType::Bool => FlagValue::get_bool_value_from_string(val),
			FlagType::String => FlagValue::String(String::from(val)),
			FlagType::Int => match val.parse::<isize>() {
				Ok(i) => FlagValue::Int(i),
				Err(_) => FlagValue::Invalid(val),
			},
			FlagType::Float => match val.parse::<f64>() {
				Ok(f) => FlagValue::Float(f),
				Err(_) => FlagValue::Invalid(val),
			},
		}
	}

	pub fn get_value_if_no_value(&self) -> FlagValue {
		match &self {
			FlagType::Bool => FlagValue::Bool(true),
			_ => FlagValue::None,
		}
	}

	pub fn is_string(&self) -> bool {
		*self == FlagType::String
	}

	pub fn is_bool(&self) -> bool {
		*self == FlagType::Bool
	}
}

impl Default for FlagType {
	fn default() -> Self {
		FlagType::String
	}
}

#[derive(PartialOrd, PartialEq, Clone, Debug)]
pub enum FlagValue {
	Bool(bool),
	String(String),
	Int(isize),
	Float(f64),
	Invalid(String),
	None,
}

impl Default for FlagValue {
	fn default() -> Self {
		FlagValue::None
	}
}

impl From<String> for FlagValue {
	fn from(val: String) -> Self {
		FlagValue::String(val)
	}
}

impl FlagValue {
	pub fn get_type(&self) -> Option<&FlagType> {
		match self {
			FlagValue::Bool(_) => Some(&FlagType::Bool),
			FlagValue::String(_) => Some(&FlagType::String),
			FlagValue::Int(_) => Some(&FlagType::Int),
			FlagValue::Float(_) => Some(&FlagType::Float),
			_ => None,
		}
	}
	pub fn is_type(&self, flag_type: &FlagType) -> bool {
		Some(flag_type) == self.get_type()
	}

	pub fn get_bool_value_from_string(val: String) -> FlagValue {
		match val.as_str() {
			"true" => FlagValue::Bool(true),
			"false" => FlagValue::Bool(false),
			_ => FlagValue::Invalid(val),
		}
	}

	pub fn get_string(self) -> String {
		match self {
			FlagValue::String(val) => val,
			FlagValue::Bool(b) => b.to_string(),
			FlagValue::Int(i) => i.to_string(),
			FlagValue::Float(f) => f.to_string(),
			FlagValue::Invalid(val) => val,
			FlagValue::None => String::default(),
		}
	}
}

impl Default for Flag {
	fn default() -> Flag {
		Flag {
			name: String::default(),
			usage: String::default(),
			short_alias: Vector::default(),
			long_alias: Vector::default(),
			flag_type: FlagType::default(),
			default_value: FlagValue::default(),
		}
	}
}

impl Flag {
	pub fn new<T: Into<String>>(name: T, usage: T, flag_type: FlagType) -> Flag {
		let default_value: FlagValue = match flag_type {
			FlagType::Bool => FlagValue::Bool(bool::default()),
			FlagType::String => FlagValue::String(String::default()),
			FlagType::Int => FlagValue::Int(isize::default()),
			FlagType::Float => FlagValue::Float(f64::default()),
		};
		Flag {
			name: name.into(),
			usage: usage.into(),
			long_alias: Vector::default(),
			short_alias: Vector::default(),
			flag_type,
			default_value,
		}
	}

	pub fn build_new(
		name: String,
		usage: String,
		short_alias: Vector<char>,
		long_alias: Vector<String>,
		flag_type: FlagType,
		default_value: FlagValue,
	) -> Flag {
		let calculated_default_value = if default_value.is_type(&flag_type) {
			default_value
		} else {
			let flag_type_str = flag_type.name();
			eprintln!("FlagType is {},but inputted default_value is not {}. default_value will be {}'s default.",flag_type_str,flag_type_str,flag_type_str);
			flag_type.default_flag_value()
		};
		Flag {
			name,
			usage,
			short_alias,
			long_alias,
			flag_type,
			default_value: calculated_default_value,
		}
	}

	pub fn with_name<T: Into<String>>(name: T) -> Self {
		Flag {
			name: name.into(),
			usage: String::default(),
			short_alias: Vector::default(),
			long_alias: Vector::default(),
			flag_type: FlagType::default(),
			default_value: FlagValue::String(String::default()),
		}
	}

	pub fn with_name_and_type<T: Into<String>>(name: T, flag_type: FlagType) -> Self {
		let default_value = flag_type.default_flag_value();
		Flag {
			name: name.into(),
			usage: String::default(),
			short_alias: Vector::default(),
			long_alias: Vector::default(),
			flag_type,
			default_value,
		}
	}

	pub fn new_bool<T: Into<String>>(name: T) -> Self {
		Flag::with_name_and_type(name, FlagType::Bool)
	}

	pub fn new_string<T: Into<String>>(name: T) -> Self {
		Flag::with_name_and_type(name, FlagType::String)
	}

	pub fn new_int<T: Into<String>>(name: T) -> Self {
		Flag::with_name_and_type(name, FlagType::Int)
	}

	pub fn new_float<T: Into<String>>(name: T) -> Self {
		Flag::with_name_and_type(name, FlagType::Float)
	}

	pub fn short_alias<T: Into<char>>(mut self, a: T) -> Self {
		self.short_alias.push(a.into());
		self
	}
	pub fn alias<T: Into<String>>(mut self, a: T) -> Self {
		self.long_alias.push(a.into());
		self
	}

	pub fn default_value(mut self, default_value: FlagValue) -> Self {
		if self.flag_type.is_type_of(&default_value) {
			self.default_value = default_value
		} else {
			println!(
				"not match flag_type: {}. default_value is not changed.",
				self.flag_type.name()
			);
		}
		self
	}

	pub fn usage<T: Into<String>>(mut self, usage: T) -> Self {
		self.usage = usage.into();
		self
	}

	pub fn is(&self, name: &str) -> bool {
		self.name == name
	}

	pub fn is_short(&self, alias: &char) -> bool {
		match &self.short_alias {
			Vector(None) => false,
			Vector(Some(short_alias)) => short_alias.iter().any(|s| s == alias),
		}
	}

	pub fn any_short(&self, aliases: std::str::Chars) -> Vector<usize> {
		match &self.short_alias {
			Vector(None) => Vector(None),
			_ => {
				let mut result = Vector::default();
				for (i, s) in aliases.enumerate() {
					if self.is_short(&s) {
						result.push(i);
					}
				}
				result
			}
		}
	}

	pub fn is_long(&self, alias: &str) -> bool {
		match &self.long_alias {
			Vector(None) => false,
			Vector(Some(long_alias)) => long_alias.iter().any(|s| s == alias),
		}
	}

	pub fn get_name_clone(&self) -> String {
		self.name.clone()
	}

	#[inline]
	pub fn derive_flag_value_from_string(&self, arg: String) -> FlagValue {
		self.flag_type.get_value_from_string(arg)
	}

	#[inline]
	pub fn derive_flag_value_if_no_value(&self) -> FlagValue {
		self.flag_type.get_value_if_no_value()
	}
}

impl From<String> for Flag {
	fn from(name: String) -> Self {
		Flag {
			name,
			usage: String::default(),
			short_alias: Vector::default(),
			long_alias: Vector::default(),
			default_value: FlagValue::default(),
			flag_type: FlagType::default(),
		}
	}
}
