#[macro_export]
/// Creates new Vector.
macro_rules! vector {
	() => {
		$crate::Vector(None)
	};
	($elem:expr; $n:expr)=>{
		$crate::Vector(Some(vec![$elem,$n]))
	};
	($($x:expr),+ $(,)?)=>{
		$crate::Vector(Some(vec![$($x),+]))
	}
}

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
		if $context.is_flag_true("version") {
			println!($context.version);
			return $crate::done!();
		}
	};
}

#[macro_export]
/// Checks context has authors flag. If the context has author flag, show authors and exit.
macro_rules! check_authors {
	($context:ident) => {
		if $context.is_flag_true("authors") {
			println!($context.authors);
			return $crate::done!();
		}
	};
}

#[macro_export]
/// Gets license file path of crate from cargo.toml.
macro_rules! crate_license_file {
	() => {
		env!("CARGO_PKG_LICENSE_FILE")
	};
}

#[macro_export]
/// Gets license of crate from cargo.toml.
macro_rules! crate_license {
	() => {
		env!("CARGO_PKG_LICENSE")
	};
}

#[macro_export]
/// Gets license information from cargo.toml.
macro_rules! include_license_file {
	() => {
		include_str!($crate: crate_license_file)
	};
}

#[macro_export]
/// Checks context has license flag. If the context has license flag, exec $license_func and return done.
macro_rules! check_license {
	($context:ident, $license_func:expr) => {
		if $context.is_flag_true("license") {
			$license_func;
			return done!();
		}
	};
}

#[macro_export]
/// Checks context has license flag. If the context has license flag, show authors and exit.
macro_rules! check_copyright {
	($context:ident) => {
		if $context.is_flag_true("copyright") {
			println!("{}", $context.copyright);
		}
	};
}

#[macro_export]
/// Checks context has values of the preset flags.
macro_rules! check_preset_flags {
	($context:ident) => {
		$crate::check_help($context)
		$crate::check_authors($context)
		$crate::check_version($context)
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
/// Simple Alias of Ok(ShowHelpRequest)
macro_rules! help_req {
	($context:expr) => {
		Ok($crate::ActionResult::ShowHelpRequest($context))
	};
}

#[macro_export]
/// Gets crate name from cargo.toml.
macro_rules! crate_name {
	() => {
		env!("CARGO_PKG_NAME")
	};
}

#[macro_export]
/// Gets crate's authors from cargo.toml.
macro_rules! crate_authors {
	() => {
		env!("CARGO_PKG_AUTHORS")
	};
}

#[macro_export]
///Gets crate's version from cargo.toml.
macro_rules! crate_version {
	() => {
		env!("CARGO_PKG_VERSION")
	};
}

#[macro_export]
///Gets crate's description from cargo.toml.
macro_rules! crate_description {
	() => {
		env!("CARGO_PKG_DESCRIPTION")
	};
}

#[macro_export]
/// Macro for convinience to create root command.
macro_rules! root_from_crate {
	() => {
		Command::with_base(
			$crate::crate_name!(),
			$crate::crate_authors!(),
			$crate::crate_version!(),
			$crate::crate_description!(),
			None
			),
	};
	($action:expr)=>{
		Command::with_base(
			$crate::crate_name!(),
			$crate::crate_authors!(),
			$crate::crate_version!(),
			$crate::crate_description!(),
			Some($action))
	}
}

#[macro_export]
/// Macro for preset root.
macro_rules! preset_root {
	() => {
		$crate::command::presets::root_with_base(
			$crate::crate_name!(),
			$crate::crate_authors!(),
			$crate::crate_version!(),
			$crate::crate_description!(),
			None
			),
	};
	($action:expr)=>{
		$crate::command::presets::root_with_base(
			$crate::crate_name!(),
			$crate::crate_authors!(),
			$crate::crate_version!(),
			$crate::crate_description!(),
			Some($action))
	}
}

#[macro_export]
/// Wrap with option if argument is not None.
macro_rules! option_wrap {
	() => {
		None
	};
	(None) => {
		None
	};
	($inner:expr) => {
		Some($inner)
	};
}

#[macro_export]
/// create cmd helper with full detail
macro_rules! cmd {
	(
		$name:expr=>
		[
			action=>$action:expr,
			authors=>$authors:expr,
			copyright=>$copyright:expr,
			license=>$license:expr,
			description=>$desc:expr,
			usage=>$usage:expr,
			local_flags=>$l_flags:expr,
			commcon_flags=>$c_flags:expr,
			alias=> $alias:expr,
			version=> $ver:expr,
			sub=> $sub:expr,
			help=> $help:expr
		]
	) => {
		Command::with_all_field(
			$name,
			$crate::option_wrap!($action),
			$authors,
			$copyright,
			$license,
			$desc,
			$usage,
			$l_flags,
			$c_flags,
			$alias,
			$ver,
			$sub,
			$help,
		)
	};
}

#[macro_export]
/// Creates function returns given string
macro_rules! string_fn {
	($string:expr) => {
		|_ctx: &Context| -> String { $string }
	};
	(into=>$str:expr) => {
		string_fn!($str.into())
	};
	(file_path=>$file_path:expr) => {
		string_fn!(include_str!($file_path))
	};
}

#[macro_export]
/// create license helper
macro_rules! license {
	(none) => {
		$crate::command::License(None)
	};
	($expr:expr,$fn:expr)=>{
		$crate::command::License(Some(($expr,$fn)))
	};
	($expr:expr,outputter=>$fn:expr)=>{
		license!(expr=>$expr,outputter=>$fn)
	};
	($expr:expr, content=>$content:expr) => {
		license!($expr, $crate::string_fn!($content))
	};
	($expr:expr, file_path=>$file_path:expr) => {
		license!($expr, $crate::string_fn!(file_path=>$file_path))
	};
	($expr:expr, $c:expr$(,)?)=>{
		license!($expr,$c)
	};
	($expr:expr, $i:ident=>$c:expr$(,)?)=>{
		license!($expr,$i=>$c)
	};
	(expr=>$expr:expr, $c:expr$(,)?)=>{
		license!($expr,$c)
	};
	(expr=>$expr:expr, $i:ident=>$c:expr$(,)?)=>{
		license!($expr,$i=>$c)
	}
}
