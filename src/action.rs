use std::{error::Error, fmt::Debug};

pub type Action<Cmd, Ctx> = fn(&mut Cmd, &Ctx) -> Result<ActionResult<Cmd, Ctx>, ActionError>;

/// `ActionResult` is the result of a command action.
///
/// 簡単のため、CmdとCtxにはトレイトによる制約は設けていません。
/// 実際には、CmdはRunnableCommandトレイト、CtxはStorageContextトレイトを実装している想定です(少なくともCombu内で使う場合は)。
/// For simplicity, there are no trait constraints on Cmd and Ctx.
/// In practice, Cmd is expected to implement the RunnableCommand trait, and Ctx is expected to implement the StorageContext trait (at least within Combu).
#[derive(Debug)]
pub enum ActionResult<Cmd, Ctx> {
	/// Done shows that action is entirely done and not return any value
	Done,
	/// `ParentActionRequest` shows that action is requesting to return to parent command and run its action.
	/// It is mainly used for sub-command to return to parent command and run its action.
	/// For example, `help` command. It returns to parent command and run its action to show help message.
	ParentActionRequest(Cmd, Ctx),
	/// Shows return `Context`, reached `Command` and `Action` as result for parse and run.
	/// By using this, it is possible to get parse result and run action without keeping command instances of tree in memory.
	Result(Cmd, Ctx),
	/// Custom result(can have Box including dyn Debug).
	Custom(Box<dyn Debug>),
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
	/// Shows other (includes custom)
	Other,
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
