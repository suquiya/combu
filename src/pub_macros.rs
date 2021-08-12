#[macro_export]
/// Creates new Vector.
macro_rules! v {
	($($t:tt)*) => {
		$crate::vector!($($t)*)
	};
}

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
	("")=>{
		String::new()
	};
	(->$from:expr)=>{
		$from
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
	(*$($t:tt)+) => {
		$crate::_fsp!(*$($t)+)
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
	(->$name:ident=$t:tt)=>{
		$crate::_fsp!(->$name=>$t)
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
	([$($t:tt)+][#$(sep:tt)*]$($ta:tt)+)=>{
		$crate::_fsp!([$($t)+]=>$($ta:tt)+)
	};
	([$($t:tt)+][$($ta:tt)*]$($tas:tt)+)=>{
		$crate::_fsp!([$($t)+]=>[$($ta)*]$($tas)+)
	};
	([$($t:tt)+]$sep:tt$($ta:tt)+)=>{
		$crate::_fsp!([$($t)+]=>$($ta)+)
	};
	($name:ident$t:tt)=>{
		$crate::_fsp!($name=>$t)
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
/// macro for inner flag
macro_rules! _fsp {
	(*$($t:tt)*)=>{
		$crate::flag!($($t)*)
	};
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
	([$($nt:tt)*]$(=>$($t:tt)*)?)=>{
		$crate::_fsp!($($nt)* =>$($($t)*)?)
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
macro_rules! _ftp{
	(->$name:expr=>$t:tt$($t2:tt)+)=>{
		{
			println!("_ftp: {:?}",stringify!($t));
			$crate::_ftp_s!(->$name=>{=,s#,l#,>,?}[$t$($t2)+])
		}
	};
	(->$name:expr=>)=>{
		$crate::_ftp!(->$name=>[])
	};
	(->$name:expr=>{$($t:tt)*})=>{
		$crate::_ftp![->$name=>[$($t)*]]
	};
	(->$name:expr=>($($t:tt)*))=>{
		$crate::_ftp![->$name=>[$($t)*]]
	};
	(->$name:expr=>[]) => {
		$crate::_ftp!(->$name=>[=String::default(),s#Vector::default(),l#Vector::default(),>FlagType::default(),?FlagValue::String(String::default())])
	};
	(->$name:expr=>[$i:ident])=>{
		$crate::_fp_ident_ft_assigner!(->$name=>[$i],_ftp,=);
	};
	(->$name:expr=>[$i:ident,$($t:tt)*])=>{
		$crate::_fp_ident_ft_assigner!(->$name=>[$i,$($t)*],_ftp,=);
	};
	(->$name:expr=>[-$($t:tt)*])=>{
		$crate::_ftp_s!(->$name=>[-$($t)+])
	};
	(->$name:expr=>[$(=)?$description:literal,$($t:tt)*])=>{
		$crate::_ftp_s!(->$name=>[=$description,$($t)*])
	};
	(->$name:expr=>[$(=)?$description:expr,$(s#)?$(-)?[$($s:tt)*]$($t:tt)*])=>{
		$crate::_ftp_s!(->$name=>[=$description,-[$($s)*]$($t)*])
	};
	(->$name:expr=>[$(=)?$description:expr,
		$short_alias:expr,
		$long_alias:expr,
		$(>)?$type:expr,
		$(?)?$default:expr$(,)?
		])=>{
			$crate::_flag_basic_constructor!(->$name=>[$description,$short_alias,$long_alias,$type,$default])
	};
	(->$name:expr=>[$(=)?$description:expr,
		s#$short_alias:expr,
		$long_alias:expr,
		$(>)?$type:expr,
		$(?)?$default:expr$(,)?
		])=>{
			$crate::_flag_basic_constructor!(->$name=>[$description,$short_alias,$long_alias,$type,$default])
	};
	(->$name:expr=>[$(=)?$description:expr,
		$short_alias:expr,
		l#$long_alias:expr,
		$(>)?$type:expr,
		$(?)?$default:expr$(,)?
		])=>{
			$crate::_flag_basic_constructor!(->$name=>[$description,$short_alias,$long_alias,$type,$default])
	};
	(->$name:expr=>[$(=)?$description:expr,
		s#$short_alias:expr,
		l#$long_alias:expr,
		$(>)?$type:expr,
		$(?)?$default:expr$(,)?
		])=>{
			$crate::_flag_basic_constructor!(->$name=>[$description,$short_alias,$long_alias,$type,$default])
	};
	(->$name:expr=>[$($t:tt)+])=>{
		$crate::_ftp_s!(->$name=>[$($t)+])
	};
}
#[macro_export]
/// macro for innser flag
macro_rules! _ftp_s {
	(->$name:expr=>{$($at:tt)*}[$($t:tt)*])=>{
		$crate::_ftp_t!(->$name=>{$($at)*}[$($t)*])
	};
	(->$name:expr=>[$($t:tt)+])=>{
		$crate::_ftp_t!(->$name=>{=,s#,l#,>,?}[$($t)+])
	};
}

#[macro_export]
/// macro for innser flag
macro_rules! _ftp_t {
	(->$name:expr=>
		{=$([$($dt:tt)*])?,s#$([$($st:tt)*])?,l#$([$($lt:tt)*])?,>$($type:ident)?,?$($default:expr)?}[,$($t:tt)*])=>{
			$crate::_ftp_t!(->$name=>{=$([$($dt)*])?,s#$([$($st)*])?,l#$([$($lt)*])?,>$($type)?,?$($default)?}[$($t)*])
		};
	(->$name:expr=>
		{=,s#$([$($st:tt)*])?,l#$([$($lt:tt)*])?,>,?}
		[$type:ident?$($t:tt)*])=>{
		$crate::_ftp_t!(->$name=>{=,s#$([$($st)*])?,l#$([$($lt)*])?,>$type,?}[?$($t)*])
	};
	(->$name:expr=>
		{=$([$($dt:tt)*])?,s#$([$($st:tt)*])?,l#$([$($lt:tt)*])?,>,?$($default:expr)?}
		[[>$type:ident]$($t:tt)*])=>{
		$crate::_ftp_t!(->$name=>{=$([$($dt)*])?,s#$([$($st)*])?,l#$([$($lt)*])?,>$type,?$($default)?}[$($t)*])
	};
	(->$name:expr=>
		{=,s#$([$($st:tt)*])?,l#$([$($lt:tt)*])?,>$($type:ident)?,?$($default:expr)?}
		[[=$($dt:tt)*]$($t:tt)*])=>{
		$crate::_ftp_t!(->$name=>{=[$($dt)*],s#$([$($st)*])?,l#$([$($lt)*])?,>$($type)?,?$($default)?}[$($t)*])
	};
	(->$name:expr=>
		{=$([$($dt:tt)*])?,s#$([$($st:tt)*])?,l#$([$($lt:tt)*])?,>,?$($default:expr)?}
		[>$type:ident$(,$($t:tt)*)?])=>{
		$crate::_ftp_t!(->$name=>{=$([$($dt)*])?,s#$([$($st)*])?,l#$([$($lt)*])?,>$type,?$($default)?}[$($($t)*)?])
	};
	(->$name:expr=>
		{=$([$($dt:tt)*])?,s#$([$($st:tt)*])?,l#$([$($lt:tt)*])?,>,?$($default:expr)?}
		[>$type:ident$($t:tt)*])=>{
		$crate::_ftp_t!(->$name=>{=$([$($dt)*])?,s#$([$($st)*])?,l#$([$($lt)*])?,>$type,?$($default)?}[$($t)*])
	};
	(->$name:expr=>
		{=,s#$([$($st:tt)*])?,l#$([$($lt:tt)*])?,>,?$($default:expr)?}
		[[$i:ident]$($t:tt)*])=>{
		$crate::_fp_ident_ft_assigner!(->$name=>{=,s#$([$($st)*])?,l#$([$($lt)*])?,>,?$($default)?}[$i,$($t)*],_ftp_t,=)
	};
	(->$name:expr=>
		{=$([$($dt:tt)*])?,s#$([$($st:tt)*])?,l#$([$($lt:tt)*])?,>$($type:ident)?,?$($default:expr)?}
		[$(l#)?--$l:ident$(,)?$la:ident$($t:tt)*])=>{
		$crate::_fp_ident_ft_assigner!(->$name=>{=$([$($dt)*])?,s#$([$($st)*])?,l#[$($($lt)*)? $l ],>$($type)?,?$($default)?}[$la$($t)*],_ftp_t,--)
	};
	(->$name:expr=>
		{=$([$($dt:tt)*])?,s#$([$($st:tt)*])?,l#$([$($lt:tt)*])?,>$($type:ident)?,?$($default:expr)?}
		[$(l#)?--$l:ident $($t:tt)*])=>{
		$crate::_ftp_t!(->$name=>{=$([$($dt)*])?,s#$([$($st)*])?,l#[$($($lt)*)? $l],>$($type)?,?$($default)?}[$($t)*])
	};
	(->$name:expr=>
		{=$([$($dt:tt)*])?,s#$([$($st:tt)*])?,l#$([$($lt:tt)*])?,>$($type:ident)?,?$($default:expr)?}
		[$(s#)?-$s:ident$(,)?$sa:ident$($t:tt)*])=>{
			$crate::_fp_ident_s_assigner!(->$name=>{=$([$($dt)*])?,s#[$($($st)*)?$s],l#$([$($lt)*])?,>$($type)?,?}[$sa$($t)*],_ftp_t)
	};
	(->$name:expr=>
		{=$([$($dt:tt)*])?,s#$([$($st:tt)*])?,l#$([$($lt:tt)*])?,>$($type:ident)?,?$($default:expr)?}
		[$(s#)?-$s:ident$(,$($t:tt)*)?])=>{
			$crate::_ftp_t!(->$name=>{=$([$($dt)*])?,s#[$($($st)*)?$s],l#$([$($lt)*])?,>$($type)?,?}[$($($t)*)?])
	};
	(->$name:expr=>
		{=$([$($dt:tt)*])?,s#$([$($st:tt)*])?,l#$([$($lt:tt)*])?,>$($type:ident)?,?$($default:expr)?}
		[$(s#)?-$s:ident$($t:tt)*])=>{
			$crate::_ftp_t!(->$name=>{=$([$($dt)*])?,s#[$($($st)*)?$s],l#$([$($lt)*])?,>$($type)?,?}[$($t)*])
	};
	(->$name:expr=>
		{=$([$($dt:tt)*])?,s#,l#$([$($lt:tt)*])?,>$($type:ident)?,?$($default:expr)?}
		[$(s#)?$(-)?[$($(-)?$s:ident$(,)?)*]$(,$($t:tt)*)?])=>{
			$crate::_ftp_t!(->$name=>{=$([$($dt)*])?,s#[$($s)*],l#$([$($lt)*])?,>$($type)?,?}[$($($t)*)?])
	};
	(->$name:expr=>
		{=$([$($dt:tt)*])?,s#,l#$([$($lt:tt)*])?,>$($type:ident)?,?$($default:expr)?}
		[$(s#)?$(-)?[$($(-)?$s:ident$(,)?)*]$($t:tt)*])=>{
			$crate::_ftp_t!(->$name=>{=$([$($dt)*])?,s#[$($s)*],l#$([$($lt)*])?,>$($type)?,?}[$($t)*])
	};
	(->$name:expr=>
		{=$([$($dt:tt)*])?,s#,l#$([$($lt:tt)*])?,>$($type:ident)?,?$($default:expr)?}
		[$(s#)?$(-)?[$($s:tt)*]$($t:tt)*])=>{
			$crate::_ftp_t!(->$name=>{=$([$($dt)*])?,s#[$($s)*],l#$([$($lt)*])?,>$($type)?,?}[$($t)*])
	};
	(->$name:expr=>
		{=$([$($dt:tt)*])?,s#,l#$([$($lt:tt)*])?,>$($type:ident)?,?$($default:expr)?}
		[s#$short_alias:ident$(,$($t:tt)*)?])=>{
		$crate::_ftp_t!(->$name=>{=$([$($dt)*])?,s#[->$short_alias],l#$([$($lt)*])?,>$($type)?,?}[$($($t)*)?])
	};
	(->$name:expr=>
		{=$([$($dt:tt)*])?,s#,l#$([$($lt:tt)*])?,>$($type:ident)?,?$($default:expr)?}
		[s#$short_alias:expr$(,$($t:tt)*)?])=>{
		$crate::_ftp_t!(->$name=>{=$([$($dt)*])?,s#[->$short_alias],l#$([$($lt)*])?,>$($type)?,?}[$($($t)*)?])
	};
	(->$name:expr=>
		{=$([$($dt:tt)*])?,s#$([$($st:tt)*])?,l#,>$($type:ident)?,?$($default:expr)?}
		[l#$long_alias:expr$(,$($t:tt)*)?])=>{
		$crate::_ftp_t!(->$name=>{=$([$($dt)*])?,s#$([$($st)*])?,l#[->$long_alias],>$($type)?,?$($default)?}[$($($t)*)?])
	};
	(->$name:expr=>
		{=$([$($dt:tt)*])?,s#$([$($st:tt)*])?,l#,>$($type:ident)?,?$($default:expr)?}
		[$(l#)?$(--)?[$($(--)?$l:ident$(,)?)*]$($t:tt)*])=>{
		$crate::_ftp_t!(->$name=>{=$([$($dt)*])?,s#$([$($st)*])?,l#[$($l)*],>$($type)?,?$($default)?}[$($t)*])
	};
	(->$name:expr=>
		{=$([$($dt:tt)*])?,s#$([$($st:tt)*])?,l#,>$($type:ident)?,?$($default:expr)?}
		[$(l#)?$(--)?[$($lt:tt)*],$($t:tt)*])=>{
		$crate::_ftp_t!(->$name=>{=$([$($dt)*])?,s#$([$($st)*])?,l#[$($lt)*],>$($type)?,?$($default)?}[$($t)*])
	};
	(->$name:expr=>
		{=$([$($dt:tt)*])?,s#$([$($st:tt)*])?,l#,>$($type:ident)?,?$($default:expr)?}
		[$(l#)?$(--)?[$($lt:tt)*]])=>{
		$crate::_ftp_t!(->$name=>{=$([$($dt)*])?,s#$([$($st)*])?,l#[$($lt)*],>$($type)?,?$($default)?}[])
	};
	(->$name:expr=>
		{=,s#$([$($st:tt)*])?,l#$([$($lt:tt)*])?,>$($type:ident)?,?$($default:expr)?}
		[= -$($t:tt)*])=>{
		$crate::_ftp_t!(->$name=>{=[],s#$([$($st)*])?,l#$([$($lt)*])?,>$($type)?,?$($default)?}[-$($t)*])
	};
	(->$name:expr=>
		{=,s#$([$($st:tt)*])?,l#$([$($lt:tt)*])?,>$($type:ident)?,?$($default:expr)?}
		[= ?$($t:tt)*])=>{
		$crate::_ftp_t!(->$name=>{=[],s#$([$($st)*])?,l#$([$($lt)*])?,>$($type)?,?$($default)?}[?$($t)*])
	};
	(->$name:expr=>
		{=,s#$([$($st:tt)*])?,l#$([$($lt:tt)*])?,>$($type:ident)?,?$($default:expr)?}
		[= >$($t:tt)*])=>{
		$crate::_ftp_t!(->$name=>{=[],s#$([$($st)*])?,l#$([$($lt)*])?,>$($type)?,?$($default)?}[>$($t)*])
	};
	(->$name:expr=>
		{=,s#$([$($st:tt)*])?,l#$([$($lt:tt)*])?,>$($type:ident)?,?$($default:expr)?}
		[=$(,$($t:tt)*)?])=>{
		$crate::_ftp_t!(->$name=>{=[],s#$([$($st)*])?,l#$([$($lt)*])?,>$($type)?,?$($default)?}[$($($t)*)?])
	};
	(->$name:expr=>
		{=$([$($dt:tt)*])?,s#$([$($st:tt)*])?,l#$([$($lt:tt)*])?,>$(bool)?,?}
		[$(?)?$(@)?false])=>{
		$crate::_ftp_t!(->$name=>{=$([$($dt)*])?,s#$([$($st)*])?,l#$([$($lt)*])?,>bool,?false}[])
	};
	(->$name:expr=>
		{=$([$($dt:tt)*])?,s#$([$($st:tt)*])?,l#$([$($lt:tt)*])?,>$(bool)?,?}
		[$(?)?$(@)?true])=>{
		$crate::_ftp_t!(->$name=>{=$([$($dt)*])?,s#$([$($st)*])?,l#$([$($lt)*])?,>bool,?true}[])
	};
	(->$name:expr=>
		{=,s#$([$($st:tt)*])?,l#$([$($lt:tt)*])?,>$($type:ident)?,?$($default:expr)?}
		[=$description:ident$($t:tt)*])=>{
		$crate::_ftp_t!(->$name=>{=[$description],s#$([$($st)*])?,l#$([$($lt)*])?,>$($type)?,?$($default)?}[$($t)*])
	};
	(->$name:expr=>
		{=,s#$([$($st:tt)*])?,l#$([$($lt:tt)*])?,>$($type:ident)?,?$($default:expr)?}
		[$(=)?$description:literal$($t:tt)*])=>{
		$crate::_ftp_t!(->$name=>{=[$description],s#$([$($st)*])?,l#$([$($lt)*])?,>$($type)?,?$($default)?}[$($t)*])
	};
	(->$name:expr=>
		{=,s#$([$($st:tt)*])?,l#$([$($lt:tt)*])?,>$($type:ident)?,?$($default:expr)?}
		[[=$description:expr]$($t:tt)*])=>{
		$crate::_ftp_t!(->$name=>{=[$description],s#$([$($st)*])?,l#$([$($lt)*])?,>$($type)?,?$($default)?}[$($t)*])
	};
	(->$name:expr=>
		{=,s#$([$($st:tt)*])?,l#$([$($lt:tt)*])?,>$($type:ident)?,?$($default:expr)?}
		[$(=)?$description:expr$(,$($t:tt)*)?])=>{
		$crate::_ftp_t!(->$name=>{=[$description],s#$([$($st)*])?,l#$([$($lt)*])?,>$($type)?,?$($default)?}[$($($t)*)?])
	};
	(->$name:expr=>
		{=[$($dt:tt)*],s#$([$($st:tt)*])?,l#$([$($lt:tt)*])?,>$type:ident,?}
		[$(?)?$(@)?$default:literal])=>{
		$crate::_ftp_t!(->$name=>{=[$($dt)*],s#$([$($st)*])?,l#$([$($lt)*])?,>$type,?$default}[])
	};
	(->$name:expr=>
		{=[$($dt:tt)*],s#$([$($st:tt)*])?,l#$([$($lt:tt)*])?,>$type:ident,?}
		[[$(?)?$(@)?$default:expr]])=>{
		$crate::_ftp_t!(->$name=>{=[$($dt)*],s#$([$($st)*])?,l#$([$($lt)*])?,>$type,?$default}[])
	};
	(->$name:expr=>
		{=[$($dt:tt)*],s#$([$($st:tt)*])?,l#$([$($lt:tt)*])?,>$type:ident,?}
		[?$default:expr])=>{
		$crate::_ftp_t!(->$name=>{=[$($dt)*],s#$([$($st)*])?,l#$([$($lt)*])?,>$type,?$default}[])
	};
	(->$name:expr=>
		{=[$($dt:tt)*],s#$([$($st:tt)*])?,l#$([$($lt:tt)*])?,>$type:ident,?}
		[@$default:expr])=>{
		$crate::_ftp_t!(->$name=>{=[$($dt)*],s#$([$($st)*])?,l#$([$($lt)*])?,>$type,?$default}[])
	};
	(->$name:expr=>
		{={$($dt:tt)*},s#[$($st:tt)*],l#[$($lt:tt)*],>$type:ident,?}
		[$(?)?$(@)?$default:expr])=>{
		$crate::_ftp_t!(->$name=>{={$($dt)*},s#[$($st)*],l#[$($lt)*],>$type,?$default}[])
	};
	(->$name:expr=>
		{=[$($dt:tt)*],s#$([$($st:tt)*])?,l#$([$($lt:tt)*])?,>,?$($default:expr)?}
		[$i:ident$($t:tt)*])=>{
		$crate::_ftp_t!(->$name=>{=[$($dt)*],s#$([$($st)*])?,l#$([$($lt)*])?,>,?$($default)?}[>$i$($t)*])
	};
	(->$name:expr=>
		{=,s#$([$($st:tt)*])?,l#$([$($lt:tt)*])?,>,?$($default:expr)?}
		[$i:ident$($t:tt)*])=>{
		$crate::_fp_ident_ft_assigner!(->$name=>{=,s#$([$($st)*])?,l#$([$($lt)*])?,>,?$($default)?}[$i$($t)*],_ftp_t,=)
	};
	(->$name:expr=>
		{=$([$($dt:tt)*])?,s#,l#$([$($lt:tt)*])?,>$($type:ident)?,?$($default:expr)?}
		[$short_alias:expr$(,$($t:tt)*)?])=>{
		$crate::_ftp_t!(->$name=>{=$([$($dt)*])?,s#[->$short_alias],l#$([$($lt)*])?,>$($type)?,?}[$($($t)*)?])
	};
	(->$name:expr=>
		{=$([$($dt:tt)*])?,s#$([$($st:tt)*])?,l#,>$($type:ident)?,?$($default:expr)?}
		[$long_alias:expr$(,$($t:tt)*)?])=>{
		$crate::_ftp_t!(->$name=>{=$([$($dt)*])?,s#$([$($st)*])?,l#[->$long_alias],>$($type)?,?$($default)?}[$($($t)*)?])
	};
	(->$name:expr=>
		{=$([$($dt:tt)*])?,s#$([$($st:tt)*])?,l#$([$($lt:tt)*])?,>$($type:ident)?,?}
		[@$($default:expr)?$(,$($t:tt)*)?])=>{
		$crate::_ftp_t!(->$name=>{=$([$($dt)*])?,s#$([$($st)*])?,l#$([$($lt)*])?,>$($type)?,?$($default)?}[$($($t)*)?])
	};
	(->$name:expr=>
		{=$([$($dt:tt)*])?,s#$([$($st:tt)*])?,l#$([$($lt:tt)*])?,>$type:ident,?}
		[[@$($default:expr)?]$($t:tt)*])=>{
		$crate::_ftp_t!(->$name=>{=$([$($dt)*])?,s#$([$($st)*])?,l#$([$($lt)*])?,>$type,?$($default)?}[$($t)*])
	};
	(->$name:expr=>
		{=$([$($dt:tt)*])?,s#$([$($st:tt)*])?,l#$([$($lt:tt)*])?,>$type:ident,?}
		[[?$($default:expr)?]$($t:tt)*])=>{
		$crate::_ftp_t!(->$name=>{=$([$($dt)*])?,s#$([$($st)*])?,l#$([$($lt)*])?,>$type,?$($default)?}[$($t)*])
	};
	(->$name:expr=>
		{=$([$($dt:tt)*])?,s#$([$($st:tt)*])?,l#$([$($lt:tt)*])?,>$type:ident,?}
		[?$($default:expr)?$(,$($t:tt)*)?])=>{
		$crate::_ftp_t!(->$name=>{=$([$($dt)*])?,s#$([$($st)*])?,l#$([$($lt)*])?,>$type,?$($default)?}[$($($t)*)?])
	};
	(->$name:expr=>
		{=$([$($dt:tt)*])?,s#$([$($st:tt)*])?,l#$([$($lt:tt)*])?,>,?}
		[?false$(,$($t:tt)*)?])=>{
		$crate::_ftp_t!(->$name=>{=$([$($dt)*])?,s#$([$($st)*])?,l#$([$($lt)*])?,>,?false}[$($($t)*)?])
	};
	(->$name:expr=>
		{=$([$($dt:tt)*])?,s#$([$($st:tt)*])?,l#$([$($lt:tt)*])?,>,?}
		[?$default:expr$(,$($t:tt)*)?])=>{
		$crate::_ftp_t!(->$name=>{=$([$($dt)*])?,s#$([$($st)*])?,l#$([$($lt)*])?,>,?$default}[$($($t)*)?])
	};
	(->$name:expr=>
		{=$([$($dt:tt)*])?,s#$([$($st:tt)*])?,l#$([$($lt:tt)*])?,>,?}
		[])=>{
		$crate::_ftp_t!(->$name=>{=$([$($dt)*])?,s#$([$($st)*])?,l#$([$($lt)*])?,>String,?}[])
	};
	(->$name:expr=>
		{=$([$($dt:tt)*])?,s#$([$($st:tt)*])?,l#$([$($lt:tt)*])?,>,?true}
		[])=>{
		$crate::_ftp_t!(->$name=>{=$([$($dt)*])?,s#$([$($st)*])?,l#$([$($lt)*])?,>bool,?true}[])
	};
	(->$name:expr=>
		{=$([$($dt:tt)*])?,s#$([$($st:tt)*])?,l#$([$($lt:tt)*])?,>,?false}
		[])=>{
		$crate::_ftp_t!(->$name=>{=$([$($dt)*])?,s#$([$($st)*])?,l#$([$($lt)*])?,>bool,?false}[])
	};
	(->$name:expr=>
		{=$([$($dt:tt)*])?,s#$([$($st:tt)*])?,l#$([$($lt:tt)*])?,>$type:ident,?}
		[])=>{
		$crate::_ftp!(->$name=>[=$crate::string_from!($($($dt)*)?),s#$crate::short_alias![$($($st)*)?],l#$crate::long_alias![$($($lt)*)?],>$crate::flag_type!($type),?$crate::flag_value!($type)])
	};
	(->$name:expr=>
		{=$([$($dt:tt)*])?,s#$([$($st:tt)*])?,l#$([$($lt:tt)*])?,>$type:ident,?$($default:expr)?}
		[])=>{
		$crate::_ftp!(->$name=>[=$crate::string_from!($($($dt)*)?),s#$crate::short_alias![$($($st)*)?],l#$crate::long_alias![$($($lt)*)?],>$crate::flag_type!($type),?$crate::flag_value!($type$(,$default)?)])
	}
}

#[macro_export]
/// short_alias_expander
macro_rules! short_alias {
	() => {
		$crate::short_alias!(None)
	};
	(None) => {
		$crate::vector!(None;char)
	};
	($(-)?[$($t:tt)*])=>{
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
	($($s:literal)+)=>{
		$crate::vector![$($s),+;:char]
	};
	($($s:literal),+)=>{
		$crate::vector![$($s),+;:char]
	};
	($(-)?$s:ident$($t:tt)*)=>{
		$crate::short_alias!(=[$crate::char![$s],],$($t)*)
	};
	(=[$($t:tt)*],$s:ident$($t2:tt)*)=>{
		$crate::short_alias!(=[$($t)*$crate::char![$s],],$(t2)*)
	};
	($(-)?$s:literal$($t:tt)*)=>{
		$cratte::short_alias!(=[$s,],$($t:tt)*)
	};
	(=[$($t:tt)*],$s:literal$($t2:tt)*)=>{
		$crate::short_alias!(=[$($t)*$s,],$(t2)*)
	};
	(=[$($t:tt)*],)=>{
		$crate::vector!($($t)*;:char)
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
	($($l:expr),+)=>{
		$crate::vector!($($l),+;=>String)
	};
	($($(--)?$l:literal),+$(,)?)=>{
		$crate::long_alias!($($l),+)
	};
	($long:expr)=>{
		$crate::long_alias!(->$long)
	};
	(->$long:expr)=>{
		$long
	}
}

#[macro_export]
/// inner macro in flag! macro
macro_rules! _fp_ident_s_assigner {
	(->$name:expr=>$({$($ta:tt)*})?[a$($t:tt)*],$macro:ident$(,$($p:tt)*)?) => {
		$crate::$macro!(->$name=>$({$($ta)*})?[-a$($t)*])
	};
	(->$name:expr=>$({$($ta:tt)*})?[b$($t:tt)*],$macro:ident$(,$($p:tt)*)?) => {
		$crate::$macro!(->$name=>$({$($ta)*})?[-b$($t)*])
	};
	(->$name:expr=>$({$($ta:tt)*})?[c$($t:tt)*],$macro:ident$(,$($p:tt)*)?) => {
		$crate::$macro!(->$name=>$({$($ta)*})?[-c$($t)*])
	};
	(->$name:expr=>$({$($ta:tt)*})?[d$($t:tt)*],$macro:ident$(,$($p:tt)*)?) => {
		$crate::$macro!(->$name=>$({$($ta)*})?[-d$($t)*])
	};
	(->$name:expr=>$({$($ta:tt)*})?[e$($t:tt)*],$macro:ident$(,$($p:tt)*)?) => {
		$crate::$macro!(->$name=>$({$($ta)*})?[-e$($t)*])
	};
	(->$name:expr=>$({$($ta:tt)*})?[f$($t:tt)*],$macro:ident$(,$($p:tt)*)?) => {
		$crate::$macro!(->$name=>$({$($ta)*})?[-f$($t)*])
	};
	(->$name:expr=>$({$($ta:tt)*})?[g$($t:tt)*],$macro:ident$(,$($p:tt)*)?) => {
		$crate::$macro!(->$name=>$({$($ta)*})?[-g$($t)*])
	};
	(->$name:expr=>$({$($ta:tt)*})?[h$($t:tt)*],$macro:ident$(,$($p:tt)*)?) => {
		$crate::$macro!(->$name=>$({$($ta)*})?[-h$($t)*])
	};
	(->$name:expr=>$({$($ta:tt)*})?[i$($t:tt)*],$macro:ident$(,$($p:tt)*)?) => {
		$crate::$macro!(->$name=>$({$($ta)*})?[-i$($t)*])
	};
	(->$name:expr=>$({$($ta:tt)*})?[j$($t:tt)*],$macro:ident$(,$($p:tt)*)?) => {
		$crate::$macro!(->$name=>$({$($ta)*})?[-j$($t)*])
	};
	(->$name:expr=>$({$($ta:tt)*})?[k$($t:tt)*],$macro:ident$(,$($p:tt)*)?) => {
		$crate::$macro!(->$name=>$({$($ta)*})?[-k$($t)*])
	};
	(->$name:expr=>$({$($ta:tt)*})?[l$($t:tt)*],$macro:ident$(,$($p:tt)*)?) => {
		$crate::$macro!(->$name=>$({$($ta)*})?[-l$($t)*])
	};
	(->$name:expr=>$({$($ta:tt)*})?[m$($t:tt)*],$macro:ident$(,$($p:tt)*)?) => {
		$crate::$macro!(->$name=>$({$($ta)*})?[-m$($t)*])
	};
	(->$name:expr=>$({$($ta:tt)*})?[n$($t:tt)*],$macro:ident$(,$($p:tt)*)?) => {
		$crate::$macro!(->$name=>$({$($ta)*})?[-n$($t)*])
	};
	(->$name:expr=>$({$($ta:tt)*})?[o$($t:tt)*],$macro:ident$(,$($p:tt)*)?) => {
		$crate::$macro!(->$name=>$({$($ta)*})?[-o$($t)*])
	};
	(->$name:expr=>$({$($ta:tt)*})?[p$($t:tt)*],$macro:ident$(,$($p:tt)*)?) => {
		$crate::$macro!(->$name=>$({$($ta)*})?[-p$($t)*])
	};
	(->$name:expr=>$({$($ta:tt)*})?[q$($t:tt)*],$macro:ident$(,$($p:tt)*)?) => {
		$crate::$macro!(->$name=>$({$($ta)*})?[-q$($t)*])
	};
	(->$name:expr=>$({$($ta:tt)*})?[r$($t:tt)*],$macro:ident$(,$($p:tt)*)?) => {
		$crate::$macro!(->$name=>$({$($ta)*})?[-r$($t)*])
	};
	(->$name:expr=>$({$($ta:tt)*})?[s$($t:tt)*],$macro:ident$(,$($p:tt)*)?) => {
		$crate::$macro!(->$name=>$({$($ta)*})?[-s$($t)*])
	};
	(->$name:expr=>$({$($ta:tt)*})?[t$($t:tt)*],$macro:ident$(,$($p:tt)*)?) => {
		$crate::$macro!(->$name=>$({$($ta)*})?[-t$($t)*])
	};
	(->$name:expr=>$({$($ta:tt)*})?[u$($t:tt)*],$macro:ident$(,$($p:tt)*)?) => {
		$crate::$macro!(->$name=>$({$($ta)*})?[-u$($t)*])
	};
	(->$name:expr=>$({$($ta:tt)*})?[v$($t:tt)*],$macro:ident$(,$($p:tt)*)?) => {
		$crate::$macro!(->$name=>$({$($ta)*})?[-v$($t)*])
	};
	(->$name:expr=>$({$($ta:tt)*})?[w$($t:tt)*],$macro:ident$(,$($p:tt)*)?) => {
		$crate::$macro!(->$name=>$({$($ta)*})?[-w$($t)*])
	};
	(->$name:expr=>$({$($ta:tt)*})?[x$($t:tt)*],$macro:ident$(,$($p:tt)*)?) => {
		$crate::$macro!(->$name=>$({$($ta)*})?[-x$($t)*])
	};
	(->$name:expr=>$({$($ta:tt)*})?[y$($t:tt)*],$macro:ident$(,$($p:tt)*)?) => {
		$crate::$macro!(->$name=>$({$($ta)*})?[-y$($t)*])
	};
	(->$name:expr=>$({$($ta:tt)*})?[z$($t:tt)*],$macro:ident$(,$($p:tt)*)?) => {
		$crate::$macro!(->$name=>$({$($ta)*})?[-z$($t)*])
	};
	(->$name:expr=>$({$($ta:tt)*})?[A$($t:tt)*],$macro:ident$(,$($p:tt)*)?) => {
		$crate::$macro!(->$name=>$({$($ta)*})?[-A$($t)*])
	};
	(->$name:expr=>$({$($ta:tt)*})?[B$($t:tt)*],$macro:ident$(,$($p:tt)*)?) => {
		$crate::$macro!(->$name=>$({$($ta)*})?[-B$($t)*])
	};
	(->$name:expr=>$({$($ta:tt)*})?[C$($t:tt)*],$macro:ident$(,$($p:tt)*)?) => {
		$crate::$macro!(->$name=>$({$($ta)*})?[-C$($t)*])
	};
	(->$name:expr=>$({$($ta:tt)*})?[D$($t:tt)*],$macro:ident$(,$($p:tt)*)?) => {
		$crate::$macro!(->$name=>$({$($ta)*})?[-D$($t)*])
	};
	(->$name:expr=>$({$($ta:tt)*})?[E$($t:tt)*],$macro:ident$(,$($p:tt)*)?) => {
		$crate::$macro!(->$name=>$({$($ta)*})?[-E$($t)*])
	};
	(->$name:expr=>$({$($ta:tt)*})?[F$($t:tt)*],$macro:ident$(,$($p:tt)*)?) => {
		$crate::$macro!(->$name=>$({$($ta)*})?[-F$($t)*])
	};
	(->$name:expr=>$({$($ta:tt)*})?[G$($t:tt)*],$macro:ident$(,$($p:tt)*)?) => {
		$crate::$macro!(->$name=>$({$($ta)*})?[-G$($t)*])
	};
	(->$name:expr=>$({$($ta:tt)*})?[H$($t:tt)*],$macro:ident$(,$($p:tt)*)?) => {
		$crate::$macro!(->$name=>$({$($ta)*})?[-H$($t)*])
	};
	(->$name:expr=>$({$($ta:tt)*})?[I$($t:tt)*],$macro:ident$(,$($p:tt)*)?) => {
		$crate::$macro!(->$name=>$({$($ta)*})?[-I$($t)*])
	};
	(->$name:expr=>$({$($ta:tt)*})?[J$($t:tt)*],$macro:ident$(,$($p:tt)*)?) => {
		$crate::$macro!(->$name=>$({$($ta)*})?[-J$($t)*])
	};
	(->$name:expr=>$({$($ta:tt)*})?[K$($t:tt)*],$macro:ident$(,$($p:tt)*)?) => {
		$crate::$macro!(->$name=>$({$($ta)*})?[-K$($t)*])
	};
	(->$name:expr=>$({$($ta:tt)*})?[L$($t:tt)*],$macro:ident$(,$($p:tt)*)?) => {
		$crate::$macro!(->$name=>$({$($ta)*})?[-L$($t)*])
	};
	(->$name:expr=>$({$($ta:tt)*})?[M$($t:tt)*],$macro:ident$(,$($p:tt)*)?) => {
		$crate::$macro!(->$name=>$({$($ta)*})?[-M$($t)*])
	};
	(->$name:expr=>$({$($ta:tt)*})?[N$($t:tt)*],$macro:ident$(,$($p:tt)*)?) => {
		$crate::$macro!(->$name=>$({$($ta)*})?[-N$($t)*])
	};
	(->$name:expr=>$({$($ta:tt)*})?[O$($t:tt)*],$macro:ident$(,$($p:tt)*)?) => {
		$crate::$macro!(->$name=>$({$($ta)*})?[-O$($t)*])
	};
	(->$name:expr=>$({$($ta:tt)*})?[P$($t:tt)*],$macro:ident$(,$($p:tt)*)?) => {
		$crate::$macro!(->$name=>$({$($ta)*})?[-P$($t)*])
	};
	(->$name:expr=>$({$($ta:tt)*})?[Q$($t:tt)*],$macro:ident$(,$($p:tt)*)?) => {
		$crate::$macro!(->$name=>$({$($ta)*})?[-Q$($t)*])
	};
	(->$name:expr=>$({$($ta:tt)*})?[R$($t:tt)*],$macro:ident$(,$($p:tt)*)?) => {
		$crate::$macro!(->$name=>$({$($ta)*})?[-R$($t)*])
	};
	(->$name:expr=>$({$($ta:tt)*})?[S$($t:tt)*],$macro:ident$(,$($p:tt)*)?) => {
		$crate::$macro!(->$name=>$({$($ta)*})?[-S$($t)*])
	};
	(->$name:expr=>$({$($ta:tt)*})?[T$($t:tt)*],$macro:ident$(,$($p:tt)*)?) => {
		$crate::$macro!(->$name=>$({$($ta)*})?[-T$($t)*])
	};
	(->$name:expr=>$({$($ta:tt)*})?[U$($t:tt)*],$macro:ident$(,$($p:tt)*)?) => {
		$crate::$macro!(->$name=>$({$($ta)*})?[-U$($t)*])
	};
	(->$name:expr=>$({$($ta:tt)*})?[V$($t:tt)*],$macro:ident$(,$($p:tt)*)?) => {
		$crate::$macro!(->$name=>$({$($ta)*})?[-V$($t)*])
	};
	(->$name:expr=>$({$($ta:tt)*})?[W$($t:tt)*],$macro:ident$(,$($p:tt)*)?) => {
		$crate::$macro!(->$name=>$({$($ta)*})?[-W$($t)*])
	};
	(->$name:expr=>$({$($ta:tt)*})?[X$($t:tt)*],$macro:ident$(,$($p:tt)*)?) => {
		$crate::$macro!(->$name=>$({$($ta)*})?[-X$($t)*])
	};
	(->$name:expr=>$({$($ta:tt)*})?[Y$($t:tt)*],$macro:ident$(,$($p:tt)*)?) => {
		$crate::$macro!(->$name=>$({$($ta)*})?[-Y$($t)*])
	};
	(->$name:expr=>$({$($ta:tt)*})?[Z$($t:tt)*],$macro:ident$(,$($p:tt)*)?) => {
		$crate::$macro!(->$name=>$({$($ta)*})?[-Z$($t)*])
	};
	(->$name:expr=>$({$($ta:tt)*})?[$i:ident$($t:tt)*],$macro:ident$(,$($p:tt)*)?) => {
		$crate::$macro!(->$name=>$({$($ta)*})?[$($($p)*)?$i$($t)*])
	};
}

#[macro_export]
/// sub macro for flag
macro_rules! _fp_ident_ft_assigner {
	(->$name:expr=>$({$($ta:tt)*})?[bool$($t:tt)*],$macro:ident$(,$($p:tt)*)?)=>{
		$crate::$macro!(->$name=>$({$($ta)*})?[>bool$($t)*])
	};
	(->$name:expr=>$({$($ta:tt)*})?[b$($t:tt)*],$macro:ident$(,$($p:tt)*)?)=>{
		$crate::$macro!(->$name=>$({$($ta)*})?[>b$($t)*])
	};
	(->$name:expr=>$({$($ta:tt)*})?[B$($t:tt)*],$macro:ident$(,$($p:tt)*)?)=>{
		$crate::$macro!(->$name=>$({$($ta)*})?[>B$($t)*])
	};
	(->$name:expr=>$({$($ta:tt)*})?[Bool$($t:tt)*],$macro:ident$(,$($p:tt)*)?)=>{
		$crate::$macro!(->$name=>$({$($ta)*})?[>Bool$($t)*])
	};
	(->$name:expr=>$({$($ta:tt)*})?[int$($t:tt)*],$macro:ident$(,$($p:tt)*)?)=>{
		$crate::$macro!(->$name=>$({$($ta)*})?[>int$($t)*])
	};
	(->$name:expr=>$({$($ta:tt)*})?[i$($t:tt)*],$macro:ident$(,$($p:tt)*)?)=>{
		$crate::$macro!(->$name=>$({$($ta)*})?[>i$($t)*])
	};
	(->$name:expr=>$({$($ta:tt)*})?[Integer$($t:tt)*],$macro:ident$(,$($p:tt)*)?)=>{
		$crate::$macro!(->$name=>$({$($ta)*})?[>Integer$($t)*])
	};
	(->$name:expr=>$({$($ta:tt)*})?[I$($t:tt)*],$macro:ident$(,$($p:tt)*)?)=>{
		$crate::$macro!(->$name=>$({$($ta)*})?[>I$($t)*])
	};
	(->$name:expr=>$({$($ta:tt)*})?[integer$($t:tt)*],$macro:ident$(,$($p:tt)*)?)=>{
		$crate::$macro!(->$name=>$({$($ta)*})?[>integer$($t)*])
	};
	(->$name:expr=>$({$($ta:tt)*})?[Int$($t:tt)*],$macro:ident$(,$($p:tt)*)?)=>{
		$crate::$macro!(->$name=>$({$($ta)*})?[>Int$($t)*])
	};
	(->$name:expr=>$({$($ta:tt)*})?[float$($t:tt)*],$macro:ident$(,$($p:tt)*)?)=>{
		$crate::$macro!(->$name=>$({$($ta)*})?[>float$($t)*])
	};
	(->$name:expr=>$({$($ta:tt)*})?[f$($t:tt)*],$macro:ident$(,$($p:tt)*)?)=>{
		$crate::$macro!(->$name=>$({$($ta)*})?[>f$($t)*])
	};
	(->$name:expr=>$({$($ta:tt)*})?[F$($t:tt)*],$macro:ident$(,$($p:tt)*)?)=>{
		$crate::$macro!(->$name=>$({$($ta)*})?[>F$($t)*])
	};
	(->$name:expr=>$({$($ta:tt)*})?[Float$($t:tt)*],$macro:ident$(,$($p:tt)*)?)=>{
		$crate::$macro!(->$name=>$({$($ta)*})?[>Float$($t)*])
	};
	(->$name:expr=>$({$($ta:tt)*})?[Str$($t:tt)*],$macro:ident$(,$($p:tt)*)?)=>{
		$crate::$macro!(->$name=>$({$($ta)*})?[>Str$($t)*])
	};
	(->$name:expr=>$({$($ta:tt)*})?[str$($t:tt)*],$macro:ident$(,$($p:tt)*)?)=>{
		$crate::$macro!(->$name=>$({$($ta)*})?[>str$($t)*])
	};
	(->$name:expr=>$({$($ta:tt)*})?[s$($t:tt)*],$macro:ident$(,$($p:tt)*)?)=>{
		$crate::$macro!(->$name=>$({$($ta)*})?[>s$($t)*])
	};
	(->$name:expr=>$({$($ta:tt)*})?[string]$($t:tt)*)=>{
		$crate::$macro!(->$name=>$({$($ta)*})?[>string$($t)*])
	};
	(->$name:expr=>$({$($ta:tt)*})?[S$($t:tt)*],$macro:ident$(,$($p:tt)*)?)=>{
		$crate::$macro!(->$name=>$({$($ta)*})?[>String$($t)*])
	};
	(->$name:expr=>$({$($ta:tt)*})?[String$($t:tt)*],$macro:ident$(,$($p:tt)*)?)=>{
		$crate::$macro!(->$name=>$({$($ta)*})?[>String$($t)*])
	};
	(->$name:expr=>$({$($ta:tt)*})?[$description:ident$($t:tt)*],$macro:ident)=>{
		$crate::$macro!(->$name=>$({$($ta)*})?[=$description$($t)*])
	};
	(->$name:expr=>$({$($ta:tt)*})?[$description:ident$($t:tt)*],$macro:ident,$($p:tt)*)=>{
		$crate::$macro!(->$name=>$({$($ta)*})?[$($p)*$description$($t)*])
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
	(F, $val:expr) => {
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
	(S,$val:expr) => {
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
	(I) => {
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
	(F) => {
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
	(S) => {
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

#[cfg(test)]
mod tests {
	use crate::{Flag, FlagType, FlagValue, Vector};

	macro_rules! assert_eqs {
		($left:expr,$($right:expr),+$(,)?) => {
			$(assert_eq!($left,$right);)+
			//println!("OK: {:?}",$left);
		};
	}
	#[test]
	fn flag_test() {
		// flag![(test_flag=>[bool,-s,-f,--long,@"test",@def false]),];
		let _t = "test";
		let _t_string = String::from(_t);
		let full = Flag {
			name: "test_flag".into(),
			description: "test".into(),
			short_alias: Vector(Some(vec!['s', 'f'])),
			long_alias: Vector(Some(vec!["long".to_owned(), "long2".to_owned()])),
			default_value: FlagValue::Bool(false),
			flag_type: FlagType::Bool,
		};
		let _flag_name = String::from("test_flag");
		let _flag_name2 = _flag_name.clone();
		let _flag_name3 = _flag_name.clone();
		assert_eqs!(
			full.clone(),
			flag!(->String::from("test_flag")=>[
				=String::from("test"),
				Vector(Some(vec!['s', 'f'])),
				Vector(Some(vec!["long".to_owned(), "long2".to_owned()])),
				FlagType::Bool,
				FlagValue::Bool(false)
			]),
			flag!(@->String::from("test_flag")=>[
					=String::from("test"),
					Vector(Some(vec!['s', 'f'])),
					Vector(Some(vec!["long".to_owned(), "long2".to_owned()])),
					FlagType::Bool,
					FlagValue::Bool(false)
				]
			),
			flag!(->[String::from("test_flag")]=>[
					=String::from(_t),
					Vector(Some(vec!['s', 'f'])),
					Vector(Some(vec!["long".to_owned(), "long2".to_owned()])),
					FlagType::Bool,
					FlagValue::Bool(false)
				]
			),
			flag!([->String::from("test_flag")]=>[
					=String::from(_t),
					Vector(Some(vec!['s', 'f'])),
					Vector(Some(vec!["long".to_owned(), "long2".to_owned()])),
					FlagType::Bool,
					FlagValue::Bool(false)
				]
			),
			flag!([->String::from("test_flag")][
					=String::from(_t),
					Vector(Some(vec!['s', 'f'])),
					Vector(Some(vec!["long".to_owned(), "long2".to_owned()])),
					FlagType::Bool,
					FlagValue::Bool(false)
				]
			),
			flag!(&String::from("test_flag")=>[
					=String::from(_t),
					Vector(Some(vec!['s', 'f'])),
					Vector(Some(vec!["long".to_owned(), "long2".to_owned()])),
					FlagType::Bool,
					FlagValue::Bool(false)
				]
			),
			flag!("test_flag"=>[
				=String::from(_t),
				Vector(Some(vec!['s', 'f'])),
				Vector(Some(vec!["long".to_owned(), "long2".to_owned()])),
				FlagType::Bool,
				FlagValue::Bool(false)
			]),
			flag!(&_flag_name=>[
				=String::from(_t),
				Vector(Some(vec!['s', 'f'])),
				Vector(Some(vec!["long".to_owned(), "long2".to_owned()])),
				FlagType::Bool,
				FlagValue::Bool(false)
			]),
			flag!(
				&_flag_name2 = [
					=String::from(_t),
					Vector(Some(vec!['s', 'f'])),
					Vector(Some(vec!["long".to_owned(), "long2".to_owned()])),
					FlagType::Bool,
					FlagValue::Bool(false)
				]
			),
			flag!(
				&_flag_name3[
					=String::from(_t),
					Vector(Some(vec!['s', 'f'])),
					Vector(Some(vec!["long".to_owned(), "long2".to_owned()])),
					FlagType::Bool,
					FlagValue::Bool(false)
				]
			),
			flag!(test_flag=>[
				=String::from(_t),
				Vector(Some(vec!['s', 'f'])),
				Vector(Some(vec!["long".to_owned(), "long2".to_owned()])),
				FlagType::Bool,
				FlagValue::Bool(false)
			]),
			flag!(test_flag[
				=String::from(_t),
				Vector(Some(vec!['s', 'f'])),
				Vector(Some(vec!["long".to_owned(), "long2".to_owned()])),
				FlagType::Bool,
				FlagValue::Bool(false)
			]),
			flag!(
				[test_flag][
					=String::from(_t),
					Vector(Some(vec!['s', 'f'])),
					Vector(Some(vec!["long".to_owned(), "long2".to_owned()])),
					FlagType::Bool,
					FlagValue::Bool(false)
				]
			),
			flag!(
				[test_flag]=>[
					=String::from(_t),
					Vector(Some(vec!['s', 'f'])),
					Vector(Some(vec!["long".to_owned(), "long2".to_owned()])),
					FlagType::Bool,
					FlagValue::Bool(false)
				]
			),
			flag!(
				[test_flag] = [
					=String::from(_t),
					Vector(Some(vec!['s', 'f'])),
					Vector(Some(vec!["long".to_owned(), "long2".to_owned()])),
					FlagType::Bool,
					FlagValue::Bool(false)
				]
			)
		);
		let _f = String::from("test_flag");
		assert_eqs!(
			Flag::with_name("test_flag"),
			flag!(->String::from("test_flag")),
			flag!([->String::from("test_flag")]),
			flag!(->[String::from("test_flag")]),
			flag!("test_flag"),
			flag!("test_flag"=>),
			flag!(test_flag),
			flag!(->_f),
		);
		assert_eqs!(
			full.clone(),
			flag!(test_flag=>[
				bool,
				_t,
				Vector(Some(vec!['s', 'f'])),
				Vector(Some(vec!["long".to_owned(), "long2".to_owned()])),
				?false
				]
			),
		);
		assert_eqs!(
			full.clone(),
			flag!(test_flag=>[
				bool,
				_t,
				Vector(Some(vec!['s', 'f'])),
				Vector(Some(vec!["long".to_owned(), "long2".to_owned()])),
				?false
				]
			),
			flag!(test_flag=>[
				>bool,
				=_t,
				s#Vector(Some(vec!['s', 'f'])),
				l#Vector(Some(vec!["long".to_owned(), "long2".to_owned()])),
				?false
				]
			),
			flag!(test_flag=>[
				>bool,
				=_t,
				l#Vector(Some(vec!["long".to_owned(), "long2".to_owned()])),
				s#Vector(Some(vec!['s', 'f'])),
				?false
				]
			),
			flag!(test_flag=>[
				_t,
				>bool,
				l#Vector(Some(vec!["long".to_owned(), "long2".to_owned()])),
				s#Vector(Some(vec!['s', 'f'])),
				?false
				]
			),
		);
		assert_eqs!(
			full.clone(),
			flag!(test_flag=>[
			bool,
			_t,
			-[-s,-f],
			Vector(Some(vec!["long".to_owned(), "long2".to_owned()])),
			?false]),
			flag!(test_flag=>[
			bool,
			_t,
			s#[s, f],
			Vector(Some(vec!["long".to_owned(), "long2".to_owned()])),
			?false]),
			flag!(test_flag=>[
			bool,
			_t,
			['s', 'f'],
			Vector(Some(vec!["long".to_owned(), "long2".to_owned()])),
			?false]),
			flag!(test_flag=>[
			bool,
			_t,
			-[s f],
			--["long", "long2"],
			?false]),
			flag!(test_flag=>[
			>bool,
			=_t,
			-s,-f,
			Vector(Some(vec!["long".to_owned(), "long2".to_owned()])),
			?false]),
			flag!(test_flag=>[
				>bool,
				=_t,
				-s f,
				[long long2],
				?false]),
			flag!(test_flag=>[
				bool,
				=_t,
				-s -f,
				--long long2,
				?false]),
			flag!(test_flag=>[
				>bool,
				=_t,
				-s,f,
				--long long2]),
			flag!(test_flag=>[
				bool,
				=_t,
				-s -f,
				--long long2,]),
		);
		assert_eqs!(
			full.clone(),
			flag!(test_flag=>[
				=_t,
				-s,-f,
				--[long,long2],
				bool?false
			]),
			flag!(test_flag=>[
				=_t,
				s#Vector(Some(vec!['s', 'f'])),
				--[long, long2],
				>bool?false
			]),
			flag!(test_flag[
				=_t,
				s#[s f],
				[long long2],
				>bool?false
			]),
			flag!(test_flag[
				=_t,
				-s ,-f,
				--long long2,
				bool?false
			]),
		);

		assert_eqs!(
			full.clone().short_alias('a'),
			flag!(test_flag=>[
			>bool,
			=_t,
			-s f a,
			--long long2]),
			flag!(test_flag=>[
				>bool,
				=_t,
				-s,f -a,
				--long long2 ?false]),
			flag!(test_flag=>[
			>bool?false,
			_t,
			-s,f -a,
			--long, long2
			]),
			flag!(test_flag=>[
				>bool,
				_t,
				-s,f,a,
				--[long, long2],
				?false]),
		);
		assert_eqs!(
			{
				let mut f = full.clone().short_alias('a');
				f.long_alias.take();
				f
			},
			flag!(test_flag=>[
				>bool,
				=_t,
				-s f a,
				?false
			]),
			flag!(test_flag=>[
				>bool?false,
				=_t,
				Vector(Some(vec!['s', 'f','a']))
			]),
			flag!(test_flag=>[
				>bool,
				=_t,
				-s,f,a,
				?false
			]),
			flag!(test_flag=>[
				>bool,
				=_t,
				-s,f,a ?false
			]),
			flag!(test_flag=>[
				>bool?false,
				=_t,
				-s,f,a
			])
		);
		let mut s = full.clone();
		s.long_alias.take();
		assert_eq!(s, flag!(test_flag=>[bool,_t,-s,-f,?false]));
		s.short_alias.take();
		assert_eqs!(
			s,
			flag!(test_flag=>[bool,_t]),
			flag!(test_flag=>[bool,_t,[],[]]),
			flag!(test_flag=>[bool,_t,[]]),
			flag!(test_flag=>[>bool,=_t,false]),
			flag!(test_flag=>[bool,_t,?]),
			flag!(test_flag=>[>bool,_t,?false])
		);
		assert_eqs!(
			full.clone().description(""),
			flag!(test_flag=>[bool,=,-s,-f,--long,--long2,?false]),
			flag!(test_flag=>[bool,= -s -f --long --long2 ?false])
		);
		assert_eq!(
			{
				let mut f = full.clone().description("");
				f.long_alias.take();
				f
			},
			flag!(test_flag=>[bool,,-s,-f,?false])
		);
		assert_eqs!(
			Flag::with_all_field(
				"test_flag".into(),
				"aaa".into(),
				Vector(None),
				Vector(None),
				FlagType::Bool,
				FlagValue::Bool(false)
			),
			flag!(test_flag=>[bool,"aaa",?false]),
			flag!(test_flag=>[bool,"aaa",@false]),
			flag!(test_flag=>[Bool,"aaa",false]),
			flag!(test_flag=>[Bool,"aaa"])
		);
		assert_eqs!(
			Flag::new_bool("test_flag"),
			flag!(test_flag=>[>bool,]),
			flag!(test_flag=>[>bool]),
			flag!(test_flag=>[>Bool]),
			flag!(test_flag=>[bool,,?false]),
			flag!(test_flag=>[bool,,false])
		);
		assert_eqs!(
			Flag::new_bool("test_flag").description("desc"),
			flag!(test_flag=>[bool,="desc",?false]),
			flag!(test_flag=>[="desc",?false]),
			flag!(test_flag=>[="desc",bool?false]),
			flag!(test_flag=>[="desc",bool]),
		);
		let _i = "desc_only";
		assert_eqs!(
			Flag::new_string("test_flag").description(_i),
			flag!(test_flag=>[_i])
		);
		assert_eqs!(
			Flag::new_bool("test_flag")
				.description("test")
				.short_alias('s')
				.short_alias('f'),
			flag!(test_flag[bool,"test",-s f]),
			flag!(test_flag=>[="test",-s,-f, >bool?false]),
			flag!(test_flag=>[="test",-s,-f, ?false])
		);
		let _t = "desc";
		assert_eqs!(
			{ full.clone().description("desc") },
			flag!(test_flag=>[="desc",-s,-f,--long,--long2,>bool?false]),
			flag!(test_flag=>[="desc",-s,-f,--long,--long2 bool?false]),
			flag!(test_flag=>[="desc",-s,-f,--long,--long2,bool?false]),
			flag!(test_flag=>[="desc",-s,-f,--long,--long2 bool]),
			flag!(test_flag=>[="desc",-s,-f,--long,--long2,bool]),
			flag!(test_flag:[="desc",-s,-f,--long,--long2,?false]),
			flag!(test_flag=[="desc",-s,-f,--long,--long2 ?false]),
			flag!(test_flag[-s,f,--long,--long2,="desc",bool?false]),
			flag!(test_flag[-s,f,--long,--long2,="desc",bool?false]),
			flag!(test_flag=>[-s,-f,--long,--long2,="desc",bool]),
			flag!(test_flag=>[bool,"desc",-s,-f,--long,--long2,?false]),
			flag!(test_flag[bool,-s,-f,--long,--long2,=_t,?false]),
			flag!(test_flag=>[bool,-s,-f,--long,--long2,="desc",?false])
		);
		assert_eqs!(
			{ full.clone().description("desc") },
			flag!(test_flag=>[="desc",-s,-f,--long,--long2,>bool?false]),
			flag!(test_flag=>[="desc" -s -f --long --long2 >bool?false]),
			flag!(test_flag=>[="desc" -s -f --long --long2 >bool?false]),
			flag!(test_flag=>[_t -s -f --long --long2 bool?false]),
			flag!(test_flag=>[>bool =_t -s -f --long --long2 ?false]),
			flag!(test_flag=>[bool =_t -s -f --long --long2 ?false]),
		);
		assert_eqs!(
			Flag::new("test_flag", FlagType::Bool, "desc")
				.short_alias('s')
				.short_alias('f'),
			flag!(test_flag=>[="desc",-s,-f, bool]),
			flag!(test_flag=>["desc",-s,-f, bool]),
			flag!(test_flag=>[="desc",-s,-f,?false]),
			flag!(test_flag=>[bool,-s,-f,="desc",?false])
		);
		assert_eqs!(
			Flag::new("test_flag", FlagType::Bool, "desc"),
			flag!(test_flag=>["desc",?false]),
			flag!(test_flag["desc",b])
		);
		assert_eqs!(
			Flag::new_string("test_flag").description("desc"),
			flag!(test_flag["desc"]),
			flag!(test_flag[_t]),
		);
		assert_eqs!(
			Flag::new_string("test_flag")
				.description("aaa")
				.default_value("aaa".to_owned().into()),
			flag!(test_flag=>[str,"aaa","aaa"])
		);
		assert_eqs!(
			{
				let mut f = full.clone().description("desc");
				f.short_alias.take();
				f
			},
			flag!(test_flag=>[bool,_t,--long,--long2,?false])
		);
		assert_eqs!(
			full.clone(),
			flag!([test_flag][bool, -s,-f,--long,--long2,="test",false]),
			flag!(*test_flag[bool, -s,-f,--long,--long2,="test",false]),
			flag!(test_flag{bool, -s,-f,--long,--long2,="test",false}),
			flag!(*test_flag{bool, -s,-f,--long,--long2,="test",false}),
			flag!(@[String::from("test_flag")]=>{bool, -s,-f,--long,--long2,="test",false}),
			flag!([test_flag]=>{bool, -s,-f,--long long2 ="test",false}),
		);
		assert_eqs!(
			full,
			flag!([test_flag]
			[>bool]
			[-s -f]
			[--long --long2]
			[="test"]
			[?false]
			),
			flag!([test_flag]
			[bool]
			[="test"]
			[-s -f]
			[--long --long2]
			[false]
			),
		);
	}
}
