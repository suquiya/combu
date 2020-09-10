use crate::Context;
use std::error::Error;
use std::fmt;
//Action Type for Command
pub type Action = fn(Context) -> Result<ActionResult, ActionError>;

#[derive(Debug)]
pub enum ActionResult {
	Done,
	ShowHelpRequest(Context),
}

#[derive(Debug)]
pub struct ActionError {
	pub description: String,
	pub context: Context,
	pub related_error: Option<Box<dyn Error>>,
}

impl ActionError {
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

	pub fn without_related_error(description: String, context: Context) -> Self {
		Self {
			description,
			context,
			related_error: None,
		}
	}
}

impl fmt::Display for ActionError {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> fmt::Result {
		write!(f, "{}", self.description)
	}
}

impl Error for ActionError {}
