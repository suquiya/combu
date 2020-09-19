#![warn(missing_docs)]
#![forbid(unsafe_code)]
#![warn(rust_2018_idioms)]
#![warn(clippy::missing_docs_in_private_items)]

/*! combu is crate for creating cli */

/// action is a module about Action
pub mod action;
/// command is a module about command
pub mod command;

mod context;
/// flag is a module about flag
pub mod flag;
/// parser is a module about command args parser
pub mod parser;
/// vector is a moudle about vector
pub mod vector;

pub use action::{
	Action, ActionError, ActionResult, ActionResult::Done, ActionResult::ShowHelpRequest,
};
pub use command::Command;
pub use context::Context;
pub use flag::{Flag, FlagType, FlagValue};
pub use parser::Parser;
pub use vector::Vector;

/// Macros for combu
pub mod pub_macros;
