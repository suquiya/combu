use std::fmt::Debug;

use crate::{
	Action,
	flag::{DefaultFlagValue, Flag, ParsableFlagValue},
};

/// The struct for command information store and command execution
/// This can be root and edge
/// コマンドの情報格納＆実行用構造体です。root（根）にもedge（葉）にもなりえます。
#[derive(Default, Debug)]
pub struct Command<FlagValue: ParsableFlagValue = DefaultFlagValue> {
	///Command name
	pub name: String,
	///Command action
	pub action: Option<Action>,
	///Command authors
	pub authors: String,
	///Command copyright
	pub copyright: String,
	/// License of command
	pub license: License,
	/// Command description
	pub description: Option<String>,
	///Command usage
	pub usage: String,
	///local flags of command
	pub l_flags: Vec<Flag<FlagValue>>,
	///common flags of command
	pub c_flags: Vec<Flag<FlagValue>>,
	///alias for command
	pub alias: Vec<String>,
	///Command version
	pub version: String,
	///container of sub-command
	pub sub: Vec<Command>,
}

/// `LicenseFunc` shows type alias for license function
pub type LicenseFunc = fn(command: &Command, context: &Context) -> String;

#[derive(Clone)]
/// `License` shows license information
#[derive(Default)]
pub struct License(pub Option<(String, LicenseFunc)>);

impl Debug for License {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		if let License(Some(val)) = self {
			write!(f, "Some({},Func for Output)", val.0)
		} else {
			write!(f, "None")
		}
	}
}

impl License {
	/// Creates new license information.
	pub fn new(inner: Option<(String, LicenseFunc)>) -> Self {
		License(inner)
	}
	/// Returns true if self has some license information.
	pub fn is_some(&self) -> bool {
		matches!(*self, License(Some(_)))
	}
	/// Returns true if self has no license information.
	pub fn is_none(&self) -> bool {
		matches!(*self, License(None))
	}
	/// Takes inner license information of self and returns.
	pub fn take(&mut self) -> Self {
		core::mem::take(self)
	}
	/// Returns (short) expression of license. If no license information(self is `License(None)`, returns None.
	pub fn expr(&self) -> Option<String> {
		match self {
			License(Some((expr, _))) => Some(expr.into()),
			License(None) => None,
		}
	}
	/// Returns (short) expression of license. If no license information(self is `License(None)`, this function may panic.
	pub fn expr_unwrap(&self) -> String {
		match self {
			License(Some((expr, _))) => expr.clone(),
			License(None) => panic!("called `License::expr_unwrap()` on a `None` value"),
		}
	}
	/// Returns (long) expression - detail of license.
	pub fn output(&self, cmd: &Command, ctx: &Context) -> Option<String> {
		match self {
			License(Some((_, outputter))) => Some(outputter(cmd, ctx)),
			License(None) => None,
		}
	}
	/// Returns (long) expression - detail of license. If self is `License(None)`, this function may panic.
	pub fn output_unwrap(&self, cmd: &Command, ctx: &Context) -> String {
		match self {
			License(Some((_, outputter))) => outputter(cmd, ctx),
			_ => panic!("called `License::expr_unwrap()` on a `None`value"),
		}
	}
	/// Returns function which outputs (long) expression (or detail of license) with wrapping Option.
	pub fn output_fn(&self) -> Option<LicenseFunc> {
		match self {
			License(Some((_, outputter))) => Some(*outputter),
			License(None) => None,
		}
	}
	/// Returns function of (long) expression (or detail of license). If self is License(None), this will panic.
	pub fn output_fn_unwrap(&self) -> LicenseFunc {
		match self {
			License(Some((_, outputter))) => *outputter,
			_ => panic!("called `License::output_fn_wrap` on a `None` value"),
		}
	}
	/// Returns unwrap inner
	pub fn unwrap(self) -> (String, LicenseFunc) {
		let License(inner) = self;
		inner.unwrap()
	}
}
