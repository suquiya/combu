use crate::{action_result, Command, Context, Vector};

#[derive(Clone, Default)]
/// Hook is struct for action needs a context before sub.run.
pub struct Hook {
	/// name
	pub name: String,
	/// action
	pub action: Option<HookAction>,
	/// authors
	pub authors: String,
	/// copyright
	pub copyright: String,
	/// license
	pub license: Option<(String, String)>,
	/// description
	pub description: Option<String>,
	/// usage
	pub usage: String,
	/// alias
	pub alias: Vector<String>,
}

/// Type for action of hook.
pub type HookAction = fn(&Command, Context) -> action_result!();

impl Hook {
	/// Creates new hook.
	pub fn new() -> Hook {
		Hook::default()
	}

	/// Creates new hook with name
	pub fn with_name<T: Into<String>>(name: T) -> Hook {
		Hook {
			name: name.into(),
			action: None,
			authors: String::default(),
			copyright: String::default(),
			license: None,
			description: None,
			usage: String::default(),
			alias: Vector::default(),
		}
	}

	/// Creates new hook with all field
	pub fn with_all_field(
		name: String,
		action: Option<HookAction>,
		authors: String,
		copyright: String,
		license: Option<(String, String)>,
		description: Option<String>,
		usage: String,
		alias: Vector<String>,
	) -> Hook {
		Hook {
			name,
			action,
			authors,
			copyright,
			license,
			description,
			usage,
			alias,
		}
	}

	/// Sets action
	pub fn action(mut self, action: HookAction) -> Self {
		self.action = Some(action);
		self
	}

	/// Sets name
	pub fn name<T: Into<String>>(mut self, name: T) -> Self {
		self.name = name.into();
		self
	}
}
