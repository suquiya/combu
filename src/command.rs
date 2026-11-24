use std::fmt::Debug;

use crate::{
	action::{Action, ActionResult},
	context::{Context, StorableContext},
	describer::Describer,
	opt::{Opt, OptValue, ParsableOpt, ParseValueError},
	parser::Parser,
};

/// The struct for command information store and command execution
/// This can be root and edge
/// コマンドの情報格納＆実行用構造体です。root（根）にもedge（葉）にもなりえます。
#[derive(Default, Debug)]
pub struct Command<
	OptVal = OptValue,
	OptErr: std::error::Error = ParseValueError,
	OptItem: ParsableOpt<OptVal, OptErr> = Opt<OptVal, OptErr>,
	Ctx: StorableContext<OptVal, OptErr, OptItem> = Context<OptVal, OptErr, OptItem>,
> {
	///Command name
	pub name: String,
	///Command action
	pub action: Option<Action<Self, Ctx>>,
	///Command authors
	pub authors: String,
	///Command copyright
	pub copyright: String,
	/// License of command
	pub license: Option<Describer>,
	/// Command description
	pub description: Option<String>,
	///Command usage
	pub usage: String,
	///local options of command
	pub local_options: Vec<OptItem>,
	///common options of command
	pub common_options: Vec<OptItem>,
	///alias for command
	pub alias: Vec<String>,
	///Command version
	pub version: String,
	///container of sub-command
	pub sub: Vec<Self>,

	// Marker for generic type parameters
	_marker_v: std::marker::PhantomData<(OptVal, OptErr, OptItem, Ctx)>,
}

impl<
	OptVal,
	OptErr: std::error::Error,
	OptItem: ParsableOpt<OptVal, OptErr>,
	Ctx: StorableContext<OptVal, OptErr, OptItem>,
> Command<OptVal, OptErr, OptItem, Ctx>
{
	/// Creates a new command with the given name and no action.
	pub fn new() -> Self {
		Self::with_name(String::new())
	}

	/// Creates a new command with the given name.
	pub fn with_name<T: Into<String>>(name: T) -> Self {
		Self {
			name: name.into(),
			action: None,
			authors: String::new(),
			copyright: String::new(),
			license: None,
			description: None,
			usage: String::new(),
			local_options: Vec::new(),
			common_options: Vec::new(),
			alias: Vec::new(),
			version: String::new(),
			sub: Vec::new(),
			_marker_v: std::marker::PhantomData,
		}
	}

	/// Creates a new command with the given name and base information.
	pub fn with_base<T: Into<String>>(
		name: T,
		authors: T,
		version: T,
		description: T,
		action: Option<Action<Self, Ctx>>,
	) -> Self {
		Self {
			name: name.into(),
			authors: authors.into(),
			copyright: String::new(),
			version: version.into(),
			description: Some(description.into()),
			action,
			usage: String::new(),
			license: None,
			local_options: Vec::new(),
			common_options: Vec::new(),
			alias: Vec::new(),
			sub: Vec::new(),
			_marker_v: std::marker::PhantomData,
		}
	}

	/// Creates a new command with all fields set.
	#[allow(clippy::too_many_arguments)]
	pub fn with_all_fields<T: Into<String>>(
		name: T,
		authors: T,
		copyright: T,
		version: T,
		description: T,
		action: Option<Action<Self, Ctx>>,
		usage: T,
		license: Option<Describer>,
		local_options: Vec<OptItem>,
		common_options: Vec<OptItem>,
		alias: Vec<String>,
		sub: Vec<Self>,
	) -> Self {
		Self {
			name: name.into(),
			authors: authors.into(),
			copyright: copyright.into(),
			version: version.into(),
			description: Some(description.into()),
			action,
			usage: usage.into(),
			license,
			local_options,
			common_options,
			alias,
			sub,
			_marker_v: std::marker::PhantomData,
		}
	}
}

/// A trait for runnable commands.
pub trait RunnableCommand<
	OptVal,
	OptErr: std::error::Error,
	OptItem: ParsableOpt<OptVal, OptErr>,
	Ctx: StorableContext<OptVal, OptErr, OptItem>,
>: Sized
{
	/// Returns the name of the command.
	fn name(&self) -> &str;

	/// Returns the alias of the command.
	fn alias(&self) -> &[String];

	/// Returns the subcommands of the command.
	fn sub(&self) -> &[Self];

	/// Returns subcommand position specified by name or alias.
	fn index_sub(&self, name: &str) -> Option<usize> {
		self
			.sub()
			.iter()
			.position(|sub| sub.name() == name || sub.alias().contains(&name.to_string()))
	}

	/// Removes the subcommand from the command and returns it.
	fn take_sub_by_index(&mut self, index: usize) -> Option<Self>;

	/// Returns subcommand specified by name or alias.
	fn take_sub(&mut self, name: &str) -> Option<(Self, usize)> {
		let index = self.index_sub(name)?;
		Some((self.take_sub_by_index(index)?, index))
	}

	/// Get match local option for long command name or long alias. Returns `None` if no match found.
	fn get_local_option_by_name_or_long_alias(&self, long_option: &str) -> Option<&OptItem>;

	/// Get match local option for short alias. Returns `None` if no match found.
	/// If you want to match short name, you should use `get_local_option_by_name_or_long_alias` instead or implement your own this method.
	fn get_local_option_by_short_alias(&self, short_option: char) -> Option<&OptItem>;

	/// Get match common option for long command name or long alias. Returns `None` if no match found.
	fn get_common_option_by_name_or_long_alias(&self, long_option: &str) -> Option<&OptItem>;

	/// Get match common option for short alias. Returns `None` if no match found.
	/// If you want to match short name, you should use `get_common_option_by_name_or_long_alias` instead or implement your own this method.
	fn get_common_option_by_short_alias(&self, short_option: char) -> Option<&OptItem>;

	/// Get match option for long command name or long alias. Returns `None` if no match found.
	fn get_option_by_name_or_long_alias(&self, long_option: &str) -> Option<&OptItem> {
		let opt = self.get_local_option_by_name_or_long_alias(long_option);
		match opt {
			Some(_) => opt,
			None => self.get_common_option_by_name_or_long_alias(long_option),
		}
	}

	fn get_option_by_short_alias(&self, short_option: char) -> Option<&OptItem> {
		let opt = self.get_local_option_by_short_alias(short_option);
		match opt {
			Some(_) => opt,
			None => self.get_common_option_by_short_alias(short_option),
		}
	}

	/// Takes common options of the command and returns them.
	fn take_common_options(&mut self) -> Vec<OptItem>;

	/// Runs the command with the given arguments. parser is default-parser.
	fn run_with_args<T: Iterator<Item = String>>(self, args: T) -> ActionResult<Self, Ctx> {
		self.run_with_parser(args, Parser::default())
	}
	/// Runs the command with the given arguments. parser is used to parse the arguments.
	fn run_with_parser<T: Iterator<Item = String>>(
		self,
		mut args: T,
		parser: Parser,
	) -> ActionResult<Self, Ctx> {
		let (exe_path, middle_args) = parser
			.parse_to_exe_path_and_middle_args(&mut args)
			.ok_or_else(|| {
				let message = "Failed to parse exe path and arguments".to_string();
				let kind = crate::action::ActionErrorKind::Unexpected;
				crate::action::ActionError::new(message, kind)
			})?;
		// Create a new context with the parsed arguments and exe path.
		// Self has common_options and name, so we don't pass them to the context (run_with_context also has self).
		// コンテキストを作成する。selfにcommon_optionsとnameがあるので、contextに渡さない(`run_with_context`もselfもちであるので)で処理する
		self.run_with_context(Ctx::with_init_base(middle_args, exe_path, vec![], vec![]))
	}

	/// Runs the command with the given context.
	fn run_with_context(self, context: Ctx) -> ActionResult<Self, Ctx> {
		if self.sub().is_empty() {
			// If there are no subcommands, parse the options and run the action of the current command.
			self.run_action(context)
		} else {
			todo!()
			// // If there are subcommands, check if the first argument matches a subcommand.
			// if let Some(arg) = context.args().next() {
			// 	if let Some((subcommand, _)) = self.take_sub(arg) {
			// 		// If a matching subcommand is found, run it with the remaining arguments.
			// 		let mut new_context = context;
			// 		new_context.pop_front_arg();
			// 		subcommand.run_with_context(new_context)
			// 	} else {
			// 		// If no matching subcommand is found, run the action of the current command.
			// 		if let Some(action) = self.action() {
			// 			action(self, context)
			// 		} else {
			// 			done!()
			// 		}
			// 	}
			// } else {
			// 	// If there are no arguments left, run the action of the current command.
			// 	if let Some(action) = self.action() {
			// 		action(self, context)
			// 	} else {
			// 		done!()
			// 	}
			// }
		}
	}

	fn action(&self) -> Option<Action<Self, Ctx>>;

	fn create_no_action_error(cmd: &Self) -> crate::action::ActionError {
		crate::action::ActionError::new(
			"No action registered for ".to_string() + cmd.name(),
			crate::action::ActionErrorKind::NoActionRegistered,
		)
	}

	/// Runs the command action with the given context.
	fn run_action(self, context: Ctx) -> ActionResult<Self, Ctx> {
		if let Some(action) = self.action() {
			action(self, context)
		} else {
			Err(Self::create_no_action_error(&self))
		}
	}
}

impl<
	OptVal,
	OptErr: std::error::Error,
	OptItem: ParsableOpt<OptVal, OptErr>,
	Ctx: StorableContext<OptVal, OptErr, OptItem>,
> RunnableCommand<OptVal, OptErr, OptItem, Ctx> for Command<OptVal, OptErr, OptItem, Ctx>
{
	fn name(&self) -> &str {
		&self.name
	}

	fn alias(&self) -> &[String] {
		&self.alias
	}

	fn sub(&self) -> &[Self] {
		&self.sub
	}

	fn take_sub_by_index(&mut self, index: usize) -> Option<Self> {
		if index < self.sub.len() {
			Some(self.sub.swap_remove(index))
		} else {
			None
		}
	}

	fn take_common_options(&mut self) -> Vec<OptItem> {
		std::mem::take(&mut self.common_options)
	}

	fn action(&self) -> Option<Action<Self, Ctx>> {
		self.action
	}

	fn get_local_option_by_name_or_long_alias(&self, long_option: &str) -> Option<&OptItem> {
		self
			.local_options
			.iter()
			.find(|opt| opt.match_long_call(long_option))
	}

	fn get_local_option_by_short_alias(&self, short_option: char) -> Option<&OptItem> {
		self
			.local_options
			.iter()
			.find(|opt| opt.match_short_call(short_option))
	}

	fn get_common_option_by_name_or_long_alias(&self, long_option: &str) -> Option<&OptItem> {
		self
			.common_options
			.iter()
			.rfind(|opt| opt.match_long_call(long_option))
	}

	fn get_common_option_by_short_alias(&self, short_option: char) -> Option<&OptItem> {
		self
			.common_options
			.iter()
			.rfind(|opt| opt.match_short_call(short_option))
	}
}
