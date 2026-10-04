use std::fmt::Debug;

use crate::{
	Action, action_result,
	context::{Context, StorableContext},
	describer::Describer,
	done,
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
	pub l_options: Vec<OptItem>,
	///common options of command
	pub c_options: Vec<OptItem>,
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
			l_options: Vec::new(),
			c_options: Vec::new(),
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
			l_options: Vec::new(),
			c_options: Vec::new(),
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
		l_options: Vec<OptItem>,
		c_options: Vec<OptItem>,
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
			l_options,
			c_options,
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

	/// Returns local options of the command.
	fn local_options(&self) -> &[OptItem];

	/// Takes local options of the command and returns them.
	fn take_local_options(&mut self) -> Vec<OptItem>;

	/// Returns common options of the command.
	fn common_options(&self) -> &[OptItem];

	/// Takes common options of the command and returns them.
	fn take_common_options(&mut self) -> Vec<OptItem>;

	/// Returns the action of the command.
	fn action(&self) -> Option<&Action<Self, Ctx>>;

	/// Takes the action of the command and returns it.
	fn take_action(&mut self) -> Option<Action<Self, Ctx>>;

	/// Runs the command with the given arguments. parser is default-parser.
	fn run_with_args<T: Iterator<Item = String>>(self, args: T) -> action_result!(Self, Ctx) {
		self.run_with_parser(args, Parser::default())
	}
	/// Runs the command with the given arguments. parser is used to parse the arguments.
	fn run_with_parser<T: Iterator<Item = String>>(
		self,
		mut args: T,
		parser: Parser,
	) -> action_result!(Self, Ctx) {
		let (exe_path, middle_args) = parser
			.parse_to_exe_path_and_middle_args(&mut args)
			.ok_or_else(|| {
				let message = "Failed to parse exe path and arguments".to_string();
				let kind = crate::action::ActionErrorKind::Unexpected;
				crate::action::ActionError::new(message, kind)
			})?;
		done!()
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

	fn local_options(&self) -> &[OptItem] {
		&self.l_options
	}

	fn take_local_options(&mut self) -> Vec<OptItem> {
		std::mem::take(&mut self.l_options)
	}

	fn common_options(&self) -> &[OptItem] {
		&self.c_options
	}

	fn take_common_options(&mut self) -> Vec<OptItem> {
		std::mem::take(&mut self.c_options)
	}

	fn action(&self) -> Option<&Action<Self, Ctx>> {
		self.action.as_ref()
	}

	fn take_action(&mut self) -> Option<Action<Self, Ctx>> {
		self.action.take()
	}
}
