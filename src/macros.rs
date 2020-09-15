#[macro_export]
/// Checks context has help flag. If the context has help flag, return ShowHelpRequest.
macro_rules! check_help {
	($context:ident) => {
		if $context.is_help_flag_true() {
			return Ok($crate::ShowHelpRequest($context));
			}
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
