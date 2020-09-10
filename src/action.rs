use crate::Context;
use std::error::Error;
use std::fmt;

///Action is type for command action. It returns Result<ActionResult, ActionError>.
pub type Action = fn(Context) -> Result<ActionResult, ActionError>;

#[derive(Debug)]
///ActionResult stores result of action.
pub enum ActionResult {
	///Done shows that action is done.
	Done,
	///ShowHelpRequest shows that action requested to show help.
	ShowHelpRequest(Context),
}

/// ActionError stores error of action.
#[derive(Debug)]
pub struct ActionError {
	/// ActionError's description
	pub description: String,
	/// context is a field for storing context that error occured
	pub context: Context,
	/// If there is an error which is not ActionError, related_error can stores it.
	pub related_error: Option<Box<dyn Error>>,
}

impl ActionError {
	/// Creates new ActionError.
	pub fn new<T: Into<String>>(
		description: T,
		context: Context,
		related_error: Option<Box<dyn Error>>,
	) -> Self {
		Self {
			description: description.into(),
			context,
			related_error,
		}
	}

	/// Creates new ActionError without (not action) error info.
	pub fn without_related_error(description: String, context: Context) -> Self {
		Self {
			description,
			context,
			related_error: None,
		}
	}
}

impl fmt::Display for ActionError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}", self.description)
	}
}

impl Error for ActionError {}
