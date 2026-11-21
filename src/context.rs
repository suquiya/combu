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
	/// First arg is moved to `exe_path`
	pub raw_args: Vec<Arg>,
	/// non-flag args storage
	/// Note: processing args are stored here in parsing
	pub args: Vec<Arg>,
	/// exe path
	/// This is the first arg of `raw_args`, and is moved to here in parsing
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
		args: Vec<Arg>,
		exe_path: String,
		common_options: Vec<OptionItem>,
		routes: Vec<String>,
	) -> Self {
		let raw_args = args;
		let args = raw_args.clone();
		Self {
			raw_args,
			args,
			exe_path,
			routes,
			common_options: vec![common_options],
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
		args: Vec<Arg>,
		exe_path: String,
		init_common_options: Vec<OptItem>,
		routes: Vec<String>,
	) -> Self;
	/// Takes the args from the context. Mainly for parsing.
	fn take_args(mut self) -> (Self, impl Iterator<Item = Arg>);
	/// Add arg to args. Mainly for push normal args in parsing.
	fn push_arg(&mut self, arg: Arg);
	/// Returns the exe path.
	fn exe_path(&self) -> &str;
	/// Returns the command name routes.
	fn add_route(&mut self, route_name: String);
	/// Returns the common options.
	fn common_options(&self) -> &[Vec<OptItem>];
	/// Adds the given parsed option to the context.
	fn add_parsed_option(&mut self, opt: &OptItem, value: OptValue);
	/// Returns the errors.
	fn errors(&self) -> &[(Arg, OptError)];
}

impl<OptValue, OptError: std::error::Error, OptItem: ParsableOpt<OptValue, OptError>>
	StorableContext<OptValue, OptError, OptItem> for Context<OptValue, OptError, OptItem>
{
	fn with_init_base(
		args: Vec<Arg>,
		exe_path: String,
		init_common_options: Vec<OptItem>,
		routes: Vec<String>,
	) -> Self {
		Self::new(args, exe_path, init_common_options, routes)
	}

	fn take_args(mut self) -> (Self, impl Iterator<Item = Arg>) {
		let args = std::mem::take(&mut self.args);
		(self, args.into_iter())
	}

	fn push_arg(&mut self, arg: Arg) {
		self.args.push(arg);
	}

	fn exe_path(&self) -> &str {
		&self.exe_path
	}

	fn add_route(&mut self, route_name: String) {
		self.routes.push(route_name);
	}

	fn common_options(&self) -> &[Vec<OptItem>] {
		&self.common_options
	}

	fn errors(&self) -> &[(Arg, OptError)] {
		&self.errors
	}

	fn add_parsed_option(&mut self, opt: &OptItem, value: OptValue) {
		self.parsed_options.push((opt.name().to_string(), value));
	}
}
