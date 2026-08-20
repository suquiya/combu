/// Struct for option setting's information
/// オプション情報格納のための構造体です。
#[derive(Debug)]
pub struct Opt<Value = OptValue, ValueParseError = ParseValueError>
where
	ValueParseError: std::error::Error,
	Value: ParsableOptValue<ValueParseError>,
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
	pub default_with_type: Value,

	_marker_fv: std::marker::PhantomData<ValueParseError>,
}

pub trait ParsableOpt<OptValue, ParseError: std::error::Error> {
	/// Returns option's name.
	fn name(&self) -> &str;
	/// Returns option's description.
	fn description(&self) -> &str;
	/// Returns option's short alias.
	fn short_alias(&self) -> &[char];
	/// Returns option's long alias.
	fn long_alias(&self) -> &[String];
	/// Returns option's default value with type information.
	fn default_with_type(&self) -> &OptValue;

	/// Parses the option value from a string.
	fn parse(&self, value: &str) -> Result<OptValue, ()>;

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
	fn has_possible_short_option_like_value(&self) -> bool;

	/// Returns true if this option has a possible long option-like value.
	fn has_possible_long_option_like_value(&self) -> bool;
}

impl<OptValue: ParsableOptValue<ParseError>, ParseError: std::error::Error>
	ParsableOpt<OptValue, ParseError> for Opt<OptValue, ParseError>
{
	fn name(&self) -> &str {
		&self.name
	}

	fn description(&self) -> &str {
		&self.description
	}

	fn parse(&self, value: &str) -> Result<OptValue, ()> {
		self.default_with_type.parse(value)
	}

	fn parse_with_error(&self, value: &str) -> Result<OptValue, ParseError> {
		self.default_with_type.parse_with_error(value)
	}

	fn value_type_name(&self) -> &str {
		self.default_with_type.type_name()
	}

	fn default_value(&self) -> &OptValue {
		&self.default_with_type
	}

	fn is_bool_option(&self) -> bool {
		self.default_with_type.is_bool_type()
	}

	fn get_true_value(&self) -> &OptValue {
		&self.default_with_type
	}

	fn has_possible_short_option_like_value(&self) -> bool {
		self
			.default_with_type
			.has_possible_short_option_like_value()
	}

	fn has_possible_long_option_like_value(&self) -> bool {
		self.default_with_type.has_possible_long_option_like_value()
	}

	fn short_alias(&self) -> &[char] {
		&self.short_alias
	}

	fn long_alias(&self) -> &[String] {
		&self.long_alias
	}

	fn default_with_type(&self) -> &OptValue {
		&self.default_with_type
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
pub enum ParseValueError {
	BoolError(std::str::ParseBoolError),
	IntError(std::num::ParseIntError),
	FloatError(std::num::ParseFloatError),
	StringError(std::string::ParseError),
}

impl ParseValueError {
	fn from_bool_error(e: std::str::ParseBoolError) -> Self {
		Self::BoolError(e)
	}

	fn from_int_error(e: std::num::ParseIntError) -> Self {
		Self::IntError(e)
	}

	fn from_float_error(e: std::num::ParseFloatError) -> Self {
		Self::FloatError(e)
	}

	fn from_string_error(e: std::string::ParseError) -> Self {
		Self::StringError(e)
	}
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

impl ParsableOptValue<ParseValueError> for OptValue {
	fn parse(&self, value: &str) -> Result<Self, ()> {
		match self {
			OptValue::Bool(_) => value.parse::<bool>().map(Self::Bool).map_err(|_| ()),
			OptValue::Int(_) => value.parse::<i64>().map(Self::Int).map_err(|_| ()),
			OptValue::Float(_) => value.parse::<f64>().map(Self::Float).map_err(|_| ()),
			OptValue::String(_) => value.parse::<String>().map(Self::String).map_err(|_| ()),
		}
	}

	fn parse_with_error(&self, value: &str) -> Result<Self, ParseValueError> {
		match self {
			OptValue::Bool(_) => value
				.parse::<bool>()
				.map(Self::Bool)
				.map_err(|e| ParseValueError::BoolError(e)),
			OptValue::Int(_) => value
				.parse::<i64>()
				.map(Self::Int)
				.map_err(|e| ParseValueError::IntError(e)),
			OptValue::Float(_) => value
				.parse::<f64>()
				.map(Self::Float)
				.map_err(|e| ParseValueError::FloatError(e)),
			OptValue::String(_) => value
				.parse::<String>()
				.map(Self::String)
				.map_err(|e| ParseValueError::StringError(e)),
		}
	}

	fn is_bool_type(&self) -> bool {
		matches!(self, Self::Bool(_))
	}

	fn get_true_value(&self) -> Self {
		Self::Bool(true)
	}

	fn has_possible_short_option_like_value(&self) -> bool {
		!matches!(self, Self::Bool(_))
	}

	fn has_possible_long_option_like_value(&self) -> bool {
		matches!(self, Self::String(_))
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

/// Trait for parsing option values from strings.
///
/// This trait is implemented for [`DefaultOptValue`] by default.
/// If you want to implement your own option value type, you should implement this trait.
pub trait ParsableOptValue<Error: std::error::Error>:
	std::marker::Sized + std::default::Default
{
	/// Parses a string value into this option value.
	fn parse(&self, value: &str) -> Result<Self, ()>;

	/// Parses a string value into this option value. This method should return an error message if the value cannot be parsed.
	fn parse_with_error(&self, value: &str) -> Result<Self, Error>;

	/// Returns true if this option value should be treated as a boolean.
	fn is_bool_type(&self) -> bool;

	/// Returns the boolean true value. This is used when the boolean option is set and the value is not specified.
	fn get_true_value(&self) -> Self;

	/// Returns the type name of this option value.
	fn type_name(&self) -> &str;

	/// Returns true if this option value has a possible short option-like value.
	fn has_possible_short_option_like_value(&self) -> bool;

	/// Returns true if this option value has a possible long option-like value.
	fn has_possible_long_option_like_value(&self) -> bool;
}
