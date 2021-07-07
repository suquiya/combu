#[macro_export]
/// Creates new Vector.
macro_rules! vector {
	() => {
		$crate::Vector(None)
	};
	(None) => {
		vector!(None)
	};
	($elem:expr; $n:expr)=>{
		$crate::Vector(Some(vec![$elem,$n]))
	};
	($($x:expr),+ $(,)?)=>{
		$crate::Vector(Some(vec![$($x),+]))
	}
}

#[macro_export]
/// Returns char from ident
macro_rules! char {
	(0) => {
		'0'
	};
	(1) => {
		'1'
	};
	(2) => {
		'2'
	};
	(3) => {
		'3'
	};
	(4) => {
		'4'
	};
	(5) => {
		'5'
	};
	(6) => {
		'6'
	};
	(7) => {
		'7'
	};
	(8) => {
		'8'
	};
	(9) => {
		'9'
	};
	(a) => {
		'a'
	};
	(b) => {
		'b'
	};
	(c) => {
		'c'
	};
	(d) => {
		'd'
	};
	(e) => {
		'e'
	};
	(f) => {
		'f'
	};
	(g) => {
		'g'
	};
	(h) => {
		'h'
	};
	(i) => {
		'i'
	};
	(j) => {
		'j'
	};
	(k) => {
		'k'
	};
	(l) => {
		'l'
	};
	(m) => {
		'm'
	};
	(n) => {
		'n'
	};
	(o) => {
		'o'
	};
	(p) => {
		'p'
	};
	(q) => {
		'q'
	};
	(r) => {
		'r'
	};
	(s) => {
		's'
	};
	(t) => {
		't'
	};
	(u) => {
		'u'
	};
	(v) => {
		'v'
	};
	(w) => {
		'w'
	};
	(x) => {
		'x'
	};
	(y) => {
		'y'
	};
	(z) => {
		'z'
	};
	(A) => {
		'A'
	};
	(B) => {
		'B'
	};
	(C) => {
		'C'
	};
	(D) => {
		'D'
	};
	(E) => {
		'E'
	};
	(F) => {
		'F'
	};
	(G) => {
		'G'
	};
	(H) => {
		'H'
	};
	(I) => {
		'I'
	};
	(J) => {
		'J'
	};
	(K) => {
		'K'
	};
	(L) => {
		'L'
	};
	(M) => {
		'M'
	};
	(N) => {
		'N'
	};
	(O) => {
		'O'
	};
	(P) => {
		'P'
	};
	(Q) => {
		'Q'
	};
	(R) => {
		'R'
	};
	(S) => {
		'S'
	};
	(T) => {
		'T'
	};
	(U) => {
		'U'
	};
	(V) => {
		'V'
	};
	(W) => {
		'W'
	};
	(X) => {
		'X'
	};
	(Y) => {
		'Y'
	};
	(Z) => {
		'Z'
	};
	(+)=>{
		'+'
	};
	(*)=>{
		'*'
	};
	(@)=>{
		'@'
	};
	(%)=>{
		'%'
	};
	(&)=>{
		'&'
	};
	($)=>{
		'$'
	};
	(#)=>{
		'#'
	};
	(!)=>{
		'!'
	};
	(-)=>{
		'-'
	};
	(=)=>{
		'='
	};
	(^)=>{
		'^'
	};
	(~) => {
		'~'
	};
	(|)=>{
		'|'
	};
	(:) => {
		':'
	};
	(;) => {
		';'
	};
	(_) => {
		'_'
	};
	(?) => {
		'?'
	};
	(<) => {
		'<'
	};
	(>) => {
		'>'
	};
	(.) => {
		'.'
	};
	(,) => {
		','
	};
	($expr:expr)=>{
		$expr
	};
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
	($inner:literal) => {
		Some($inner.into())
	};
	($inner:expr) => {
		Some($inner)
	};
}

#[macro_export]
/// create cmd helper with full detail
macro_rules! cmd {
	($name:ident=>$t:tt)=>{
		cmd!(stringify!($name)=>$t)
	};
	($name:ident=:$t:tt)=>{
		cmd!(stringify!($name)=>$t)
	};
	($name:ident$sep:tt$t:tt)=>{
		cmd!(stringify!($name)=>$t)
	};
	($name:literal$sep:tt$t:tt)=>{
		cmd!($name.into()=>$t)
	};
	(
		$name:expr=>
		[
			$(=)?$action:expr,
			<$authors:expr,
			@$copyright:expr,
			#$license:tt,
			$(=)+$desc:expr,
			~$usage:expr,
			l#$l_flags:expr,
			c#$c_flags:expr,
			&$alias:expr,
			n $ver:expr,
			| $sub:expr,
			?$help:expr $(,)?
		]
	) => {
		Command::with_all_field(
			String::from($name),
			$crate::option_wrap!($action),
			$authors.into(),
			$copyright.into(),
			$crate::license!$license,
			$crate::option_wrap!($desc),
			$usage.into(),
			$l_flags,
			$c_flags,
			$alias,
			$ver.into(),
			$sub,
			$crate::option_wrap!($help),
		)
	};

}

#[macro_export]
/// Helps for creating flag*s*.
macro_rules! flags {
	($($flag_arg:tt),* $(,)?) => {
		$crate::vector![$($crate::flag!$flag_arg),*]
	};
	($($flag_name:ident$flag_arg:tt),* $(,)?)=>{
		flags!($([$flag_name$flag_arg]),*);
	};
	($flag_name:ident$flag_arg:tt,$($t:tt)+)=>{
		flags!($flag_name=>$flag_arg,$($t)+)
	};
	($($flag_name:ident$sep:tt$flag_arg:tt),* $(,)?)=>{
		flags!($([$flag_name=>$flag_arg]),*);
	};
}

#[macro_export]
/// Helps for creating flag.
macro_rules! flag {
	($name:ident=>[$type:ident,$(-$s:ident),*,$(--$long:ident),*, =$description:expr,@$default:expr]) => {
		Flag::with_all_field(
			String::from(stringify!($name)),
			String::from($description),
			$crate::vector![$($crate::char!($s)),*],
			$crate::vector![$(stringify!($long).to_owned()),*],
			$crate::flag_type!($type),
			$crate::flag_value!($type, $default),
		);
	};
	($(@)?$name:ident=>[$type:ident$(,)?$(-$s:ident),*$(,)?$(--$long:ident),*, =$description:expr,$(@)?$default:expr]) => {
		flag!($name=>[$type,$(-$s),*,$(--$long),*, =$description, @$default])
	};
	($(@)?$name:ident=>[$type:ident$(,)?$(-$s:ident),*$(,)?$(--$long:ident),*, =$description:expr,$(?)?$default:expr]) => {
		flag!($name=>[$type,$(-$s),*,$(--$long),*, =$description, @$default])
	};
	($(@)?$name:ident=>{$($t:tt)+})=>{
		flag!($name=>[$($t)+])
	};
	($(@)?$name:ident=>($($t:tt)+)) => {
		flag!($name=>[$($t)+])
	};
	($(@)?$name:ident:$args:tt)=>{
		flag!($name=>$args)
	};
	($(@)?$name:ident=$args:tt)=>{
		flag!($name=>$args)
	};
	($(@)?$name:ident$args:tt)=>{
		flag!($name=>$args)
	}
}

#[macro_export]
/// Gets FlagType from keyword
macro_rules! flag_type {
	(bool) => {
		$crate::flag_type!(Bool)
	};
	(b) => {
		flag_type!(b)
	};
	(int) => {
		$crate::flag_type!(Int)
	};
	(i) => {
		flag_type!(int)
	};
	(Integer) => {
		flag_type!(int)
	};
	(integer) => {
		flag_type!(int)
	};
	(float) => {
		$crate::flag_type!(Float)
	};
	(f) => {
		flag_type!(f)
	};
	(string) => {
		$crate::flag_type!(String)
	};
	(str) => {
		flag_type!(string)
	};
	(Str) => {
		flag_type!(string)
	};
	($i:ident) => {
		FlagType::$i
	};
}

#[macro_export]
/// Creates flag_value
macro_rules! flag_value {
	(bool,$val:expr) => {
		$crate::FlagValue::Bool($val)
	};
	(b, $val:expr) => {
		flag_value!(bool, $val)
	};
	(Bool,$val:expr) => {
		flag_value!(bool, $val)
	};
	(true) => {
		flag_value!(bool, true)
	};
	(false) => {
		flag_value!(bool, false)
	};
	(int, $val:expr) => {
		$crate::FlagValue::Int($val)
	};
	(i, $val:expr) => {
		flag_value!(int, $val)
	};
	(Int, $val:expr) => {
		flag_value!(int, $val)
	};
	(Integer, $val:expr) => {
		flag_value!(int, $val)
	};
	(integer, $val:expr) => {
		flag_value!(int, $val)
	};
	(float, $val:expr) => {
		$crate::FlagValue::Float($val)
	};
	(f,$val:expr) => {
		flag_value!(float, $val)
	};
	(Float, $val:expr) => {
		flag_value!(float, $val)
	};
	(str, $val:expr) => {
		$crate::FlagValue::String($val)
	};
	(s,$val:expr) => {
		flag_value!(str, $val)
	};
	(Str, $val:expr) => {
		flag_value!(str, $val)
	};
	(String, $val:expr) => {
		flag_value!(str, $val)
	};
	(string, $val:expr) => {
		flag_value!(str, $val)
	};
}

#[macro_export]
/// Creates function returns given string
macro_rules! string_fn {
	($string:literal) => {
		$crate::string_fn!($string.to_owned())
	};
	($string:expr) => {
		|_ctx: &$crate::Context| -> String { $string }
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
	($expr:literal, $fn:expr)=>{
		$crate::license!($expr.into(),$fn)
	};
	($expr:expr,$fn:expr)=>{
		$crate::command::License(Some(($expr,$fn)))
	};
	($expr:expr,outputter=>$fn:expr)=>{
		license!($expr,$fn)
	};
	($expr:expr, content=>$content:expr) => {
		$crate::license!($expr, $crate::string_fn!($content))
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
