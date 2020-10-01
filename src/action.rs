use crate::{Context, Vector};
use std::{error::Error, fmt};

///Action is type for command action. It returns Result<ActionResult, ActionError>.
pub type Action = fn(Context) -> Result<ActionResult, ActionError>;

#[derive(Debug)]
///ActionResult stores result of action.
pub enum ActionResult {
	///Done shows that action is done.
	Done,
	/// ShowHelpRequest shows that action requested to show help.
	ShowHelpRequest(Context),
	/// ShowHelpReq shows that action requested to show relative's help.
	ShowOtherHelpReq(Context, usize, Vector<String>),
	/// Shows return Context and Action as result for parse and run.
	Result(Context, Action),
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

		//write!(f, "{}", description)
	}
}

impl Error for ActionError {}
