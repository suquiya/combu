#[macro_export]
/// Checks context has help flag. If the context has help flag, return ShowHelpRequest.
macro_rules! check_help {
	($context:ident) => {
		if $context.is_flag_true("help") {
			return Ok($crate::ShowHelpRequest($context));
			}
	};
}

#[macro_export]
/// Checks context has version flag. If the context has help flag, show version and exit.
macro_rules! check_version {
	($context:ident) => {
		if $context.is_flag_true("version") {}
	};
}

#[macro_export]
/// Checks context has authors flag. If the context has author flag, show authors and exit.
macro_rules! check_authors {
	($context:ident) => {
		if $context.is_flag_true("authors") {}
	};
}

#[macro_export]
/// Checks context has license flag. If the context has license flag, show authors and exit.
macro_rules! check_license {
	($context:ident) => {
		if $context.is_flag_true("license") {}
	};
}

#[macro_export]
/// Checks context has license flag. If the context has license flag, show authors and exit.
macro_rules! check_copyright {
	($context:ident) => {
		if $context.is_flag_true("copyright") {}
	};
}

#[macro_export]
/// Checks context has values of the preset flags.
macro_rules! check_preset_flags {
	($context:ident) => {
		$crate::check_help($context)
	};
}

#[macro_export]
/// action_result type annotation
macro_rules! action_result {
	() => {
		Result<$crate::ActionResult,$crate::ActionError>
	};
}

#[macro_export]
/// Simple Alias of Ok(Done)
macro_rules! done {
	() => {
		Ok($crate::ActionResult::Done)
	};
}

#[macro_export]
/// Macro for convinience to create root command.
macro_rules! root_from_crate {
	() => {
		Command::with_base(
			env!("CARGO_PKG_NAME"),
			env!("CARGO_PKG_AUTHORS"),
			env!("CARGO_PKG_VERSION"),
			env!("CARGO_PKG_DESCRIPTION"),
			None
			),
	};
	($action:ident)=>{
		Command::with_base(env!("CARGO_PKG_NAME"),
			env!("CARGO_PKG_AUTHORS"),
			env!("CARGO_PKG_VERSION"),
			env!("CARGO_PKG_DESCRIPTION"),
			Some($action))
	}
}

#[macro_export]
/// Macro for preset root.
macro_rules! preset_root {
	() => {
		Command::with_base(
			env!("CARGO_PKG_NAME"),
			env!("CARGO_PKG_AUTHORS"),
			env!("CARGO_PKG_VERSION"),
			env!("CARGO_PKG_DESCRIPTION"),
			None
			),
	};
	($action:ident)=>{
		Command::with_base(env!("CARGO_PKG_NAME"),
			env!("CARGO_PKG_AUTHORS"),
			env!("CARGO_PKG_VERSION"),
			env!("CARGO_PKG_DESCRIPTION"),
			Some($action))
	}
}

#[macro_export]
/// Takes string inner.
macro_rules! take_string {
	($var:ident) => {{
		let mut empty = String::new();
		std::mem::swap($var, &mut empty);
		empty
		}};
	($var:expr) => {{
		let mut empty = String::new();
		std::mem::swap($var, &mut empty);
		empty
		}};
}
