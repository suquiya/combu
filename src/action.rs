use std::error::Error;

/// `Action` is type for command action. It returns `Result<ActionResult, ActionError>`
pub type Action = Result<ActionResult, ActionError>;

/// `ActionResult` is the result of a command action.
pub enum ActionResult {
	/// Done shows that action is entirely done and not return any value
	Done,
}

/// `ActionError` is the error type for command actions.
#[derive(Debug)]
pub struct ActionError {
	/// `ActionError`'s message
	pub message: String,
	/// `ActionError`'s kind
	pub kind: ActionErrorKind,
	/// source error
	pub source: Option<Box<dyn Error>>,
}

/// `ActionErrorKind` is the kind of `ActionError`.
#[derive(Debug)]
pub enum ActionErrorKind {
	/// Unexpected error
	Unexpected,
	/// Shows that no action is registered to command
	NoActionRegistered,
	/// Shows None
	None,
	/// Custom error
	Custom,
}

impl std::fmt::Display for ActionError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", self.message)
	}
}

impl Error for ActionError {
	fn source(&self) -> Option<&(dyn Error + 'static)> {
		self.source.as_ref().map(|e| e.as_ref())
	}
}
