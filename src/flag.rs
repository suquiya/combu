use crate::Vector;

/// Struct for Flag setting's information
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
#[derive(PartialOrd, PartialEq, Clone, Debug)]
pub enum FlagType {
	/// Variant shows bool
	Bool,
	/// Variant shows string
	String,
	/// Variant shows int
	Int,
	/// Variant shows float
	Float,
	//Unknown,
}

impl FlagType {
	/// Get this FlagType variant's name as str
	pub fn name<'a>(&self) -> &'a str {
		match self {
			FlagType::Bool => "Bool",
			FlagType::String => "String",
			FlagType::Int => "Int",
			FlagType::Float => "Float",
		}
	}
	/// Get this FlagType variant's default value
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

	/// Returns true if &self equals FlagType::Bool
	pub fn is_bool(&self) -> bool {
		*self == FlagType::Bool
	}
}

impl Default for FlagType {
	fn default() -> Self {
		FlagType::String
	}
}

/// Enum for storage FlagValue
#[derive(PartialOrd, PartialEq, Clone, Debug)]
pub enum FlagValue {
	/// Variant shows bool flag value
	Bool(bool),
	/// Variant shows string flag value
	String(String),
	/// Variant shows int flag value
	Int(isize),
	/// Variant for float flag value
	Float(f64),
	/// Variant for invalid flag value
	Invalid(String),
	/// Variant for no flag value
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
		Some(flag_type) == self.get_type()
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
		self == &FlagValue::Bool(true)
	}

	/// Returns inner bool value. If self is not FlagValue::Bool(val), panic will occur.
	pub fn get_bool(&self) -> bool {
		match self {
			FlagValue::Bool(val) => *val,
			_ => panic!("get_bool mut use against FlagValue::Bool"),
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

impl Flag {
	/// Creates a new instance of Flag
	pub fn new<T: Into<String>>(name: T, flag_type: FlagType, description: T) -> Flag {
		let default_value: FlagValue = match flag_type {
			FlagType::Bool => FlagValue::Bool(bool::default()),
			FlagType::String => FlagValue::String(String::default()),
			FlagType::Int => FlagValue::Int(isize::default()),
			FlagType::Float => FlagValue::Float(f64::default()),
		};
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
			default_value: FlagValue::String(String::default()),
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
		Flag::with_name_and_type(name, FlagType::Bool)
	}

	/// Creates a new instance of string Flag
	pub fn new_string<T: Into<String>>(name: T) -> Self {
		Flag::with_name_and_type(name, FlagType::String)
	}

	/// Creates a new instance of int Flag
	pub fn new_int<T: Into<String>>(name: T) -> Self {
		Flag::with_name_and_type(name, FlagType::Int)
	}

	/// Creates a new instance of float Flag
	pub fn new_float<T: Into<String>>(name: T) -> Self {
		Flag::with_name_and_type(name, FlagType::Float)
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

	/// Add help for this flag to append_to. name_and_alias_min_width means min width of name and alias' field.  
	/// Flagに対するヘルプをappend_toに追加する。nameとalias表示部分のずれをある程度吸収できるようにその部分の最小幅をname_and_alias_min_widthで指定する
	pub fn help(&self, append_to: String, name_and_alias_min_width: usize) -> String {
		let mut help = append_to;
		let first_help_width = help.len();

		if let Vector(Some(short_alias)) = &self.short_alias {
			help = short_alias
				.iter()
				.fold(help, |help, s| format!("{}-{},", help, s));
		} else {
			help += "   ";
		}
		help = help + " --" + &self.name;
		if let Vector(Some(long_alias)) = &self.long_alias {
			help = long_alias.iter().fold(help, |help, l| {
				//ロングフラグ出力
				format!("{}, --{}", help, l)
			})
		};
		let name_and_alias_width = help.len() - first_help_width;

		if name_and_alias_width < name_and_alias_min_width {
			help += &" ".repeat(name_and_alias_min_width - name_and_alias_width);
		}

		help + "\t" + &self.description + "\n"
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
		crate::help_flag!()
	}
	/// Creates preset help flag with arg descripton.
	pub fn help_flag_with_description<T: Into<String>>(description: T) -> Flag {
		crate::help_flag!(->description.into())
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
