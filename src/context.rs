use std::collections::VecDeque;

use crate::{
	args::Arg,
	opt::{Opt, ParsableOpt, ParseValueError},
};

/// Storage information for command execution.
/// This storage raw args, non-flag args, flag values, and etc.
/// コマンドからrunを通ってactionにたどり着くまでの情報およびパース結果を格納する構造体。
/// フラグの値、たどってきたルートなどを保管しています。
pub struct Context<
	OptValue = crate::opt::OptValue,
	OptError: std::error::Error = ParseValueError,
	OptionItem: ParsableOpt<OptValue, OptError> = Opt<OptValue, OptError>,
> {
	/// args (almost raw, recoverable by using parser)
	/// first arg is moved to `exe_path`
	pub raw_args: Vec<Arg>,
	/// non-flag args storage
	/// Note: processing args are stored here in parsing
	pub args: VecDeque<Arg>,
	/// exe path
	pub exe_path: String,
	/// command name routes from root to end
	pub routes: Vec<String>,
	/// common options of now command and inherited
	pub common_options: Vec<Vec<OptionItem>>,
	/// parsed options
	pub parsed_options: Vec<(String, OptValue)>,
	/// error info list
	pub errors: Vec<(Arg, OptError)>,
}

impl<OptValue, OptError: std::error::Error, OptionItem: ParsableOpt<OptValue, OptError>>
	Context<OptValue, OptError, OptionItem>
{
	/// Creates a new `Context` with the given raw args, exe path, and parsed args.
	pub fn new(
		raw_args: Vec<Arg>,
		exe_path: String,
		args: VecDeque<Arg>,
		common_flags: Vec<OptionItem>,
		routes: Vec<String>,
	) -> Self {
		Self {
			raw_args,
			args,
			exe_path,
			routes,
			common_options: vec![common_flags],
			parsed_options: vec![],
			errors: vec![],
		}
	}
}

/// A trait for storing context information.
pub trait StorableContext<
	OptValue,
	OptError: std::error::Error,
	OptItem: ParsableOpt<OptValue, OptError>,
>
{
	/// Creates a new `Context` with the given raw args, exe path, and parsed args.
	fn with_init_base(
		raw_args: Vec<Arg>,
		exe_path: String,
		args: VecDeque<Arg>,
		common_flags: Vec<OptItem>,
		routes: Vec<String>,
	) -> Self;
	/// Returns the raw args.
	fn raw_args(&self) -> &[Arg];
	/// Returns the parsed args.
	fn args(&self) -> &VecDeque<Arg>;
	/// Returns the exe path.
	fn exe_path(&self) -> &str;
	/// Returns the command name routes.
	fn routes(&self) -> &[String];
	/// Returns the common options.
	fn common_options(&self) -> &[OptItem];
	/// Returns the parsed options.
	fn parsed_options(&self) -> &[(String, OptValue)];
	/// Returns the errors.
	fn errors(&self) -> &[(Arg, OptError)];
}

impl<OptValue, OptError: std::error::Error, OptItem: ParsableOpt<OptValue, OptError>>
	StorableContext<OptValue, OptError, OptItem> for Context<OptValue, OptError, OptItem>
{
	fn with_init_base(
		raw_args: Vec<Arg>,
		exe_path: String,
		args: VecDeque<Arg>,
		common_flags: Vec<OptItem>,
		routes: Vec<String>,
	) -> Self {
		Self {
			raw_args,
			args,
			exe_path,
			routes,
			common_options: vec![common_flags],
			parsed_options: vec![],
			errors: vec![],
		}
	}

	fn raw_args(&self) -> &[Arg] {
		&self.raw_args
	}
	fn args(&self) -> &VecDeque<Arg> {
		&self.args
	}
	fn exe_path(&self) -> &str {
		&self.exe_path
	}
	fn routes(&self) -> &[String] {
		&self.routes
	}
	fn common_options(&self) -> &[OptItem] {
		&self.common_options
	}
	fn parsed_options(&self) -> &[(String, OptValue)] {
		&self.parsed_options
	}
	fn errors(&self) -> &[(Arg, OptError)] {
		&self.errors
	}
}
