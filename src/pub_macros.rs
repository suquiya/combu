//! export macros for combu
//! combuのためのエクスポート用マクロ集

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
	(::<$ptype:ty>,$(,)*$($t:tt)*)=>{
		$crate::vector!($($t)*;:$ptype)
	};
	(=>$ptype:ty,$(,)*$($t:tt)*)=>{
		$crate::vector!($($t)*;=>$ptype)
	};
	(None$($(;)?$(::)?<$type:ty>)?) => {
		$crate::Vector$(::<$type>)?(None)
	};
	(None$($(;)?::$type:ty)?) => {
		$crate::Vector$(::<$type>)?(None)
	};
	(None$($(;)?:$type:ty)?) => {
		$crate::Vector$(::<$type>)?(None)
	};
	($($(;)?:$type:ty)?) => {
		$crate::Vector$(::<$type>)?(None)
	};
	($($(;)?$(::)?<$type:ty>$(,)?)?) => {
		$crate::Vector$(::<$type>)?(None)
	};
	($elem:expr; $n:expr$(;:$type:ty)?)=>{
		$crate::Vector$(::<$type>)?(Some(vec![$elem,$n]))
	};
	($elem:expr; $n:expr$(;$(::)?<$type:ty>)?)=>{
		$crate::Vector$(::<$type>)?(Some(vec![$elem,$n]))
	};
	($elem:expr; $n:expr;$(=>$type:ty)?)=>{
		$crate::Vector$(::<$type>)?(Some(vec![$elem.into(),$n]))
	};
	($($x:expr),+ $(,)*$(;:$type:ty)?)=>{
		$crate::Vector$(::<$type>)?(Some(vec![$($x),+]))
	};
	($($x:expr),+ $(,)*$(;::$type:ty)?)=>{
		$crate::Vector$(::<$type>)?(Some(vec![$($x),+]))
	};
	($($x:expr),+ $(,)*$(;$(::)?<$type:ty>)?)=>{
		$crate::Vector$(::<$type>)?(Some(vec![$($x),+]))
	};
	($($x:expr),+ $(,)*;$(=>$type:ty)?)=>{
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
/// Checks context macro.
macro_rules! check {
	(error)=>{
		$crate::check_error!();
	};
	(error, $($t:tt)*)=>{
		$crate::check_error!($($t)*);
	};
	(help, $cmd:ident, $ctx:ident)=>{
		$crate::check!(help,$cmd,$ctx,combu::command::presets::func::help_tablize_with_alias_dedup);
	};
	(help,$cmd:ident,$ctx:ident,$func:path)=>{
		$crate::check!(help,$cmd,$ctx,{
			println!("{}",$func(&$cmd,&$ctx));
			return $crate::done!();
		})
	};
	(license,$cmd:ident,$ctx:ident)=>{
		$crate::check!(license,$cmd,$ctx,{
			println!($cmd.license.output().unwrap());
			return $crate::done!();
		})
	};
	($member:ident,$cmd:ident,$ctx:ident) => {
		$crate::check!($member,$cmd,$ctx,{
			println!("{}",$cmd.$member);
			return $crate::done!();
		})
	};
	($member:ident,$cmd:ident,$ctx:ident,$func:path)=>{
		$crate::check!($member,$cmd,$ctx,{
			println!("{}",$func(&$cmd,&$ctx));
			return $crate::done!()
		})
	};
	($member:ident,$cmd:ident,$ctx:ident,{$($t:tt)*})=>{
		if($ctx.is_flag_true(stringify!($member),&$cmd)){
			$($t)*
		}
	};
}

#[macro_export]
/// List check macro
macro_rules! checks {
	 ($cmd:ident,$ctx:ident,[$($ct:ident),*]) => {
		  $($crate::check!($ct,$cmd,$ctx);)*
	 };
	 ([$($ct:ident),*],$cmd:ident,$ctx:ident)=>{
		 $($crate::check!($ct,$cmd,$ctx);)*
	 };
}

#[macro_export]
/// Macro for parse error check
macro_rules! check_error {
	($ctx:ident)=>{
		$crate::check_error!($ctx,{
			println!("{}", combu::parser::preset::gen_error_description(error_info));
			return $crate::done!();
		})
	};
	($cmd:ident,$ctx:ident)=>{
		$crate::check_error!($cmd,$ctx,>error_info,{
			println!("{}", combu::parser::preset::gen_error_description(error_info));
			println!("{}",combu::command::presets::func::help_tablize_with_alias_dedup(&$cmd,&$ctx));
			return $crate::done!();
		})
	};
	($ctx:ident,{$($t:tt)*})=>{
		$crate::check_error!($ctx,>error_info,{$($t)*})
	};
	($cmd:ident,$ctx:ident,{$($t:tt)*})=>{
		$crate::check_error!($cmd,$ctx,>error_info,{$($t)*})
	};
	($ctx:ident, >$error_info:ident, {$($t:tt)*}) => {
		if let Some($error_info) = $ctx.first_error() {
			$($t)*
		}
	};
	($cmd:ident, $ctx:ident,>$error_info:ident,{$($t:tt)*})=>{
		if let Some($error_info) = $ctx.first_error(){
			$($t)*
		}
	}
}

#[macro_export]
/// Checks context has help flag. If the context has help flag, return ShowHelpRequest.
macro_rules! check_help {
	($($t:tt)*)=>{
		$crate::check!(help,$($t)*);
	}
}

#[macro_export]
/// Checks context has version flag. If the context has help flag, show version and exit.
macro_rules! check_version {
	($context:ident,$current_command:ident) => {
		$crate::check!(version, $context, $current_command)
	};
}

#[macro_export]
/// Checks context has authors flag. If the context has author flag, show authors and exit.
macro_rules! check_authors {
	($context:ident,$current_command:ident) => {
		$crate::check!(authors, $context, $current_command)
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
	($ctx:ident,$cmd:ident) => {
		$crate::ckeck!(license, $ctx, $cmd)
	};
	($context:ident,$current_command:ident, $license_func:path) => {
		$crate::check!(license, $context, $current_command, $license_func)
	};
}

#[macro_export]
/// Checks context has license flag. If the context has license flag, show authors and exit.
macro_rules! check_copyright {
	($context:ident,$current_command:ident) => {
		$crate::check!(copyright, $context, $current_command);
	};
}

#[macro_export]
/// Checks context has values of the preset flags.
macro_rules! check_preset_flags {
	($cmd:ident$(,)?$ctx:ident$(,)?) => {
		$crate::checks!($cmd, $ctx, [error, help, version, license, authors]);
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
/// Preset for help request to parent command
macro_rules! parent_help_request_action {
	($func:ident) => {
		|cmd, mut ctx| -> action_result!() {
			let help_str = if ctx.args.is_empty() {
				$func(&cmd, &ctx)
			} else {
				// argsを辿って対象のサブコマンドを特定
				let mut tail_cmd = cmd;
				for arg in &ctx.args {
					match tail_cmd.take_sub(&arg) {
						Some(sub) => {
							ctx.routes.push(tail_cmd.name.clone());
							ctx.common_flags.push(tail_cmd.c_flags);
							tail_cmd = sub;
						}
						_ => {
							// マッチしないものはとりあえず無視
						}
					}
				}
				$func(&tail_cmd, &ctx)
			};
			println!("{}", help_str);
			$crate::done!()
		}
	};
}

#[macro_export]
/// Preset for definition function that can use help request action
macro_rules! define_parent_help_request_action {
	($func_name:ident,$help_func:ident)=>{
		$crate::define_parent_help_request_action!(name=>$func_name,help_func=>$help_func);
	};
	(name=>$func_name:ident, help_func=>$help_func:ident) => {
		fn $func_name(cmd: $crate::Command, mut ctx: $crate::Context) -> action_result!() {
			let help_str = if ctx.args.is_empty() {
				$help_func(&cmd, &ctx)
			} else {
				// argsを辿って対象のサブコマンドを特定
				let mut tail_cmd = cmd;
				for arg in &ctx.args {
					match tail_cmd.take_sub(&arg) {
						Some(sub) => {
							ctx.routes.push(tail_cmd.name.clone());
							ctx.common_flags.push(tail_cmd.c_flags);
							tail_cmd = sub;
						}
						_ => {
							// マッチしないものはとりあえず無視
						}
					}
				}
				$help_func(&tail_cmd, &ctx)
			};
			println!("{}", help_str);
			crate::done!()
		}
	};
}

#[macro_export]
/// Preset of definition of help command action
macro_rules! define_help_command_action {
	($action_name:ident, $req_action_name:ident, $help_func:ident) => {
		fn $action_name(mut cmd: $crate::Command, ctx: $crate::Context) -> action_result!() {
			check_help!(cmd, ctx, $help_func);
			cmd.action = Some($req_action_name);
			Ok($crate::ActionResult::ParentActionRequest(cmd, ctx))
		}
		$crate::define_parent_help_request_action!($req_action_name, $help_func);
	};
}

#[macro_export]
/// preset of help command action
macro_rules! help_command_action {
	($func:ident) => {
		|mut cmd, ctx| -> action_result!() {
			$crate::check_help!(cmd, ctx, $func);
			cmd.action = Some($crate::parent_help_request_action!($func));
			Ok($crate::ActionResult::ParentActionRequest(cmd, ctx))
		}
	};
}

#[macro_export]
/// Build helper for action
macro_rules! action {
	(check_preset_flags,{$($t:tt)*})=>{
		$crate::action!(cmd,ctx,check_preset_flags,{$($t)*})
	};
	($cmd:ident,$ctx:ident,check_preset_flags,{$($t:tt)*}) => {
		$crate::action!($cmd,$ctx,{
			check_preset_flags!($cmd, $ctx);
			$($t)*
		})
	};
	($cmd:ident,$ctx:ident,[$ct:ident],{$($t:tt)*})=>{
		$crate::action!($cmd,$ctx,core,{
			$crate::check!($ct,$cmd,$ctx);
			$($t)*
		})
	};
	($cmd:ident,$ctx:ident,[$($ct:ident),*],{$($t:tt)*})=>{
		$crate::action!($cmd,$ctx,core,{
			$($crate::check!($ct,$cmd,$ctx);)*
			$($t)*
		})
	};
	($cmd:ident,$ctx:ident,s,{$($t:tt)*})=>{
		$crate::action!($cmd,$ctx,{$($t)*})
	};
	($cmd:ident,$ctx:ident,base,{$($t:tt)*})=>{
		$crate::action!($cmd,$ctx,{$($t)*})
	};
	($cmd:ident,$ctx:ident,{$($t:tt)*})=>{
		$crate::action!($cmd,$ctx,core,{$($t)*})
	};
	($cmd:ident,$ctx:ident,core,{$($t:tt)*})=>{
		|$cmd:Command, $ctx: Context|->action_result!(){
			$($t)*
		}
	};
	({$($t:tt)*})=>{
		$crate::action(cmd,ctx,core,{$($t)*})
	};
	($($t:tt)*)=>{
		$crate::action(cmd,ctx,core,{$($t)*})
	}
}

#[macro_export]
/// Build helper for action
macro_rules! def_action_func {
	($func_name:ident,$cmd:ident,$ctx:ident,check_preset_flags,{$($t:tt)*}) => {
		$crate::def_action_func!($func_name,$cmd,$ctx,core,{
			check_preset_flags!($cmd,$ctx);
			$($t)*
		})
	};
	($func_name:ident,$cmd:ident,$ctx:ident,[$ct:ident],{$($t:tt)*}) => {
		$crate::def_action_func!($func_name,$cmd,$ctx,core,{
			$crate::check!($ct,$cmd,$ctx);
			$($t)*
		});
	};
	($func_name:ident,$cmd:ident,$ctx:ident,[$($ct:ident),*],{$($t:tt)*}) => {
		$crate::def_action_func!($func_name,$cmd,$ctx,core,{
			$crate::checks!([$($ct),*],$cmd,$ctx);
			$($t)*
		});
	};
	($func_name:ident,$cmd:ident,$ctx:ident,core,{$($t:tt)*}) => {
		fn $func_name($cmd:$crate::Command,$ctx:$crate::Context)->$crate::action_result!(){
			$($t)*
		}
	};
	($func_name:ident,$cmd:ident,$ctx:ident,base,{$($t:tt)*}) => {
		$crate::def_action_func!($func_name,$cmd,$ctx,core,$($t)*)
	};
	($func_name:ident,$cmd:ident,$ctx:ident,core,$($t:tt)*) => {
		$crate::def_action_func!($func_name,$cmd,$ctx,{$($t)*})
	};
	($func_name:ident,{$($t:tt)*})=>{
		$crate::def_action_func($func_name,core,{$($t)*})
	};
	($func_name:ident,$type:ident,$($t:tt)*)=>{
		$crate::def_action_func!($func_name,cmd,ctx,$type,$($t)*)
	};
	($func_name:ident,$($t:tt)*)=>{
		$crate::def_action_func($func_name,core,{$($t)*})
	};
}

#[macro_export]
/// Default value for presets. (Alias)
macro_rules! default_value {
	($($t:tt)*) => {
		$crate::default_val!($($t)*)
	};
}

#[macro_export]
/// Default value for presets.
macro_rules! default_val {
	(description$(.$ident:ident)+) => {
		$crate::default_description!($($ident).+)
	};
	(name$(.$ident:ident)+) => {
		$crate::default_name!($($ident).+)
	};
	(flag.$ident:ident.alias.short)=>{
		$crate::default_flag_short_alias!($ident)
	};
	($ident:ident.flag.alias.short)=>{
		$crate::default_flag_short_alias!($ident)
	};
	(flag.$ident:ident.short.alias)=>{
		$crate::default_flag_short_alias!($ident)
	};
	($ident:ident.flag.short.alias)=>{
		$crate::default_flag_short_alias!($ident)
	};
	(flag.$ident:ident.short_alias)=>{
		$crate::default_flag_short_alias!($ident)
	};
	($ident:ident.flag.short_alias)=>{
		$crate::default_flag_short_alias!($ident)
	};
	($ident:ident$(.$ident2:ident)+)=>{
		$crate::default_val!([$ident]$(.$ident2)+)
	};
	([$($ident:ident).+].description) => {
		$crate::default_description!($($ident).+)
	};
	([$($ident:ident).+].name) => {
		$crate::default_name!($($ident).+)
	};
	([$($ident:ident).+].$ident2:ident$(.$ident3:ident)*)=>{
		$crate::default_val!([$($ident).+.$ident2]$(.$ident3)*)
	}
}

#[macro_export]
/// Default name for presets
macro_rules! default_name {
	(flag.$ident:ident) => {
		$crate::default_flag_name!($ident)
	};
	($ident:ident.flag) => {
		$crate::default_flag_name!($ident)
	};
}

#[macro_export]
/// Default name for preset flags
macro_rules! default_flag_name {
	($ident:ident) => {
		stringify!($ident)
	};
}

#[macro_export]
/// Default alias for preset flags
macro_rules! default_flag_short_alias {
	(help) => {
		$crate::short_alias![h, '?']
	};
	(version) => {
		$crate::short_alias![v]
	};
	(authors) => {
		$crate::short_alias![a]
	};
	(license) => {
		$crate::short_alias![l]
	};
	(yes) => {
		$crate::short_alias![y]
	};
	(no) => {
		$crate::short_alias![n]
	};
}

#[macro_export]
/// Default description for presets.
macro_rules! default_description {
	(flag.help) => {
		"Prints help information"
	};
	(flag.version) => {
		"Prints version information"
	};
	(flag.authors) => {
		"Prints authors' information"
	};
	(flag.license) => {
		"Prints license information"
	};
	(flag.yes) => {
		"Assumes as yes choosed in all prompts"
	};
	(flag.no) => {
		"Assumes as no choosed in all prompts"
	};
	($ident:ident.flag) => {
		$crate::default_description!(flag.$ident)
	};
}

#[macro_export]
#[doc(hidden)]
// Preset flag base
macro_rules! _preset_flag_constructor {
	($ident:ident)=>{
		$crate::_preset_flag_constructor!($ident,$crate::default_value!(flag.$ident.description))
	};
	($ident:ident,$description:literal)=>{
		$crate::_preset_flag_constructor!($ident,->String::from($description))
	};
	($ident:ident,$description:expr)=>{
		$crate::_preset_flag_constructor!($ident,->$description.into())
	};
	($ident:ident,->$description:expr) => {
		Flag::with_all_field(
			$crate::default_name!(flag.$ident).to_owned(),
			$description,
			$crate::default_value!(flag.$ident.short.alias),
			$crate::vector![None;:String],
			crate::FlagType::Bool,
			crate::FlagValue::Bool(false)
		)
	};
}

#[macro_export]
/// Macro for preset help flag.
macro_rules! help_flag {
	($($($description:tt)+)?)=>{
		$crate::_preset_flag_constructor!(help$(,$($description)+)?)
	};
}

#[macro_export]
/// Macro for preset version flag.
macro_rules! version_flag {
	($($($description:tt)+)?)=>{
		$crate::_preset_flag_constructor!(version$(,$($description)+)?)
	};
}

#[macro_export]
/// Macro for preset authors flag.
macro_rules! authors_flag {
	($($($description:tt)+)?)=>{
		$crate::_preset_flag_constructor!(authors$(,$($description)+)?)
	};
}

#[macro_export]
/// Macro for preset license flag.
macro_rules! license_flag {
	($($($description:tt)+)?)=>{
		$crate::_preset_flag_constructor!(license$(,$($description)+)?)
	};
}

#[macro_export]
/// Macro for preset yes flag.
macro_rules! yes_flag {
	($($($description:tt)+)?)=>{
		$crate::_preset_flag_constructor!(yes$(,$($description)+)?)
	};
}

#[macro_export]
/// Macro for preset no flag.
macro_rules! no_flag {
	($($($description:tt)+)?)=>{
		$crate::_preset_flag_constructor!(no$(,$($description)+)?)
	};
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
	($($inner:tt)+) => {
		Some($($inner)+)
	};
}

#[macro_export]
/// create cmd helper with full detail
macro_rules! cmd {
	($name:ident=>$t:tt)=>{
		$crate::cmd!(stringify!($name)=>$t)
	};
	($name:ident=:$t:tt)=>{
		$crate::cmd!(stringify!($name)=>$t)
	};
	($name:ident$sep:tt$t:tt)=>{
		$crate::cmd!(stringify!($name)=>$t)
	};
	($name:literal$sep:tt$t:tt)=>{
		$crate::cmd!(->$crate::string_from!($name)=>$t)
	};
	($name:ident$t:tt)=>{
		$crate::cmd!($name=>$t)
	};
	($name:literal$t:tt)=>{
		$crate::cmd!($name=>$t)
	};
	($name:expr=>{$($t:tt)*})=>{
		$crate::cmd!($name=>[$($t)*])
	};
	($name:expr=>($($t:tt)*))=>{
		$crate::cmd!($name=>[$($t)*])
	};
	($name:expr=>[$($t:tt)*])=>{
		$crate::cmd!(->$crate::string_from!($name)=>[$($t)*])
	};
	(->$name:expr=>{$($at:tt)*}[,$($t:tt)*])=>{
		$crate::cmd!(->$name=>{$($at)*}[$($t)*])
	};
	(->$name:expr=>{$($at:tt)*}[;$($t:tt)*])=>{
		$crate::cmd!(->$name=>{$($at)*}[$($t)*])
	};
	(->$name:expr=>{>,$($t:tt)+}[action->$($t2:tt)*])=>{
		$crate::cmd!(->$name=>{>,$($t)+}[>$($t2)*])
	};
	(->$name:expr=>{>,$($t:tt)+}[action=>$($t2:tt)*])=>{
		$crate::cmd!(->$name=>{>,$($t)+}[>$($t2)*])
	};
	(->$name:expr=>{>,$($t:tt)+}[action=$($t2:tt)*])=>{
		$crate::cmd!(->$name=>{>,$($t)+}[>$($t2)*])
	};
	(->$name:expr=>{>,$($t:tt)+}[action:$($t2:tt)*])=>{
		$crate::cmd!(->$name=>{>,$($t)+}[>$($t2)*])
	};
	(->$name:expr=>{>,$($t:tt)+}[>$(>)+$($t2:tt)*])=>{
		$crate::cmd!(->$name=>{>,$($t)+}[>$($t2)*])
	};
	(->$name:expr=>{$($t:tt)+}[authors:$($t2:tt)*])=>{
		$crate::cmd!(->$name=>{$($t)+}[<$($t2)*])
	};
	(->$name:expr=>{$($t:tt)+}[authors=$($t2:tt)*])=>{
		$crate::cmd!(->$name=>{$($t)+}[<$($t2)*])
	};
	(->$name:expr=>{$($t:tt)+}[copyright:$($t2:tt)*])=>{
		$crate::cmd!(->$name=>{$($t)+}[(c)$($t2)*])
	};
	(->$name:expr=>{$($t:tt)+}[copyright=$($t2:tt)*])=>{
		$crate::cmd!(->$name=>{$($t)+}[(c)$($t2)*])
	};
	(->$name:expr=>{$($t:tt)+}[license:$($t2:tt)*])=>{
		$crate::cmd!(->$name=>{$($t)+}[(l)$($t2)*])
	};
	(->$name:expr=>{$($t:tt)+}[license=$($t2:tt)*])=>{
		$crate::cmd!(->$name=>{$($t)+}[(l)$($t2)*])
	};
	(->$name:expr=>{$($t:tt)+}[description:$($t2:tt)*])=>{
		$crate::cmd!(->$name=>{$($t)+}[=$($t2)*])
	};
	(->$name:expr=>{$($t:tt)+}[description=$($t2:tt)*])=>{
		$crate::cmd!(->$name=>{$($t)+}[=$($t2)*])
	};
	(->$name:expr=>{$($t:tt)+}[usage:$($t2:tt)*])=>{
		$crate::cmd!(->$name=>{$($t)+}[:$($t2)*])
	};
	(->$name:expr=>{$($t:tt)+}[usage=$($t2:tt)*])=>{
		$crate::cmd!(->$name=>{$($t)+}[:$($t2)*])
	};
	(->$name:expr=>{$($t:tt)+}[#$($t2:tt)*])=>{
		$crate::cmd!(->$name=>{$($t)+}[l~$($t2)*])
	};
	(->$name:expr=>{$($t:tt)+}[l:$($t2:tt)*])=>{
		$crate::cmd!(->$name=>{$($t)+}[l~$($t2)*])
	};
	(->$name:expr=>{$($t:tt)+}[l=$($t2:tt)*])=>{
		$crate::cmd!(->$name=>{$($t)+}[l~$($t2)*])
	};
	(->$name:expr=>{$($t:tt)+}[l+$($t2:tt)*])=>{
		$crate::cmd!(->$name=>{$($t)+}[l~$($t2)*])
	};
	(->$name:expr=>{$($t:tt)+}[lf:$($t2:tt)*])=>{
		$crate::cmd!(->$name=>{$($t)+}[l~$($t2)*])
	};
	(->$name:expr=>{$($t:tt)+}[l_flag:$($t2:tt)*])=>{
		$crate::cmd!(->$name=>{$($t)+}[l~$($t2)*])
	};
	(->$name:expr=>{$($t:tt)+}[l_flag=$($t2:tt)*])=>{
		$crate::cmd!(->$name=>{$($t)+}[l~$($t2)*])
	};
	(->$name:expr=>{$($t:tt)+}[l_flags:$($t2:tt)*])=>{
		$crate::cmd!(->$name=>{$($t)+}[l~$($t2)*])
	};
	(->$name:expr=>{$($t:tt)+}[l_flags=$($t2:tt)*])=>{
		$crate::cmd!(->$name=>{$($t)+}[l~$($t2)*])
	};
	(->$name:expr=>{$($t:tt)+}[local_flag:$($t2:tt)*])=>{
		$crate::cmd!(->$name=>{$($t)+}[l~$($t2)*])
	};
	(->$name:expr=>{$($t:tt)+}[local_flag=$($t2:tt)*])=>{
		$crate::cmd!(->$name=>{$($t)+}[l~$($t2)*])
	};
	(->$name:expr=>{$($t:tt)+}[local_flags:$($t2:tt)*])=>{
		$crate::cmd!(->$name=>{$($t)+}[l~$($t2)*])
	};
	(->$name:expr=>{$($t:tt)+}[local_flags=$($t2:tt)*])=>{
		$crate::cmd!(->$name=>{$($t)+}[l~$($t2)*])
	};
	(->$name:expr=>{$($t:tt)+}[c=$($t2:tt)*])=>{
		$crate::cmd!(->$name=>{$($t)+}[c~$($t2)*])
	};
	(->$name:expr=>{$($t:tt)+}[c:$($t2:tt)*])=>{
		$crate::cmd!(->$name=>{$($t)+}[c~$($t2)*])
	};
	(->$name:expr=>{$($t:tt)+}[c+$($t2:tt)*])=>{
		$crate::cmd!(->$name=>{$($t)+}[c~$($t2)*])
	};
	(->$name:expr=>{$($t:tt)+}[cf:$($t2:tt)*])=>{
		$crate::cmd!(->$name=>{$($t)+}[c~$($t2)*])
	};
	(->$name:expr=>{$($t:tt)+}[c_flag:$($t2:tt)*])=>{
		$crate::cmd!(->$name=>{$($t)+}[c~$($t2)*])
	};
	(->$name:expr=>{$($t:tt)+}[c_flag=$($t2:tt)*])=>{
		$crate::cmd!(->$name=>{$($t)+}[c~$($t2)*])
	};
	(->$name:expr=>{$($t:tt)+}[c_flags:$($t2:tt)*])=>{
		$crate::cmd!(->$name=>{$($t)+}[c~$($t2)*])
	};
	(->$name:expr=>{$($t:tt)+}[c_flags=$($t2:tt)*])=>{
		$crate::cmd!(->$name=>{$($t)+}[c~$($t2)*])
	};
	(->$name:expr=>{$($t:tt)+}[common_flag:$($t2:tt)*])=>{
		$crate::cmd!(->$name=>{$($t)+}[c~$($t2)*])
	};
	(->$name:expr=>{$($t:tt)+}[common_flag=$($t2:tt)*])=>{
		$crate::cmd!(->$name=>{$($t)+}[c~$($t2)*])
	};
	(->$name:expr=>{$($t:tt)+}[common_flags:$($t2:tt)*])=>{
		$crate::cmd!(->$name=>{$($t)+}[c~$($t2)*])
	};
	(->$name:expr=>{$($t:tt)+}[common_flags=$($t2:tt)*])=>{
		$crate::cmd!(->$name=>{$($t)+}[c~$($t2)*])
	};
	(->$name:expr=>{$($t:tt)+}[alias:$($t2:tt)*])=>{
		$crate::cmd!(->$name=>{$($t)+}[&$($t2)*])
	};
	(->$name:expr=>{$($t:tt)+}[alias=$($t2:tt)*])=>{
		$crate::cmd!(->$name=>{$($t)+}[&$($t2)*])
	};
	(->$name:expr=>{$($t:tt)+}[version:$($t2:tt)*])=>{
		$crate::cmd!(->$name=>{$($t)+}[n$($t2)*])
	};
	(->$name:expr=>{$($t:tt)+}[version=$($t2:tt)*])=>{
		$crate::cmd!(->$name=>{$($t)+}[n$($t2)*])
	};
	(->$name:expr=>{$($t:tt)+}[ver:$($t2:tt)*])=>{
		$crate::cmd!(->$name=>{$($t)+}[n$($t2)*])
	};
	(->$name:expr=>{$($t:tt)+}[ver=$($t2:tt)*])=>{
		$crate::cmd!(->$name=>{$($t)+}[n$($t2)*])
	};
	(->$name:expr=>{$($t:tt)+}[v:$($t2:tt)*])=>{
		$crate::cmd!(->$name=>{$($t)+}[n$($t2)*])
	};
	(->$name:expr=>{$($t:tt)+}[v=$($t2:tt)*])=>{
		$crate::cmd!(->$name=>{$($t)+}[n$($t2)*])
	};
	(->$name:expr=>{$($t:tt)+}[n:$($t2:tt)*])=>{
		$crate::cmd!(->$name=>{$($t)+}[n$($t2)*])
	};
	(->$name:expr=>{$($t:tt)+}[n=$($t2:tt)*])=>{
		$crate::cmd!(->$name=>{$($t)+}[n$($t2)*])
	};
	(->$name:expr=>{$($t:tt)+}[|=>$($t2:tt)*])=>{
		$crate::cmd!(->$name=>{$($t)+}[+$($t2)*])
	};
	(->$name:expr=>{$($t:tt)+}[|=$($t2:tt)*])=>{
		$crate::cmd!(->$name=>{$($t)+}[+$($t2)*])
	};
	(->$name:expr=>{$($t:tt)+}[|$($t2:tt)*])=>{
		$crate::cmd!(->$name=>{$($t)+}[+$($t2)*])
	};
	(->$name:expr=>{$($t:tt)+}[sub:$($t2:tt)*])=>{
		$crate::cmd!(->$name=>{$($t)+}[+$($t2)*])
	};
	(->$name:expr=>{$($t:tt)+}[sub=$($t2:tt)*])=>{
		$crate::cmd!(->$name=>{$($t)+}[+$($t2)*])
	};
	(->$name:expr=>[>>$(>)*$action:expr$(,$($t:tt)*)?])=>{
		$crate::cmd!(->$name=>{>,<,@,@,=,:,l~,c~,&,n,+}[>$action$(,$($t)*)?])
	};
	(->$name:expr=>[->$action:expr$(,$($t:tt)*)?])=>{
		$crate::cmd!(->$name=>{>,<,@,@,=,:,l~,c~,&,n,+}[>$action$(,$($t)*)?])
	};
	(->$name:expr=>[=>$action:expr$(,$($t:tt)*)?])=>{
		$crate::cmd!(->$name=>{>,<,@,@,=,:,l~,c~,&,n,+}[>$action$(,$($t)*)?])
	};
	(->$name:expr=>[>$action:expr,<$authors:literal$(,$($t:tt)*)?])=>{
		$crate::cmd!(->$name=>{>,<,@,@,=,:,l~,c~,&,n,+}[>$action,<$authors$(,$($t)*)?])
	};
	(->$name:expr=>{>,<$([$($authors:tt)*])?,@$([$($copyright:tt)*])?,@$([$($license:tt)*])?,=$([$($description:tt)*])?,:$([$($usage:tt)*])?,l~$([$($l_flags:tt)*])?,c~$([$($c_flags:tt)*])?,&$([$($alias:tt)*])?,n$([$($version:tt)*])?,+$([$($sub:tt)*])?}
	[$(>)?|$c:ident$(:Context)?,$cmd:ident$(:Command)?|$(->$crate::action_result!())?$(->$r:ty)?{$($c2:tt)*}$($t:tt)*])=>{
		$crate::cmd!(->$name=>{>[|$c:Context,$cmd:Command|->$crate::action_result!(){$($c2)*}],<$([$($authors)*])?,@$([$($copyright)*])?,@$([$($license)*])?,=$([$($description)*])?,:$([$($usage)*])?,l~$([$($l_flags)*])?,c~$([$($c_flags)*])?,&$([$($alias)*])?,n$([$($version)*])?,+$([$($sub)*])?}
		[$($t)*])
	};
	(->$name:expr=>{>,<$([$($authors:tt)*])?,@$([$($copyright:tt)*])?,@$([$($license:tt)*])?,=$([$($description:tt)*])?,:$([$($usage:tt)*])?,l~$([$($l_flags:tt)*])?,c~$([$($c_flags:tt)*])?,&$([$($alias:tt)*])?,n$([$($version:tt)*])?,+$([$($sub:tt)*])?}
	[$(>)?$action:ident$($t:tt)*])=>{
		$crate::cmd!(->$name=>{>[$action],<$([$($authors)*])?,@$([$($copyright)*])?,@$([$($license)*])?,=$([$($description)*])?,:$([$($usage)*])?,l~$([$($l_flags)*])?,c~$([$($c_flags)*])?,&$([$($alias)*])?,n$([$($version)*])?,+$([$($sub)*])?}
		[$($t)*])
	};
	(->$name:expr=>{>,<$([$($authors:tt)*])?,@$([$($copyright:tt)*])?,@$([$($license:tt)*])?,=$([$($description:tt)*])?,:$([$($usage:tt)*])?,l~$([$($l_flags:tt)*])?,c~$([$($c_flags:tt)*])?,&$([$($alias:tt)*])?,n$([$($version:tt)*])?,+$([$($sub:tt)*])?}
	[$(>)?$action:expr$(,$($t:tt)*)?])=>{
		$crate::cmd!(->$name=>{>[$action],<$([$($authors)*])?,@$([$($copyright)*])?,@$([$($license)*])?,=$([$($description)*])?,:$([$($usage)*])?,l~$([$($l_flags)*])?,c~$([$($c_flags)*])?,&$([$($alias)*])?,n$([$($version)*])?,+$([$($sub)*])?}
		[$($($t)*)?])
	};
	(->$name:expr=>{>,<$([$($authors:tt)*])?,@$([$($copyright:tt)*])?,@$([$($license:tt)*])?,=$([$($description:tt)*])?,:$([$($usage:tt)*])?,l~$([$($l_flags:tt)*])?,c~$([$($c_flags:tt)*])?,&$([$($alias:tt)*])?,n$([$($version:tt)*])?,+$([$($sub:tt)*])?}
	[$(>)?$action:expr$(;$($t:tt)*)?])=>{
		$crate::cmd!(->$name=>{>[$action],<$([$($authors)*])?,@$([$($copyright)*])?,@$([$($license)*])?,=$([$($description)*])?,:$([$($usage)*])?,l~$([$($l_flags)*])?,c~$([$($c_flags)*])?,&$([$($alias)*])?,n$([$($version)*])?,+$([$($sub)*])?}
		[$($($t)*)?])
	};
	(->$name:expr=>{>$([$($action:tt)*])?,<,@$([$($copyright:tt)*])?,@$([$($license:tt)*])?,=$([$($description:tt)*])?,:$([$($usage:tt)*])?,l~$([$($l_flags:tt)*])?,c~$([$($c_flags:tt)*])?,&$([$($alias:tt)*])?,n$([$($version:tt)*])?,+$([$($sub:tt)*])?}
	[<...$(,$($t:tt)*)?])=>{
		$crate::cmd!(->$name=>{>$([$($action)*])?,<[$crate::crate_authors!()],@$([$($copyright)*])?,@$([$($license)*])?,=$([$($description)*])?,:$([$($usage)*])?,l~$([$($l_flags)*])?,c~$([$($c_flags)*])?,&$([$($alias)*])?,n$([$($version)*])?,+$([$($sub)*])?}
		[$($($t)*)?])
	};
	(->$name:expr=>{>$([$($action:tt)*])?,<,@$([$($copyright:tt)*])?,@$([$($license:tt)*])?,=$([$($description:tt)*])?,:$([$($usage:tt)*])?,l~$([$($l_flags:tt)*])?,c~$([$($c_flags:tt)*])?,&$([$($alias:tt)*])?,n$([$($version:tt)*])?,+$([$($sub:tt)*])?}
	[<$(,$($t:tt)*)?])=>{
		$crate::cmd!(->$name=>{>$([$($action)*])?,<[$crate::crate_authors!()],@$([$($copyright)*])?,@$([$($license)*])?,=$([$($description)*])?,:$([$($usage)*])?,l~$([$($l_flags)*])?,c~$([$($c_flags)*])?,&$([$($alias)*])?,n$([$($version)*])?,+$([$($sub)*])?}
		[$($($t)*)?])
	};
	(->$name:expr=>{>$([$($action:tt)*])?,<,@$([$($copyright:tt)*])?,@$([$($license:tt)*])?,=$([$($description:tt)*])?,:$([$($usage:tt)*])?,l~$([$($l_flags:tt)*])?,c~$([$($c_flags:tt)*])?,&$([$($alias:tt)*])?,n$([$($version:tt)*])?,+$([$($sub:tt)*])?}
	[<fc$(,$($t:tt)*)?])=>{
		$crate::cmd!(->$name=>{>$([$($action)*])?,<[$crate::crate_authors!()],@$([$($copyright)*])?,@$([$($license)*])?,=$([$($description)*])?,:$([$($usage)*])?,l~$([$($l_flags)*])?,c~$([$($c_flags)*])?,&$([$($alias)*])?,n$([$($version)*])?,+$([$($sub)*])?}
		[$($($t)*)?])
	};
	(->$name:expr=>{>$([$($action:tt)*])?,<,@$([$($copyright:tt)*])?,@$([$($license:tt)*])?,=$([$($description:tt)*])?,:$([$($usage:tt)*])?,l~$([$($l_flags:tt)*])?,c~$([$($c_flags:tt)*])?,&$([$($alias:tt)*])?,n$([$($version:tt)*])?,+$([$($sub:tt)*])?}
	[<from_crate$(,$($t:tt)*)?])=>{
		$crate::cmd!(->$name=>{>$([$($action)*])?,<[$crate::crate_authors!()],@$([$($copyright)*])?,@$([$($license)*])?,=$([$($description)*])?,:$([$($usage)*])?,l~$([$($l_flags)*])?,c~$([$($c_flags)*])?,&$([$($alias)*])?,n$([$($version)*])?,+$([$($sub)*])?}
		[$($($t)*)?])
	};
	(->$name:expr=>{>$([$($action:tt)*])?,<,@$([$($copyright:tt)*])?,@$([$($license:tt)*])?,=$([$($description:tt)*])?,:$([$($usage:tt)*])?,l~$([$($l_flags:tt)*])?,c~$([$($c_flags:tt)*])?,&$([$($alias:tt)*])?,n$([$($version:tt)*])?,+$([$($sub:tt)*])?}
	[<$authors:literal$(,$($t:tt)*)?])=>{
		$crate::cmd!(->$name=>{>$([$($action)*])?,<[$authors],@$([$($copyright)*])?,@$([$($license)*])?,=$([$($description)*])?,:$([$($usage)*])?,l~$([$($l_flags)*])?,c~$([$($c_flags)*])?,&$([$($alias)*])?,n$([$($version)*])?,+$([$($sub)*])?}
		[$($($t)*)?])
	};
	(->$name:expr=>{>$([$($action:tt)*])?,<,@$([$($copyright:tt)*])?,@$([$($license:tt)*])?,=$([$($description:tt)*])?,:$([$($usage:tt)*])?,l~$([$($l_flags:tt)*])?,c~$([$($c_flags:tt)*])?,&$([$($alias:tt)*])?,n$([$($version:tt)*])?,+$([$($sub:tt)*])?}
	[<[$($authors:tt)*]$(,$($t:tt)*)?])=>{
		$crate::cmd!(->$name=>{>$([$($action)*])?,<[$($authors)*],@$([$($copyright)*])?,@$([$($license)*])?,=$([$($description)*])?,:$([$($usage)*])?,l~$([$($l_flags)*])?,c~$([$($c_flags)*])?,&$([$($alias)*])?,n$([$($version)*])?,+$([$($sub)*])?}
		[$($($t)*)?])
	};
	(->$name:expr=>{>$([$($action:tt)*])?,<,@$([$($copyright:tt)*])?,@$([$($license:tt)*])?,=$([$($description:tt)*])?,:$([$($usage:tt)*])?,l~$([$($l_flags:tt)*])?,c~$([$($c_flags:tt)*])?,&$([$($alias:tt)*])?,n$([$($version:tt)*])?,+$([$($sub:tt)*])?}
	[<$authors:expr$(,$($t:tt)*)?])=>{
		$crate::cmd!(->$name=>{>$([$($action)*])?,<[$authors],@$([$($copyright)*])?,@$([$($license)*])?,=$([$($description)*])?,:$([$($usage)*])?,l~$([$($l_flags)*])?,c~$([$($c_flags)*])?,&$([$($alias)*])?,n$([$($version)*])?,+$([$($sub)*])?}
		[$($($t)*)?])
	};
	(->$name:expr=>{>$([$($action:tt)*])?,<$([$($authors:tt)*])?,@,@$([$($license:tt)*])?,=$([$($description:tt)*])?,:$([$($usage:tt)*])?,l~$([$($l_flags:tt)*])?,c~$([$($c_flags:tt)*])?,&$([$($alias:tt)*])?,n$([$($version:tt)*])?,+$([$($sub:tt)*])?}
	[@$($t:tt)*])=>{
		$crate::cmd!(->$name=>{>$([$($action)*])?,<$([$($authors)*])?,@,@$([$($license)*])?,=$([$($description)*])?,:$([$($usage)*])?,l~$([$($l_flags)*])?,c~$([$($c_flags)*])?,&$([$($alias)*])?,n$([$($version)*])?,+$([$($sub)*])?}
		[(c)$($t)*])
	};
	(->$name:expr=>{>$([$($action:tt)*])?,<$([$($authors:tt)*])?,@,@$([$($license:tt)*])?,=$([$($description:tt)*])?,:$([$($usage:tt)*])?,l~$([$($l_flags:tt)*])?,c~$([$($c_flags:tt)*])?,&$([$($alias:tt)*])?,n$([$($version:tt)*])?,+$([$($sub:tt)*])?}
	[(c)$($copyright:literal),+$(;$($t:tt)*)?])=>{
		$crate::cmd!(->$name=>{>$([$($action)*])?,<$([$($authors)*])?,@[$($copyright),+],@$([$($license)*])?,=$([$($description)*])?,:$([$($usage)*])?,l~$([$($l_flags)*])?,c~$([$($c_flags)*])?,&$([$($alias)*])?,n$([$($version)*])?,+$([$($sub)*])?}
		[$($($t)*)?])
	};
	(->$name:expr=>{>$([$($action:tt)*])?,<$([$($authors:tt)*])?,@,@$([$($license:tt)*])?,=$([$($description:tt)*])?,:$([$($usage:tt)*])?,l~$([$($l_flags:tt)*])?,c~$([$($c_flags:tt)*])?,&$([$($alias:tt)*])?,n$([$($version:tt)*])?,+$([$($sub:tt)*])?}
	[(c)from_crate$(,$($t:tt)*)?])=>{
		$crate::cmd!(->$name=>{>$([$($action)*])?,<$([$($authors)*])?,@[from_crate],@$([$($license)*])?,=$([$($description)*])?,:$([$($usage)*])?,l~$([$($l_flags)*])?,c~$([$($c_flags)*])?,&$([$($alias)*])?,n$([$($version)*])?,+$([$($sub)*])?}
		[$($($t)*)?])
	};
	(->$name:expr=>{>$([$($action:tt)*])?,<$([$($authors:tt)*])?,@,@$([$($license:tt)*])?,=$([$($description:tt)*])?,:$([$($usage:tt)*])?,l~$([$($l_flags:tt)*])?,c~$([$($c_flags:tt)*])?,&$([$($alias:tt)*])?,n$([$($version:tt)*])?,+$([$($sub:tt)*])?}
	[(c)$year:literal$(,)?from_crate$(,$($t:tt)*)?])=>{
		$crate::cmd!(->$name=>{>$([$($action)*])?,<$([$($authors)*])?,@[$year,from_crate],@$([$($license)*])?,=$([$($description)*])?,:$([$($usage)*])?,l~$([$($l_flags)*])?,c~$([$($c_flags)*])?,&$([$($alias)*])?,n$([$($version)*])?,+$([$($sub)*])?}
		[$($($t)*)?])
	};
	(->$name:expr=>{>$([$($action:tt)*])?,<$([$($authors:tt)*])?,@,@$([$($license:tt)*])?,=$([$($description:tt)*])?,:$([$($usage:tt)*])?,l~$([$($l_flags:tt)*])?,c~$([$($c_flags:tt)*])?,&$([$($alias:tt)*])?,n$([$($version:tt)*])?,+$([$($sub:tt)*])?}
	[(c)$($year:literal)?$(,)?...$(,$($t:tt)*)?])=>{
		$crate::cmd!(->$name=>{>$([$($action)*])?,<$([$($authors)*])?,@[$($year,)?...],@$([$($license)*])?,=$([$($description)*])?,:$([$($usage)*])?,l~$([$($l_flags)*])?,c~$([$($c_flags)*])?,&$([$($alias)*])?,n$([$($version)*])?,+$([$($sub)*])?}
		[$($($t)*)?])
	};
	(->$name:expr=>{>$([$($action:tt)*])?,<$([$($authors:tt)*])?,@,@$([$($license:tt)*])?,=$([$($description:tt)*])?,:$([$($usage:tt)*])?,l~$([$($l_flags:tt)*])?,c~$([$($c_flags:tt)*])?,&$([$($alias:tt)*])?,n$([$($version:tt)*])?,+$([$($sub:tt)*])?}
	[(c)$($copyright:literal)+$(,$($t:tt)*)?])=>{
		$crate::cmd!(->$name=>{>$([$($action)*])?,<$([$($authors)*])?,@[$($copyright),+],@$([$($license)*])?,=$([$($description)*])?,:$([$($usage)*])?,l~$([$($l_flags)*])?,c~$([$($c_flags)*])?,&$([$($alias)*])?,n$([$($version)*])?,+$([$($sub)*])?}
		[$($($t)*)?])
	};
	(->$name:expr=>{>$([$($action:tt)*])?,<$([$($authors:tt)*])?,@,@$([$($license:tt)*])?,=$([$($description:tt)*])?,:$([$($usage:tt)*])?,l~$([$($l_flags:tt)*])?,c~$([$($c_flags:tt)*])?,&$([$($alias:tt)*])?,n$([$($version:tt)*])?,+$([$($sub:tt)*])?}
	[(c)$copyright:literal$(,$($t:tt)*)?])=>{
		$crate::cmd!(->$name=>{>$([$($action)*])?,<$([$($authors)*])?,@[$copyright],@$([$($license)*])?,=$([$($description)*])?,:$([$($usage)*])?,l~$([$($l_flags)*])?,c~$([$($c_flags)*])?,&$([$($alias)*])?,n$([$($version)*])?,+$([$($sub)*])?}
		[$($($t)*)?])
	};
	(->$name:expr=>{>$([$($action:tt)*])?,<$([$($authors:tt)*])?,@,@$([$($license:tt)*])?,=$([$($description:tt)*])?,:$([$($usage:tt)*])?,l~$([$($l_flags:tt)*])?,c~$([$($c_flags:tt)*])?,&$([$($alias:tt)*])?,n$([$($version:tt)*])?,+$([$($sub:tt)*])?}
	[(c)[$($copyright:tt)*]$(,$($t:tt)*)?])=>{
		$crate::cmd!(->$name=>{>$([$($action)*])?,<$([$($authors)*])?,@[$($copyright)*],@$([$($license)*])?,=$([$($description)*])?,:$([$($usage)*])?,l~$([$($l_flags)*])?,c~$([$($c_flags)*])?,&$([$($alias)*])?,n$([$($version)*])?,+$([$($sub)*])?}
		[$($($t)*)?])
	};
	(->$name:expr=>{>$([$($action:tt)*])?,<$([$($authors:tt)*])?,@[$($copyright:tt)*],@,=$([$($description:tt)*])?,:$([$($usage:tt)*])?,l~$([$($l_flags:tt)*])?,c~$([$($c_flags:tt)*])?,&$([$($alias:tt)*])?,n$([$($version:tt)*])?,+$([$($sub:tt)*])?}
	[@$($t:tt)*])=>{
		$crate::cmd!(->$name=>{>$([$($action)*])?,<$([$($authors)*])?,@[$($copyright)*],@,=$([$($description)*])?,:$([$($usage)*])?,l~$([$($l_flags)*])?,c~$([$($c_flags)*])?,&$([$($alias)*])?,n$([$($version)*])?,+$([$($sub)*])?}
		[(l)$($t)*])
	};
	(->$name:expr=>{>$([$($action:tt)*])?,<$([$($authors:tt)*])?,@[$($copyright:tt)*],@,=$([$($description:tt)*])?,:$([$($usage:tt)*])?,l~$([$($l_flags:tt)*])?,c~$([$($c_flags:tt)*])?,&$([$($alias:tt)*])?,n$([$($version:tt)*])?,+$([$($sub:tt)*])?}
	[(l)[$($license:tt)*]$($t:tt)*])=>{
		$crate::cmd!(->$name=>{>$([$($action)*])?,<$([$($authors)*])?,@[$($copyright)*],@[$($license)*],=$([$($description)*])?,:$([$($usage)*])?,l~$([$($l_flags)*])?,c~$([$($c_flags)*])?,&$([$($alias)*])?,n$([$($version)*])?,+$([$($sub)*])?}
		[$($t)*])
	};
	(->$name:expr=>{>$([$($action:tt)*])?,<$([$($authors:tt)*])?,@[$($copyright:tt)*],@,=$([$($description:tt)*])?,:$([$($usage:tt)*])?,l~$([$($l_flags:tt)*])?,c~$([$($c_flags:tt)*])?,&$([$($alias:tt)*])?,n$([$($version:tt)*])?,+$([$($sub:tt)*])?}
	[(l)$lexpr:literal$(,)?$(->)?$lcontent:literal$($t:tt)*])=>{
		$crate::cmd!(->$name=>{>$([$($action)*])?,<$([$($authors)*])?,@[$($copyright)*],@[$lexpr,->$lcontent],=$([$($description)*])?,:$([$($usage)*])?,l~$([$($l_flags)*])?,c~$([$($c_flags)*])?,&$([$($alias)*])?,n$([$($version)*])?,+$([$($sub)*])?}
		[$($t)*])
	};
	(->$name:expr=>{>$([$($action:tt)*])?,<$([$($authors:tt)*])?,@[$($copyright:tt)*],@,=$([$($description:tt)*])?,:$([$($usage:tt)*])?,l~$([$($l_flags:tt)*])?,c~$([$($c_flags:tt)*])?,&$([$($alias:tt)*])?,n$([$($version:tt)*])?,+$([$($sub:tt)*])?}
	[(l)$lexpr:expr,$(->)?$lcontent:literal$($t:tt)*])=>{
		$crate::cmd!(->$name=>{>$([$($action)*])?,<$([$($authors)*])?,@[$($copyright)*],@[$lexpr,->$lcontent],=$([$($description)*])?,:$([$($usage)*])?,l~$([$($l_flags)*])?,c~$([$($c_flags)*])?,&$([$($alias)*])?,n$([$($version)*])?,+$([$($sub)*])?}
		[$($t)*])
	};
	(->$name:expr=>{>$([$($action:tt)*])?,<$([$($authors:tt)*])?,@[$($copyright:tt)*],@,=$([$($description:tt)*])?,:$([$($usage:tt)*])?,l~$([$($l_flags:tt)*])?,c~$([$($c_flags:tt)*])?,&$([$($alias:tt)*])?,n$([$($version:tt)*])?,+$([$($sub:tt)*])?}
	[(l)$lexpr:literal,->$lcontent:expr$(,$t:tt)*])=>{
		$crate::cmd!(->$name=>{>$([$($action)*])?,<$([$($authors)*])?,@[$($copyright)*],@[$lexpr,->$lcontent],=$([$($description)*])?,:$([$($usage)*])?,l~$([$($l_flags)*])?,c~$([$($c_flags)*])?,&$([$($alias)*])?,n$([$($version)*])?,+$([$($sub)*])?}
		[$($t)*])
	};
	(->$name:expr=>{>$([$($action:tt)*])?,<$([$($authors:tt)*])?,@[$($copyright:tt)*],@,=$([$($description:tt)*])?,:$([$($usage:tt)*])?,l~$([$($l_flags:tt)*])?,c~$([$($c_flags:tt)*])?,&$([$($alias:tt)*])?,n$([$($version:tt)*])?,+$([$($sub:tt)*])?}
	[(l)$lexpr:literal,->$lcontent:expr$(;$t:tt)*])=>{
		$crate::cmd!(->$name=>{>$([$($action)*])?,<$([$($authors)*])?,@[$($copyright)*],@[$lexpr,->$lcontent],=$([$($description)*])?,:$([$($usage)*])?,l~$([$($l_flags)*])?,c~$([$($c_flags)*])?,&$([$($alias)*])?,n$([$($version)*])?,+$([$($sub)*])?}
		[$($t)*])
	};
	(->$name:expr=>{>$([$($action:tt)*])?,<$([$($authors:tt)*])?,@[$($copyright:tt)*],@,=$([$($description:tt)*])?,:$([$($usage:tt)*])?,l~$([$($l_flags:tt)*])?,c~$([$($c_flags:tt)*])?,&$([$($alias:tt)*])?,n$([$($version:tt)*])?,+$([$($sub:tt)*])?}
	[(l)$lexpr:literal$(,)?$(fp)?:$lfile:expr$(,$t:tt)*])=>{
		$crate::cmd!(->$name=>{>$([$($action)*])?,<$([$($authors)*])?,@[$($copyright)*],@[$lexpr,:$lfile],=$([$($description)*])?,:$([$($usage)*])?,l~$([$($l_flags)*])?,c~$([$($c_flags)*])?,&$([$($alias)*])?,n$([$($version)*])?,+$([$($sub)*])?}
		[$($t)*])
	};
	(->$name:expr=>{>$([$($action:tt)*])?,<$([$($authors:tt)*])?,@[$($copyright:tt)*],@,=$([$($description:tt)*])?,:$([$($usage:tt)*])?,l~$([$($l_flags:tt)*])?,c~$([$($c_flags:tt)*])?,&$([$($alias:tt)*])?,n$([$($version:tt)*])?,+$([$($sub:tt)*])?}
	[(l)$lexpr:literal,$lcontent:expr$(;$t:tt)*])=>{
		$crate::cmd!(->$name=>{>$([$($action)*])?,<$([$($authors)*])?,@[$($copyright)*],@[$lexpr,:$lfile],=$([$($description)*])?,:$([$($usage)*])?,l~$([$($l_flags)*])?,c~$([$($c_flags)*])?,&$([$($alias)*])?,n$([$($version)*])?,+$([$($sub)*])?}
		[$($t)*])
	};
	(->$name:expr=>{>$([$($action:tt)*])?,<$([$($authors:tt)*])?,@[$($copyright:tt)*],@,=$([$($description:tt)*])?,:$([$($usage:tt)*])?,l~$([$($l_flags:tt)*])?,c~$([$($c_flags:tt)*])?,&$([$($alias:tt)*])?,n$([$($version:tt)*])?,+$([$($sub:tt)*])?}
	[(l)$lexpr:literal,$lcontent:expr$(,$t:tt)*])=>{
		$crate::cmd!(->$name=>{>$([$($action)*])?,<$([$($authors)*])?,@[$($copyright)*],@[$lexpr,$lcontent],=$([$($description)*])?,:$([$($usage)*])?,l~$([$($l_flags)*])?,c~$([$($c_flags)*])?,&$([$($alias)*])?,n$([$($version)*])?,+$([$($sub)*])?}
		[$($t)*])
	};
	(->$name:expr=>{>$([$($action:tt)*])?,<$([$($authors:tt)*])?,@[$($copyright:tt)*],@,=$([$($description:tt)*])?,:$([$($usage:tt)*])?,l~$([$($l_flags:tt)*])?,c~$([$($c_flags:tt)*])?,&$([$($alias:tt)*])?,n$([$($version:tt)*])?,+$([$($sub:tt)*])?}
	[(l)$lexpr:literal,$lcontent:expr$(;$t:tt)*])=>{
		$crate::cmd!(->$name=>{>$([$($action)*])?,<$([$($authors)*])?,@[$($copyright)*],@[$lexpr,$lcontent],=$([$($description)*])?,:$([$($usage)*])?,l~$([$($l_flags)*])?,c~$([$($c_flags)*])?,&$([$($alias)*])?,n$([$($version)*])?,+$([$($sub)*])?}
		[$($t)*])
	};
	(->$name:expr=>{>$([$($action:tt)*])?,<$([$($authors:tt)*])?,@[$($copyright:tt)*],@,=$([$($description:tt)*])?,:$([$($usage:tt)*])?,l~$([$($l_flags:tt)*])?,c~$([$($c_flags:tt)*])?,&$([$($alias:tt)*])?,n$([$($version:tt)*])?,+$([$($sub:tt)*])?}
	[(l)$lexpr:literal $lcontent:expr$(;$t:tt)*])=>{
		$crate::cmd!(->$name=>{>$([$($action)*])?,<$([$($authors)*])?,@[$($copyright)*],@[$lexpr,$lcontent],=$([$($description)*])?,:$([$($usage)*])?,l~$([$($l_flags)*])?,c~$([$($c_flags)*])?,&$([$($alias)*])?,n$([$($version)*])?,+$([$($sub)*])?}
		[$($t)*])
	};
	(->$name:expr=>{>$([$($action:tt)*])?,<$([$($authors:tt)*])?,@[$($copyright:tt)*],@,=$([$($description:tt)*])?,:$([$($usage:tt)*])?,l~$([$($l_flags:tt)*])?,c~$([$($c_flags:tt)*])?,&$([$($alias:tt)*])?,n$([$($version:tt)*])?,+$([$($sub:tt)*])?}
	[(l)$license:expr$(,$($t:tt)*)?])=>{
		$crate::cmd!(->$name=>{>$([$($action)*])?,<$([$($authors)*])?,@[$($copyright)*],@[->$license],=$([$($description)*])?,:$([$($usage)*])?,l~$([$($l_flags)*])?,c~$([$($c_flags)*])?,&$([$($alias)*])?,n$([$($version)*])?,+$([$($sub)*])?}
		[$($($t)*)?])
	};
	(->$name:expr=>{>$([$($action:tt)*])?,<$([$($authors:tt)*])?,@$([$($copyright:tt)*])?,@$([$($license:tt)*])?,=,:$([$($usage:tt)*])?,l~$([$($l_flags:tt)*])?,c~$([$($c_flags:tt)*])?,&$([$($alias:tt)*])?,n$([$($version:tt)*])?,+$([$($sub:tt)*])?}
	[=$description:literal$($t:tt)*])=>{
		$crate::cmd!(->$name=>{>$([$($action)*])?,<$([$($authors)*])?,@$([$($copyright)*])?,@$([$($license)*])?,=[$description],:$([$($usage)*])?,l~$([$($l_flags)*])?,c~$([$($c_flags)*])?,&$([$($alias)*])?,n$([$($version)*])?,+$([$($sub)*])?}
		[$($t)*])
	};
	(->$name:expr=>{>$([$($action:tt)*])?,<$([$($authors:tt)*])?,@$([$($copyright:tt)*])?,@$([$($license:tt)*])?,=,:$([$($usage:tt)*])?,l~$([$($l_flags:tt)*])?,c~$([$($c_flags:tt)*])?,&$([$($alias:tt)*])?,n$([$($version:tt)*])?,+$([$($sub:tt)*])?}
	[=[$($description:tt)*]$($t:tt)*])=>{
		$crate::cmd!(->$name=>{>$([$($action)*])?,<$([$($authors)*])?,@$([$($copyright)*])?,@$([$($license)*])?,=[$($description)*],:$([$($usage)*])?,l~$([$($l_flags)*])?,c~$([$($c_flags)*])?,&$([$($alias)*])?,n$([$($version)*])?,+$([$($sub)*])?}
		[$($t)*])
	};
	(->$name:expr=>{>$([$($action:tt)*])?,<$([$($authors:tt)*])?,@$([$($copyright:tt)*])?,@$([$($license:tt)*])?,=$([$($description:tt)*])?,:,l~$([$($l_flags:tt)*])?,c~$([$($c_flags:tt)*])?,&$([$($alias:tt)*])?,n$([$($version:tt)*])?,+$([$($sub:tt)*])?}
	[:$usage:ident$(,$($t:tt)*)?])=>{
		$crate::cmd!(->$name=>{>$([$($action)*])?,<$([$($authors)*])?,@$([$($copyright)*])?,@$([$($license)*])?,=$([$($description)*])?,:[$usage],l~$([$($l_flags)*])?,c~$([$($c_flags)*])?,&$([$($alias)*])?,n$([$($version)*])?,+$([$($sub)*])?}
		[$($($t)*)?])
	};
	(->$name:expr=>{>$([$($action:tt)*])?,<$([$($authors:tt)*])?,@$([$($copyright:tt)*])?,@$([$($license:tt)*])?,=$([$($description:tt)*])?,:,l~$([$($l_flags:tt)*])?,c~$([$($c_flags:tt)*])?,&$([$($alias:tt)*])?,n$([$($version:tt)*])?,+$([$($sub:tt)*])?}
	[:$usage:literal$($t:tt)*])=>{
		$crate::cmd!(->$name=>{>$([$($action)*])?,<$([$($authors)*])?,@$([$($copyright)*])?,@$([$($license)*])?,=$([$($description)*])?,:[$usage],l~$([$($l_flags)*])?,c~$([$($c_flags)*])?,&$([$($alias)*])?,n$([$($version)*])?,+$([$($sub)*])?}
		[$($t)*])
	};
	(->$name:expr=>{>$([$($action:tt)*])?,<$([$($authors:tt)*])?,@$([$($copyright:tt)*])?,@$([$($license:tt)*])?,=$([$($description:tt)*])?,:,l~$([$($l_flags:tt)*])?,c~$([$($c_flags:tt)*])?,&$([$($alias:tt)*])?,n$([$($version:tt)*])?,+$([$($sub:tt)*])?}
	[:[$($usage:tt)*]$($t:tt)*])=>{
		$crate::cmd!(->$name=>{>$([$($action)*])?,<$([$($authors)*])?,@$([$($copyright)*])?,@$([$($license)*])?,=$([$($description)*])?,:[$($usage)*],l~$([$($l_flags)*])?,c~$([$($c_flags)*])?,&$([$($alias)*])?,n$([$($version)*])?,+$([$($sub)*])?}
		[$($t)*])
	};
	(->$name:expr=>{>$([$($action:tt)*])?,<$([$($authors:tt)*])?,@$([$($copyright:tt)*])?,@$([$($license:tt)*])?,=$([$($description:tt)*])?,:,l~$([$($l_flags:tt)*])?,c~$([$($c_flags:tt)*])?,&$([$($alias:tt)*])?,n$([$($version:tt)*])?,+$([$($sub:tt)*])?}
	[:$usage:expr$(,$($t:tt)*)?])=>{
		$crate::cmd!(->$name=>{>$([$($action)*])?,<$([$($authors)*])?,@$([$($copyright)*])?,@$([$($license)*])?,=$([$($description)*])?,:[->$usage],l~$([$($l_flags)*])?,c~$([$($c_flags)*])?,&$([$($alias)*])?,n$([$($version)*])?,+$([$($sub)*])?}
		[$($($t)*)?])
	};
	(->$name:expr=>{$($args:tt)+}
	[l~{$($lf:tt)*}$($t:tt)*])=>{
		$crate::cmd!(->$name=>{$($args)+}
		[l~[$($lf)*]$($t)*])
	};
	(->$name:expr=>{$($args:tt)+}
	[l~($($lf:tt)*)$($t:tt)*])=>{
		$crate::cmd!(->$name=>{$($args)+}
		[l~[$($lf)*]$($t)*])
	};
	(->$name:expr=>{>$($args:tt)+}
	[l~,$($t:tt)*])=>{
		$crate::cmd!(->$name=>{>$($args)+}[l~[],$($t)*])
	};
	(->$name:expr=>{>$($args:tt)+}
	[l~None$($t:tt)*])=>{
		$crate::cmd!(->$name=>{>$($args)+}[l~[]$($t)*])
	};
	(->$name:expr=>{>$([$($action:tt)*])?,<$([$($authors:tt)*])?,@$([$($copyright:tt)*])?,@$([$($license:tt)*])?,=$([$($description:tt)*])?,:$([$($usage:tt)*])?,l~$([$($l_flags:tt)*])?,c~$([$($c_flags:tt)*])?,&$([$($alias:tt)*])?,n$([$($version:tt)*])?,+$([$($sub:tt)*])?}
	[l~[$($lf:tt)*]$($t:tt)*])=>{
		$crate::cmd!(->$name=>{>$([$($action)*])?,<$([$($authors)*])?,@$([$($copyright)*])?,@$([$($license)*])?,=$([$($description)*])?,:$([$($usage)*])?,l~[$($($l_flags)*)?$($lf)*],c~$([$($c_flags)*])?,&$([$($alias)*])?,n$([$($version)*])?,+$([$($sub)*])?}
		[$($t)*])
	};
	(->$name:expr=>{>$($args:tt)+}
	[c~{$($cf:tt)*}$($t:tt)*])=>{
		$crate::cmd!(->$name=>{>$($args)+}
		[c~[$($cf)*]$($t)*])
	};
	(->$name:expr=>{>$($args:tt)+}
	[c~($($cf:tt)*)$($t:tt)*])=>{
		$crate::cmd!(->$name=>{>$($args)+}
		[c~[$($cf)*]$($t)*])
	};
	(->$name:expr=>{>$($args:tt)+}
	[c~,$($t:tt)*])=>{
		$crate::cmd!(->$name=>{>$($args)+}[c~[],$($t)*])
	};
	(->$name:expr=>{>$($args:tt)+}
	[c~None$($t:tt)*])=>{
		$crate::cmd!(->$name=>{>$($args)+}[c~[]$($t)*])
	};
	(->$name:expr=>{>$([$($action:tt)*])?,<$([$($authors:tt)*])?,@$([$($copyright:tt)*])?,@$([$($license:tt)*])?,=$([$($description:tt)*])?,:$([$($usage:tt)*])?,l~$([$($l_flags:tt)*])?,c~$([$($c_flags:tt)*])?,&$([$($alias:tt)*])?,n$([$($version:tt)*])?,+$([$($sub:tt)*])?}
	[c~[$($cf:tt)*]$($t:tt)*])=>{
		$crate::cmd!(->$name=>{>$([$($action)*])?,<$([$($authors)*])?,@$([$($copyright)*])?,@$([$($license)*])?,=$([$($description)*])?,:$([$($usage)*])?,l~$([$($l_flags)*])?,c~[$($($c_flags)*)?$($cf)*],&$([$($alias)*])?,n$([$($version)*])?,+$([$($sub)*])?}
		[$($t)*])
	};
	(->$name:expr=>{>$([$($action:tt)*])?,<$([$($authors:tt)*])?,@$([$($copyright:tt)*])?,@$([$($license:tt)*])?,=$([$($description:tt)*])?,:$([$($usage:tt)*])?,l~$([$($l_flags:tt)*])?,c~,&$([$($alias:tt)*])?,n$([$($version:tt)*])?,+$([$($sub:tt)*])?}
	[c~Vector$b:tt$(,$($t:tt)*)?])=>{
		$crate::cmd!(->$name=>{>$([$($action)*])?,<$([$($authors)*])?,@$([$($copyright)*])?,@$([$($license)*])?,=$([$($description)*])?,:$([$($usage)*])?,l~$([$($l_flags)*])?,c~[->Vector$b],&$([$($alias)*])?,n$([$($version)*])?,+$([$($sub)*])?}
		[$($($t)*)?])
	};
	(->$name:expr=>{>$([$($action:tt)*])?,<$([$($authors:tt)*])?,@$([$($copyright:tt)*])?,@$([$($license:tt)*])?,=$([$($description:tt)*])?,:$([$($usage:tt)*])?,l~$([$($l_flags:tt)*])?,c~,&$([$($alias:tt)*])?,n$([$($version:tt)*])?,+$([$($sub:tt)*])?}
	[c~$macro:ident!$b:tt$(,$($t:tt)*)?])=>{
		$crate::cmd!(->$name=>{>$([$($action)*])?,<$([$($authors)*])?,@$([$($copyright)*])?,@$([$($license)*])?,=$([$($description)*])?,:$([$($usage)*])?,l~$([$($l_flags)*])?,c~[->$macro!$b],&$([$($alias)*])?,n$([$($version)*])?,+$([$($sub)*])?}
		[$($($t)*)?])
	};

	(->$name:expr=>{>$([$($action:tt)*])?,<$([$($authors:tt)*])?,@$([$($copyright:tt)*])?,@$([$($license:tt)*])?,=$([$($description:tt)*])?,:$([$($usage:tt)*])?,l~,c~$([$($c_flags:tt)*])?,&$([$($alias:tt)*])?,n$([$($version:tt)*])?,+$([$($sub:tt)*])?}
	[l~Vector$b:tt$(,$($t:tt)*)?])=>{
		$crate::cmd!(->$name=>{>$([$($action)*])?,<$([$($authors)*])?,@$([$($copyright)*])?,@$([$($license)*])?,=$([$($description)*])?,:$([$($usage)*])?,l~[->Vector$b],c~$([$($c_flags)*])?,&$([$($alias)*])?,n$([$($version)*])?,+$([$($sub)*])?}
		[$($($t)*)?])
	};
	(->$name:expr=>{>$([$($action:tt)*])?,<$([$($authors:tt)*])?,@$([$($copyright:tt)*])?,@$([$($license:tt)*])?,=$([$($description:tt)*])?,:$([$($usage:tt)*])?,l~,c~$([$($c_flags:tt)*])?,&$([$($alias:tt)*])?,n$([$($version:tt)*])?,+$([$($sub:tt)*])?}
	[l~$macro:ident!$b:tt$(,$($t:tt)*)?])=>{
		$crate::cmd!(->$name=>{>$([$($action)*])?,<$([$($authors)*])?,@$([$($copyright)*])?,@$([$($license)*])?,=$([$($description)*])?,:$([$($usage)*])?,l~[->$macro!$b],c~$([$($c_flags)*])?,&$([$($alias)*])?,n$([$($version)*])?,+$([$($sub)*])?}
		[$($($t)*)?])
	};
	(->$name:expr=>{$($args:tt)+}[l~$fn:tt$fa:tt$(;$($t:tt)*)?])=>{
			$crate::cmd!(->$name=>{$($args)+}[l~[$fn$fa]$(,$($t)*)?])
		};
	(->$name:expr=>{$($args:tt)+}[l~$fn:tt$sep:tt$fa:tt$(;$($t:tt)*)?])=>{
		$crate::cmd!(->$name=>{$($args)+}[l~[$fn$sep$fa]$(,$($t)*)?])
	};
	(->$name:expr=>{$($args:tt)+}[l~$fn:tt$fa:tt$(,$($t:tt)*)?])=>{
		$crate::cmd!(->$name=>{$($args)+}[l~[$fn$fa]$(,$($t)*)?])
	};
	(->$name:expr=>{$($args:tt)+}[l~$fn:tt$sep:tt$fa:tt$(,$($t:tt)*)?])=>{
		$crate::cmd!(->$name=>{$($args)+}[l~[$fn$sep$fa]$(,$($t)*)?])
	};
	(->$name:expr=>{$($args:tt)+}[c~$fn:tt$fa:tt$(;$($t:tt)*)?])=>{
			$crate::cmd!(->$name=>{$($args)+}[c~[$fn$fa]$(,$($t)*)?])
		};
	(->$name:expr=>{$($args:tt)+}[c~$fn:tt$sep:tt$fa:tt$(;$($t:tt)*)?])=>{
		$crate::cmd!(->$name=>{$($args)+}[c~[$fn$sep$fa]$(,$($t)*)?])
	};
	(->$name:expr=>{$($args:tt)+}[c~$fn:tt$fa:tt$(,$($t:tt)*)?])=>{
		$crate::cmd!(->$name=>{$($args)+}[c~[$fn$fa]$(,$($t)*)?])
	};
	(->$name:expr=>{$($args:tt)+}[c~$fn:tt$sep:tt$fa:tt$(,$($t:tt)*)?])=>{
		$crate::cmd!(->$name=>{$($args)+}[c~[$fn$sep$fa]$(,$($t)*)?])
	};
	(->$name:expr=>{>$([$($action:tt)*])?,<$([$($authors:tt)*])?,@$([$($copyright:tt)*])?,@$([$($license:tt)*])?,=$([$($description:tt)*])?,:$([$($usage:tt)*])?,l~$([$($l_flags:tt)*])?,c~,&$([$($alias:tt)*])?,n$([$($version:tt)*])?,+$([$($sub:tt)*])?}
	[c~$c_flags:expr$(,$($t:tt)*)?])=>{
		$crate::cmd!(->$name=>{>$([$($action)*])?,<$([$($authors)*])?,@$([$($copyright)*])?,@$([$($license)*])?,=$([$($description)*])?,:$([$($usage)*])?,l~$([$($l_flags)*])?,c~[->$c_flags],&$([$($alias)*])?,n$([$($version)*])?,+$([$($sub)*])?}
		[$($($t)*)?])
	};
	(->$name:expr=>{>$([$($action:tt)*])?,<$([$($authors:tt)*])?,@$([$($copyright:tt)*])?,@$([$($license:tt)*])?,=$([$($description:tt)*])?,:$([$($usage:tt)*])?,l~,c~$([$($c_flags:tt)*])?,&$([$($alias:tt)*])?,n$([$($version:tt)*])?,+$([$($sub:tt)*])?}
	[l~$l_flags:expr$(,$($t:tt)*)?])=>{
		$crate::cmd!(->$name=>{>$([$($action)*])?,<$([$($authors)*])?,@$([$($copyright)*])?,@$([$($license)*])?,=$([$($description)*])?,:$([$($usage)*])?,l~[->$l_flags],c~$([$($c_flags)*])?,&$([$($alias)*])?,n$([$($version)*])?,+$([$($sub)*])?}
		[$($($t)*)?])
	};
	(->$name:expr=>{>$([$($action:tt)*])?,<$([$($authors:tt)*])?,@$([$($copyright:tt)*])?,@$([$($license:tt)*])?,=$([$($description:tt)*])?,:$([$($usage:tt)*])?,l~$([$($l_flags:tt)*])?,c~$([$($c_flags:tt)*])?,&$([$($at:tt)*])?,n$([$($version:tt)*])?,+$([$($sub:tt)*])?}
	[&$(None)?$(,$($t:tt)*)?])=>{
		$crate::cmd!(->$name=>{>$([$($action)*])?,<$([$($authors)*])?,@$([$($copyright)*])?,@$([$($license)*])?,=$([$($description)*])?,:$([$($usage)*])?,l~$([$($l_flags)*])?,c~$([$($c_flags)*])?,&[$($($at)*)?],n$([$($version)*])?,+$([$($sub)*])?}
		[$($($t)*)?])
	};
	(->$name:expr=>{>$([$($action:tt)*])?,<$([$($authors:tt)*])?,@$([$($copyright:tt)*])?,@$([$($license:tt)*])?,=$([$($description:tt)*])?,:$([$($usage:tt)*])?,l~$([$($l_flags:tt)*])?,c~$([$($c_flags:tt)*])?,&$([$($at:tt)*])?,n$([$($version:tt)*])?,+$([$($sub:tt)*])?}
	[&[$($alias:tt)*]$(,$($t:tt)*)?])=>{
		$crate::cmd!(->$name=>{>$([$($action)*])?,<$([$($authors)*])?,@$([$($copyright)*])?,@$([$($license)*])?,=$([$($description)*])?,:$([$($usage)*])?,l~$([$($l_flags)*])?,c~$([$($c_flags)*])?,&[$($($at)*)?$($alias)*],n$([$($version)*])?,+$([$($sub)*])?}
		[$($($t)*)?])
	};
	(->$name:expr=>{>$([$($action:tt)*])?,<$([$($authors:tt)*])?,@$([$($copyright:tt)*])?,@$([$($license:tt)*])?,=$([$($description:tt)*])?,:$([$($usage:tt)*])?,l~$([$($l_flags:tt)*])?,c~$([$($c_flags:tt)*])?,&,n$([$($version:tt)*])?,+$([$($sub:tt)*])?}
	[&Vector$vt:tt$($t:tt)*])=>{
		$crate::cmd!(->$name=>{>$([$($action)*])?,<$([$($authors)*])?,@$([$($copyright)*])?,@$([$($license)*])?,=$([$($description)*])?,:$([$($usage)*])?,l~$([$($l_flags)*])?,c~$([$($c_flags)*])?,&[->Vector$vt],n$([$($version)*])?,+$([$($sub)*])?}
		[$($t)*])
	};
	(->$name:expr=>{>$([$($action:tt)*])?,<$([$($authors:tt)*])?,@$([$($copyright:tt)*])?,@$([$($license:tt)*])?,=$([$($description:tt)*])?,:$([$($usage:tt)*])?,l~$([$($l_flags:tt)*])?,c~$([$($c_flags:tt)*])?,&,n$([$($version:tt)*])?,+$([$($sub:tt)*])?}
	[&vector!$vt:tt$($t:tt)*])=>{
		$crate::cmd!(->$name=>{>$([$($action)*])?,<$([$($authors)*])?,@$([$($copyright)*])?,@$([$($license)*])?,=$([$($description)*])?,:$([$($usage)*])?,l~$([$($l_flags)*])?,c~$([$($c_flags)*])?,&[->vector!$vt],n$([$($version)*])?,+$([$($sub)*])?}
		[$($t)*])
	};
	(->$name:expr=>{>$([$($action:tt)*])?,<$([$($authors:tt)*])?,@$([$($copyright:tt)*])?,@$([$($license:tt)*])?,=$([$($description:tt)*])?,:$([$($usage:tt)*])?,l~$([$($l_flags:tt)*])?,c~$([$($c_flags:tt)*])?,&$([$($at:tt)*])?,n$([$($version:tt)*])?,+$([$($sub:tt)*])?}
	[&$alias:ident$(,)?n $($t:tt)*])=>{
		$crate::cmd!(->$name=>{>$([$($action)*])?,<$([$($authors)*])?,@$([$($copyright)*])?,@$([$($license)*])?,=$([$($description)*])?,:$([$($usage)*])?,l~$([$($l_flags)*])?,c~$([$($c_flags)*])?,&[$($($at)*)?$alias,],n$([$($version)*])?,+$([$($sub)*])?}
		[n$($t)*])
	};
	(->$name:expr=>{>$([$($action:tt)*])?,<$([$($authors:tt)*])?,@$([$($copyright:tt)*])?,@$([$($license:tt)*])?,=$([$($description:tt)*])?,:$([$($usage:tt)*])?,l~$([$($l_flags:tt)*])?,c~$([$($c_flags:tt)*])?,&$([$($at:tt)*])?,n$([$($version:tt)*])?,+$([$($sub:tt)*])?}
	[&$alias:ident$(,)?$i:ident:$($t:tt)*])=>{
		$crate::cmd!(->$name=>{>$([$($action)*])?,<$([$($authors)*])?,@$([$($copyright)*])?,@$([$($license)*])?,=$([$($description)*])?,:$([$($usage)*])?,l~$([$($l_flags)*])?,c~$([$($c_flags)*])?,&[$($($at)*)?$alias,],n$([$($version)*])?,+$([$($sub)*])?}
		[$i:$($t)*])
	};
	(->$name:expr=>{>$([$($action:tt)*])?,<$([$($authors:tt)*])?,@$([$($copyright:tt)*])?,@$([$($license:tt)*])?,=$([$($description:tt)*])?,:$([$($usage:tt)*])?,l~$([$($l_flags:tt)*])?,c~$([$($c_flags:tt)*])?,&$([$($at:tt)*])?,n$([$($version:tt)*])?,+$([$($sub:tt)*])?}
	[&$alias:ident$(,)?$i:ident=$($t:tt)*])=>{
		$crate::cmd!(->$name=>{>$([$($action)*])?,<$([$($authors)*])?,@$([$($copyright)*])?,@$([$($license)*])?,=$([$($description)*])?,:$([$($usage)*])?,l~$([$($l_flags)*])?,c~$([$($c_flags)*])?,&[$($($at)*)?$alias,],n$([$($version)*])?,+$([$($sub)*])?}
		[$i=$($t)*])
	};
	(->$name:expr=>{>$([$($action:tt)*])?,<$([$($authors:tt)*])?,@$([$($copyright:tt)*])?,@$([$($license:tt)*])?,=$([$($description:tt)*])?,:$([$($usage:tt)*])?,l~$([$($l_flags:tt)*])?,c~$([$($c_flags:tt)*])?,&$([$($at:tt)*])?,n$([$($version:tt)*])?,+$([$($sub:tt)*])?}
	[&$alias:ident$(,)?$next_alias:ident$($t:tt)*])=>{
		$crate::cmd!(->$name=>{>$([$($action)*])?,<$([$($authors)*])?,@$([$($copyright)*])?,@$([$($license)*])?,=$([$($description)*])?,:$([$($usage)*])?,l~$([$($l_flags)*])?,c~$([$($c_flags)*])?,&[$($($at)*)?$alias,],n$([$($version)*])?,+$([$($sub)*])?}
		[&$next_alias$($t)*])
	};
	(->$name:expr=>{>$([$($action:tt)*])?,<$([$($authors:tt)*])?,@$([$($copyright:tt)*])?,@$([$($license:tt)*])?,=$([$($description:tt)*])?,:$([$($usage:tt)*])?,l~$([$($l_flags:tt)*])?,c~$([$($c_flags:tt)*])?,&$([$($at:tt)*])?,n$([$($version:tt)*])?,+$([$($sub:tt)*])?}
	[&$alias:literal$(,)?n$($t:tt)*])=>{
		$crate::cmd!(->$name=>{>$([$($action)*])?,<$([$($authors)*])?,@$([$($copyright)*])?,@$([$($license)*])?,=$([$($description)*])?,:$([$($usage)*])?,l~$([$($l_flags)*])?,c~$([$($c_flags)*])?,&[$($($at)*)?$alias,],n$([$($version)*])?,+$([$($sub)*])?}
		[n$($t)*])
	};
	(->$name:expr=>{>$([$($action:tt)*])?,<$([$($authors:tt)*])?,@$([$($copyright:tt)*])?,@$([$($license:tt)*])?,=$([$($description:tt)*])?,:$([$($usage:tt)*])?,l~$([$($l_flags:tt)*])?,c~$([$($c_flags:tt)*])?,&$([$($at:tt)*])?,n$([$($version:tt)*])?,+$([$($sub:tt)*])?}
	[&$alias:literal$(,)?$next_alias:ident$($t:tt)*])=>{
		$crate::cmd!(->$name=>{>$([$($action)*])?,<$([$($authors)*])?,@$([$($copyright)*])?,@$([$($license)*])?,=$([$($description)*])?,:$([$($usage)*])?,l~$([$($l_flags)*])?,c~$([$($c_flags)*])?,&[$($($at)*)?$alias,],n$([$($version)*])?,+$([$($sub)*])?}
		[&$next_alias$($t)*])
	};
	(->$name:expr=>{>$([$($action:tt)*])?,<$([$($authors:tt)*])?,@$([$($copyright:tt)*])?,@$([$($license:tt)*])?,=$([$($description:tt)*])?,:$([$($usage:tt)*])?,l~$([$($l_flags:tt)*])?,c~$([$($c_flags:tt)*])?,&$([$($at:tt)*])?,n$([$($version:tt)*])?,+$([$($sub:tt)*])?}
	[&$alias:ident$(,)?$next_alias:literal$($t:tt)*])=>{
		$crate::cmd!(->$name=>{>$([$($action)*])?,<$([$($authors)*])?,@$([$($copyright)*])?,@$([$($license)*])?,=$([$($description)*])?,:$([$($usage)*])?,l~$([$($l_flags)*])?,c~$([$($c_flags)*])?,&[$($($at)*)?$alias,],n$([$($version)*])?,+$([$($sub)*])?}
		[&$next_alias$($t)*])
	};
	(->$name:expr=>{>$([$($action:tt)*])?,<$([$($authors:tt)*])?,@$([$($copyright:tt)*])?,@$([$($license:tt)*])?,=$([$($description:tt)*])?,:$([$($usage:tt)*])?,l~$([$($l_flags:tt)*])?,c~$([$($c_flags:tt)*])?,&$([$($at:tt)*])?,n$([$($version:tt)*])?,+$([$($sub:tt)*])?}
	[&$alias:literal$(,)?$next_alias:literal$($t:tt)*])=>{
		$crate::cmd!(->$name=>{>$([$($action)*])?,<$([$($authors)*])?,@$([$($copyright)*])?,@$([$($license)*])?,=$([$($description)*])?,:$([$($usage)*])?,l~$([$($l_flags)*])?,c~$([$($c_flags)*])?,&[$($($at)*)?$alias,],n$([$($version)*])?,+$([$($sub)*])?}
		[&$next_alias$($t)*])
	};
	(->$name:expr=>{>$([$($action:tt)*])?,<$([$($authors:tt)*])?,@$([$($copyright:tt)*])?,@$([$($license:tt)*])?,=$([$($description:tt)*])?,:$([$($usage:tt)*])?,l~$([$($l_flags:tt)*])?,c~$([$($c_flags:tt)*])?,&$([$($at:tt)*])?,n$([$($version:tt)*])?,+$([$($sub:tt)*])?}
	[&$alias:ident$($t:tt)*])=>{
		$crate::cmd!(->$name=>{>$([$($action)*])?,<$([$($authors)*])?,@$([$($copyright)*])?,@$([$($license)*])?,=$([$($description)*])?,:$([$($usage)*])?,l~$([$($l_flags)*])?,c~$([$($c_flags)*])?,&[$($($at)*)?$alias,],n$([$($version)*])?,+$([$($sub)*])?}
		[$($t)*])
	};
	(->$name:expr=>{>$([$($action:tt)*])?,<$([$($authors:tt)*])?,@$([$($copyright:tt)*])?,@$([$($license:tt)*])?,=$([$($description:tt)*])?,:$([$($usage:tt)*])?,l~$([$($l_flags:tt)*])?,c~$([$($c_flags:tt)*])?,&$([$($at:tt)*])?,n$([$($version:tt)*])?,+$([$($sub:tt)*])?}
	[&$alias:literal$($t:tt)*])=>{
		$crate::cmd!(->$name=>{>$([$($action)*])?,<$([$($authors)*])?,@$([$($copyright)*])?,@$([$($license)*])?,=$([$($description)*])?,:$([$($usage)*])?,l~$([$($l_flags)*])?,c~$([$($c_flags)*])?,&[$($($at)*)?$alias,],n$([$($version)*])?,+$([$($sub)*])?}
		[$($t)*])
	};
	(->$name:expr=>{>$([$($action:tt)*])?,<$([$($authors:tt)*])?,@$([$($copyright:tt)*])?,@$([$($license:tt)*])?,=$([$($description:tt)*])?,:$([$($usage:tt)*])?,l~$([$($l_flags:tt)*])?,c~$([$($c_flags:tt)*])?,&,n$([$($version:tt)*])?,+$([$($sub:tt)*])?}
	[&$alias:expr$(,$($t:tt)*)?])=>{
		$crate::cmd!(->$name=>{>$([$($action)*])?,<$([$($authors)*])?,@$([$($copyright)*])?,@$([$($license)*])?,=$([$($description)*])?,:$([$($usage)*])?,l~$([$($l_flags)*])?,c~$([$($c_flags)*])?,&[->$alias],n$([$($version)*])?,+$([$($sub)*])?}
		[$($($t)*)?])
	};
	(->$name:expr=>{>$([$($action:tt)*])?,<$([$($authors:tt)*])?,@$([$($copyright:tt)*])?,@$([$($license:tt)*])?,=$([$($description:tt)*])?,:$([$($usage:tt)*])?,l~$([$($l_flags:tt)*])?,c~$([$($c_flags:tt)*])?,&$([$($alias:tt)*])?,n,+$([$($sub:tt)*])?}
	[n $(,$($t:tt)*)?])=>{
		$crate::cmd!(->$name=>{>$([$($action)*])?,<$([$($authors)*])?,@$([$($copyright)*])?,@$([$($license)*])?,=$([$($description)*])?,:$([$($usage)*])?,l~$([$($l_flags)*])?,c~$([$($c_flags)*])?,&$([$($alias)*])?,n[from_crate],+$([$($sub:tt)*])?}
		[$($($t)*)?])
	};
	(->$name:expr=>{>$([$($action:tt)*])?,<$([$($authors:tt)*])?,@$([$($copyright:tt)*])?,@$([$($license:tt)*])?,=$([$($description:tt)*])?,:$([$($usage:tt)*])?,l~$([$($l_flags:tt)*])?,c~$([$($c_flags:tt)*])?,&$([$($alias:tt)*])?,n,+$([$($sub:tt)*])?}
	[n from_crate$($t:tt)*])=>{
		$crate::cmd!(->$name=>{>$([$($action)*])?,<$([$($authors)*])?,@$([$($copyright)*])?,@$([$($license)*])?,=$([$($description)*])?,:$([$($usage)*])?,l~$([$($l_flags)*])?,c~$([$($c_flags)*])?,&$([$($alias)*])?,n[from_crate],+$([$($sub:tt)*])?}
		[$($($t)*)?])
	};
	(->$name:expr=>{>$([$($action:tt)*])?,<$([$($authors:tt)*])?,@$([$($copyright:tt)*])?,@$([$($license:tt)*])?,=$([$($description:tt)*])?,:$([$($usage:tt)*])?,l~$([$($l_flags:tt)*])?,c~$([$($c_flags:tt)*])?,&$([$($alias:tt)*])?,n,+$([$($sub:tt)*])?}
	[n ...$($t:tt)*])=>{
		$crate::cmd!(->$name=>{>$([$($action)*])?,<$([$($authors)*])?,@$([$($copyright)*])?,@$([$($license)*])?,=$([$($description)*])?,:$([$($usage)*])?,l~$([$($l_flags)*])?,c~$([$($c_flags)*])?,&$([$($alias)*])?,n[...],+$([$($sub:tt)*])?}
		[$($($t)*)?])
	};
	(->$name:expr=>{>$([$($action:tt)*])?,<$([$($authors:tt)*])?,@$([$($copyright:tt)*])?,@$([$($license:tt)*])?,=$([$($description:tt)*])?,:$([$($usage:tt)*])?,l~$([$($l_flags:tt)*])?,c~$([$($c_flags:tt)*])?,&$([$($alias:tt)*])?,n,+$([$($sub:tt)*])?}
	[n $major:literal$(,)?$minor:literal$(,)?$patch:literal $($vt:literal)*$($t:tt)*])=>{
		$crate::cmd!(->$name=>{>$([$($action)*])?,<$([$($authors)*])?,@$([$($copyright)*])?,@$([$($license)*])?,=$([$($description)*])?,:$([$($usage)*])?,l~$([$($l_flags)*])?,c~$([$($c_flags)*])?,&$([$($alias)*])?,n[$major $minor $patch $($vt)*],+$([$($sub:tt)*])?}
		[$($t)*])
	};
	(->$name:expr=>{>$([$($action:tt)*])?,<$([$($authors:tt)*])?,@$([$($copyright:tt)*])?,@$([$($license:tt)*])?,=$([$($description:tt)*])?,:$([$($usage:tt)*])?,l~$([$($l_flags:tt)*])?,c~$([$($c_flags:tt)*])?,&$([$($alias:tt)*])?,n,+$([$($sub:tt)*])?}
	[n $version:literal.$versionp:literal $($vt:literal)*$($t:tt)*])=>{
		$crate::cmd!(->$name=>{>$([$($action)*])?,<$([$($authors)*])?,@$([$($copyright)*])?,@$([$($license)*])?,=$([$($description)*])?,:$([$($usage)*])?,l~$([$($l_flags)*])?,c~$([$($c_flags)*])?,&$([$($alias)*])?,n[$version.$versionp $($vt)*],+$([$($sub:tt)*])?}
		[$($t)*])
	};
	(->$name:expr=>{>$([$($action:tt)*])?,<$([$($authors:tt)*])?,@$([$($copyright:tt)*])?,@$([$($license:tt)*])?,=$([$($description:tt)*])?,:$([$($usage:tt)*])?,l~$([$($l_flags:tt)*])?,c~$([$($c_flags:tt)*])?,&$([$($alias:tt)*])?,n,+$([$($sub:tt)*])?}
	[n $version:literal$($t:tt)*])=>{
		$crate::cmd!(->$name=>{>$([$($action)*])?,<$([$($authors)*])?,@$([$($copyright)*])?,@$([$($license)*])?,=$([$($description)*])?,:$([$($usage)*])?,l~$([$($l_flags)*])?,c~$([$($c_flags)*])?,&$([$($alias)*])?,n[$version],+$([$($sub:tt)*])?}
		[$($t)*])
	};
	(->$name:expr=>{>$([$($action:tt)*])?,<$([$($authors:tt)*])?,@$([$($copyright:tt)*])?,@$([$($license:tt)*])?,=$([$($description:tt)*])?,:$([$($usage:tt)*])?,l~$([$($l_flags:tt)*])?,c~$([$($c_flags:tt)*])?,&$([$($alias:tt)*])?,n,+$([$($sub:tt)*])?}
	[n [$($version:tt)*]$(,$($t:tt)*)?])=>{
		$crate::cmd!(->$name=>{>$([$($action)*])?,<$([$($authors)*])?,@$([$($copyright)*])?,@$([$($license)*])?,=$([$($description)*])?,:$([$($usage)*])?,l~$([$($l_flags)*])?,c~$([$($c_flags)*])?,&$([$($alias)*])?,n[$($version)*],+$([$($sub:tt)*])?}
		[$($($t)*)?])
	};
	(->$name:expr=>{>$([$($action:tt)*])?,<$([$($authors:tt)*])?,@$([$($copyright:tt)*])?,@$([$($license:tt)*])?,=$([$($description:tt)*])?,:$([$($usage:tt)*])?,l~$([$($l_flags:tt)*])?,c~$([$($c_flags:tt)*])?,&$([$($alias:tt)*])?,n$([$($version:tt)*])?,+$([$($subt:tt)*])?}
	[+ [$($sub:tt)*] $(,$($t:tt)*)?])=>{
		$crate::cmd!(->$name=>{>$([$($action)*])?,<$([$($authors)*])?,@$([$($copyright)*])?,@$([$($license)*])?,=$([$($description)*])?,:$([$($usage)*])?,l~$([$($l_flags)*])?,c~$([$($c_flags)*])?,&$([$($alias)*])?,n$([$($version)*])?,+[$($($subt)*)?$($sub)*,]}
		[$($($t)*)?])
	};
	(->$name:expr=>{>$([$($action:tt)*])?,<$([$($authors:tt)*])?,@$([$($copyright:tt)*])?,@$([$($license:tt)*])?,=$([$($description:tt)*])?,:$([$($usage:tt)*])?,l~$([$($l_flags:tt)*])?,c~$([$($c_flags:tt)*])?,&$([$($alias:tt)*])?,n$([$($version:tt)*])?,+}
	[+ Vector$sub:tt$($t:tt)*])=>{
		$crate::cmd!(->$name=>{>$([$($action)*])?,<$([$($authors)*])?,@$([$($copyright)*])?,@$([$($license)*])?,=$([$($description)*])?,:$([$($usage)*])?,l~$([$($l_flags)*])?,c~$([$($c_flags)*])?,&$([$($alias)*])?,n$([$($version)*])?,+[->Vector$sub]}
		[$($t)*])
	};
	(->$name:expr=>{>$([$($action:tt)*])?,<$([$($authors:tt)*])?,@$([$($copyright:tt)*])?,@$([$($license:tt)*])?,=$([$($description:tt)*])?,:$([$($usage:tt)*])?,l~$([$($l_flags:tt)*])?,c~$([$($c_flags:tt)*])?,&$([$($alias:tt)*])?,n$([$($version:tt)*])?,+}
	[+ vector!$sub:tt$($t:tt)*])=>{
		$crate::cmd!(->$name=>{>$([$($action)*])?,<$([$($authors)*])?,@$([$($copyright)*])?,@$([$($license)*])?,=$([$($description)*])?,:$([$($usage)*])?,l~$([$($l_flags)*])?,c~$([$($c_flags)*])?,&$([$($alias)*])?,n$([$($version)*])?,+[->vector!$sub]}
		[$($t)*])
	};
	(->$name:expr=>{>$([$($action:tt)*])?,<$([$($authors:tt)*])?,@$([$($copyright:tt)*])?,@$([$($license:tt)*])?,=$([$($description:tt)*])?,:$([$($usage:tt)*])?,l~$([$($l_flags:tt)*])?,c~$([$($c_flags:tt)*])?,&$([$($alias:tt)*])?,n$([$($version:tt)*])?,+$([$($subt:tt)*])?}
	[+ $macro:ident!$body:tt $($t:tt)*])=>{
		$crate::cmd!(->$name=>{>$([$($action)*])?,<$([$($authors)*])?,@$([$($copyright)*])?,@$([$($license)*])?,=$([$($description)*])?,:$([$($usage)*])?,l~$([$($l_flags)*])?,c~$([$($c_flags)*])?,&$([$($alias)*])?,n$([$($version)*])?,+[$($($subt)*)?$macro!$body,]}
		[$($t)*])
	};
	(->$name:expr=>{>$([$($action:tt)*])?,<$([$($authors:tt)*])?,@$([$($copyright:tt)*])?,@$([$($license:tt)*])?,=$([$($description:tt)*])?,:$([$($usage:tt)*])?,l~$([$($l_flags:tt)*])?,c~$([$($c_flags:tt)*])?,&$([$($alias:tt)*])?,n$([$($version:tt)*])?,+$([$($subt:tt)*])?}
	[+ $sub:expr $(,$($t:tt)*)?])=>{
		$crate::cmd!(->$name=>{>$([$($action)*])?,<$([$($authors)*])?,@$([$($copyright)*])?,@$([$($license)*])?,=$([$($description)*])?,:$([$($usage)*])?,l~$([$($l_flags)*])?,c~$([$($c_flags)*])?,&$([$($alias)*])?,n$([$($version)*])?,+[$($($subt)*)?$sub,]}
		[$($($t)*)?])
	};
	(->$name:expr=>{>$([$($action:tt)*])?,<$([$($authors:tt)*])?,@$([$($copyright:tt)*])?,@$([$($license:tt)*])?,=$([$($description:tt)*])?,:$([$($usage:tt)*])?,l~$([$($l_flags:tt)*])?,c~$([$($c_flags:tt)*])?,&$([$($alias:tt)*])?,n$([$($version:tt)*])?,+$([$($sub:tt)*])?}
	[])=>{
		$crate::cmd!(->$name=>[>$($($action)*)?,<[$($($authors)*)?],@[$($($copyright)*)?],@[$($($license)*)?],=[$($crate::string_from!{$($description)*})?],:[$($($usage)*)?],l~[$($($l_flags)*)?],c~[$($($c_flags)*)?],&[$($($alias)*)?],n[$($($version)*)?],+[$($($sub)*)?]])
	};
	(
		->$name:expr=>
		[
			>$action:expr,
			<$authors:tt,
			@$copyright:tt,
			@$license:tt,
			$(=)+$desc:tt,
			:$usage:tt,
			l~$l_flags:tt,
			c~$c_flags:tt,
			&$alias:tt,
			n $ver:tt,
			+ $sub:tt$(,)?
		]
	) => {
		Command::with_all_field(
			$name,
			$crate::option_wrap!($action),
			$crate::string_from!$authors,
			$crate::copyright!$copyright,
			$crate::license!$license,
			$crate::option_string_from!$desc,
			$crate::string_from!$usage,
			$crate::flags!$l_flags,
			$crate::flags!$c_flags,
			$crate::alias!$alias,
			$crate::version!$ver,
			$crate::cmds!$sub,
		)
	};
	(->$name:expr=>[$($t:tt)*])=>{
		$crate::cmd!(->$name=>{>,<,@,@,=,:,l~,c~,&,n,+}[$($t)*])
	};
}

/// Create usage
#[macro_export]
macro_rules! default_usage {
	($name:ident) => {
		default_usage!(stringify!($name))
	};
	($name:ident  as var) => {
		$name.into() + "[SUBCOMMAND OR ARG] [OPTIONS]"
	};
	($name: literal) => {
		concat!($name, "[SUBCOMMAND OR ARG] [OPTIONS]")
	};
	($name:ident : into) => {
		$name.into() + "[SUBCOMMAND OR ARG] [OPTIONS]"
	};
	($name:ident : string) => {
		$name + "[SUBCOMMAND OR ARG] [OPTIONS]"
	};
	(nameString=>$name:expr) => {
		$name + "[SUBCOMMAND OR ARG] [OPTIONS]"
	};
}

#[macro_export]
/// Macro for creating copyright
macro_rules! copyright {
	()=>{
		$crate::string_from!()
	};
	([])=>{
		$crate::string_from!()
	};
	($raw_str:literal) => {
		$raw_str.to_string()
	};
	(->$raw_str:literal) => {
		$raw_str.to_string()
	};
	(->$raw:expr)=>{
		$raw
	};
	(...)=>{
		$crate::copyright!(from_crate)
	};
	(from_crate) => {
		concat!("Copyright (c) ", $crate::crate_authors!()).to_owned()
	};
	($prefix:literal$(,)? $year:literal$(,)?$holder:ident) => {
		$crate::copyright!($prefix, $year, stringify!($holder))
	};
	($prefix:literal, $year:literal,$holder:expr) => {
		concat!($prefix," ", $year," ", $holder).to_owned()
	};
	($prefix:literal $year:literal $holder:expr) => {
		copyright!($prefix, $year, $holder)
	};
	(from_crate,$year:literal)=>{
		concat!("Copyright (c) ", $year," ", $crate::crate_authors!()).to_owned()
	};
	($year:literal,from_crate)=>{
		copyright!(from_crate,$year)
	};
	($prefix:literal, $year:literal,$holder:literal) => {
		concat!($prefix," ", $year," ", $holder).to_owned()
	};
	($prefix:literal $year:literal $holder:literal) => {
		copyright!($prefix, $year, $holder)
	};
	(...$(,)?$year:literal)=>{
		$crate::copyright!(from_crate,$year)
	};
	($year:literal$(,)?...)=>{
		$crate::copyright!($year,from_crate)
	};
	({$($t:tt)*})=>{
		$crate::copyright!([$($t:tt)+])
	};
	(($($t:tt)*))=>{
		$crate::copyright!([$($t:tt)+])
	};
	($year:literal$(,)?$holder:ident) => {
		$crate::copyright!($year, stringify!($holder))
	};
	($year:literal$(,)?$holder:expr) => {
		$crate::copyright!("Copyright (c)", $year, $holder)
	};
	($prefix:expr, $year:expr,$holder:ident) => {
		$crate::copyright!($prefix, $year, stringify!($holder))
	};
	($prefix:expr, $year:expr,$holder:expr) => {
		concat!($prefix, " ", $year, " ", $holder).to_owned()
	};
}

#[macro_export]
/// Helps for creating flag*s*.
macro_rules! flags {
	()=>{
		flags!(None)
	};
	(None)=>{
		$crate::vector!(None;:Flag)
	};
	(->$expr:expr)=>{
		$expr
	};
	($($flag_arg:tt);* $(;)?) => {
		$crate::vector![$($crate::flag!$flag_arg),*]
	};
	($($flag_arg:tt),* $(,)?) => {
		$crate::vector![$($crate::flag!$flag_arg),*]
	};
	($($flag_name:ident$sep:tt$flag_arg:tt),* $(,)?)=>{
		$crate::flags!($([$flag_name=>$flag_arg]),*);
	};
	($($flag_name:ident$sep:tt$flag_arg:tt);* $(,)?)=>{
		$crate::flags!($([$flag_name=>$flag_arg]),*);
	};
	($ft:tt$(,$($t:tt)*)?)=>{
		$crate::flags!(={$ft,},$($($t)*)?)
	};
	(={$($st:tt),+,},$ft:tt$(,$($t:tt)*)?)=>{
		$crate::flags!(={$($st),+,$ft,},$($($t)*)?)
	};
	(={$($st:tt),+,},$ft:tt$(;$($t:tt)*)?)=>{
		$crate::flags!(={$($st),+,$ft,},$($($t)*)?)
	};
	($(={$($st:tt),+,},)?$flag_name:ident$flag_arg:tt$(,$($t:tt)*)?)=>{
		$crate::flags!(={$($($st),+,)?[$flag_name$flag_arg],},$($($t)*)?)
	};
	($(={$($st:tt),+,},)?$flag_name:ident$flag_arg:tt$(;$($t:tt)*)?)=>{
		$crate::flags!(={$($($st),+,)?[$flag_name$flag_arg],},$($($t)*)?)
	};
	($(={$($st:tt),+,},)?$flag_name:ident$sep:tt$flag_arg:tt$(,$($t:tt)*)?)=>{
		$crate::flags!(={$($($st),+,)?[$flag_name$sep$flag_arg],},$($($t)*)?)
	};
	($(={$($st:tt),+,},)?$flag_name:ident$sep:tt$flag_arg:tt$(;$($t:tt)*)?)=>{
		$crate::flags!(={$($($st),+,)?[$flag_name$sep$flag_arg],},$($($t)*)?)
	};
	($(={$($st:tt),+,},)?[$($ft:tt)+]$($t:tt)*)=>{
		$crate::flags!(={$($($st),+,)?[$($ft)+],},$($t)*);
	};
	($(={$($st:tt),+,},)?{$($ft:tt)+}$($t:tt)*)=>{
		$crate::flags!(={$($($st),+,)?[$($ft)+],},$($t)*);
	};
	($(={$($st:tt),+,},)?($($ft:tt)+)$($t:tt)*)=>{
		$crate::flags!(={$($($st),+,)?[$($ft)+],},$($t)*);
	};
	(={$($st:tt),+,},)=>{
		$crate::flags!($($st),+,)
	};
}

#[macro_export]
/// Returns alias for Command Construction
macro_rules! alias {
	() => {
		$crate::vector!(;:String)
	};
	(None) => {
		$crate::vector!(;:String)
	};
	(=[$($at:tt)*],,$($t:tt)*)=>{
		alias!(=[$($at)*],$($t)*)
	};
	(=[$($at:tt)*])=>{
		$crate::vector!($($at)*;:String)
	};
	(=[$($at:tt)*],)=>{
		$crate::vector!($($at)*;:String)
	};
	(=[$($at:tt)*],$alias:ident,$($t:tt)*)=>{
		$crate::alias!(=[$($at)*$crate::string_from!(stringify!($alias)),],$($t)*)
	};
	(=[$($at:tt)*],$alias:literal,$($t:tt)*)=>{
		alias!(=[$($at)*$crate::string_from!($alias),],$($t)*)
	};
	(=[$($at:tt)*],$alias:expr,$($t:tt)*)=>{
		alias!(=[$($at)*stringify!($alias),],$($t)*)
	};
	(=[$($at:tt)*],$alias:ident$($t:tt)*)=>{
		alias!(=[$($at)*$crate::string_from!(stringify!($alias)),],$($t)*)
	};
	(=[$($at:tt)*],$alias:literal$($t:tt)*)=>{
		alias!(=[$($at)*$crate::string_from!($alias),],$($t)*)
	};
	(=[$($at:tt)*],$alias:expr)=>{
		alias!(=[$($at)*$alias,],)
	};
	(->$raw:expr) => {
		$raw
	};
	($($t:tt)+)=>{
		$crate::alias!(=[],$($t)+)
	};
}

#[macro_export]
/// Macro for specification of command version
macro_rules! version {
	()=>{
		version!(None)
	};
	(None)=>{
		$crate::string_from!()
	};
	(...) => {
		$crate::string_from!($crate::crate_version!())
	};
	(from_crate)=>{
		version!(...)
	};
	($vm:literal.$vp:literal)=>{
		$crate::string_from!(concat!($vm,".",$vp))
	};
	($version:literal) => {
		$crate::string_from!($version)
	};
	($major:literal$(,)?$minor:literal$(,)?$patch:literal $($($tail:tt)+)?)=>{
		$crate::string_from!(concat!($major,".",$minor,".",$patch$(,$(" ",tt_stringify!($tail)),+)?))
	};
	(&$from:expr) => {
		$from
	};
	(->$raw:expr) => {
		$raw
	};
}

#[macro_export]
/// stringifier(not stringify about literal)
macro_rules! tt_stringify {
	($ident:ident) => {
		stringify!($ident)
	};
	($literal:literal) => {
		$literal
	};
	($t:tt) => {
		stringify!($t)
	};
}

#[macro_export]
/// alias of commands! macro
macro_rules! cmds{
	($($t:tt)*)=>{
		$crate::commands!($($t)*)
	}
}

#[macro_export]
/// create command array
macro_rules! commands{
	(->$($raw:tt)*)=>{
		$($raw)*
	};
	()=>{
		commands![None]
	};
	(None)=>{
		$crate::vector![None;:Command]
	};
	($([$($cmd:tt)+]$(,)?)+)=>{
		$crate::vector![$(cmd![$($cmd)+])+;:Command]
	};
	($($expr:expr),+;:Command$(,)?)=>{
		$crate::vector![$($expr),+;:Command]
	};
	($($expr:expr),+ $(,)?;:Command$(,)?)=>{
		$crate::vector![$($expr),+;:Command]
	};
	($($expr:expr),*$(,)?)=>{
		$crate::vector![$($expr),*;:Command]
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
	(String::from($from:expr))=>{
		String::from($from)
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
/// option_string_from macro
macro_rules! option_string_from {
	() => {
		None
	};
	("")=>{
		None
	};
	(->$from:expr)=>{
		$from
	};
	(&$from:expr)=>{
		$from
	};
	($from:literal)=>{
		Some(String::from($from))
	};
	($from:expr)=>{
		Some(String::from($from))
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
#[doc(hidden)]
// flag basic constructor
macro_rules! _flag_basic_constructor {
	(->$name:expr=>[
		$(=)?$description:expr,
		$(s~)?$short_alias:expr,
		$(l~)?$long_alias:expr,
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
#[doc(hidden)]
// inner for first parse name and tify little after name.
macro_rules! _ffp {
	(->$name:ident$t:tt)=>{
 		$crate::_fsp!(->$name=>$t)
 	};
	(->[$name:expr]$t:tt)=>{
	 	$crate::_fsp!(->[$name]=>$t)
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
	($name:literal[#$($sep:tt)*]$($tas:tt)+)=>{
		$crate::_fsp!($name=>$($tas)+)
	};
	($name:literal[$($ta:tt)*]$($tas:tt)+)=>{
		$crate::_fsp!($name=>[$($ta)*]$($tas)+)
	};
	($name:ident[#$($sep:tt)*]$($tas:tt)+)=>{
		$crate::_fsp!($name=>$($tas)+)
	};
	($name:ident[$($ta:tt)*]$($tas:tt)+)=>{
		$crate::_fsp!($name=>[$($ta)*]$($tas)+)
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
	($name:ident$sep:tt$($t:tt)+)=>{
		$crate::_fsp!($name=>$($t)+)
	};
	($name:expr$(=>$t:tt)?)=>{
		$crate::_fsp!($name=>$($t)?)
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
#[doc(hidden)]
// macro for inner flag
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
#[doc(hidden)]
// macro for innser flag
macro_rules! _ftp{
	(->$name:expr=>$t:tt$($t2:tt)+)=>{
		{
			println!("_ftp: {:?}",stringify!($t));
			$crate::_ftp_s!(->$name=>{=,s~,l~,>,?}[$t$($t2)+])
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
		$crate::_ftp!(->$name=>[=String::default(),s~Vector::default(),l~Vector::default(),>FlagType::default(),?$crate::flag_value!(bool)])
	};
	(->$name:expr=>[$i:ident])=>{
		$crate::_fp_ident_ft_assigner!(->$name=>[$i],_ftp,=)
	};
	(->$name:expr=>[$i:ident,$($t:tt)*])=>{
		$crate::_fp_ident_ft_assigner!(->$name=>[$i,$($t)*],_ftp,=)
	};
	(->$name:expr=>[-$($t:tt)*])=>{
		$crate::_ftp_s!(->$name=>[-$($t)+])
	};
	(->$name:expr=>[$(=)?$description:literal,$($t:tt)*])=>{
		$crate::_ftp_s!(->$name=>[=$description,$($t)*])
	};
	(->$name:expr=>[=[$($dt:tt)*]$($t:tt)*])=>{
		$crate::_ftp_s!(->$name=>[=[$($dt)*]$($t)*])
	};
	(->$name:expr=>[$(=)?$description:expr,$(s~)?$(-)?[$($s:tt)*]$($t:tt)*])=>{
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
		s~$short_alias:expr,
		$long_alias:expr,
		$(>)?$type:expr,
		$(?)?$default:expr$(,)?
		])=>{
			$crate::_flag_basic_constructor!(->$name=>[$description,$short_alias,$long_alias,$type,$default])
	};
	(->$name:expr=>[$(=)?$description:expr,
		$short_alias:expr,
		l~$long_alias:expr,
		$(>)?$type:expr,
		$(?)?$default:expr$(,)?
		])=>{
			$crate::_flag_basic_constructor!(->$name=>[$description,$short_alias,$long_alias,$type,$default])
	};
	(->$name:expr=>[$(=)?$description:expr,
		s~$short_alias:expr,
		l~$long_alias:expr,
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
#[doc(hidden)]
// macro for innser flag
macro_rules! _ftp_s {
	(->$name:expr=>{$($at:tt)*}[$($t:tt)*])=>{
		$crate::_ftp_t!(->$name=>{$($at)*}[$($t)*])
	};
	(->$name:expr=>[$($t:tt)+])=>{
		$crate::_ftp_t!(->$name=>{=,s~,l~,>,?}[$($t)+])
	};
}

#[macro_export]
#[doc(hidden)]
// macro for innser flag
macro_rules! _ftp_t {
	(->$name:expr=>
		{=$([$($dt:tt)*])?,s~$([$($st:tt)*])?,l~$([$($lt:tt)*])?,>$($type:ident)?,?$($default:expr)?}[,$($t:tt)*])=>{
			$crate::_ftp_t!(->$name=>{=$([$($dt)*])?,s~$([$($st)*])?,l~$([$($lt)*])?,>$($type)?,?$($default)?}[$($t)*])
		};
	(->$name:expr=>
		{=,s~$([$($st:tt)*])?,l~$([$($lt:tt)*])?,>,?}
		[$type:ident?$($t:tt)*])=>{
		$crate::_ftp_t!(->$name=>{=,s~$([$($st)*])?,l~$([$($lt)*])?,>$type,?}[?$($t)*])
	};
	(->$name:expr=>
		{=$([$($dt:tt)*])?,s~$([$($st:tt)*])?,l~$([$($lt:tt)*])?,>,?$($default:expr)?}
		[[>$type:ident$($t:tt)*]$($t2:tt)*])=>{
		$crate::_ftp_t!(->$name=>{=$([$($dt)*])?,s~$([$($st)*])?,l~$([$($lt)*])?,>$type,?$($default)?}[$($t)*,$($t2)*])
	};
	(->$name:expr=>
		{=,s~$([$($st:tt)*])?,l~$([$($lt:tt)*])?,>$($type:ident)?,?$($default:expr)?}
		[[=$($dt:tt)*]$($t:tt)*])=>{
		$crate::_ftp_t!(->$name=>{=,s~$([$($st)*])?,l~$([$($lt)*])?,>$($type)?,?$($default)?}[=$($dt)*,$($t)*])
	};
	(->$name:expr=>
		{=,s~$([$($st:tt)*])?,l~$([$($lt:tt)*])?,>$($type:ident)?,?$($default:expr)?}
		[[=->$description:expr]$($t:tt)*])=>{
		$crate::_ftp_t!(->$name=>{=[->$description],s~$([$($st)*])?,l~$([$($lt)*])?,>$($type)?,?$($default)?}[$($t)*])
	};
	(->$name:expr=>
		{=$([$($dt:tt)*])?,s~$([$($st:tt)*])?,l~$([$($lt:tt)*])?,>,?$($default:expr)?}
		[>$type:ident$(,$($t:tt)*)?])=>{
		$crate::_ftp_t!(->$name=>{=$([$($dt)*])?,s~$([$($st)*])?,l~$([$($lt)*])?,>$type,?$($default)?}[$($($t)*)?])
	};
	(->$name:expr=>
		{=$([$($dt:tt)*])?,s~$([$($st:tt)*])?,l~$([$($lt:tt)*])?,>,?$($default:expr)?}
		[>$type:ident$($t:tt)*])=>{
		$crate::_ftp_t!(->$name=>{=$([$($dt)*])?,s~$([$($st)*])?,l~$([$($lt)*])?,>$type,?$($default)?}[$($t)*])
	};
	(->$name:expr=>
		{=,s~$([$($st:tt)*])?,l~$([$($lt:tt)*])?,>,?$($default:expr)?}
		[[$i:ident]$($t:tt)*])=>{
		$crate::_fp_ident_ft_assigner!(->$name=>{=,s~$([$($st)*])?,l~$([$($lt)*])?,>,?$($default)?}[$i,$($t)*],_ftp_t,=)
	};
	(->$name:expr=>
		{=$([$($dt:tt)*])?,s~$([$($st:tt)*])?,l~$([$($lt:tt)*])?,>$type:ident,?}
		[[@$($default:expr)?]$($t:tt)*])=>{
		$crate::_ftp_t!(->$name=>{=$([$($dt)*])?,s~$([$($st)*])?,l~$([$($lt)*])?,>$type,?$($default)?}[$($t)*])
	};
	(->$name:expr=>
		{=$([$($dt:tt)*])?,s~$([$($st:tt)*])?,l~$([$($lt:tt)*])?,>$type:ident,?}
		[[?$($default:expr)?]$($t:tt)*])=>{
		$crate::_ftp_t!(->$name=>{=$([$($dt)*])?,s~$([$($st)*])?,l~$([$($lt)*])?,>$type,?$($default)?}[$($t)*])
	};
	(->$name:expr=>
		{=$([$($dt:tt)*])?,s~$([$($st:tt)*])?,l~$([$($lt:tt)*])?,>$($type:ident)?,?$($default:expr)?}
		[$(l~)?--$l:ident$(,)?$la:ident$($t:tt)*])=>{
		$crate::_fp_ident_ft_assigner!(->$name=>{=$([$($dt)*])?,s~$([$($st)*])?,l~[$($($lt)*)? $l ],>$($type)?,?$($default)?}[$la$($t)*],_ftp_t,--)
	};
	(->$name:expr=>
		{=$([$($dt:tt)*])?,s~$([$($st:tt)*])?,l~$([$($lt:tt)*])?,>$($type:ident)?,?$($default:expr)?}
		[$(l~)?--$l:ident $($t:tt)*])=>{
		$crate::_ftp_t!(->$name=>{=$([$($dt)*])?,s~$([$($st)*])?,l~[$($($lt)*)? $l],>$($type)?,?$($default)?}[$($t)*])
	};
	(->$name:expr=>
		{=$([$($dt:tt)*])?,s~$([$($st:tt)*])?,l~$([$($lt:tt)*])?,>$($type:ident)?,?$($default:expr)?}
		[$(s~)?-$s:ident$(,)?$sa:ident$($t:tt)*])=>{
			$crate::_fp_ident_s_assigner!(->$name=>{=$([$($dt)*])?,s~[$($($st)*)?$s],l~$([$($lt)*])?,>$($type)?,?}[$sa$($t)*],_ftp_t)
	};
	(->$name:expr=>
		{=$([$($dt:tt)*])?,s~$([$($st:tt)*])?,l~$([$($lt:tt)*])?,>$($type:ident)?,?$($default:expr)?}
		[$(s~)?-$s:ident$(,$($t:tt)*)?])=>{
			$crate::_ftp_t!(->$name=>{=$([$($dt)*])?,s~[$($($st)*)?$s],l~$([$($lt)*])?,>$($type)?,?}[$($($t)*)?])
	};
	(->$name:expr=>
		{=$([$($dt:tt)*])?,s~$([$($st:tt)*])?,l~$([$($lt:tt)*])?,>$($type:ident)?,?$($default:expr)?}
		[$(s~)?-$s:ident$($t:tt)*])=>{
			$crate::_ftp_t!(->$name=>{=$([$($dt)*])?,s~[$($($st)*)?$s],l~$([$($lt)*])?,>$($type)?,?}[$($t)*])
	};
	(->$name:expr=>
		{=$([$($dt:tt)*])?,s~,l~$([$($lt:tt)*])?,>$($type:ident)?,?$($default:expr)?}
		[$(s~)?$(-)?[$($(-)?$s:ident$(,)?)*]$(,$($t:tt)*)?])=>{
			$crate::_ftp_t!(->$name=>{=$([$($dt)*])?,s~[$($s)*],l~$([$($lt)*])?,>$($type)?,?}[$($($t)*)?])
	};
	(->$name:expr=>
		{=$([$($dt:tt)*])?,s~,l~$([$($lt:tt)*])?,>$($type:ident)?,?$($default:expr)?}
		[$(s~)?$(-)?[$($(-)?$s:ident$(,)?)*]$($t:tt)*])=>{
			$crate::_ftp_t!(->$name=>{=$([$($dt)*])?,s~[$($s)*],l~$([$($lt)*])?,>$($type)?,?}[$($t)*])
	};
	(->$name:expr=>
		{=$([$($dt:tt)*])?,s~,l~$([$($lt:tt)*])?,>$($type:ident)?,?$($default:expr)?}
		[[-$($s:tt)*]$($t:tt)*])=>{
			$crate::_ftp_t!(->$name=>{=$([$($dt)*])?,s~,l~$([$($lt)*])?,>$($type)?,?}[-$($s)*,$($t)*])
	};
	(->$name:expr=>
		{=$([$($dt:tt)*])?,s~,l~$([$($lt:tt)*])?,>$($type:ident)?,?$($default:expr)?}
		[$(s~)?$(-)?[$($s:tt)*]$($t:tt)*])=>{
			$crate::_ftp_t!(->$name=>{=$([$($dt)*])?,s~[$($s)*],l~$([$($lt)*])?,>$($type)?,?}[$($t)*])
	};
	(->$name:expr=>
		{=$([$($dt:tt)*])?,s~,l~$([$($lt:tt)*])?,>$($type:ident)?,?$($default:expr)?}
		[s~$short_alias:ident$(,$($t:tt)*)?])=>{
		$crate::_ftp_t!(->$name=>{=$([$($dt)*])?,s~[->$short_alias],l~$([$($lt)*])?,>$($type)?,?}[$($($t)*)?])
	};
	(->$name:expr=>
		{=$([$($dt:tt)*])?,s~,l~$([$($lt:tt)*])?,>$($type:ident)?,?$($default:expr)?}
		[s~$short_alias:expr$(,$($t:tt)*)?])=>{
		$crate::_ftp_t!(->$name=>{=$([$($dt)*])?,s~[->$short_alias],l~$([$($lt)*])?,>$($type)?,?}[$($($t)*)?])
	};
	(->$name:expr=>
		{=$([$($dt:tt)*])?,s~$([$($st:tt)*])?,l~,>$($type:ident)?,?$($default:expr)?}
		[l~$long_alias:expr$(,$($t:tt)*)?])=>{
		$crate::_ftp_t!(->$name=>{=$([$($dt)*])?,s~$([$($st)*])?,l~[->$long_alias],>$($type)?,?$($default)?}[$($($t)*)?])
	};
	(->$name:expr=>
		{=$([$($dt:tt)*])?,s~$([$($st:tt)*])?,l~,>$($type:ident)?,?$($default:expr)?}
		[$(l~)?$(--)?[$($(--)?$l:ident$(,)?)*]$($t:tt)*])=>{
		$crate::_ftp_t!(->$name=>{=$([$($dt)*])?,s~$([$($st)*])?,l~[$($l)*],>$($type)?,?$($default)?}[$($t)*])
	};
	(->$name:expr=>
		{=$([$($dt:tt)*])?,s~$([$($st:tt)*])?,l~,>$($type:ident)?,?$($default:expr)?}
		[[--$($lt:tt)*]$($t:tt)*])=>{
		$crate::_ftp_t!(->$name=>{=$([$($dt)*])?,s~$([$($st)*])?,l~,>$($type)?,?$($default)?}[--$($lt)*,$($t)*])
	};
	(->$name:expr=>
		{=$([$($dt:tt)*])?,s~$([$($st:tt)*])?,l~,>$($type:ident)?,?$($default:expr)?}
		[$(l~)?$(--)?[$($lt:tt)*],$($t:tt)*])=>{
		$crate::_ftp_t!(->$name=>{=$([$($dt)*])?,s~$([$($st)*])?,l~[$($lt)*],>$($type)?,?$($default)?}[$($t)*])
	};
	(->$name:expr=>
		{=$([$($dt:tt)*])?,s~$([$($st:tt)*])?,l~,>$($type:ident)?,?$($default:expr)?}
		[$(l~)?$(--)?[$($lt:tt)*]])=>{
		$crate::_ftp_t!(->$name=>{=$([$($dt)*])?,s~$([$($st)*])?,l~[$($lt)*],>$($type)?,?$($default)?}[])
	};
	(->$name:expr=>
		{=,s~$([$($st:tt)*])?,l~$([$($lt:tt)*])?,>$($type:ident)?,?$($default:expr)?}
		[= -$($t:tt)*])=>{
		$crate::_ftp_t!(->$name=>{=[],s~$([$($st)*])?,l~$([$($lt)*])?,>$($type)?,?$($default)?}[-$($t)*])
	};
	(->$name:expr=>
		{=,s~$([$($st:tt)*])?,l~$([$($lt:tt)*])?,>$($type:ident)?,?$($default:expr)?}
		[= ?$($t:tt)*])=>{
		$crate::_ftp_t!(->$name=>{=[],s~$([$($st)*])?,l~$([$($lt)*])?,>$($type)?,?$($default)?}[?$($t)*])
	};
	(->$name:expr=>
		{=,s~$([$($st:tt)*])?,l~$([$($lt:tt)*])?,>$($type:ident)?,?$($default:expr)?}
		[= >$($t:tt)*])=>{
		$crate::_ftp_t!(->$name=>{=[],s~$([$($st)*])?,l~$([$($lt)*])?,>$($type)?,?$($default)?}[>$($t)*])
	};
	(->$name:expr=>
		{=,s~$([$($st:tt)*])?,l~$([$($lt:tt)*])?,>$($type:ident)?,?$($default:expr)?}
		[=$(,$($t:tt)*)?])=>{
		$crate::_ftp_t!(->$name=>{=[],s~$([$($st)*])?,l~$([$($lt)*])?,>$($type)?,?$($default)?}[$($($t)*)?])
	};
	(->$name:expr=>
		{=$([$($dt:tt)*])?,s~$([$($st:tt)*])?,l~$([$($lt:tt)*])?,>$(bool)?,?}
		[$(?)?$(@)?false])=>{
		$crate::_ftp_t!(->$name=>{=$([$($dt)*])?,s~$([$($st)*])?,l~$([$($lt)*])?,>bool,?false}[])
	};
	(->$name:expr=>
		{=$([$($dt:tt)*])?,s~$([$($st:tt)*])?,l~$([$($lt:tt)*])?,>$(bool)?,?}
		[$(?)?$(@)?true])=>{
		$crate::_ftp_t!(->$name=>{=$([$($dt)*])?,s~$([$($st)*])?,l~$([$($lt)*])?,>bool,?true}[])
	};
	(->$name:expr=>
		{=,s~$([$($st:tt)*])?,l~$([$($lt:tt)*])?,>$($type:ident)?,?$($default:expr)?}
		[=$description:ident$($t:tt)*])=>{
		$crate::_ftp_t!(->$name=>{=[$description],s~$([$($st)*])?,l~$([$($lt)*])?,>$($type)?,?$($default)?}[$($t)*])
	};
	(->$name:expr=>
		{=,s~$([$($st:tt)*])?,l~$([$($lt:tt)*])?,>$($type:ident)?,?$($default:expr)?}
		[$(=)?$description:literal$($t:tt)*])=>{
		$crate::_ftp_t!(->$name=>{=[$description],s~$([$($st)*])?,l~$([$($lt)*])?,>$($type)?,?$($default)?}[$($t)*])
	};
	(->$name:expr=>
		{=,s~$([$($st:tt)*])?,l~$([$($lt:tt)*])?,>$($type:ident)?,?$($default:expr)?}
		[=[->$description:expr]$($t:tt)*])=>{
		$crate::_ftp_t!(->$name=>{=[->$description],s~$([$($st)*])?,l~$([$($lt)*])?,>$($type)?,?$($default)?}[$($t)*])
	};
	(->$name:expr=>
		{=,s~$([$($st:tt)*])?,l~$([$($lt:tt)*])?,>$($type:ident)?,?$($default:expr)?}
		[[=[$($dt:tt)*]]$($t:tt)*])=>{
		$crate::_ftp_t!(->$name=>{=[$($dt)*],s~$([$($st)*])?,l~$([$($lt)*])?,>$($type)?,?$($default)?}[$($t)*])
	};
	(->$name:expr=>
		{=,s~$([$($st:tt)*])?,l~$([$($lt:tt)*])?,>$($type:ident)?,?$($default:expr)?}
		[[=$description:expr]$($t:tt)*])=>{
		$crate::_ftp_t!(->$name=>{=[$description],s~$([$($st)*])?,l~$([$($lt)*])?,>$($type)?,?$($default)?}[$($t)*])
	};
	(->$name:expr=>
		{=,s~$([$($st:tt)*])?,l~$([$($lt:tt)*])?,>$($type:ident)?,?$($default:expr)?}
		[$(=)?$description:expr$(,$($t:tt)*)?])=>{
		$crate::_ftp_t!(->$name=>{=[$description],s~$([$($st)*])?,l~$([$($lt)*])?,>$($type)?,?$($default)?}[$($($t)*)?])
	};
	(->$name:expr=>
		{=[$($dt:tt)*],s~$([$($st:tt)*])?,l~$([$($lt:tt)*])?,>$type:ident,?}
		[$(?)?$(@)?$default:literal])=>{
		$crate::_ftp_t!(->$name=>{=[$($dt)*],s~$([$($st)*])?,l~$([$($lt)*])?,>$type,?$default}[])
	};
	(->$name:expr=>
		{=[$($dt:tt)*],s~$([$($st:tt)*])?,l~$([$($lt:tt)*])?,>$type:ident,?}
		[[$(?)?$(@)?$default:expr]])=>{
		$crate::_ftp_t!(->$name=>{=[$($dt)*],s~$([$($st)*])?,l~$([$($lt)*])?,>$type,?$default}[])
	};
	(->$name:expr=>
		{=[$($dt:tt)*],s~$([$($st:tt)*])?,l~$([$($lt:tt)*])?,>$type:ident,?}
		[?$default:expr])=>{
		$crate::_ftp_t!(->$name=>{=[$($dt)*],s~$([$($st)*])?,l~$([$($lt)*])?,>$type,?$default}[])
	};
	(->$name:expr=>
		{=[$($dt:tt)*],s~$([$($st:tt)*])?,l~$([$($lt:tt)*])?,>$type:ident,?}
		[@$default:expr])=>{
		$crate::_ftp_t!(->$name=>{=[$($dt)*],s~$([$($st)*])?,l~$([$($lt)*])?,>$type,?$default}[])
	};
	(->$name:expr=>
		{={$($dt:tt)*},s~[$($st:tt)*],l~[$($lt:tt)*],>$type:ident,?}
		[$(?)?$(@)?$default:expr])=>{
		$crate::_ftp_t!(->$name=>{={$($dt)*},s~[$($st)*],l~[$($lt)*],>$type,?$default}[])
	};
	(->$name:expr=>
		{=[$($dt:tt)*],s~$([$($st:tt)*])?,l~$([$($lt:tt)*])?,>,?$($default:expr)?}
		[$i:ident$($t:tt)*])=>{
		$crate::_ftp_t!(->$name=>{=[$($dt)*],s~$([$($st)*])?,l~$([$($lt)*])?,>,?$($default)?}[>$i$($t)*])
	};
	(->$name:expr=>
		{=,s~$([$($st:tt)*])?,l~$([$($lt:tt)*])?,>,?$($default:expr)?}
		[$i:ident$($t:tt)*])=>{
		$crate::_fp_ident_ft_assigner!(->$name=>{=,s~$([$($st)*])?,l~$([$($lt)*])?,>,?$($default)?}[$i$($t)*],_ftp_t,=)
	};
	(->$name:expr=>
		{=$([$($dt:tt)*])?,s~,l~$([$($lt:tt)*])?,>$($type:ident)?,?$($default:expr)?}
		[$short_alias:expr$(,$($t:tt)*)?])=>{
		$crate::_ftp_t!(->$name=>{=$([$($dt)*])?,s~[->$short_alias],l~$([$($lt)*])?,>$($type)?,?}[$($($t)*)?])
	};
	(->$name:expr=>
		{=$([$($dt:tt)*])?,s~$([$($st:tt)*])?,l~,>$($type:ident)?,?$($default:expr)?}
		[$long_alias:expr$(,$($t:tt)*)?])=>{
		$crate::_ftp_t!(->$name=>{=$([$($dt)*])?,s~$([$($st)*])?,l~[->$long_alias],>$($type)?,?$($default)?}[$($($t)*)?])
	};
	(->$name:expr=>
		{=$([$($dt:tt)*])?,s~$([$($st:tt)*])?,l~$([$($lt:tt)*])?,>$($type:ident)?,?}
		[@$($default:expr)?$(,$($t:tt)*)?])=>{
		$crate::_ftp_t!(->$name=>{=$([$($dt)*])?,s~$([$($st)*])?,l~$([$($lt)*])?,>$($type)?,?$($default)?}[$($($t)*)?])
	};
	(->$name:expr=>
		{=$([$($dt:tt)*])?,s~$([$($st:tt)*])?,l~$([$($lt:tt)*])?,>$type:ident,?}
		[?$($default:expr)?$(,$($t:tt)*)?])=>{
		$crate::_ftp_t!(->$name=>{=$([$($dt)*])?,s~$([$($st)*])?,l~$([$($lt)*])?,>$type,?$($default)?}[$($($t)*)?])
	};
	(->$name:expr=>
		{=$([$($dt:tt)*])?,s~$([$($st:tt)*])?,l~$([$($lt:tt)*])?,>,?}
		[?false$(,$($t:tt)*)?])=>{
		$crate::_ftp_t!(->$name=>{=$([$($dt)*])?,s~$([$($st)*])?,l~$([$($lt)*])?,>,?false}[$($($t)*)?])
	};
	(->$name:expr=>
		{=$([$($dt:tt)*])?,s~$([$($st:tt)*])?,l~$([$($lt:tt)*])?,>,?}
		[?$default:expr$(,$($t:tt)*)?])=>{
		$crate::_ftp_t!(->$name=>{=$([$($dt)*])?,s~$([$($st)*])?,l~$([$($lt)*])?,>,?$default}[$($($t)*)?])
	};
	(->$name:expr=>
		{=$([$($dt:tt)*])?,s~$([$($st:tt)*])?,l~$([$($lt:tt)*])?,>,?}
		[])=>{
		$crate::_ftp_t!(->$name=>{=$([$($dt)*])?,s~$([$($st)*])?,l~$([$($lt)*])?,>String,?}[])
	};
	(->$name:expr=>
		{=$([$($dt:tt)*])?,s~$([$($st:tt)*])?,l~$([$($lt:tt)*])?,>,?true}
		[])=>{
		$crate::_ftp_t!(->$name=>{=$([$($dt)*])?,s~$([$($st)*])?,l~$([$($lt)*])?,>bool,?true}[])
	};
	(->$name:expr=>
		{=$([$($dt:tt)*])?,s~$([$($st:tt)*])?,l~$([$($lt:tt)*])?,>,?false}
		[])=>{
		$crate::_ftp_t!(->$name=>{=$([$($dt)*])?,s~$([$($st)*])?,l~$([$($lt)*])?,>bool,?false}[])
	};
	(->$name:expr=>
		{=$([$($dt:tt)*])?,s~$([$($st:tt)*])?,l~$([$($lt:tt)*])?,>$type:ident,?}
		[])=>{
		$crate::_ftp!(->$name=>[=$crate::string_from!($($($dt)*)?),s~$crate::short_alias![$($($st)*)?],l~$crate::long_alias![$($($lt)*)?],>$crate::flag_type!($type),?$crate::flag_value!($type)])
	};
	(->$name:expr=>
		{=$([$($dt:tt)*])?,s~$([$($st:tt)*])?,l~$([$($lt:tt)*])?,>$type:ident,?$($default:expr)?}
		[])=>{
		$crate::_ftp!(->$name=>[=$crate::string_from!($($($dt)*)?),s~$crate::short_alias![$($($st)*)?],l~$crate::long_alias![$($($lt)*)?],>$crate::flag_type!($type),?$crate::flag_value!($type$(,$default)?)])
	}
}

#[macro_export]
/// short_alias_expander
macro_rules! short_alias {
	() => {
		$crate::short_alias!(None)
	};
	(None) => {
		$crate::vector!(None;:char)
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
	($(-)?$s:ident$($t:tt)*)=>{
		$crate::short_alias!(=[],$s$($t)*)
	};
	($($s:literal),+)=>{
		$crate::vector![$($s),+;:char]
	};
	($($s:literal)+)=>{
		$crate::vector![$($s),+;:char]
	};
	(=[$($t:tt)*],$(-)?$s:ident,$($t2:tt)*)=>{
		$crate::short_alias!(=[$($t)*$crate::char![$s],],$($t2)*)
	};
	(=[$($t:tt)*],$(-)?$s:ident$($t2:tt)*)=>{
		$crate::short_alias!(=[$($t)*$crate::char![$s],],$($t2)*)
	};
	(-$s:literal$($t:tt)*)=>{
		$crate::short_alias!(=[],$s$($t)*)
	};
	(=[$($t:tt)*],-$s:literal $($t2:tt)*)=>{
		$crate::short_alias!(=[$($t)*],$s$($t2)*)
	};
	($s:literal$($t:tt)*)=>{
		$crate::short_alias!(=[],$s$($t)*)
	};
	(=[$($t:tt)*],$s:literal,$($t2:tt)*)=>{
		$crate::short_alias!(=[$($t)*$s,],$($t2)*)
	};
	(=[$($t:tt)*],$s:literal $($t2:tt)*)=>{
		$crate::short_alias!(=[$($t)*$s,],$($t2)*)
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
		$crate::vector!(None;:String)
	};
	(--[$($t:tt)*])=>{
		$crate::long_alias!($($t)*)
	};
	($($l:ident)+)=>{
		$crate::vector!($(stringify!($l)),+;=>String)
	};
	($($(--)?$l:ident),+$(,)?)=>{
		$crate::long_alias!($($l)+)
	};
	($($(--)?$l:ident)+$(,)?)=>{
		$crate::long_alias!($($l)+)
	};
	(--$l:ident$($t:tt)*)=>{
		$crate::long_alias!(=[],$l$($t)*)
	};
	($(--$l:literal)+$(,)?)=>{
		$crate::long_alias!($($l),+)
	};
	($(--$l:literal),+$(,)?)=>{
		$crate::vector!($($l),+;=>String)
	};
	($($l:literal)+$(,)?)=>{
		$crate::long_alias!($($l),+)
	};
	($($l:literal),+$(,)?)=>{
		$crate::vector!($($l),+;=>String)
	};
	($(--)?$l:ident$($t:tt)*)=>{
		$crate::long_alias!(=[],$l$($t)*)
	};
	(=[$($t:tt)*],$(--)?$l:ident,$($t2:tt)*)=>{
		$crate::long_alias!(=[$($t)*stringify!($l),],$($t2)*)
	};
	(=[$($t:tt)*],$(--)?$l:ident$($t2:tt)*)=>{
		$crate::long_alias!(=[$($t)*stringify!($l),],$($t2)*)
	};
	(--$l:literal$($t:tt)*)=>{
		$crate::long_alias!(=[],$l$($t)*)
	};
	($l:literal$($t:tt)*)=>{
		$crate::long_alias!(=[],$l$($t)*)
	};
	($($(--)?$l:literal)+$(,)?)=>{
		$crate::long_alias!($($l),+)
	};
	($($(--)?$l:literal),+$(,)?)=>{
		$crate::vector!($($l),+;=>String)
	};
	(=[$($t:tt)*],--$l:literal,$($t2:tt)*)=>{
		$crate::long_alias!(=[$($t)*$l,],$($t2)*)
	};
	(=[$($t:tt)*],--$l:literal$($t2:tt)*)=>{
		$crate::long_alias!(=[$($t)*$l,],$($t2)*)
	};
	(=[$($t:tt)*],$l:literal,$($t2:tt)*)=>{
		$crate::long_alias!(=[$($t)*$l,],$($t2)*)
	};
	(=[$($t:tt)*],$l:literal$($t2:tt)*)=>{
		$crate::long_alias!(=[$($t)*$l,],$($t2)*)
	};
	(=[$($t:tt)*],)=>{
		$crate::vector!($($t)*;=>String)
	};
	($long:expr)=>{
		$crate::long_alias!(->$long)
	};
	(->$long:expr)=>{
		$long
	}
}

#[macro_export]
#[doc(hidden)]
// inner macro in flag! macro
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
#[doc(hidden)]
// sub macro for flag
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
		$crate::flag_value!($i, $crate::type_default_value!($i))
	};
}

#[macro_export]
/// Creates default_flag_value
macro_rules! default_flag_value {
	($type:ident) => {
		$crate::flag_value!($type, $crate::type_default_value!($type))
	};
}

#[macro_export]
/// get Default falue from type
macro_rules! type_default_value {
	(b) => {
		$crate::type_default_value!(bool)
	};
	(Bool) => {
		$crate::type_default_value!(bool)
	};
	(bool) => {
		bool::default()
	};
	(true) => {
		$crate::type_default_value!(bool)
	};
	(false) => {
		$crate::type_default_value!(bool)
	};
	(i) => {
		$crate::type_default_value!(int)
	};
	(I) => {
		$crate::type_default_value!(int)
	};
	(Int) => {
		$crate::type_default_value!(int)
	};
	(Integer) => {
		$crate::type_default_value!(int)
	};
	(integer) => {
		$crate::type_default_value!(int)
	};
	(int) => {
		isize::default()
	};
	(f) => {
		$crate::type_default_value!(float)
	};
	(F) => {
		$crate::type_default_value!(float)
	};
	(Float) => {
		$crate::type_default_value!(float)
	};
	(float) => {
		f64::default()
	};
	(s) => {
		$crate::type_default_value!(str)
	};
	(S) => {
		$crate::type_default_value!(str)
	};
	(Str) => {
		$crate::type_default_value!(str)
	};
	(String) => {
		$crate::type_default_value!(str)
	};
	(string) => {
		$crate::type_default_value!(str)
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
		|_: &$crate::Command, _: &$crate::Context| -> String { $string }
	};
	(file_path=>$file_path:expr) => {
		$crate::string_fn!(include_str!($file_path).to_owned())
	};
}

#[macro_export]
/// create license helper
macro_rules! license {
	()=>{
		license!(None)
	};
	(none) => {
		$crate::command::License(None)
	};
	(None) => {
		$crate::command::License(None)
	};
	($expr:expr, $content:literal)=>{
		$crate::license!($expr,->$content)
	};
	($expr:literal, $fn:expr)=>{
		$crate::license!($expr.into(),$fn)
	};
	($expr:expr,$fn:expr)=>{
		$crate::command::License(Some(($expr,$fn)))
	};
	($expr:expr,fp:$file_path:expr)=>{
		$crate::license!($expr,file_path=>$file_path)
	};
	($expr:expr,:$file_path:expr)=>{
		$crate::license!($expr,fp:$file_path)
	};
	($expr:expr,outputter=>$fn:expr)=>{
		license!($expr,$fn)
	};
	($expr:expr, ->$content_result:literal)=>{
		$crate::license!($expr,$crate::string_fn!($content_result))
	};
	($expr:expr, ->$content_result:expr)=>{
		$crate::license!($expr,$crate::string_fn!($content_result))
	};
	($expr:expr, content=>$content:literal) => {
		$crate::license!($expr, $crate::string_fn!($content))
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
	};
	(->$license:expr)=>{
		$license
	};
}

/// Preset of help command
#[macro_export]
macro_rules! preset_help_command {
	($help_func:ident) => {
		Command::with_all_field(
			String::from("help"),
			Some($crate::help_command_action!($help_func)),
			String::default(),
			String::default(),
			$crate::command::License::default(),
			Some("show help".into()),
			String::from("help <sub command>"),
			$crate::Vector::default(),
			$crate::Vector::default(),
			$crate::Vector::default(),
			String::default(),
			$crate::Vector::default(),
		)
	};
}

#[cfg(test)]
mod tests {
	use std::collections::VecDeque;

	use crate::{command::License, Command, Context, Flag, FlagType, FlagValue, Vector};

	macro_rules! assert_eqs {
		($left:expr,$($right:expr),+$(,)?) => {
			$(assert_eq!($left,$right);)+
		};
	}

	#[test]
	fn vector_macro_test() {
		assert_eqs!(
			Vector::<String>(None),
			v!(),
			v!(None),
			v!(None; :String),
			v!(::<String>)
		);
		let _r: Vector<String> = Vector(Some(vec!["a".to_owned(), "b".to_owned(), "c".to_owned()]));
		assert_eq!(_r, v!("a".to_owned(), "b".to_owned(), "c".to_owned()));
		assert_eq!(_r, v!("a".to_owned(), "b".to_owned(), "c".to_owned(),));
		assert_eq!(
			_r,
			v!("a".to_owned(), "b".to_owned(), "c".to_owned();:String)
		);
		assert_eq!(_r, v!["a","b","c";=>String]);
		assert_eq!(_r, v!["a","b","c",;=>String]);
		let _r: Vector<String> = Vector(Some(vec!["a".to_owned(), "a".to_owned(), "a".to_owned()]));
	}
	#[test]
	fn short_alias_macro_test() {
		let _r = v!('a', 'b', 'c');
		assert_eqs!(
			_r,
			short_alias![a, -b, c],
			short_alias!('a' 'b' 'c'),
			short_alias!['a' b c],
			short_alias![-a,-'b' c],
			short_alias!(a b,'c')
		);
		let _r = v!('s', 'f');
		assert_eqs!(_r, short_alias!('s', 'f'), short_alias!('s' 'f'));
	}

	#[test]
	fn long_alias_macro_test() {
		let _r = v!["aaa","bbb","ccc";=>String];
		assert_eq!(_r, long_alias!(--aaa, --bbb, --ccc));
		assert_eq!(_r, long_alias!(aaa, bbb, ccc));
		assert_eq!(_r, long_alias!(--aaa, bbb, ccc));
		assert_eq!(_r, long_alias!(--"aaa", --"bbb", --"ccc"));
		assert_eq!(_r, long_alias!(aaa, "bbb", ccc));
		assert_eq!(_r, long_alias!(aaa, "bbb", --ccc));
		assert_eq!(_r, long_alias!(--aaa, --"bbb", ccc));
	}

	#[test]
	fn version_macro_test() {
		assert_eqs!(crate_version!(), version!(...), version!(from_crate));
		assert_eq!(String::from("0.1.2"), version!("0.1.2"));
		assert_eq!(String::from("0.1.2"), version!(0.1.2));
		assert_eqs!("0.1.2".to_owned(), version!(0 1 2), version!(0, 1, 2),);
		assert_eqs!(
			"0.1.2 aaa".to_owned(),
			version!(0 1 2 aaa),
			version!(0, 1, 2 aaa),
			version!(0 1 2 "aaa"),
			version!(0, 1, 2 "aaa"),
		);
	}

	#[test]
	fn flag_test() {
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
				s~Vector(Some(vec!['s', 'f'])),
				l~Vector(Some(vec!["long".to_owned(), "long2".to_owned()])),
				?false
				]
			),
			flag!(test_flag=>[
				>bool,
				=_t,
				l~Vector(Some(vec!["long".to_owned(), "long2".to_owned()])),
				s~Vector(Some(vec!['s', 'f'])),
				?false
				]
			),
			flag!(test_flag=>[
				_t,
				>bool,
				l~Vector(Some(vec!["long".to_owned(), "long2".to_owned()])),
				s~Vector(Some(vec!['s', 'f'])),
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
			s~[s, f],
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
				=[->String::from(_t)],
				-s,f,
				--long long2]),
			flag!(test_flag=>[
				bool,
				=[->_t.into()],
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
				s~Vector(Some(vec!['s', 'f'])),
				--[long, long2],
				>bool?false
			]),
			flag!(test_flag[
				=_t,
				s~[s f],
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
			flag!([test_flag]=>{bool, -[s,'f'],--long long2 ="test",false}),
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
			flag!(test_flag:
			[bool]
			[="test"]
			[-s -f]
			[--long --long2]
			[false]
			),
			flag!(test_flag
			[bool]
			[="test"]
			[-s -f]
			[--long --long2]
			[?false]
			),
		);
		assert_eqs!(
			Flag::with_all_field(
				"test_flag".into(),
				String::from("test"),
				vector!['e';:char],
				Vector(None),
				FlagType::Bool,
				FlagValue::Bool(false),
			),
			flag![[test_flag][>bool ="test" -e ?false]],
			flag![[test_flag][bool][="test"][-e][?false]],
			flag![[test_flag][>bool?false][="test"][-e]],
		);
	}

	#[test]
	fn flags_marco_test() {
		assert_eqs!(vector!(None;:Flag), flags!(), flags!(None));
		let _r = Vector(Some(vec![
			flag!(test_flag[>bool?false,-s,-f,="test"]),
			flag!(test_flag2[>bool?false,="test2",-t]),
			Flag::with_all_field(
				"test_flag3".into(),
				String::from("test3"),
				vector!['e';:char],
				Vector(None),
				FlagType::Bool,
				FlagValue::Bool(false),
			),
		]));
		assert_eqs!(
			_r.clone(),
			flags!(
				[test_flag[>bool?false,-s,-f,="test"]],
				[test_flag2=>[>bool?false,="test2" -t]],
				[[test_flag3][>bool][="test3"][-e][?false]]
			),
		);
		assert_eq!(
			_r.clone(),
			flags!(
				test_flag[>bool?false,-s,-f,="test"],
				test_flag2[>bool?false,="test2" -t],
				test_flag3[>bool ="test3" -e ?false],
			)
		);
	}

	fn get_context_for_test() -> Context {
		let _r = License::new(Some(("test".into(), |_, _| -> String {
			"test_license_content".to_owned()
		})));
		Context::new(
			vec!["raw".into(), "arg".into(), "test".into()],
			VecDeque::default(),
			Vector::default(),
			Vector::default(),
			"exe_path".into(),
		)
	}

	#[test]
	fn license_macro_test() {
		let _r = License::new(Some(("test".into(), |_, _| -> String {
			"test_license_content".to_owned()
		})));
		let _c = get_context_for_test();
		let _cmd = Command::with_name("test");

		let comp = |left: &License, right: &License, cmd: &Command, c: &Context| {
			assert_eq!(left.expr(), right.expr());
			assert_eq!(left.output(cmd, c), right.output(cmd, c));
		};
		comp(&_r, &license!("test",->"test_license_content"), &_cmd, &_c);
		comp(
			&_r,
			&license!("test",->"test_license_content".to_owned()),
			&_cmd,
			&_c,
		);
		comp(&_r, &license!("test", "test_license_content"), &_cmd, &_c);
		comp(
			&_r,
			&license!("test", string_fn!("test_license_content".to_owned())),
			&_cmd,
			&_c,
		);
		comp(
			&_r,
			&license!("test", |_, _| -> String { "test_license_content".into() }),
			&_cmd,
			&_c,
		);
		comp(
			&_r,
			&license!(expr=>"test", outputter=>|_,_| -> String {
				"test_license_content".into()
			}),
			&_cmd,
			&_c,
		);
	}

	fn compare_cmd(left: Command, right: Command, c: Context) {
		assert_eq!(left.name, right.name);
		match (left.action, right.action) {
			(Some(la), Some(ra)) => {
				assert!(la(left.clone(), c.clone()).unwrap().is_done());
				assert!(ra(right.clone(), c.clone()).unwrap().is_done());
				assert_eq!(la, ra);
				assert_eq!(la as usize, ra as usize);
			}
			(None, None) => {}
			(None, Some(_)) => {
				panic!("action is not equal")
			}
			(Some(_), None) => {
				panic!("action is not equal")
			}
		}
		assert_eq!(left.authors, right.authors);
		assert_eq!(left.copyright, right.copyright);
		assert_eqs!(left.license.expr(), right.license.expr());
		assert_eq!(
			left.license.output(&left, &c),
			right.license.output(&right, &c)
		);
		match (left.license.output_fn(), right.license.output_fn()) {
			(Some(_loutput), Some(_routput)) => {
				//assert_eq!(_loutput as usize, _routput as usize);
			}
			(None, None) => {}
			(_, _) => {
				panic!("license output_fn not match!")
			}
		}
		assert_eq!(left.description, right.description);
		assert_eq!(left.usage, right.usage);
		assert_eq!(left.alias, right.alias);
		assert_eq!(left.version, right.version);
		match (left.sub, right.sub) {
			(Vector(None), Vector(None)) => {}
			(Vector(Some(lsub)), Vector(Some(rsub))) => {
				for (index, lsc) in lsub.iter().enumerate() {
					compare_cmd(lsc.clone(), rsub[index].clone(), c.clone())
				}
			}
			(_, _) => {
				panic!("sub commands does not match");
			}
		}
	}

	macro_rules! compare_cmds{
		($c:expr,$left:expr, $($right:expr),+$(,)?)=>{
			$(compare_cmd($left,$right,$c);)+
		}
	}

	#[test]
	fn cmd_macro_test() {
		let act = |_, c: Context| -> action_result!() {
			println!("action!{:?}", c);
			assert_eq!(
				c.raw_args,
				vec!["raw".to_owned(), "arg".to_owned(), "test".to_owned()]
			);
			done!()
		};
		let _l = license!("test_license",->"test_license_fn");
		let _lsub = license!("test_license_sub",->"test_license_fn_sub");
		let _lleaf = license!("test_license_sub2",->"test_license_fn_sub2");
		let leaf = Command::with_name("leaf")
			.action(|c, _| {
				println!("Context: {:?}", c);
				done!()
			})
			.license(_lleaf.clone());
		let sub = Command::with_name("sub")
			.local_flag(Flag::new_bool("sub_local"))
			.local_flag(Flag::new_string("sub_lstr"))
			.common_flag(Flag::new_bool("sub_common"))
			.local_flag(Flag::new_string("sub_cstr"))
			.action(|_, _| {
				println!("sub");
				done!()
			})
			.sub_command(leaf)
			.license(_lsub.clone());
		let sub2_act = |_, _| {
			println!("sub2");
			done!()
		};
		let sub2 = Command::with_name("sub2").action(sub2_act);
		let _r = Command::with_all_field(
			"test".into(),
			Some(act),
			"suquiya test".into(),
			"suquiya".into(),
			_l.clone(),
			Some("test_command".into()),
			"test_usage".into(),
			flags!({tlf[="test_local_flag" -l >bool?false]}),
			flags!({tcf[="test_common_flag" -c >bool?false]}),
			Vector(Some(vec!["alias".into(), "alias2".into()])),
			"0.0.1".into(),
			vector![sub.clone(),sub2.clone();:Command],
		);
		let _c = get_context_for_test();

		compare_cmd(
			_r.clone(),
			cmd!("test":[
				>act,
				<["suquiya test"],
				@["suquiya"],
				@[->_l.clone()],
				=["test_command"],
				:["test_usage"],
				l~{tlf[="test_local_flag" -l >bool?false]},
				c~{tcf[="test_common_flag" -c >bool?false]},
				&["alias", "alias2"],
				n ["0.0.1"],
				+ [sub.clone(),sub2.clone();:Command],
			]),
			_c.clone(),
		);

		compare_cmd(
			_r.clone(),
			cmd!(test:[
				>act,
				<["suquiya test"],
				@["suquiya"],
				@[->_l.clone()],
				=["test_command"],
				:["test_usage"],
				l~flags!({tlf[="test_local_flag" -l >bool?false]}),
				c~flags!({tcf[="test_common_flag" -c >bool?false]}),
				&Vector(Some(vec!["alias".into(), "alias2".into()])),
				n ["0.0.1"],
				+ vector![sub.clone(),sub2.clone();:Command],
			]),
			_c.clone(),
		);

		compare_cmd(
			_r.clone(),
			cmd!(test[
				>>act,
				<"suquiya test",
				@"suquiya",
				@[->_l.clone()],
				="test_command",
				:"test_usage",
				l~flags!({tlf[="test_local_flag" -l >bool?false]}),
				c~flags!({tcf[="test_common_flag" -c >bool?false]}),
				&Vector(Some(vec!["alias".into(), "alias2".into()])),
				n "0.0.1",
				+ vector![sub.clone(),sub2.clone();:Command],
			]),
			_c.clone(),
		);

		compare_cmds!(
			_c.clone(),
			_r.clone().authors("suquiya").copyright("suquiya copyright"),
			cmd!(test[
				>act,
				<"suquiya",
				@"suquiya copyright",
				@_l.clone(),
				="test_command",
				:"test_usage",
				l~flags!({tlf[="test_local_flag" -l >bool?false]}),
				c~flags!({tcf[="test_common_flag" -c >bool?false]}),
				&Vector(Some(vec!["alias".into(), "alias2".into()])),
				n "0.0.1",
				+ vector![sub.clone(),sub2.clone();:Command],
			]),
		);

		let r = _r
			.clone()
			.authors(crate_authors!())
			.copyright("suquiya copyright");
		compare_cmds!(
			_c.clone(),
			r.clone(),
			cmd!(test[
				>act,
				<crate_authors!(),
				@"suquiya copyright",
				@_l.clone(),
				="test_command",
				:"test_usage",
				l~flags!({tlf[="test_local_flag" -l >bool?false]}),
				c~flags!({tcf[="test_common_flag" -c >bool?false]}),
				&vector!["alias".into(), "alias2".into()],
				n "0.0.1",
				+ vector![sub.clone(),sub2.clone();:Command],
			]),
			cmd!("test"[
				>act
				<crate_authors!(),
				@"suquiya copyright",
				@["test_license",->"test_license_fn"],
				="test_command",
				:"test_usage",
				l~flags!({tlf[="test_local_flag" -l >bool?false]}),
				c~flags!({tcf[="test_common_flag" -c >bool?false]}),
				&[alias, alias2],
				n "0.0.1",
				+ [sub.clone(),sub2.clone();:Command],
			]),
			cmd!("test"[
				>act
				<crate_authors!(),
				@"suquiya copyright",
				@"test_license","test_license_fn",
				="test_command",
				:"test_usage",
				l~({tlf[="test_local_flag" -l >bool?false]}),
				c~({tcf[="test_common_flag" -c >bool?false]}),
				&[alias, alias2],
				n "0.0.1",
				+ [sub.clone(),sub2.clone()],
			]),
			cmd!("test"[
				>act
				<crate_authors!(),
				@"suquiya copyright",
				@"test_license","test_license_fn",
				="test_command",
				:"test_usage",
				l~({tlf[="test_local_flag" -l >bool?false]}),
				c~({tcf[="test_common_flag" -c >bool?false]}),
				&alias alias2,
				n "0.0.1",
				+ [sub.clone(),sub2.clone()],
			]),
			cmd!("test"[
				>act
				<crate_authors!(),
				@"suquiya copyright",
				@"test_license","test_license_fn",
				="test_command",
				:"test_usage",
				l~{tlf[="test_local_flag" -l >bool?false]},
				c~{tcf[="test_common_flag" -c >bool?false]},
				&alias,
				&alias2,
				n "0.0.1",
				+ sub.clone(),
				+ sub2.clone(),
			]),
		);

		let _r = r.clone().copyright(copyright![2020, "suquiya"]);
		compare_cmd(
			_r.clone(),
			cmd!("test"[
				action=>act
				authors=crate_authors!(),
				(c)2020,"suquiya";
				(l)"test_license" "test_license_fn",
				="test_command",
				:"test_usage",
				l~{tlf[="test_local_flag" -l >bool?false]},
				c~{tcf[="test_common_flag" -c >bool?false]},
				&alias,
				&alias2,
				n "0.0.1",
				+ sub.clone(),
				+ sub2.clone(),
			]),
			_c.clone(),
		);

		//rust-analyzerが警告を出すが通るテスト、一時的にコメントアウト
		let _r = r
			.clone()
			.copyright("Copyright (c) ".to_owned() + crate_authors!());
		compare_cmd(
			_r.clone(),
			cmd!("test"[
				action=>act
				authors=crate_authors!(),
				(c)...,
				(l)"test_license" "test_license_fn",
				="test_command",
				:"test_usage",
				l~{tlf[="test_local_flag" -l >bool?false]},
				c~{tcf[="test_common_flag" -c >bool?false]},
				&alias,
				&alias2,
				version= "0.0.1",
				+ sub.clone(),
				+ sub2.clone(),
			]),
			_c.clone(),
		);
		let _r = r.clone().copyright(copyright![2020, from_crate]);
		compare_cmd(
			_r.clone(),
			cmd!("test"[
				action=>act
				authors=crate_authors!(),
				copyright:2020,from_crate,
				license:"test_license" "test_license_fn",
				description="test_command",
				usage:"test_usage",
				l~tlf[="test_local_flag" -l >bool?false],
				l_flag:[],
				l~None,
				l~,
				c~tcf[="test_common_flag" -c >bool?false],
				c_flag:[],
				c~None,
				alias:alias,
				alias=alias2,
				n 0 0 1,
				|=> sub.clone(),
				| sub2.clone(),
			]),
			_c.clone(),
		);
		compare_cmd(
			_r.clone(),
			cmd!("test"[
				action=>act
				authors=...,
				copyright:2020,...,
				license:"test_license" "test_license_fn",
				description="test_command",
				usage:"test_usage",
				l~tlf[="test_local_flag" -l >bool?false],
				l_flag:[],
				l~None,
				l~,
				c~tcf[="test_common_flag" -c >bool?false],
				c_flag:[],
				c~None,
				alias:alias,
				alias=alias2,
				n 0 0 1,
				+ sub.clone(),
				| cmd!(sub2[>sub2_act]),
			]),
			_c.clone(),
		);
	}

	#[test]
	fn default_val_macro_test() {
		assert_eqs!(
			default_value!(flag.help.description),
			default_value!(help.flag.description),
			default_value!(description.flag.help),
			default_description!(flag.help)
		);
		assert_eqs!(
			default_value!(flag.help.name),
			default_value!(help.flag.name),
			default_value!(name.flag.help),
			default_name!(flag.help)
		);
	}
}
