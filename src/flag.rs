/// Struct for Flag setting's information
/// フラグ（オプション）情報格納のための構造体です。
#[derive(Debug)]
pub struct Flag<FlagValue: ParsableFlagValue = DefaultFlagValue> {
	/// This flag's name
	pub name: String,
	/// This flag's description
	pub description: String,
	/// Vector of this flag's short alias
	pub short_alias: Vec<char>,
	/// Vector of this flag's long alias
	pub long_alias: Vec<String>,
	/// This flag's default value
	pub default_value: FlagValue,
}

#[derive(Debug, Clone, PartialEq)]
pub enum DefaultFlagValue {
	Bool(bool),
	Int(i64),
	Float(f64),
	String(String),
}

impl Default for DefaultFlagValue {
	fn default() -> Self {
		Self::Bool(bool::default())
	}
}

impl ParsableFlagValue for DefaultFlagValue {
	type Error = ();

	fn parse(&self, value: &str) -> Result<Self, Self::Error> {
		match self {
			DefaultFlagValue::Bool(_) => value.parse::<bool>().map(Self::Bool).map_err(|_| ()),
			DefaultFlagValue::Int(_) => value.parse::<i64>().map(Self::Int).map_err(|_| ()),
			DefaultFlagValue::Float(_) => value.parse::<f64>().map(Self::Float).map_err(|_| ()),
			DefaultFlagValue::String(_) => value.parse::<String>().map(Self::String).map_err(|_| ()),
		}
	}

	fn is_bool(&self) -> bool {
		match self {
			Self::Bool(_) => true,
			_ => false,
		}
	}

	fn has_possible_short_flag_like_value(&self) -> bool {
		match self {
			Self::Bool(_) => false,
			Self::Int(_) => true,
			Self::Float(_) => true,
			Self::String(_) => true,
		}
	}

	fn has_possible_long_flag_like_value(&self) -> bool {
		match self {
			Self::Bool(_) => false,
			Self::Int(_) => true,
			Self::Float(_) => true,
			Self::String(_) => true,
		}
	}

	fn type_name(&self) -> &str {
		match self {
			Self::Bool(_) => "bool",
			Self::Int(_) => "int",
			Self::Float(_) => "float",
			Self::String(_) => "string",
		}
	}
}

pub trait ParsableFlagValue: std::marker::Sized + std::default::Default {
	type Error;

	fn parse(&self, value: &str) -> Result<Self, Self::Error>;

	fn is_bool(&self) -> bool;

	fn type_name(&self) -> &str;

	fn has_possible_short_flag_like_value(&self) -> bool;

	fn has_possible_long_flag_like_value(&self) -> bool;
}
