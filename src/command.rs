use std::fmt::Debug;

use crate::{
	Action,
	describer::Describer,
	opt::{Opt, OptValue, ParsableOpt, ParseValueError},
};

/// The struct for command information store and command execution
/// This can be root and edge
/// コマンドの情報格納＆実行用構造体です。root（根）にもedge（葉）にもなりえます。
#[derive(Default, Debug)]
pub struct Command<
	OptVal = OptValue,
	OptErr: std::error::Error = ParseValueError,
	OptionItem: ParsableOpt<OptVal, OptErr> = Opt<OptVal, OptErr>,
> {
	///Command name
	pub name: String,
	///Command action
	pub action: Option<Action>,
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
	pub l_options: Vec<OptionItem>,
	///common options of command
	pub c_options: Vec<OptionItem>,
	///alias for command
	pub alias: Vec<String>,
	///Command version
	pub version: String,
	///container of sub-command
	pub sub: Vec<Self>,

	// Marker for generic type parameters
	_marker_v: std::marker::PhantomData<(OptVal, OptErr)>,
}

impl<OptVal, OptErr: std::error::Error, OptionItem: ParsableOpt<OptVal, OptErr>>
	Command<OptVal, OptErr, OptionItem>
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
		action: Option<Action>,
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
		action: Option<Action>,
		usage: T,
		license: Option<Describer>,
		l_options: Vec<OptionItem>,
		c_options: Vec<OptionItem>,
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
