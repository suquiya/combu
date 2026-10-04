#[macro_export]
/// Type alias for `Result<ActionResult, ActionError>`.
macro_rules! action_result {
    ($cmd:ty, $ctx:ty) => {
        Result<$crate::action::ActionResult<$cmd, $ctx>, $crate::action::ActionError>
    };
}

#[macro_export]
/// Short alias for Ok(ActionResult::Done)
macro_rules! done {
	() => {
		Ok($crate::action::ActionResult::Done)
	};
}

#[macro_export]
/// Utility macro to implement `From<$from>` for `$to` using `$variant` as the variant name.
///
/// Example:
/// ```
/// from_impl_to_enum!(
///     OptValue,
///     (i64, Int),
///     (f64, Float),
///     (String, String),
/// )
/// ```
macro_rules! from_impl_to_enum {
	(=>$to:ty, $from: ty, $variant: ident) => {
		$crate::from_impl_to_enum!($from, $variant, $to);
	};
	($from: ty, $variant: ident, $to: ty) => {
		impl From<$from> for $to {
			fn from(value: $from) -> Self {
				Self::$variant(value)
			}
		}
	};
	($to:ty, $([$from: ty, $variant: ident]),+) => {
		$(
			$crate::from_impl_to_enum!($from, $variant, $to);
		)*
	};
}

// TODO: implement macro for generating OptValue enum and its From impls for customize option values.
