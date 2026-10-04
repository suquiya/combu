use crate::args::Arg;

/// Struct for parsing command args
/// 引数をパースするための構造体、フラグパターンを格納している
pub struct Parser {
	/// `flag_pattern` is the character used to denote a flag in the command args
	pub flag_pattern: char,
	/// Long-flag prefix
	pub long_flag_prefix: String,
	/// equal symbol
	pub equal_symbol: char,
}

impl Default for Parser {
	fn default() -> Self {
		Self {
			flag_pattern: '-',
			long_flag_prefix: String::from("--"),
			equal_symbol: '=',
		}
	}
}

impl From<char> for Parser {
	fn from(c: char) -> Self {
		Self {
			flag_pattern: c,
			long_flag_prefix: c.to_string().repeat(2),
			equal_symbol: '=',
		}
	}
}

impl Parser {
	/// Creates a new `Parser` with the specified flag pattern, long flag prefix, and equal symbol.
	pub fn new_with_all_fields(
		flag_pattern: char,
		long_flag_prefix: String,
		equal_symbol: char,
	) -> Self {
		Self {
			flag_pattern,
			long_flag_prefix,
			equal_symbol,
		}
	}

	/// Creates a new `Parser` with the specified flag pattern.
	pub fn new(flag_pattern: char) -> Self {
		Self::from(flag_pattern)
	}

	/// Parse exe path and rest of args from the given iterator. Returns a tuple of exe path and a vector of middle-parsed arguments.
	pub fn parse_to_exe_path_and_middle_args<T: Iterator<Item = String>>(
		&self,
		iter: &mut T,
	) -> Option<(String, Vec<Arg>)> {
		let exe_path = iter.next()?;
		let args = self.args_middle_parse(iter);
		Some((exe_path, args))
	}

	/// Parses a sequence of arguments using the `arg_middle_parse` method.
	pub fn args_middle_parse<T: Iterator<Item = String>>(&self, iter: T) -> Vec<Arg> {
		iter.map(|arg| self.arg_middle_parse(arg)).collect()
	}

	/// Parses a single argument string into an `Arg` enum variant.
	pub fn arg_middle_parse(&self, arg: String) -> Arg {
		if let Some(stripped) = arg.strip_prefix(&self.long_flag_prefix) {
			stripped.split_once(self.equal_symbol).map_or_else(
				|| Arg::LongOnly(stripped.to_owned()),
				|(key, value)| Arg::HasValueLong(key.to_string(), value.to_string()),
			)
		} else if let Some(stripped) = arg.strip_prefix(&self.flag_pattern.to_string()) {
			stripped.split_once(self.equal_symbol).map_or_else(
				|| Arg::ShortOnly(stripped.to_owned()),
				|(key, value)| Arg::HasValueShort(key.to_string(), value.to_string()),
			)
		} else {
			Arg::Normal(arg)
		}
	}

	/// Restores a parsed `Arg` enum variant back to its original string form.
	pub fn arg_middle_parse_restore(&self, arg: Arg) -> String {
		match arg {
			Arg::Normal(arg) => arg,
			Arg::LongOnly(arg) => {
				let mut s = self.long_flag_prefix.clone();
				s.push_str(&arg);
				s
			}
			Arg::HasValueLong(key, value) => {
				let mut s = self.long_flag_prefix.clone();
				s.push_str(&key);
				s.push(self.equal_symbol);
				s.push_str(&value);
				s
			}
			Arg::ShortOnly(arg) => {
				let mut s = self.flag_pattern.to_string();
				s.push_str(&arg);
				s
			}
			Arg::HasValueShort(key, value) => {
				let mut s = self.flag_pattern.to_string();
				s.push_str(&key);
				s.push(self.equal_symbol);
				s.push_str(&value);
				s
			}
		}
	}
}
