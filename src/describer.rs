use crate::{command::Command, context::Context};

#[derive(Clone, Debug)]
/// A struct that holds the description that has a short expression and a detail print function (ex. License).
/// In combu, this mainly holds the information about the command's license.
pub struct Describer {
	short_expr: String,
	detail_print: fn(command: &Command, context: &Context) -> String,
}

impl Describer {
	/// Creates a new `Describer` with the given short expression and detail print function.
	pub fn new(
		short_expr: String,
		detail_print: fn(command: &Command, context: &Context) -> String,
	) -> Self {
		Self {
			short_expr,
			detail_print,
		}
	}

	/// Returns the description of the command.
	pub fn describe(&self, command: &Command, context: &Context) -> String {
		(self.detail_print)(command, context)
	}

	/// Returns the short expression of the command.
	pub fn short_expr(&self) -> &str {
		&self.short_expr
	}
}
