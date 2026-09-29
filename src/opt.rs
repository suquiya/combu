use crate::from_impl_to_enum;

/// Struct for option setting's information
/// オプション情報格納のための構造体です。
#[derive(Debug)]
pub struct Opt<Value = OptValue, ValueParseError = ParseValueError>
where
	ValueParseError: std::error::Error,
	Value: ParsableOptValue,
{
	/// This option's name
	pub name: String,
	/// This option's description
	pub description: String,
	/// Vector of this option's short alias
	pub short_alias: Vec<char>,
	/// Vector of this option's long alias
	pub long_alias: Vec<String>,
	/// This option's default value (includes option's value type information)
	pub default_value: Value,
	/// This option's value type name
	pub value_type_name: String,
	/// Whether this option is a boolean flag (i.e. does not require a value)
	pub is_bool_flag: bool,
	/// Whether this option has a possible short option-like value (ex. -1.0)
	pub can_have_short_option_like_value: bool,
	/// Whether this option has a possible long option-like value (ex. --1.0)
	pub can_have_long_option_like_value: bool,
	/// parser function for this option.
	/// In other words, the guard function for parsing this option's value from a string.
	pub parser: fn(&Self, &str) -> Option<Value>,
	/// parser function for this option with error handling
	/// In other words, the guard function for parsing this option's value from a string with error handling.
	pub parser_with_error: fn(&Self, &str) -> Result<Value, ValueParseError>,
}

/// Trait for option settings (can be parsed from strings).
pub trait ParsableOpt<OptValue, ParseError: std::error::Error> {
	/// Returns option's name.
	fn name(&self) -> &str;
	/// Returns option's description.
	fn description(&self) -> &str;
	/// Returns option's short alias.
	fn short_alias(&self) -> &[char];
	/// Returns option's long alias.
	fn long_alias(&self) -> &[String];

	/// Parses the option value from a string.
	fn parse(&self, value: &str) -> Option<OptValue>;

	/// Parses the option value from a string with error handling.
	fn parse_with_error(&self, value: &str) -> Result<OptValue, ParseError>;

	/// Returns option's value type name.
	fn value_type_name(&self) -> &str;

	/// Returns option's default value.
	fn default_value(&self) -> &OptValue;

	/// Returns true if this option is a boolean option.
	fn is_bool_option(&self) -> bool;

	/// Returns the true value of this option.
	fn get_true_value(&self) -> &OptValue;

	/// Returns true if this option has a possible short option-like value.
	fn has_possibility_short_option_like_value(&self) -> bool;

	/// Returns true if this option has a possible long option-like value.
	fn has_possibility_long_option_like_value(&self) -> bool;
}

impl<OptValue: ParsableOptValue, ParseError: std::error::Error> ParsableOpt<OptValue, ParseError>
	for Opt<OptValue, ParseError>
{
	fn name(&self) -> &str {
		&self.name
	}

	fn description(&self) -> &str {
		&self.description
	}

	fn parse(&self, value: &str) -> Option<OptValue> {
		(self.parser)(self, value)
	}

	fn parse_with_error(&self, value: &str) -> Result<OptValue, ParseError> {
		(self.parser_with_error)(self, value)
	}

	fn value_type_name(&self) -> &str {
		&self.value_type_name
	}

	fn default_value(&self) -> &OptValue {
		&self.default_value
	}

	fn is_bool_option(&self) -> bool {
		self.is_bool_flag
	}

	fn get_true_value(&self) -> &OptValue {
		&self.default_value
	}

	fn has_possibility_short_option_like_value(&self) -> bool {
		self.can_have_short_option_like_value
	}

	fn has_possibility_long_option_like_value(&self) -> bool {
		self.can_have_long_option_like_value
	}

	fn short_alias(&self) -> &[char] {
		&self.short_alias
	}

	fn long_alias(&self) -> &[String] {
		&self.long_alias
	}
}

#[derive(Debug, Clone, PartialEq)]
/// Default option value type. Also represents own default in [`option`] struct.
///
/// If you want to use a custom option value type, you should implement option value type.
/// Then you should implement [`ParsableOptValue`] trait for it.
/// If you don't want to use a custom option value type, you can use this enum.
pub enum OptValue {
	/// Boolean option value.
	Bool(bool),
	/// Integer option value.
	Int(i64),
	/// Float option value.
	Float(f64),
	/// String option value.
	String(String),
}

impl Default for OptValue {
	fn default() -> Self {
		Self::Bool(bool::default())
	}
}

#[derive(Debug, Clone, PartialEq)]
/// Errors that can occur when parsing an option value from a string.
pub enum ParseValueError {
	/// Error parsing a boolean value.
	BoolError(std::str::ParseBoolError),
	/// Error parsing an integer value.
	IntError(std::num::ParseIntError),
	/// Error parsing a float value.
	FloatError(std::num::ParseFloatError),
	/// Error parsing a string value.
	StringError(std::string::ParseError),
}

impl std::fmt::Display for ParseValueError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::BoolError(e) => write!(f, "{}", e),
			Self::IntError(e) => write!(f, "{}", e),
			Self::FloatError(e) => write!(f, "{}", e),
			Self::StringError(e) => write!(f, "{}", e),
		}
	}
}

impl std::error::Error for ParseValueError {
	fn cause(&self) -> std::option::Option<&dyn std::error::Error> {
		match self {
			Self::BoolError(e) => Some(e),
			Self::IntError(e) => Some(e),
			Self::FloatError(e) => Some(e),
			Self::StringError(e) => Some(e),
		}
	}
}

impl ParsableOptValue for OptValue {
	fn get_true_value(&self) -> Self {
		Self::Bool(true)
	}
}

/// Option value trait for parsing option values from strings.
///
/// This trait is implemented for [`OptValue`] by default.
/// If you want to implement your own option value type, you should implement this trait.
pub trait ParsableOptValue: std::marker::Sized + std::default::Default {
	/// Returns the boolean true value. This is used when the boolean option is set and the value is not specified.
	fn get_true_value(&self) -> Self;
}

from_impl_to_enum!(i64, Int, OptValue);
from_impl_to_enum!(bool, Bool, OptValue);
from_impl_to_enum!(f64, Float, OptValue);
from_impl_to_enum!(String, String, OptValue);

/// Parses a value from a string using the default `std::str::FromStr` implementation.
pub fn default_parser<O, T: std::str::FromStr, V: From<T>>(_opt: &Opt, value: &str) -> Option<V> {
	value.parse::<T>().ok().map(|v| V::from(v))
}

from_impl_to_enum!(std::num::ParseIntError, IntError, ParseValueError);
from_impl_to_enum!(std::num::ParseFloatError, FloatError, ParseValueError);
from_impl_to_enum!(std::str::ParseBoolError, BoolError, ParseValueError);
from_impl_to_enum!(std::string::ParseError, StringError, ParseValueError);

/// Parses a value from a string with error using the default `std::str::FromStr` implementation.
pub fn default_parser_with_error<
	O,
	T: std::str::FromStr,
	V: From<T>,
	E: From<<T as std::str::FromStr>::Err>,
>(
	_opt: &Opt,
	value: &str,
) -> Result<V, E> {
	value
		.parse::<T>()
		.map_err(|e| E::from(e))
		.map(|v| V::from(v))
}
