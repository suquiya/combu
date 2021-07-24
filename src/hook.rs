// Hookをするとややこしくなりそうなので却下
use std::collections::VecDeque;

use crate::{ActionError, ActionResult, Command, Context, Vector};

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
pub type HookAction = fn(&mut Command, HookInfo) -> (Result<ActionResult, ActionError>, HookInfo);

/// Enum for hookAction arg.
pub enum HookInfo {
	/// shows Context given.
	Context(Context),
	/// shows args(args,inter_mediate_args) given.
	Args(VecDeque<String>, Option<Vector<crate::parser::MiddleArg>>),
}

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
}

macro_rules! gen_doc {
	($field:ident) => {
		concat!("Sets ", stringify!($field), ".")
	};
}

macro_rules! setter {
	($field:ident)=>{
		setter!($field, gen_doc!($field));
	};
	($field:ident, $fn_doc:expr) => {
		#[doc=$fn_doc]
		pub fn $field<T: Into<String>>(mut self, $field: T) -> Self {
			self.$field = $field.into();
			self
		}
	};
	($field:ident :Option String)=>{
		setter!($field,gen_doc!($field));
	};
	($field:ident :Option String, $fn_doc:expr)=>{
		#[doc=$fn_doc]
		pub fn $field<T:Into<String>>(mut self, $field: T)->Self{
			self.$field = Some($field.into());
			self
		}
	};
	($field:ident :Option $t:ty)=>{
		setter!($field :Option $t, gen_doc!($field));
	};
	($field:ident :Option $t:ty, $fn_doc:expr)=>{
		#[doc=$fn_doc]
		pub fn $field(mut self, $field: $t)->Self{
			self.$field = Some($field);
			self
		}
	};
	($($field:ident),+)=>{
		$(setter!($field);)+
	};
	($($field:ident :Option $t:ty),+)=>{
		$(setter!($field :Option $t);)+
	}
}

impl Hook {
	setter!(name, authors, copyright, usage);
	setter!(action:Option HookAction,license: Option (String, String),description:Option String);

	/// Add alias to this hook.
	pub fn alias<T: Into<String>>(mut self, alias: T) -> Self {
		self.alias.push(alias.into());
		self
	}

	/// Beturns true if name_or_alias matches command's name or one of alias at leaast.
	/// name_or_aliasがコマンド名がエイリアスのうち少なくとも一つにマッチした場合trueを返す
	pub fn is(&self, name_or_alias: &str) -> bool {
		if name_or_alias == self.name {
			true
		} else {
			match self.alias.inner() {
				None => false,
				Some(inner) => inner.iter().any(|a| a == name_or_alias),
			}
		}
	}
}
