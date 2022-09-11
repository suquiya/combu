use crate::{
	parser::{ErrorInfo, MiddleArg},
	vector::flag::FlagSearch,
	Command, Flag, FlagValue, Vector,
};
use std::collections::VecDeque;

/// Storage information for command execution.
/// This storage raw args, non-flag args, flag values, and etc.
/// コマンドからrunを通ってactionにたどり着くまでの情報およびパース結果を格納する構造体。
/// フラグの値、たどってきたルートなどを保管しています。
#[derive(Debug, Clone)]
pub struct Context {
	/// raw args
	pub raw_args: Vec<String>,
	/// non-flag args
	pub args: VecDeque<String>,
	/// common_flags of its own and inherited
	pub common_flags: Vector<Vector<Flag>>,
	/// routes of from root to end
	pub routes: Vector<String>,
	/// exe_path (String, not PathBuf)
	pub exe_path: String,
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
		routes: Vector<String>,
		exe_path: String,
	) -> Context {
		Context {
			raw_args,
			args,
			common_flags: Vector(Some(vec![common_flags])),
			routes: routes.into(),
			exe_path,
			common_flags_values: Vector::default(),
			local_flags_values: Vector::default(),
			parsing_args: None,
			error_info_list: Vector::default(),
		}
	}

	/// Creates a new instance of Context with all options.
	pub fn with_all_field(
		raw_args: Vec<String>,
		args: VecDeque<String>,
		common_flags: Vector<Vector<Flag>>,
		exe_path: String,
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
			exe_path,
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

	/// Get exe_path as &str
	pub fn exe_path(&self) -> &str {
		&self.exe_path
	}

	/// Change exe_path's value
	pub fn change_exe_path(mut self, path: String) {
		self.exe_path = path;
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

	/// Shift(Push front) middle_arg to this context's parsing_args
	pub fn push_front_to_parsing_args(&mut self, middle_arg: MiddleArg) {
		match self.parsing_args {
			None => {
				self.parsing_args = Some({
					let mut inner = VecDeque::new();
					inner.push_front(middle_arg);
					inner
				})
			}
			Some(ref mut vd) => (*vd).push_back(middle_arg),
		}
	}

	/// Takes flag value from context. Different from get, returns flag_value instance own (not reference) that has context.
	/// contextからフラグ値を取得する。Getとは違い、参照ではなくcontextに格納されているもの（格納されていない場合はデフォルト値のコピー）そのものを返す
	pub fn take_flag_value_of(
		&mut self,
		flag_name: &str,
		current_command: &Command,
	) -> Option<FlagValue> {
		match self.take_local_flag_value_of(flag_name, current_command) {
			None => self.take_common_flag_value_of(flag_name, current_command),
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
	pub fn take_local_flag_value_of(
		&mut self,
		flag_name: &str,
		current_command: &Command,
	) -> Option<FlagValue> {
		match self.take_inputted_common_flag_value_of(flag_name) {
			None => match current_command.l_flags.find(flag_name) {
				None => None,
				Some(f) => Some(f.default_value.clone()),
			},
			val => val,
		}
	}

	/// Takes inputted flag value from context. Different from get, returns flag_value instance own (not reference) that has context.
	/// contextからコモンフラグの値を取得する。Getとは違い、参照ではなくcontextに格納されているもの（格納されていない場合はデフォルト値のコピー）そのものを返す
	pub fn take_common_flag_value_of(
		&mut self,
		flag_name: &str,
		current_command: &Command,
	) -> Option<FlagValue> {
		match self.take_inputted_common_flag_value_of(flag_name) {
			None => match (&current_command.c_flags, &self.common_flags).find(flag_name) {
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
	/// なお明示的に値が指定されない場合、Bool型のフラグであればFlagValue::Bool(true)とし、String型のフラグであればFlagValue::String(String::new())、それ以外の型のフラグではFlagValue::NoneをSomeで包んで返す
	pub fn get_flag_value_of(
		&self,
		flag_name: &str,
		current_command: &Command,
	) -> Option<FlagValue> {
		match self.get_local_flag_value_of(flag_name, current_command) {
			None => self.get_common_flag_value_of(flag_name, current_command),
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
	/// なお明示的に値が指定されない場合、Bool型のフラグであればFlagValue::Bool(true)とし、String型のフラグであればFlagValue::String(String::new())、それ以外の型のフラグではFlagValue::NoneをSomeで包んで返す
	pub fn get_common_flag_value_of(
		&self,
		flag_name: &str,
		current_command: &Command,
	) -> Option<FlagValue> {
		match self.get_inputted_common_flag_value_of(flag_name) {
			None => match (&current_command.c_flags, &self.common_flags).find(flag_name) {
				Some(f) => Some(f.default_value.clone()),
				None => None,
			},
			Some(FlagValue::None) => {
				match (&current_command.c_flags, &self.common_flags).find(flag_name) {
					Some(f) => Some(f.derive_flag_value_if_no_value()),
					None => None,
				}
			}
			val => val,
		}
	}

	/// Gets FlagValue's clone of the common flag matches flag_name from context. If it is not defined, Returns None.
	/// contextからユーザから指定された場合のローカルフラグ値のcloneを取得する。ユーザから入力されていないが定義されている場合はデフォルト値のクローンを返す。定義もされていない場合はNoneを返す。
	/// なお明示的に値が指定されない場合、Bool型のフラグであればFlagValue::Bool(true)とし、String型のフラグであればFlagValue::String(String::new())、それ以外の型のフラグではFlagValue::NoneをSomeで包んで返す
	pub fn get_local_flag_value_of(
		&self,
		flag_name: &str,
		current_command: &Command,
	) -> Option<FlagValue> {
		match self.get_inputted_local_flag_value_of(flag_name) {
			None => match current_command.l_flags.find(flag_name) {
				Some(f) => Some(f.default_value.clone()),
				None => None,
			},
			Some(FlagValue::None) => match current_command.l_flags.find(flag_name) {
				Some(f) => Some(f.derive_flag_value_if_no_value()),
				None => None,
			},
			val => val,
		}
	}

	/// Gets the flag value of the local flag matches flag_name if inputted. If it is not defined or not inputted, returns None.
	/// flag_nameとnameが一致するローカルフラグがあり、それがユーザからコマンド引数で指定されていた場合、その値のクローンをSomeで包んで返す。flag_nameと一致するnameを持たない場合はローカルフラグ値として保存されていないためNoneを返し、ユーザがコマンド引数で指定していない場合もNoneを返す。
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
	/// flag_nameとnameが一致するコモンフラグがあり、それがユーザからコマンド引数で指定されていた場合、その値のクローンをSomeで包んで返す。flag_nameと一致するnameをどのコモンフラグも持たない場合はコモンフラグ値としてそのフラグ値は保存されないためNoneを返し、ユーザがコマンド引数で指定していない場合もNoneを返す。
	pub fn get_inputted_common_flag_value_of(&self, flag_name: &str) -> Option<FlagValue> {
		match &self.common_flags_values {
			Vector(None) => None,
			Vector(Some(common)) => match common.iter().find(|(name, _)| name == flag_name) {
				None => None,
				Some((_, flag_val)) => Some(flag_val.to_owned()),
			},
		}
	}

	/// Returns flag has specified name is true flag.
	pub fn is_flag_true(&self, name: &str, current_command: &Command) -> bool {
		Some(FlagValue::Bool(true)) == self.get_flag_value_of(name, current_command)
	}

	/// Returns depth of command - root:0
	pub fn depth(&self) -> usize {
		self.common_flags.len()
	}

	/// Returns true is self has no error in error_info_list
	pub fn has_no_error(&self) -> bool {
		self.error_info_list.is_empty()
	}

	/// Returns error info list's length
	pub fn num_of_error(&self) -> usize {
		self.error_info_list.len()
	}

	/// Returns true if error info list's length is more than one.
	pub fn has_error(&self) -> bool {
		self.error_info_list.has_at_least_one()
	}

	/// Returns info of first parse error, or None if it does not exist.
	pub fn first_error(&self) -> Option<&ErrorInfo> {
		self.error_info_list.first()
	}
}

impl<'a> From<Vec<String>> for Context {
	fn from(raw_args: Vec<String>) -> Context {
		let args = VecDeque::from(raw_args.clone());
		let exe_path = match raw_args.get(0) {
			Some(str) => String::from(str),
			None => String::new(),
		};
		Context {
			raw_args,
			args,
			common_flags: Vector::default(),
			routes: Vector::default(),
			exe_path,
			common_flags_values: Vector::default(),
			local_flags_values: Vector::default(),
			parsing_args: None,
			error_info_list: Vector::default(),
		}
	}
}
