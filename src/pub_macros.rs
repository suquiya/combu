#[macro_export]
/// Utility macro to implement `From<$from>` for `$to` using `$variant` as the variant name.
macro_rules! from_impl_to_enum {
	($from: ty, $variant: ident, $to: ty) => {
		impl From<$from> for $to {
			fn from(value: $from) -> Self {
				Self::$variant(value)
			}
		}
	};
}
