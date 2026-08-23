/// Represents a middle-parsed command argument.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Arg {
	/// A normal argument without any flag prefix.
	Normal(String),
	/// A long-only argument prefixed with long flag's prefix.
	LongOnly(String),
	/// A long argument with a value, prefixed with long flag's prefix.
	HasValueLong(String, String),
	/// A short-only argument prefixed with short flag's prefix.
	ShortOnly(String),
	/// A short argument with a value, prefixed with short flag's prefix.
	HasValueShort(String, String),
}
