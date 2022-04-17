use fmt::Debug;

use crate::{Command, Context};
use std::{error::Error, fmt};

///Action is type for command action. It returns Result<ActionResult, ActionError>.
pub type Action = fn(Command, Context) -> Result<ActionResult, ActionError>;

/// ActionResult stores result of action.
/// Commandのアクション結果を格納し、runの結果として返却するためのenum。
#[derive(Debug)]
pub enum ActionResult {
	///Done shows that action is done.
	Done,
	/// ParentActionRequest shows that action requested to show help.
	ParentActionRequest(Context, Command),
	/// Shows return Context, reached Command and Action as result for parse and run.
	Result(Context, Command),
	/// Custom result(can have Box including dyn Debug).
	Custom(Box<dyn Debug>),
}

impl ActionResult {
	/// Returns true if self is done.
	pub fn is_done(&self) -> bool {
		matches!(self, ActionResult::Done)
	}
}

/// ActionError stores error of action.
#[derive(Debug)]
pub struct ActionError {
	/// ActionError's value
	pub value: String,
	/// ActionError's Kind
	pub kind: ActionErrorKind,
	/// command is a field for storing command that error occured
	pub command: Command,
	/// context is a field for storing context that error occured
	pub context: Context,
	/// If there is an error which is not ActionError, related_error can stores it.
	pub related_error: Option<Box<dyn Error>>,
	/// printed flag. If this is true, this shows error is not printed yet.
	pub printed: bool,
}

/// ErrorKind of ActionError
#[derive(Debug)]
pub enum ActionErrorKind {
	/// Shows a custom(Normal) error
	Custom,
	/// Shows that no action is registered to specidied command.
	NoActionRegistered,
	/// Shows None.
	None,
}

impl ActionError {
	/// Creates new ActionError.
	pub fn new<T: Into<String>>(
		value: T,
		kind: ActionErrorKind,
		command: Command,
		context: Context,
		related_error: Option<Box<dyn Error>>,
	) -> Self {
		Self {
			value: value.into(),
			kind,
			command,
			context,
			related_error,
			printed: false,
		}
	}

	/// Creates new ActionError without (not action) error info.
	pub fn without_related_error(
		value: String,
		kind: ActionErrorKind,
		command: Command,
		context: Context,
	) -> Self {
		Self {
			value,
			kind,
			command,
			context,
			related_error: None,
			printed: false,
		}
	}
}

impl fmt::Display for ActionError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
		match self.kind {
			ActionErrorKind::Custom => write!(f, "{}", self.value),
			ActionErrorKind::NoActionRegistered => {
				write!(f, "{} does not have its own action.", self.value)
			}
			ActionErrorKind::None => write!(f, "no action error"),
		}
	}
}

impl Error for ActionError {}

/// bundle is a helper for processing in action.
pub mod bundle {
	use crate::{Command, Context, FlagValue};

	/// New-type for help processing in action. Inner is simple tuple - (Context,Command)
	pub struct Bundle(pub Context, pub Command);

	impl From<(Context, Command)> for Bundle {
		fn from(val: (Context, Command)) -> Self {
			Bundle(val.0, val.1)
		}
	}

	impl From<Bundle> for (Context, Command) {
		fn from(val: Bundle) -> Self {
			val.unpack()
		}
	}

	impl Bundle {
		/// Creates new instance of bundle.
		pub fn new(ctx: Context, cmd: Command) -> Self {
			Bundle(ctx, cmd)
		}
		/// Returns inner tuple (Context,Command)
		pub fn unpack(self) -> (Context, Command) {
			let Bundle(ctx, cmd) = self;
			(ctx, cmd)
		}
		/// Returns ref of Context
		pub fn context(&self) -> &Context {
			&self.0
		}
		/// Returns ref of Command
		pub fn command(&self) -> &Command {
			&self.1
		}
		/// Returns mut ref of Context
		pub fn context_mut(&mut self) -> &mut Context {
			&mut self.0
		}
		/// Returns mut ref of Command
		pub fn command_mut(&mut self) -> &mut Command {
			&mut self.1
		}
		/// Get exe_path as &str
		pub fn exe_path(&self) -> &str {
			self.context().exe_path()
		}
		/// Takes flag value from context. Different from get, returns flag_value instance own (not reference) that has context.
		/// contextからフラグ値を取得する。Getとは違い、参照ではなくcontextに格納されているもの（格納されていない場合はデフォルト値のコピー）そのものを返す
		pub fn take_flag_value_of(&mut self, flag_name: &str) -> Option<FlagValue> {
			self.0.take_flag_value_of(flag_name, &self.1)
		}
		/// Takes inputted flag value from context. Different from get, returns flag_value instance own (not reference) that has context.
		/// contextからフラグ値を（ユーザによりargに指定（入力）されている場合）取得する。Getとは違い、参照ではなくcontextに格納されているもの（格納されていない場合はNoneを）そのものを返す
		pub fn take_inputted_flag_value_of(&mut self, flag_name: &str) -> Option<FlagValue> {
			self.0.take_inputted_flag_value_of(flag_name)
		}
		/// Takes flag value from context. Different from get, returns flag_value instance own (not reference) that has context.
		/// contextからローカルフラグの値を取得する。Getとは違い、参照ではなくcontextに格納されているもの（格納されていない場合はデフォルト値のコピー）そのものを返す
		pub fn take_local_flag_value_of(&mut self, flag_name: &str) -> Option<FlagValue> {
			self.0.take_local_flag_value_of(flag_name, &self.1)
		}
		/// Takes inputted flag value from context. Different from get, returns flag_value instance own (not reference) that has context.
		/// contextからコモンフラグの値を取得する。Getとは違い、参照ではなくcontextに格納されているもの（格納されていない場合はデフォルト値のコピー）そのものを返す
		pub fn take_common_flag_value_of(&mut self, flag_name: &str) -> Option<FlagValue> {
			self.0.take_common_flag_value_of(flag_name, &self.1)
		}

		/// Takes inputted local flag value from context. Different from get, returns flag_value instance own (not reference) that has context.
		/// contextからローカルフラグ値を（ユーザによりargに指定（入力）されている場合）取得する。Getとは違い、参照ではなくcontextに格納されているものそのもの（格納されていない場合はNone）を返す
		pub fn take_inputted_local_flag_value_of(&mut self, flag_name: &str) -> Option<FlagValue> {
			self.0.take_inputted_local_flag_value_of(flag_name)
		}

		/// Takes inputted local flag value from context. Different from get, returns flag_value instance own (not reference) that has context.
		/// contextからコモンフラグ値を（ユーザによりargに指定（入力）されている場合）取得する。Getとは違い、参照ではなくcontextに格納されているものそのもの（格納されていない場合はNone）を返す
		pub fn take_inputted_common_flag_value_of(&mut self, flag_name: &str) -> Option<FlagValue> {
			self.0.take_inputted_common_flag_value_of(flag_name)
		}

		/// Gets FlagValue's clone of the flag matches flag_name from context.
		/// contextからフラグ値のcloneを取得する。フラグが設定されていない場合はNoneを返す
		/// なお明示的に値が指定されない場合、Bool型のフラグであればFlagValue::Bool(true)とし、String型のフラグであればFlagValue::String(String::new())、それ以外の型のフラグではFlagValue::NoneをSomeで包んで返す
		pub fn get_flag_value_of(&self, flag_name: &str) -> Option<FlagValue> {
			self.0.get_local_flag_value_of(flag_name, &self.1)
		}

		/// Gets FlagValue's clone of the inputted flag matches flag_name from context.
		/// contextからユーザから指定された場合のフラグ値のcloneを取得する。ユーザから入力されていない場合はNoneを返す。
		pub fn get_inputted_flag_value_of(&self, flag_name: &str) -> Option<FlagValue> {
			self.0.get_inputted_flag_value_of(flag_name)
		}

		/// Gets FlagValue's clone of the common flag matches flag_name from context. If it is not defined, Returns None.
		/// contextからユーザから指定された場合のローカルフラグ値のcloneを取得する。ユーザから入力されていないが定義されている場合はデフォルト値のクローンを返す。定義もされていない場合はNoneを返す。
		/// なお明示的に値が指定されない場合、Bool型のフラグであればFlagValue::Bool(true)とし、String型のフラグであればFlagValue::String(String::new())、それ以外の型のフラグではFlagValue::NoneをSomeで包んで返す
		pub fn get_local_flag_value_of(&self, flag_name: &str) -> Option<FlagValue> {
			self.0.get_local_flag_value_of(flag_name, &self.1)
		}

		/// Gets the flag value of the local flag matches flag_name if inputted. If it is not defined or not inputted, returns None.
		/// flag_nameとnameが一致するローカルフラグがあり、それがユーザからコマンド引数で指定されていた場合、その値のクローンをSomeで包んで返す。flag_nameと一致するnameをどのローカルフラグも持たないか、ユーザがコマンド引数で指定していない場合はNoneを返す。
		pub fn get_inputted_local_flag_value_of(&self, flag_name: &str) -> Option<FlagValue> {
			self.0.get_inputted_common_flag_value_of(flag_name)
		}

		/// Gets the flag value of the common flag whose name matches flag_name. If it is not defined or not inputted, returns None.
		/// flag_nameとnameが一致するコモンフラグがあり、それがユーザからコマンド引数で指定されていた場合、その値のクローンをSomeで包んで返す。flag_nameと一致するnameをどのコモンフラグも持たないか、ユーザがコマンド引数で指定していない場合はNoneを返す。
		pub fn get_inputted_common_flag_value_of(&self, flag_name: &str) -> Option<FlagValue> {
			self.0.get_inputted_common_flag_value_of(flag_name)
		}

		/// Returns true the value of the flag which has specified name is true.
		pub fn is_flag_true(&self, name: &str) -> bool {
			Some(FlagValue::Bool(true)) == self.0.get_flag_value_of(name, &self.1)
		}

		/// Returns depth of command - root:0
		pub fn depth(&self) -> usize {
			self.0.common_flags.len()
		}
	}
}
