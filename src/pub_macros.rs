#[macro_export]
/// Creates new Vector.
macro_rules! vector {
	(:$ptype:ty,$($t:tt)*)=>{
		vector!($($t)*;:$ptype)
	};
	(=>$ptype:ty,$($t:tt)*)=>{
		vector!($($t)*;=>$ptype)
	};
	($($(;)?$(:)?$type:ty$(,)?)?) => {
		$crate::Vector$(::<$type>)?(None)
	};
	(None$($(;)?$(:)?$type:ty)?) => {
		$crate::Vector$(::<$type>)?(None)
	};
	($elem:expr; $n:expr$(;$(:)?$type:ty)?)=>{
		$crate::Vector$(::<$type>)?(Some(vec![$elem,$n]))
	};
	($elem:expr; $n:expr;$(=>$type:ty)?)=>{
		$crate::Vector$(::<$type>)?(Some(vec![$elem.into(),$n]))
	};
	($($x:expr),+ $(,)?$(;$(:)?$type:ty)?)=>{
		$crate::Vector$(::<$type>)?(Some(vec![$($x),+]))
	};
	($($x:expr),+ $(,)?;$(=>$type:ty)?)=>{
		$crate::Vector$(::<$type>)?(Some(vec![$($x.into()),+]))
	};
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
	($name:literal$sep:tt$t:tt)=>{
		cmd!($name.into()=>$t)
	};
	($name:expr=>{$($t:tt)+})=>{
		cmd!($name=>[$($t)+])
	};
	($name:expr=>($($t:tt)+))=>{
		cmd!($name=>[$($t)+])
	};
	($name:expr=>[$(=)?$action:expr,<...,$($t:tt)+])=>{
		cmd!($name=>[=$action,<$crate::crate_authors!(),$($t)+])
	};
	($name:expr=>[$(=)?$action:expr,<from_crate,$($t:tt)+])=>{
		cmd!($name=>[=$action,<$crate::crate_authors!(),$($t)+])
	};
	(
		$name:expr=>
		[
			$(=)?$action:expr,
			<$authors:expr,
			@$copyright:tt,
			+$license:tt,
			$(=)+$desc:expr,
			:$usage:expr,
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
			$crate::copyright!($copyright),
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
/// Create copyright macro
macro_rules! copyright {
	($raw_str:literal) => {
		$raw_str.to_string()
	};
	(...) => {
		$crate::copyright!("Copyright (c) ", "", $crate::crate_authors!())
	};
	({$($t:tt)+})=>{
		copyright!([$($t:tt)+])
	};
	(($($t:tt)+))=>{
		copyright!([$($t:tt)+])
	};
	([$year:expr]) => {
		$crate::copyright!($year, $crate::crate_authors!())
	};
	([$year:expr,$holder:ident]) => {
		$crate::copyright!($year, stringify!($holder))
	};
	([$year:expr,$holder:expr]) => {
		copyright!($year, $holder)
	};
	($year:expr,$holder:ident) => {
		copyright!($year, stringify!($holder))
	};
	($year:expr,$holder:expr) => {
		$crate::copyright!("Copyright (c)", $year, $holder)
	};
	([$prefix:expr, $year:expr,$holder:ident]) => {
		$crate::copyright!($prefix, $year, stringify!($holder))
	};
	([$prefix:expr, $year:expr,$holder:expr]) => {
		$crate::copyright!($prefix, $year, $holder)
	};
	($prefix:expr, $year:expr,$holder:ident) => {
		copyright!($prefix, $year, stringify!($holder))
	};
	($prefix:expr, $year:expr,$holder:expr) => {
		concat!($prefix, " ", $year, " ", $holder).to_owned()
	};
}

#[macro_export]
/// Helps for creating flag*s*.
macro_rules! flags {
	($($flag_arg:tt),* $(,)?) => {
		$crate::vector![$($crate::flag!$flag_arg),*]
	};
	($($flag_name:ident$flag_arg:tt),* $(,)?)=>{
		flags!($([$flag_name:$flag_arg]),*);
	};
	($flag_name:ident$flag_arg:tt,$($t:tt)+)=>{
		flags!($flag_name=>$flag_arg,$($t)+)
	};
	($($flag_name:ident$sep:tt$flag_arg:tt),* $(,)?)=>{
		flags!($([$flag_name=>$flag_arg]),*);
	};
}

#[macro_export]
/// string_from macro. based on Strong::from, but no arg can generate String::new()
macro_rules! string_from {
	() => {
		String::new()
	};
	(&$from:expr)=>{
		$from
	};
	($from:expr)=>{
		String::from($from)
	};
	(&$($from_tt:tt)+)=>{
		String::from($($from_tt)+)
	};
	($($from_tt:tt)+)=>{
		String::from($($from_tt)+)
	};
}

#[macro_export]
/// Helps for creating flag.
macro_rules! flag {
	(@$($t:tt)*) => {
		$crate::_fsp!(@$($t)*)
	};
	(&$($t:tt)+) => {
		$crate::_fsp!(&$($t)+)
	};
	($($t:tt)*)=>{
		$crate::_ffp!($($t)*)
	};
}

#[macro_export]
/// flag basic constructor
macro_rules! _flag_basic_constructor {
	(->$name:expr=>[
		$(=)?$description:expr,
		$(s#)?$short_alias:expr,
		$(l#)?$long_alias:expr,
		$(>)?$type:expr,
		$(?)?$default:expr$(,)?]) => {
		$crate::Flag::with_all_field(
			$name,
			$description,
			$short_alias,
			$long_alias,
			$type,
			$default,
		)
	};
	($name:expr,$description:expr,$short_alias:expr,$long_alias:expr,$type:expr,$default:expr) => {
		$crate::Flag::with_all_field(
			$name,
			$description,
			$short_alias,
			$long_alias,
			$type,
			$default,
		)
	};
}

#[macro_export]
/// inner for first parse name and tify little after name.
macro_rules! _ffp {
	(->$name:ident$t:tt)=>{
 		$crate::_fsp!(->$name=>$t)
 	};
	(->[$name:expr]$t:tt)=>{
	 	$crate::_fsp!(->[$name]=>$t)?
	};
	(->[$name:expr]$sep:tt$($t:tt)+)=>{
	 	$crate::_fsp!(->[$name]=>$($t)+)
	};
	(->$name:expr=>$($t:tt)*)=>{
		$crate::_fsp!(->$name=>$($t)*)
	};
	(->$name:ident$sep:tt$t:tt)=>{
		$crate::_fsp!(->$name=>$t)
	};
	([$($t:tt)+]$($ta:tt)?)=>{
		$crate::_fsp!([$($t)+]$(=>$ta)?)
	};
	([$($t:tt)+]$sep:tt$($ta:tt)+)=>{
		$crate::_fsp!([$($t)+]=>$($ta)+)
	};
	($name:ident$t:tt)=>{
		$crate::_ffp!($name=>$t)
	};
	($name:ident$sep:tt$t:tt)=>{
		$crate::_fsp!($name=>$t)
	};
	($name:ident)=>{
		$crate::_fsp!($name)
	};
	($name:expr$(=>$t:tt)?)=>{
		$crate::_fsp!($name=>$($t)?)
	};
	($name:ident$sep:tt$($t:tt)+)=>{
		$crate::_fsp!($name=>$($t)+)
	};
	($name:expr=>$($t:tt)*)=>{
		$crate::_fsp!($name=>$($t)*)
	};
	(->[$name:expr])=>{
	 	$crate::_fsp!(->[$name])
	};
	(->$name:expr)=>{
		$crate::_fsp!(->$name)
	}
}

#[macro_export]
/// macro for innser flag
macro_rules! _fsp {
	(@$($t:tt)*)=>{
		$crate::flag!($($t)*)
	};
	(&$($t:tt)*)=>{
		$crate::flag!(->$($t)*)
	};
	($name:ident=>$($t:tt)*)=>{
		$crate::_fsp!(stringify!($name)=>$($t)*)
	};
	($name:ident)=>{
		$crate::_fsp!(stringify!($name))
	};
	([$($nt:tt)*]$(=>$t:tt)*)=>{
		$crate::_fsp!($($nt)* =>$($t)*)
	};
	($name:expr=>$($t:tt)*)=>{
		$crate::_fsp!(->$crate::string_from!($name)=>$($t)*)
	};
	($name:expr)=>{
		$crate::_fsp!(->$crate::string_from!($name))
	};
	(->[$name:expr]=>$($t:tt)*)=>{
		$crate::_fsp!(->$name=>$($t)*)
	};
	(->$name:expr=>$($t:tt)*)=>{
		$crate::_ftp!(->$name=>$($t)*)
	};
	(->[$name:expr])=>{
		$crate::_fsp!(->$name)
	};
	(->$name:expr)=>{
		$crate::_fsp!(->$name=>)
	};
}

#[macro_export]
/// macro for innser flag
macro_rules! _ftp {
	(->$name:expr=>)=>{
		$crate::_ftp!(->$name=>[])
	};
	(->$name:expr=>{$($t:tt)*})=>{
		$crate::_ftp![$($t)*]
	};
	(->$name:expr=>($($t:tt)*))=>{
		$crate::_ftp![$($t)*]
	};
	(->$name:expr=>[]) => {
		$crate::_ftp!(->$name=>[String::default(),Vector::default(),Vector::default(),FlagType::default(),FlagValue::String(String::default())])
	};
	(->$name:expr=>[$(>)?$type:ident,$(=)?$description:expr,-$sf:ident$($(,)?$(-)?$st:ident)+$(,)?--$($t:tt)+])=>{
		$crate::_ftp!(->$name=>[$type,$description,[$sf$($st)+],--$($t)+])
	};
	(->$name:expr=>[$(>)?$type:ident,$(=)?$description:expr,-$sf:ident$($(,)?$(-)?$st:ident)+$(,)?[$($t:tt)*]$($t2:tt)*])=>{
		$crate::_ftp!(->$name=>[$type,$description,[$sf$($st)+],[$($t)*]$($t2)*])
	};
	(->$name:expr=>[$(>)?$type:ident,$(=)?$description:expr,-$sf:ident$($(,)?$(-)?$st:ident)+$(,)?l#$($t:tt)+])=>{
		$crate::_ftp!(->$name=>[$type,$description,[$sf$($st)+],l#$($t)+])
	};
	(->$name:expr=>[$(>)?$type:ident,$(=)?$description:expr,-$sf:ident$($(,)?$(-)?$st:ident)+$(,)??$default:expr])=>{
		$crate::_ftp!(->$name=>[$type,$description,[$sf$($st)+],[],?$default])
	};
	(->$name:expr=>[$(>)?$type:ident,$(=)?$description:expr,$($(-)?$s:ident)*$(,$($t:tt)*)?])=>{
		$crate::_ftp!(->$name=>[$type,$description,[$($s)*]$(,$($t)*)?])
	};
	(->$name:expr=>[$(>)?$type:ident,$(=)?$description:expr,$($(-)?$s:ident)*?$($t:tt)*])=>{
		$crate::_ftp!(->$name=>[$type,$description,[$($s)*]?$($t)*])
	};
	(->$name:expr=>[$(>)?$type:ident,$(=)?$description:expr,$($(-)?$s:ident)*--$($t:tt)*])=>{
		$crate::_ftp!(->$name=>[$type,$description,[$($s)*],--$($t)*])
	};
	(->$name:expr=>[$(>)?$type:ident,$(=)?$description:expr,$(s#)?$(-)?[$($st:tt)*]$($t:tt)*])=>{
		$crate::_ftp!(->$name=>[$type,$description,$crate::short_alias![$($st)*]$($t)*])
	};
	(->$name:expr=>[$(>)?$type:ident,$(=)?$description:expr,$(s#)?$short_alias:expr$(,)?])=>{
		$crate::_ftp!(->$name=>[$crate::string_from!($description),$short_alias,$crate::long_alias![],>$crate::flag_type!($type),?$crate::flag_value!($type)])
	};
	(->$name:expr=>[$(>)?$type:ident,$(=)?$description:expr,$short_alias:expr,--$l:ident$($(,)?$(--)?$lt:ident)*])=>{
		$crate::_ftp!(->$name=>[$type,$description,$short_alias,$crate::long_alias![$l$($lt)*]])
	};
	(->$name:expr=>[$(>)?$type:ident,$(=)?$description:expr,$short_alias:expr,--$l:ident$($(,)?$(--)?$lt:ident)*$(,)??$default:expr])=>{
		$crate::_ftp!(->$name=>[$type,$description,$short_alias,$crate::long_alias![$l$($lt)*],?$default])
	};
	(->$name:expr=>[$(>)?$type:ident,$(=)?$description:expr,$short_alias:expr,$($(--)?$l:ident)*$(,$($t:tt)*)?])=>{
		$crate::_ftp!(->$name=>[$type,$description,$short_alias,$crate::long_alias![$($l)*]$(,$($t)*)?])
	};
	(->$name:expr=>[$(>)?$type:ident,$(=)?$description:expr,$short_alias:expr,$($(--)?$l:ident)*,$($t:tt)*])=>{
		$crate::_ftp!(->$name=>[$type,$description,$short_alias,$crate::long_alias![$($l)*],$($t)*])
	};
	(->$name:expr=>[$(>)?$type:ident,$(=)?$description:expr,$(s#)?$short_alias:expr,$(l#)?$(--)?[$($lt:tt)*]$($t:tt)*])=>{
		$crate::_ftp!(->$name=>[$type,$description,$short_alias,$crate::long_alias![$($lt)*]$($t)*])
	};
	(->$name:expr=>[$(>)?$type:ident,$(=)?$description:expr,$(s#)?$short_alias:expr,?$default:expr])=>{
		$crate::_ftp!(->$name=>[$type,$description,$short_alias,[],?$default])
	};
	(->$name:expr=>[$(>)?$type:ident,$(=)?$description:expr,$(s#)?$short_alias:expr,$(l#)?$long_alias:expr,$($(?)?$default:expr)?])=>{
		$crate::_ftp!(->$name=>[$crate::string_from!($description),$short_alias,$long_alias,>$crate::flag_type!($type),?$crate::flag_value!($type,$($default)?)])
	};
	(->$name:expr=>[$(>)?$type:ident,$(=)?$description:expr,$(s#)?$short_alias:expr,$(l#)?$long_alias:expr])=>{
		$crate::_ftp!(->$name=>[$crate::string_from!($description),$short_alias,$long_alias,>$crate::flag_type!($type),$crate::flag_value!($type)])
	};
	(->$name:expr=>[$(>)?$type:ident,$(=)?$description:expr,$(s#)?$short_alias:expr$(,$(?)?$default:expr)?])=>{
		$crate::_ftp!(->$name=>[$crate::string_from!($description),$short_alias,$crate::long_alias[],>$crate::flag_type!($type),?$crate::flag_value!($type,$($default)?)])
	};
	(->$name:expr=>[$(=)?$description:expr,
		$(s#)?$short_alias:expr,
		$(l#)?$long_alias:expr,
		$(>)?$type:expr,
		$(?)?$default:expr$(,)?])=>{
			$crate::_flag_basic_constructor!(->$name=>[$description,$short_alias,$long_alias,$type,$default])
		};
}

#[macro_export]
/// short_alias_expander
macro_rules! short_alias {
	() => {
		short_alias!(None)
	};
	(None) => {
		$crate::vector!(None;char)
	};
	(-[$($t:tt)*])=>{
		$crate::short_alias!($($t)*)
	};
	($($s:ident)+)=>{
		$crate::vector![$($crate::char![$s]),+;:char]
	};
	($(-$s:ident),+$(,)?)=>{
		$crate::short_alias![$($s)+]
	};
	($($(-)?$s:ident)+$(,)?)=>{
		$crate::short_alias![$($s)+]
	};
	($($(-)?$s:ident),+$(,)?)=>{
		$crate::short_alias![$($s)+]
	};

	($($s:literal),+)=>{
		$crate::vector![$($s),+;:char]
	};
	(->$s:expr)=>{
		$s
	};
}

#[macro_export]
/// long_alias_expander
macro_rules! long_alias {
	()=>{
		$crate::long_alias!(None)
	};
	(None) => {
		$crate::vector!(None;String)
	};
	(--[$($t:tt)*])=>{
		long_alias!($($t)*)
	};
	($($l:ident)+)=>{
		$crate::long_alias!($(stringify!($l)),+)
	};
	($(--$l:ident),+$(,)?)=>{
		$crate::long_alias!($($l)+)
	};
	($(--$l:ident)+$(,)?)=>{
		$crate::long_alias!($($l)+)
	};
	($($(--)?$l:ident),+$(,)?)=>{
		$crate::long_alias!($($l)+)
	};
	($($(--)?$l:ident)+$(,)?)=>{
		$crate::long_alias!($($l)+)
	};
	($($(--)?$l:literal)+$(,)?)=>{
		$crate::long_alias!($($l),+)
	};
	($($(--)?$l:literal),+$(,)?)=>{
		$crate::long_alias!($($l),+)
	};
	($($l:expr),+)=>{
		$crate::vector!($($l),+;=>String)
	};
	($long:expr)=>{
		$crate::long_alias!(->$long)
	};
	(->$long:expr)=>{
		$long
	}
}

#[macro_export]
/// sub macro for flag
macro_rules! _flag_one_ident {
	(->$name:expr=>[bool])=>{
		$crate::flag!($name=>[>bool])
	};
	(->$name:expr=>[b])=>{
		$crate::flag!($name=>[>b])
	};
	(->$name:expr=>[B])=>{
		$crate::flag!($name=>[>B])
	};
	(->$name:expr=>[Bool])=>{
		$crate::flag!($name=>[>Bool])
	};
	(->$name:expr=>[int])=>{
		$crate::flag!($name=>[>int])
	};
	(->$name:expr=>[i])=>{
		$crate::flag!($name=>[>i])
	};
	(->$name:expr=>[Integer])=>{
		$crate::flag!($name=>[>Integer])
	};
	(->$name:expr=>[I])=>{
		$crate::flag!($name=>[>I])
	};
	(->$name:expr=>[integer])=>{
		$crate::flag!($name=>[>integer])
	};
	(->$name:expr=>[Int])=>{
		$crate::flag!($name=>[>Int])
	};
	(->$name:expr=>[>$type:ident])=>{
		$crate::flag!($name=>[$type,""])
	};
	(->$name:expr=>[float])=>{
		$crate::flag!($name=>[>float])
	};
	(->$name:expr=>[f])=>{
		$crate::flag!($name=>[>f])
	};
	(->$name:expr=>[F])=>{
		$crate::flag!($name=>[>F])
	};
	(->$name:expr=>[Float])=>{
		$crate::flag!($name=>[>Float])
	};
	(->$name:expr=>[Str])=>{
		$crate::flag!($name=>[>Str])
	};
	(->$name:expr=>[string])=>{
		$crate::flag!($name=>[>string])
	};
	(->$name:expr=>[String])=>{
		$crate::flag!($name=>[>String])
	};
	(->$name:expr=>[$description:ident])=>{
		$crate::flag!($name=>[str,$description])
	}
}

#[macro_export]
/// Gets FlagType from keyword
macro_rules! flag_type {
	(bool) => {
		$crate::flag_type!(Bool)
	};
	(b) => {
		$crate::flag_type!(bool)
	};
	(B) => {
		flag_type!(boool)
	};
	(int) => {
		$crate::flag_type!(Int)
	};
	(i) => {
		flag_type!(int)
	};
	(I) => {
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
	(F) => {
		flag_type!(f)
	};
	(str) => {
		$crate::flag_type!(string)
	};
	(Str) => {
		flag_type!(string)
	};
	(s) => {
		flag_type!(str)
	};
	(S) => {
		flag_type!(str)
	};
	(string) => {
		$crate::flag_type!(String)
	};
	($i:ident) => {
		$crate::FlagType::$i
	};
}

#[macro_export]
/// Creates flag_value
macro_rules! flag_value {
	(bool,$val:expr) => {
		$crate::FlagValue::Bool($val)
	};
	(b, $val:expr) => {
		$crate::flag_value!(bool, $val)
	};
	(Bool,$val:expr) => {
		$crate::flag_value!(bool, $val)
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
	(str,$val:literal) => {
		$crate::flag_value!(str, $val.into())
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
	($i:ident$(,)?) => {
		$crate::flag_value!($i, $crate::default_value!($i))
	};
}

#[macro_export]
/// Creates default_flag_value
macro_rules! default_flag_value {
	($type:ident) => {
		$crate::flag_value!($type, $crate::default_value!($type))
	};
}

#[macro_export]
/// get Default falue from type
macro_rules! default_value {
	(b) => {
		$crate::default_value!(bool)
	};
	(Bool) => {
		$crate::default_value!(bool)
	};
	(bool) => {
		bool::default()
	};
	(true) => {
		default_value!(bool)
	};
	(false) => {
		default_value!(bool)
	};
	(i) => {
		default_value!(int)
	};
	(Int) => {
		default_value!(int)
	};
	(Integer) => {
		default_value!(int)
	};
	(integer) => {
		default_value!(int)
	};
	(int) => {
		isize::default()
	};
	(f) => {
		default_value!(float)
	};
	(Float) => {
		default_value!(float)
	};
	(float) => {
		f64::default()
	};
	(s) => {
		default_value!(str)
	};
	(Str) => {
		default_value!(str)
	};
	(String) => {
		default_value!(str)
	};
	(string) => {
		default_value!(str)
	};
	(str) => {
		String::default()
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
