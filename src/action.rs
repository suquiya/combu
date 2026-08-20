use std::error::Error;

use crate::{command::Command, context::Context};

/// `Action` is type for command action (function). It returns `Result<ActionResult, ActionError>`
pub type Action = fn(Command, Context) -> Result<ActionResult, ActionError>;

/// `ActionResult` is the result of a command action.
#[derive(Debug)]
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

impl ActionError {
	/// Creates new `ActionError` with the given message and kind.
	pub fn new(message: String, kind: ActionErrorKind) -> Self {
		Self {
			message,
			kind,
			source: None,
		}
	}
	/// Creates new `ActionError` with the given message and kind, and source error.
	pub fn with_source(message: String, kind: ActionErrorKind, source: Box<dyn Error>) -> Self {
		Self {
			message,
			kind,
			source: Some(source),
		}
	}
}
