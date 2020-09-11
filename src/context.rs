use crate::parser::{ErrorInfo, MiddleArg};
use crate::vector::flag::{FlagSearch, LongFound};
use crate::Vector;
use crate::{Flag, FlagValue};
use std::collections::VecDeque;

/// Storage information for command execution.
/// This storage raw args, non-flag args, flag values, and etc.
/// コマンドからrunを通ってactionにたどり着くまでの情報およびパース結果を格納する構造体。
/// フラグの値、たどってきたルートなどを保管。
#[derive(Debug)]
pub struct Context {
	/// raw args
	pub raw_args: Vec<String>,
	/// non-flag args
	pub args: VecDeque<String>,
	/// common_flags of its own and inherited
	pub common_flags: Vector<Vector<Flag>>,
	/// routes of from root to end
	pub routes: Vector<String>,
	/// local flags
	pub local_flags: Vector<Flag>,
	/// current_path (String, not PathBuf)
	pub current_path: String,
	/// storage of result of parsing common flags values
	pub common_flags_values: Vector<(String, FlagValue)>,
	/// storage of result of parsing local flags values
	pub local_flags_values: Vector<(String, FlagValue)>,
	/// On parsing, storage of parsing args.
	/// In edge(action), storage of error args
	pub parsing_args: Option<VecDeque<MiddleArg>>,
	/// error inforamation list of parsing
	pub error_info_list: Vector<ErrorInfo>,
}

impl Context {
	/// Creates a new instance of Context
	pub fn new(
		raw_args: Vec<String>,
		args: VecDeque<String>,
		common_flags: Vector<Flag>,
		local_flags: Vector<Flag>,
		routes: Vector<String>,
		current_path: String,
	) -> Context {
		Context {
			raw_args,
			args,
			common_flags: Vector(Some(vec![common_flags])),
			routes: routes.into(),
			local_flags,
			current_path,
			common_flags_values: Vector::default(),
			local_flags_values: Vector::default(),
			parsing_args: None,
			error_info_list: Vector::default(),
		}
	}

	/// Creates a new instance of Context with all options.
	pub fn build_new(
		raw_args: Vec<String>,
		args: VecDeque<String>,
		common_flags: Vector<Vector<Flag>>,
		local_flags: Vector<Flag>,
		current_path: String,
		routes: Vector<String>,
		common_flags_values: Vector<(String, FlagValue)>,
		local_flags_values: Vector<(String, FlagValue)>,
		parsing_args: Option<VecDeque<MiddleArg>>,
		error_info_list: Vector<ErrorInfo>,
	) -> Context {
		Context {
			raw_args,
			args,
			common_flags,
			routes,
			local_flags,
			current_path,
			common_flags_values,
			local_flags_values,
			parsing_args,
			error_info_list,
		}
	}

	/// Set args
	pub fn args(mut self, args: VecDeque<String>) -> Self {
		self.args = args;
		self
	}

	/// Get current_path as &str
	pub fn current(&self) -> &str {
		&self.current_path
	}

	/// Change current_path's value
	pub fn change_current(mut self, path: String) {
		self.current_path = path;
	}

	/// Find long form of local flag matches name_or_alias
	/// ロングフォームがname_or_aliasと一致するローカルフラグを検索してその結果を返す
	#[inline]
	pub fn find_local_long_flag(&self, name_or_alias: &str) -> LongFound<&Flag> {
		self.local_flags.find_long_flag(name_or_alias)
	}

	/// Find short form of local flag matches name_or_alias
	/// ショートフォームがname_or_aliasと一致するローカルフラグを検索してその結果を返す
	#[inline]
	pub fn find_local_short_flag(&self, short_alias: &char) -> Option<&Flag> {
		self.local_flags.find_short_flag(short_alias)
	}

	/// Find long form of common flag matches name_or_alias
	/// ロングフォームがname_or_aliasと一致するコモンフラグを検索してその結果を返す
	#[inline]
	pub fn find_common_long_flag(&self, name_or_alias: &str) -> LongFound<&Flag> {
		self.common_flags.find_long_flag(name_or_alias)
	}

	/// Find short form of common flag matches name_or_alias
	/// ショートフォームがname_or_aliasと一致するコモンフラグを検索してその結果を返す
	#[inline]
	pub fn find_common_short_flag(&self, short_alias: &char) -> Option<&Flag> {
		self.common_flags.find_short_flag(short_alias)
	}

	/// Add(Push back) middle_arg to this context's parsing_args
	pub fn push_back_to_parsing_args(&mut self, middle_arg: MiddleArg) {
		match self.parsing_args {
			None => {
				self.parsing_args = Some({
					let mut inner = VecDeque::new();
					inner.push_back(middle_arg);
					inner
				})
			}
			Some(ref mut vd) => (*vd).push_back(middle_arg),
		}
	}

	/// Takes flag value from context. Different from get, returns flag_value instance own (not reference) that has context.
	/// contextからフラグ値を取得する。Getとは違い、参照ではなくcontextに格納されているもの（格納されていない場合はデフォルト値のコピー）そのものを返す
	pub fn take_flag_value_of(&mut self, flag_name: &str) -> Option<FlagValue> {
		match self.take_local_flag_value_of(flag_name) {
			None => self.take_common_flag_value_of(flag_name),
			val => val,
		}
	}

	/// Takes inputted flag value from context. Different from get, returns flag_value instance own (not reference) that has context.
	/// contextからフラグ値を（ユーザによりargに指定（入力）されている場合）取得する。Getとは違い、参照ではなくcontextに格納されているもの（格納されていない場合はNoneを）そのものを返す
	pub fn take_inputted_flag_value_of(&mut self, flag_name: &str) -> Option<FlagValue> {
		match self.take_inputted_local_flag_value_of(flag_name) {
			None => self.take_inputted_common_flag_value_of(flag_name),
			val => val,
		}
	}

	/// Takes flag value from context. Different from get, returns flag_value instance own (not reference) that has context.
	/// contextからローカルフラグの値を取得する。Getとは違い、参照ではなくcontextに格納されているもの（格納されていない場合はデフォルト値のコピー）そのものを返す
	pub fn take_local_flag_value_of(&mut self, flag_name: &str) -> Option<FlagValue> {
		match self.take_inputted_common_flag_value_of(flag_name) {
			None => match self.local_flags.find(flag_name) {
				None => None,
				Some(f) => Some(f.default_value.clone()),
			},
			val => val,
		}
	}

	/// Takes inputted flag value from context. Different from get, returns flag_value instance own (not reference) that has context.
	/// contextからコモンフラグの値を取得する。Getとは違い、参照ではなくcontextに格納されているもの（格納されていない場合はデフォルト値のコピー）そのものを返す
	pub fn take_common_flag_value_of(&mut self, flag_name: &str) -> Option<FlagValue> {
		match self.take_inputted_common_flag_value_of(flag_name) {
			None => match self.common_flags.find(flag_name) {
				None => None,
				Some(f) => Some(f.default_value.clone()),
			},
			val => val,
		}
	}

	/// Takes inputted local flag value from context. Different from get, returns flag_value instance own (not reference) that has context.
	/// contextからローカルフラグ値を（ユーザによりargに指定（入力）されている場合）取得する。Getとは違い、参照ではなくcontextに格納されているものそのもの（格納されていない場合はNone）を返す
	pub fn take_inputted_local_flag_value_of(&mut self, flag_name: &str) -> Option<FlagValue> {
		match self.local_flags_values {
			Vector(None) => None,
			Vector(Some(ref mut local)) => {
				match local.iter().position(|(name, _)| name == flag_name) {
					Some(index) => {
						let (_, val) = local.remove(index);
						Some(val)
					}
					None => None,
				}
			}
		}
	}

	/// Takes inputted local flag value from context. Different from get, returns flag_value instance own (not reference) that has context.
	/// contextからコモンフラグ値を（ユーザによりargに指定（入力）されている場合）取得する。Getとは違い、参照ではなくcontextに格納されているものそのもの（格納されていない場合はNone）を返す
	pub fn take_inputted_common_flag_value_of(&mut self, flag_name: &str) -> Option<FlagValue> {
		match self.common_flags_values {
			Vector(None) => None,
			Vector(Some(ref mut common)) => {
				match common.iter().position(|(name, _)| name == flag_name) {
					Some(index) => {
						let (_, val) = common.remove(index);
						Some(val)
					}
					None => None,
				}
			}
		}
	}

	/// Gets FlagValue's clone of the flag matches flag_name from context.
	/// contextからフラグ値のcloneを取得する。フラグが設定されていない場合はNoneを返す
	pub fn get_flag_value_of(&self, flag_name: &str) -> Option<FlagValue> {
		match self.get_local_flag_value_of(flag_name) {
			None => self.get_common_flag_value_of(flag_name),
			flag_val => flag_val,
		}
	}

	/// Gets FlagValue's clone of the inputted flag matches flag_name from context.
	/// contextからユーザから指定された場合のフラグ値のcloneを取得する。ユーザから入力されていない場合はNoneを返す。
	pub fn get_inputted_flag_value_of(&self, flag_name: &str) -> Option<FlagValue> {
		match self.get_inputted_local_flag_value_of(flag_name) {
			None => self.get_inputted_common_flag_value_of(flag_name),
			flag_val => flag_val,
		}
	}

	/// Gets FlagValue's clone of the common flag matches flag_name from context. If it is not defined, Returns None.
	/// contextからユーザから指定された場合のコモンフラグ値のcloneを取得する。ユーザから入力されていないが定義されている場合はデフォルト値のクローンを返す。定義もされていない場合はNoneを返す。
	pub fn get_common_flag_value_of(&self, flag_name: &str) -> Option<FlagValue> {
		match self.get_inputted_common_flag_value_of(flag_name) {
			None | Some(FlagValue::None) => match self.common_flags.find(flag_name) {
				Some(f) => Some(f.default_value.clone()),
				None => None,
			},
			val => val,
		}
	}

	/// Gets FlagValue's clone of the common flag matches flag_name from context. If it is not defined, Returns None.
	/// contextからユーザから指定された場合のローカルフラグ値のcloneを取得する。ユーザから入力されていないが定義されている場合はデフォルト値のクローンを返す。定義もされていない場合はNoneを返す。
	pub fn get_local_flag_value_of(&self, flag_name: &str) -> Option<FlagValue> {
		match self.get_inputted_local_flag_value_of(flag_name) {
			None | Some(FlagValue::None) => match self.local_flags.find(flag_name) {
				Some(f) => Some(f.default_value.clone()),
				None => None,
			},
			val => val,
		}
	}

	/// Gets the flag value of the local flag matches flag_name if inputted. If it is not defined or not inputted, returns None.
	/// flag_nameとnameが一致するローカルフラグがあり、それがユーザからコマンド引数で指定されていた場合、その値のクローンをSomeで包んで返す。flag_nameと一致するnameをどのローカルフラグも持たないか、ユーザがコマンド引数で指定していない場合はNoneを返す。
	pub fn get_inputted_local_flag_value_of(&self, flag_name: &str) -> Option<FlagValue> {
		match &self.local_flags_values {
			Vector(None) => None,
			Vector(Some(local)) => match local.iter().find(|(name, _)| name == flag_name) {
				None => None,
				Some((_, flag_val)) => Some(flag_val.to_owned()),
			},
		}
	}

	/// Gets the flag value of the common flag whose name matches flag_name. If it is not defined or not inputted, returns None.
	/// flag_nameとnameが一致するコモンフラグがあり、それがユーザからコマンド引数で指定されていた場合、その値のクローンをSomeで包んで返す。flag_nameと一致するnameをどのコモンフラグも持たないか、ユーザがコマンド引数で指定していない場合はNoneを返す。
	pub fn get_inputted_common_flag_value_of(&self, flag_name: &str) -> Option<FlagValue> {
		match &self.common_flags_values {
			Vector(None) => None,
			Vector(Some(common)) => match common.iter().find(|(name, _)| name == flag_name) {
				None => None,
				Some((_, flag_val)) => Some(flag_val.to_owned()),
			},
		}
	}
}

impl<'a> From<Vec<String>> for Context {
	fn from(raw_args: Vec<String>) -> Context {
		let args = VecDeque::from(raw_args.clone());
		let current_path = match raw_args.get(0) {
			Some(str) => String::from(str),
			None => String::new(),
		};
		Context {
			raw_args,
			args,
			common_flags: Vector::default(),
			local_flags: Vector::default(),
			routes: Vector::default(),
			current_path,
			common_flags_values: Vector::default(),
			local_flags_values: Vector::default(),
			parsing_args: None,
			error_info_list: Vector::default(),
		}
	}
}
