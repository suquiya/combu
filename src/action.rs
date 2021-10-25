use fmt::Debug;

use crate::{Command, Context};
use std::{error::Error, fmt};

///Action is type for command action. It returns Result<ActionResult, ActionError>.
pub type Action = fn(Context, Command) -> Result<ActionResult, ActionError>;

///ActionResult stores result of action.
pub enum ActionResult {
	///Done shows that action is done.
	Done,
	/// ParentActionRequest shows that action requested to show help.
	ParentActionRequest(Context, Command, Action),
	/// Shows return Context, reached Command and Action as result for parse and run.
	Result(Context, Command, Action),
	/// Custom result(can have Box including dyn Debug).
	Custom(Box<dyn Debug>),
}

impl ActionResult {
	/// Returns true if self is done.
	pub fn is_done(&self) -> bool {
		match self {
			ActionResult::Done => true,
			_ => false,
		}
	}
}

/// ActionError stores error of action.
#[derive(Debug)]
pub struct ActionError {
	/// ActionError's value
	pub value: String,
	/// ActionError's Kind
	pub kind: ActionErrorKind,
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
		context: Context,
		related_error: Option<Box<dyn Error>>,
	) -> Self {
		Self {
			value: value.into(),
			kind,
			context,
			related_error,
			printed: false,
		}
	}

	/// Creates new ActionError without (not action) error info.
	pub fn without_related_error(value: String, kind: ActionErrorKind, context: Context) -> Self {
		Self {
			value,
			kind,
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
