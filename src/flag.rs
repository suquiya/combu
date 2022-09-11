use crate::{flag_type, flag_value, Vector};

/// Struct for Flag setting's information
/// フラグ（オプション）情報格納のための構造体です。
#[derive(Clone, Debug, PartialEq)]
pub struct Flag {
	/// This flag's name
	pub name: String,
	/// This flag's description
	pub description: String,
	/// Vector of this flag's short alias
	pub short_alias: Vector<char>,
	/// Vector of this flag's long alias
	pub long_alias: Vector<String>,
	/// This flag's default value
	pub default_value: FlagValue,
	/// This flag's flag_type
	pub flag_type: FlagType,
}

/// Enum shows FlagType
/// フラグの型を示すEnumです。
#[derive(PartialOrd, PartialEq, Clone, Debug)]
pub enum FlagType {
	/// Variant shows bool
	/// Bool型用Variant
	Bool,
	/// Variant shows string
	/// String型用Variant
	String,
	/// Variant shows int
	/// Int型用Variant
	Int,
	/// Variant shows float
	/// float型用Variant
	Float,
}

impl FlagType {
	/// Get this FlagType variant's name as str
	/// &str型の形でFlagType名を取得します。
	pub fn name<'a>(&self) -> &'a str {
		match self {
			FlagType::Bool => "Bool",
			FlagType::String => "String",
			FlagType::Int => "Int",
			FlagType::Float => "Float",
		}
	}
	/// Get this FlagType variant's default value
	/// FlagTypeのデフォルト値を取得
	pub fn default_flag_value(&self) -> FlagValue {
		match self {
			FlagType::Bool => FlagValue::Bool(bool::default()),
			FlagType::String => FlagValue::String(String::default()),
			FlagType::Int => FlagValue::Int(isize::default()),
			FlagType::Float => FlagValue::Float(f64::default()),
		}
	}

	/// If val's type is &self, returns true
	/// valが&selfが示すタイプと一致するか判定する
	pub fn is_type_of(&self, val: &FlagValue) -> bool {
		Some(self) == val.get_type()
	}

	/// Get FlagValue from val as &self type.
	/// valを&self型とした場合のFlagValueを取得する
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

	/// Returns &self type value in case of no specified value.
	/// フラグとして指定された値がなかった場合、&self型のフラグはどの値として扱うかを取得する
	pub fn get_value_if_no_value(&self) -> FlagValue {
		match &self {
			FlagType::Bool => FlagValue::Bool(true),
			FlagType::String => FlagValue::String(String::new()),
			_ => FlagValue::None,
		}
	}

	/// Returns true if &self equals FlagType::String
	pub fn is_string(&self) -> bool {
		*self == FlagType::String
	}

	/// Returns true if &self equals FlagType::Int
	pub fn is_int(&self) -> bool {
		*self == FlagType::Int
	}

	/// Returns true if &self equals FlagType::Float
	pub fn is_float(&self) -> bool {
		*self == FlagType::Float
	}

	/// Returns true if &self equals FlagType::Bool
	pub fn is_bool(&self) -> bool {
		*self == FlagType::Bool
	}
}

impl Default for FlagType {
	fn default() -> Self {
		FlagType::Bool
	}
}

/// Enum for storage FlagValue
/// フラグの値を保持するためのEnum
#[derive(PartialOrd, PartialEq, Clone, Debug)]
pub enum FlagValue {
	/// Variant shows bool flag value
	/// Bool値保存用
	Bool(bool),
	/// Variant shows string flag value
	/// String値保存用
	String(String),
	/// Variant shows int flag value
	/// Int値保存用
	Int(isize),
	/// Variant for float flag value
	/// Float値保存用
	Float(f64),
	/// Variant for invalid flag value
	/// 間違った値が指定されていた時にString値で指定されたフラグ値の保存用
	Invalid(String),
	/// Variant for no flag value
	/// None表現用
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

impl From<&str> for FlagValue {
	fn from(val: &str) -> Self {
		FlagValue::String(val.into())
	}
}

impl FlagValue {
	/// Get &self's corresponding type of FlagType. Returns None if  &self is a invalid flag value.
	/// FlagValueに対応するFlagTypeを取得する
	pub fn get_type(&self) -> Option<&FlagType> {
		match self {
			FlagValue::Bool(_) => Some(&FlagType::Bool),
			FlagValue::String(_) => Some(&FlagType::String),
			FlagValue::Int(_) => Some(&FlagType::Int),
			FlagValue::Float(_) => Some(&FlagType::Float),
			_ => None,
		}
	}

	/// Returns true if &self's FlagType is flag_type.
	pub fn is_type(&self, flag_type: &FlagType) -> bool {
		match self {
			FlagValue::Bool(_) => &FlagType::Bool == flag_type,
			FlagValue::String(_) => &FlagType::String == flag_type,
			FlagValue::Int(_) => &FlagType::Int == flag_type,
			FlagValue::Float(_) => &FlagType::Float == flag_type,
			_ => false,
		}
	}

	/// Gets bool FlagValue from string
	pub fn get_bool_value_from_string(val: String) -> FlagValue {
		match val.as_str() {
			"true" => FlagValue::Bool(true),
			"false" => FlagValue::Bool(false),
			_ => FlagValue::Invalid(val),
		}
	}

	/// Gets string from self
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

	/// Returns true if self is FlagValue::Bool(true)
	pub fn is_bool_true(&self) -> bool {
		matches!(self, FlagValue::Bool(true))
	}

	/// Returns inner bool value. If self is not FlagValue::Bool(val), returns None.
	pub fn get_bool(&self) -> Option<bool> {
		match self {
			FlagValue::Bool(val) => Some(*val),
			_ => None,
		}
	}

	/// Returns inner bool value. If self is not FlagValue::Bool(val), panic will occur.
	pub fn get_bool_unwrap(&self) -> bool {
		match self {
			FlagValue::Bool(val) => *val,
			_ => panic!("get_bool must use against FlagValue::Bool"),
		}
	}

	/// Returns inner isize value. If self is not FlagValue::Bool(val), returns None.
	pub fn get_int(&self) -> Option<isize> {
		match self {
			FlagValue::Int(val) => Some(*val),
			_ => None,
		}
	}

	/// Returns inner isize value. If self is not FlagValue::Bool(val), panic will occur.
	pub fn get_int_unwrap(&self) -> isize {
		match self {
			FlagValue::Int(val) => *val,
			_ => panic!("get_int must use against FlagValue::Int"),
		}
	}

	/// Returns inner float value. If self is not FlagValue::Float(val), returns None.
	pub fn get_float(&self) -> Option<f64> {
		match self {
			FlagValue::Float(val) => Some(*val),
			_ => None,
		}
	}
}

impl Default for Flag {
	fn default() -> Flag {
		Flag {
			name: String::default(),
			description: String::default(),
			short_alias: Vector::default(),
			long_alias: Vector::default(),
			flag_type: FlagType::default(),
			default_value: FlagValue::default(),
		}
	}
}

macro_rules! new_typed_flag {
	($name:expr, $type:ident) => {
		Flag::with_all_field(
			$name.into(),
			String::default(),
			Vector::default(),
			Vector::default(),
			flag_type!($type),
			flag_value!($type),
		)
	};
}

impl Flag {
	/// Creates a new instance of Flag
	pub fn new<T: Into<String>>(name: T, flag_type: FlagType, description: T) -> Flag {
		let default_value: FlagValue = flag_type.default_flag_value();
		Flag {
			name: name.into(),
			description: description.into(),
			long_alias: Vector::default(),
			short_alias: Vector::default(),
			flag_type,
			default_value,
		}
	}

	/// Builds a new instance of Flags with all options.
	pub fn with_all_field(
		name: String,
		description: String,
		short_alias: Vector<char>,
		long_alias: Vector<String>,
		flag_type: FlagType,
		default_value: FlagValue,
	) -> Flag {
		Flag {
			name,
			description,
			short_alias,
			long_alias,
			flag_type,
			default_value,
		}
	}

	/// Creates a new instance of Flag with name
	pub fn with_name<T: Into<String>>(name: T) -> Self {
		Flag {
			name: name.into(),
			description: String::default(),
			short_alias: Vector::default(),
			long_alias: Vector::default(),
			flag_type: FlagType::default(),
			default_value: FlagValue::Bool(bool::default()),
		}
	}

	/// Creates a new instance of Flag with name and type
	pub fn with_name_and_type<T: Into<String>>(name: T, flag_type: FlagType) -> Self {
		let default_value = flag_type.default_flag_value();
		Flag {
			name: name.into(),
			description: String::default(),
			short_alias: Vector::default(),
			long_alias: Vector::default(),
			flag_type,
			default_value,
		}
	}

	/// Creates a new instance of bool Flag
	pub fn new_bool<T: Into<String>>(name: T) -> Self {
		new_typed_flag!(name, bool)
	}

	/// Creates a new instance of string Flag
	pub fn new_string<T: Into<String>>(name: T) -> Self {
		new_typed_flag!(name, String)
	}

	/// Creates a new instance of int Flag
	pub fn new_int<T: Into<String>>(name: T) -> Self {
		new_typed_flag!(name, Int)
	}

	/// Creates a new instance of float Flag
	pub fn new_float<T: Into<String>>(name: T) -> Self {
		new_typed_flag!(name, Float)
	}

	/// Add an short alias to this Flag
	pub fn short_alias<T: Into<char>>(mut self, a: T) -> Self {
		self.short_alias.push(a.into());
		self
	}

	/// Add an alias to this Flag
	pub fn alias<T: Into<String>>(mut self, a: T) -> Self {
		self.long_alias.push(a.into());
		self
	}

	/// Set this flag's default value
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

	/// Set this flag's description
	pub fn description<T: Into<String>>(mut self, description: T) -> Self {
		self.description = description.into();
		self
	}

	/// Returns true if &self's name equals name of arg.
	pub fn is(&self, name: &str) -> bool {
		self.name == name
	}

	/// Returns true is alias equals one of short alias
	pub fn is_short(&self, alias: &char) -> bool {
		match &self.short_alias {
			Vector(None) => false,
			Vector(Some(short_alias)) => short_alias.iter().any(|s| s == alias),
		}
	}

	/// Returns positions alias in aliases matches one of short alias.
	pub fn any_short(&self, aliases: std::str::Chars<'_>) -> Vector<usize> {
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

	/// Returns true is alias equals one of long alias
	pub fn is_long(&self, alias: &str) -> bool {
		match &self.long_alias {
			Vector(None) => false,
			Vector(Some(long_alias)) => long_alias.iter().any(|s| s == alias),
		}
	}

	/// Get this Flag's name's clone
	pub fn get_name_clone(&self) -> String {
		self.name.clone()
	}

	/// Derives this Flag's value from arg
	/// Alias of self.flag_type.get_value_from_string(arg)
	#[inline]
	pub fn derive_flag_value_from_string(&self, arg: String) -> FlagValue {
		self.flag_type.get_value_from_string(arg)
	}

	/// Derives this Flag's value in case of no value
	/// Alias of self.flag_type.get_value_if_no_value()
	#[inline]
	pub fn derive_flag_value_if_no_value(&self) -> FlagValue {
		self.flag_type.get_value_if_no_value()
	}
}

impl From<String> for Flag {
	fn from(name: String) -> Self {
		Flag {
			name,
			description: String::default(),
			short_alias: Vector::default(),
			long_alias: Vector::default(),
			default_value: FlagValue::default(),
			flag_type: FlagType::default(),
		}
	}
}

impl From<&str> for Flag {
	fn from(name: &str) -> Self {
		Flag {
			name: name.into(),
			description: String::default(),
			short_alias: Vector::default(),
			long_alias: Vector::default(),
			default_value: FlagValue::default(),
			flag_type: FlagType::default(),
		}
	}
}

/// Flag's presets
pub mod presets {

	use super::Flag;

	/// Creates preset help flag
	pub fn help_flag() -> Flag {
		super::super::help_flag!()
	}
	/// Creates preset help flag with arg descripton.
	pub fn help_flag_with_description<T: Into<String>>(description: T) -> Flag {
		super::super::help_flag!(->description.into())
	}

	/// Creates preset version flag
	pub fn version_flag() -> Flag {
		crate::version_flag!()
	}
	/// Creates preset version flag with description specified.
	pub fn version_flag_with_description<T: Into<String>>(description: T) -> Flag {
		crate::version_flag!(->description.into())
	}

	/// Creates preset authors flag
	pub fn authors_flag() -> Flag {
		crate::authors_flag!()
	}

	/// Creates preset authors flag with description specified.
	pub fn authors_flag_with_description<T: Into<String>>(description: T) -> Flag {
		crate::authors_flag!(->description.into())
	}

	/// Creates preset license flag
	pub fn license_flag() -> Flag {
		crate::license_flag!()
	}

	/// Creates preset license flag with description specified.
	pub fn license_flag_with_description<T: Into<String>>(description: T) -> Flag {
		crate::license_flag!(description.into())
	}

	/// Creates yes flag
	pub fn yes_flag() -> Flag {
		crate::yes_flag!()
	}

	/// Creates yes flag flag with description specified.
	pub fn yes_flag_with_description<T: Into<String>>(description: T) -> Flag {
		crate::yes_flag!(->description.into())
	}

	/// Creates no flag
	pub fn no_flag() -> Flag {
		crate::no_flag!()
	}

	/// Creates no flag
	pub fn no_flag_with_description<T: Into<String>>(description: T) -> Flag {
		crate::no_flag!(->description.into())
	}
}
