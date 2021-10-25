use crate::{
	action::{ActionError, ActionErrorKind::NoActionRegistered, ActionResult},
	done,
	parser::MiddleArg,
	Action, Context, Flag, FlagValue, Parser, Vector,
};

use core::mem::take;
use std::{collections::VecDeque, fmt::Debug};

///The struct for command information store and command execution
///This can be root and edge
///コマンドの情報格納＆実行用構造体
#[derive(Clone, Default)]
pub struct Command {
	///Command name
	pub name: String,
	///Command action
	pub action: Option<Action>,
	///Command authors
	pub authors: String,
	///Command copyright
	pub copyright: String,
	/// License of command
	pub license: License,
	/// Command description
	pub description: Option<String>,
	///Command usage
	pub usage: String,
	///local flags of command
	pub l_flags: Vector<Flag>,
	///common flags of command
	pub c_flags: Vector<Flag>,
	///alias for command
	pub alias: Vector<String>,
	///Command version
	pub version: String,
	///container of sub-command
	pub sub: Vector<Command>,
}

// Helper inner macros
macro_rules! run_result{
	()=>{
		Result<ActionResult,ActionError>
	}
}
macro_rules! no_registered_error {
	($context:expr) => {
		Err(ActionError::without_related_error(
			"no action is registered.".into(),
			NoActionRegistered,
			$context,
		))
	};
}
macro_rules! sub_check_field {
	($context:expr,$context_field:ident,$sub:expr, $self:expr, $field:ident) => {
		if !$sub.$field.is_empty() {
			$self.$field = take(&mut $context.$context_field);
			$context.$context_field = take(&mut $sub.$field);
		}
	};
	($context:expr,$context_field:ident,$sub:expr, $self:expr, $field:ident :Option) => {
		if $sub.$field.is_some() {
			$self.$field = $context.$context_field.take();
			$context.$context_field = $sub.$field.take();
		}
	};
	($context:expr,$context_field:ident,$sub:expr, $self:expr, $field:ident :License) => {
		if $sub.$field.is_some() {
			$self.$field = $context.$context_field.take();
			$context.$context_field = $sub.$field.take();
		}
	};
}

macro_rules! sub_check {
	($context:expr, $sub:expr, $self:expr) => {
		sub_check_field!($context, cmd_authors, $sub, $self, authors);
		sub_check_field!($context, cmd_version, $sub, $self, version);
		sub_check_field!($context, cmd_copyright, $sub, $self, copyright);
		sub_check_field!($context, cmd_license, $sub, $self, license: License);
	};
}

macro_rules! check_sub_field {
	($sub: expr, $field:ident, $self:expr) => {
		if $sub.$field.is_empty() {
			take(&mut $self.$field)
		} else {
			take(&mut $sub.$field)
		}
	};
	($sub:expr, $field:ident :Option, $self:expr) => {
		if $sub.$field.is_some() {
			$sub.$field.take()
		} else {
			$self.$field.take()
		}
	};
	($sub:expr, $field:ident :License, $self:expr) => {
		if $sub.$field.is_some() {
			$sub.$field.take()
		} else {
			$self.$field.take()
		}
	};
}

/// HelpFunc shows type alias for help function
pub type HelpFunc = fn(command: &Command, context: &Context) -> String;

#[derive(Clone)]
/// License shows license information
pub struct License(pub Option<(String, fn(ctx: &Context) -> String)>);

impl Default for License {
	fn default() -> Self {
		License(None)
	}
}

impl Debug for License {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		if let License(Some(val)) = self {
			write!(f, "Some({},Func for Output)", val.0)
		} else {
			write!(f, "None")
		}
	}
}

impl License {
	/// Creates new License information.
	pub fn new(inner: Option<(String, fn(ctx: &Context) -> String)>) -> Self {
		License(inner)
	}
	/// Returns true if self has some license information.
	pub fn is_some(&self) -> bool {
		matches!(*self, License(Some(_)))
	}
	/// Returns true if self has no license information.
	pub fn is_none(&self) -> bool {
		matches!(*self, License(None))
	}
	/// Takes inner license information of self and returns.
	pub fn take(&mut self) -> Self {
		core::mem::take(self)
	}
	/// Returns (short) expression of license. If no license information(self is License(None)), returns None.
	pub fn expr(&self) -> Option<String> {
		match self {
			License(Some((expr, _))) => Some(expr.into()),
			License(None) => None,
		}
	}
	/// Returns (short) expression of license. If no license information(self is License(None)), this function may panic.
	pub fn expr_unwrap(&self) -> String {
		match self {
			License(Some((expr, _))) => expr.clone(),
			License(None) => panic!("called `License::expr_unwrap()` on a `None` value"),
		}
	}
	/// Returns (long) expression - detail of license.
	pub fn output(&self, ctx: &Context) -> Option<String> {
		match self {
			License(Some((_, outputter))) => Some(outputter(ctx)),
			License(None) => None,
		}
	}
	/// Returns (long) expression - detail of license. If self is License(None), this function may panic.
	pub fn output_unwrap(&self, ctx: &Context) -> String {
		match self {
			License(Some((_, outputter))) => outputter(ctx),
			_ => panic!("called `License::expr_unwrap()` on a `None`value"),
		}
	}
	/// Returns function which outputs (long) expression (or detail of license) with wrapping Option.
	pub fn output_fn(&self) -> Option<fn(&Context) -> String> {
		match self {
			License(Some((_, outputter))) => Some(*outputter),
			License(None) => None,
		}
	}
	/// Returns function of (long) expression (or detail of license). If self is License(None), this will panic.
	pub fn output_fn_unwrap(&self) -> fn(&Context) -> String {
		match self {
			License(Some((_, outputter))) => *outputter,
			_ => panic!("called `License::output_fn_wrap` on a `None` value"),
		}
	}
	/// Returns unwrap inner
	pub fn unwrap(self) -> (String, fn(ctx: &Context) -> String) {
		let License(inner) = self;
		inner.unwrap()
	}
}

macro_rules! gen_context_for_self_action {
	($self:expr, $raw_args:expr) => {{
		let mut args = VecDeque::from($raw_args.clone());
		let exe_path = args.pop_front().unwrap();
		gen_context_for_self_action!($self, $raw_args, args, exe_path)
	}};
	($self:expr,$raw_args:expr,$args:expr,$exe_path:expr) => {
		Context::new(
			$raw_args,
			$args,
			$self.c_flags.take(),
			$self.derive_route_init_vector(),
			$exe_path,
			String::default(),
			String::default(),
			String::default(),
			License::default(),
		)
	};
	($self:expr, $raw_args:expr,$args:expr,$exe_path:expr, $inter_mediate_args:expr) => {
		Context::with_all_field(
			$raw_args,
			$args,
			$self.c_flags.take().into(),
			$exe_path,
			$self.derive_route_init_vector(),
			Vector::default(),
			Vector::default(),
			Some($inter_mediate_args),
			Vector(None),
			String::default(),
			String::default(),
			String::default(),
			License::default(),
		)
	};
}

macro_rules! gen_context_for_sub_run {
	($self:expr,$sub:expr, $raw_args:expr,$args:expr,$exe_path:expr) => {
		gen_context_for_sub_run!(inner, $self, $sub, $raw_args, $args, $exe_path, None)
	};
	($self:expr,$sub:expr, $raw_args:expr,$args:expr,$exe_path:expr, $inter_mediate_args: expr) => {
		gen_context_for_sub_run!(
			inner,
			$self,
			$sub,
			$raw_args,
			$args,
			$exe_path,
			Some($inter_mediate_args)
		)
	};
	(inner,$self:expr, $sub:expr, $raw_args:expr,$args:expr,$exe_path:expr,$inter_mediate_args:expr) => {
		Context::with_all_field(
			$raw_args,
			$args,
			Vector::with_first_elem($self.c_flags.take()),
			$exe_path,
			$self.derive_route_init_vector(),
			Vector::default(),
			Vector::default(),
			$inter_mediate_args,
			Vector::default(),
			check_sub_field!($sub, authors, $self),
			check_sub_field!($sub, version, $self),
			check_sub_field!($sub, copyright, $self),
			check_sub_field!($sub, license: License, $self),
		)
	};
}

impl Command {
	/// Creare new instance of Command
	pub fn new() -> Command {
		Command::default()
	}

	/// Create new instance of Command with name,authors,version
	pub fn with_base<T: Into<String>>(
		name: T,
		authors: T,
		version: T,
		description: T,
		action: Option<Action>,
	) -> Command {
		Command::with_all_field(
			name.into(),
			action,
			authors.into(),
			String::default(),
			License(None),
			Some(description.into()),
			String::default(),
			Vector::default(),
			Vector::default(),
			Vector::default(),
			version.into(),
			Vector::default(),
		)
	}

	/// Create new instance of Command with name
	pub fn with_name<T: Into<String>>(name: T) -> Command {
		Command {
			name: name.into(),
			action: None,
			authors: String::default(),
			copyright: String::default(),
			license: License::default(),
			description: None,
			usage: String::default(),
			l_flags: Vector::default(),
			c_flags: Vector::default(),
			alias: Vector::default(),
			version: String::default(),
			sub: Vector::default(),
		}
	}

	/// Create new instance of Command with more options
	#[allow(clippy::too_many_arguments)]
	pub fn with_all_field(
		name: String,
		action: Option<Action>,
		authors: String,
		copyright: String,
		license: License,
		description: Option<String>,
		usage: String,
		local_flags: Vector<Flag>,
		common_flags: Vector<Flag>,
		alias: Vector<String>,
		version: String,
		sub: Vector<Command>,
	) -> Command {
		Command {
			name,
			action,
			authors,
			copyright,
			license,
			description,
			usage,
			l_flags: local_flags,
			c_flags: common_flags,
			alias,
			version,
			sub,
		}
	}

	/// Run command with collecting args automatically
	pub fn run_with_auto_arg_collect(self) -> run_result!() {
		match self.sub {
			Vector(None) => {
				let r = self.single_run(std::env::args().collect::<Vec<String>>());
				r
			}
			_ => self.run(std::env::args().collect::<Vec<String>>()),
		}
	}

	/// Run command as single(do not have sub) command
	/// ルートからサブコマンドがないシンプルな状態の時
	/// アクションが登録されていなければサブコマンドがあるかを調査する
	pub fn single_run(mut self, raw_args: Vec<String>) -> run_result!() {
		match self.action.take() {
			Some(action) => {
				if raw_args.len() < 2 {
					action(gen_context_for_self_action!(self, raw_args), self)
				} else {
					let mut context = gen_context_for_self_action!(self, raw_args);
					//println!("single_run_context: {:?}", context);
					context = Parser::default().parse_args_until_end(&self.l_flags, context);
					action(context, self)
				}
			}
			None => match self.sub {
				Vector(None) => {
					let c = gen_context_for_self_action!(self, raw_args);
					no_registered_error!(c)
				}
				_ => self.run(raw_args),
			},
		}
	}

	/// Set Command's name
	pub fn name<T: Into<String>>(mut self, name: T) -> Command {
		self.name = name.into();
		self
	}

	/// Set command's usage
	pub fn usage<T: Into<String>>(mut self, usage: T) -> Self {
		self.usage = usage.into();
		self
	}

	/// Set command's action
	pub fn action(mut self, action: Action) -> Self {
		self.action = Some(action);
		self
	}

	/// Set command's authors
	pub fn authors<T: Into<String>>(mut self, authors: T) -> Self {
		self.authors = authors.into();
		self
	}

	/// Set command's copyright
	pub fn copyright<T: Into<String>>(mut self, copyright: T) -> Self {
		self.copyright = copyright.into();
		self
	}

	/// Add a local flag to command
	pub fn local_flag(mut self, flag: Flag) -> Self {
		self.l_flags.push(flag);
		self
	}

	/// Add a local flags to command
	pub fn local_flags(mut self, flags: Vec<Flag>) -> Self {
		self.l_flags.append_vec(flags);
		self
	}

	/// Add a common flag to command
	pub fn common_flag(mut self, flag: Flag) -> Self {
		self.c_flags.push(flag);
		self
	}

	/// Add a common flag to command
	pub fn command_flags(mut self, flags: Vec<Flag>) -> Self {
		self.c_flags.append_vec(flags);
		self
	}

	/// Set command's description
	pub fn desctiption<T: Into<String>>(mut self, description: T) -> Self {
		self.description = Some(description.into());
		self
	}

	/// Set command's version
	pub fn version<T: Into<String>>(mut self, version: T) -> Self {
		self.version = version.into();
		self
	}

	/// Add command's sub command
	pub fn sub_command(mut self, sub_command: Command) -> Self {
		self.sub.push(sub_command);
		self
	}

	/// Add sub commands
	pub fn sub_commands(mut self, sub_commands: Vec<Command>) -> Self {
		self.sub.append_vec(sub_commands);
		self
	}

	/// Sets license
	pub fn license(mut self, license: License) -> Self {
		self.license = license;
		self
	}

	/// Add command's alias
	pub fn alias<T: Into<String>>(mut self, a: T) -> Self {
		self.alias.push(a.into());
		self
	}

	/// Returns true if name_or_alias matches command's name or one of alias at least
	/// name_or_aliasがコマンド名かエイリアスのうち少なくとも一つにマッチした場合trueを返す
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

	/// Take sub command matches name_or_alias.
	/// name_or_aliasに一致するサブコマンドがある場合、保持しているVectorからswap_removeで取り出して返す
	pub fn take_sub(&mut self, name_or_alias: &str) -> Option<Command> {
		match self.sub {
			Vector(None) => None,
			Vector(Some(ref mut inner)) => match inner.into_iter().position(|c| c.is(name_or_alias)) {
				None => None,
				Some(index) => Some(inner.swap_remove(index)),
			},
		}
	}

	/// Gets sub command mutable reference matches name_or_alias.
	pub fn get_mut_sub(&mut self, name_or_alias: &str) -> Option<&mut Command> {
		match self.sub {
			Vector(None) => None,
			Vector(Some(ref mut inner)) => match inner.into_iter().position(|c| c.is(name_or_alias)) {
				None => None,
				Some(index) => Some(inner.get_mut(index).unwrap()),
			},
		}
	}

	/// Returns true if this command has sub command(s).
	pub fn has_sub(&self) -> bool {
		self.sub.has_inner_vec()
	}

	/// Returns init Vector for Context's route
	pub fn derive_route_init_vector(&self) -> Vector<String> {
		if self.name.is_empty() {
			Vector(None)
		} else {
			Vector(Some(vec![self.name.clone()]))
		}
	}
}

impl From<String> for Command {
	fn from(name: String) -> Self {
		Command {
			name,
			action: None,
			authors: String::default(),
			copyright: String::default(),
			license: License::default(),
			description: None,
			usage: String::default(),
			l_flags: Vector::default(),
			c_flags: Vector::default(),
			alias: Vector::default(),
			version: String::default(),
			sub: Vector::default(),
		}
	}
}

/// Trait for implementation Run function.
pub trait Run<T> {
	/// run function
	fn run(self, args: T) -> run_result!();
}

impl Run<Vec<String>> for Command {
	fn run(self, args: Vec<String>) -> run_result!() {
		self.run_from_args(args)
	}
}

impl Run<Context> for Command {
	fn run(self, c: Context) -> run_result!() {
		self.run_with_context(c)
	}
}

impl Command {
	/// Run commands with raw_args
	pub fn run_from_args(mut self, raw_args: Vec<String>) -> run_result!() {
		if self.sub.is_none() {
			return self.single_run(raw_args);
		}
		let mut args = VecDeque::from(raw_args.clone());
		let exe_path = args.pop_front().unwrap();
		let head = args.pop_front();
		if head.is_none() {
			//引数がない場合
			let c = gen_context_for_self_action!(self, raw_args, args, exe_path);
			match self.action {
				Some(action) => action(c, self),
				None => no_registered_error!(c),
			}
		} else {
			//get before first non-flag arg with parsing flags
			let p = Parser::default();
			match head {
				Some(long_flag) if p.long_flag(&long_flag) => {
					//long flag
					let last = p.long_middle(long_flag);
					self.assign_run(args, VecDeque::new(), p, raw_args, exe_path, last)
				}
				Some(short_flag) if p.flag(&short_flag) => {
					//short flag
					let last = p.short_middle(short_flag);
					self.assign_run(args, VecDeque::new(), p, raw_args, exe_path, last)
				}
				Some(arg) => {
					match self.take_sub(&arg) {
						None => {
							// ルートコマンド実行のとき
							args.push_front(arg);
							let mut c = gen_context_for_self_action!(self, raw_args, args, exe_path);
							match self.action {
								None => no_registered_error!(c),
								Some(action) => {
									c = p.parse_args_until_end(&self.l_flags, c);
									action(c, self)
								}
							}
						}
						Some(mut sub) => {
							// サブコマンドがヒットしたとき
							let c = gen_context_for_sub_run!(self, sub, raw_args, args, exe_path);
							let r = sub.run(c);
							// サブコマンドの結果をハンドリング
							self.handle_sub_result(r)
						}
					}
				}
				_ => {
					//Because None has already excluded, this area must be unreachable.
					panic!("unexpected error");
				}
			}
		}
	}

	/// Run command with context
	pub fn run_with_context(mut self, mut context: Context) -> run_result!() {
		if self.sub.is_none() {
			// サブコマンドがない場合
			context.common_flags.push(self.c_flags.take());
			let p = Parser::default();
			let (mut context, non_flag_args) =
				p.parse_inter_mediate_args(&self.l_flags, context, true);
			if let Some(mut non_flag_args) = non_flag_args {
				non_flag_args.append(&mut context.args);
				context.args = non_flag_args;
			}
			context = p.parse_args_until_end(&self.l_flags, context);
			context.routes.push(self.name.clone());
			match self.action {
				Some(action) => action(context, self),
				None => no_registered_error!(context),
			}
		} else {
			//サブコマンドと一致するかを捜査
			context.routes.push(self.name.clone());
			let p = Parser::default();
			match context.args.pop_front() {
				Some(long_flag) if p.long_flag(&long_flag) => {
					let last = p.long_middle(long_flag);
					self.assign_context(context, p, VecDeque::new(), last)
				}
				Some(short_flag) if p.flag(&short_flag) => {
					let last = p.short_middle(short_flag);
					self.assign_context(context, p, VecDeque::new(), last)
				}
				Some(arg) => {
					//println!("arg sub-sub: {}", &arg);
					match self.take_sub(&arg) {
						Some(mut sub) => {
							context.common_flags.push(self.c_flags.take());
							sub_check!(context, sub, self);
							println!("{:?}", &context);
							let r = sub.run(context);
							self.handle_sub_result(r)
						}
						None => {
							context.common_flags.push(self.c_flags.take());
							match self.action {
								None => no_registered_error!(context),
								Some(action) => {
									let c = match p.parse_inter_mediate_args(&self.l_flags, context, true) {
										(mut context, None) => {
											context = p.parse_args_until_end(&self.l_flags, context);
											context.args.push_front(arg);
											context
										}
										(mut context, Some(mut non_flag_args)) => {
											context = p.parse_args_until_end(&self.l_flags, context);
											context.args.push_front(arg);
											non_flag_args.append(&mut context.args);
											context.args = non_flag_args;
											context
										}
									};
									action(c, self)
								}
							}
						}
					}
				}
				None => {
					//サブコマンド等の処理の必要がないのでこのまま叩き込む
					match self.action {
						Some(action) => {
							context.common_flags.push(self.c_flags.take());

							let (mut context, non_flag_args) =
								p.parse_inter_mediate_args(&self.l_flags, context, true);
							if let Some(mut non_flag_args) = non_flag_args {
								non_flag_args.append(&mut context.args);
								context.args = non_flag_args;
							}
							action(context, self)
						}
						None => {
							context.common_flags.push(self.c_flags.take());

							let (mut context, non_flag_args) =
								p.parse_inter_mediate_args(&self.l_flags, context, true);
							if let Some(mut non_flag_args) = non_flag_args {
								non_flag_args.append(&mut context.args);
								context.args = non_flag_args;
							}
							no_registered_error!(context)
						}
					}
				}
			}
		}
	}

	/// Assign context to sub command or self own action.
	/// コンテキストのargsを見てもサブコマンド行きかコマンドでそのまま処理すればいいか分からなかった時の処理用
	pub fn assign_context(
		mut self,
		mut c: Context,
		p: Parser,
		mut inter_mediate_args: VecDeque<MiddleArg>,
		last: MiddleArg,
	) -> run_result!() {
		let (next_non_flag, args, _inter_mediate_args, last) =
			p.middle_parse(c.args, inter_mediate_args, last);
		inter_mediate_args = _inter_mediate_args;
		//println!("next_non_flag: {:?}", next_non_flag);
		match next_non_flag {
			Some(arg) => match self.take_sub(&arg) {
				Some(mut sub) => {
					// サブコマンド実行だった時
					c.common_flags.push(self.c_flags.take());
					c.args = args;
					inter_mediate_args.push_back(last);
					if let Some(mut parsing_args) = c.parsing_args {
						parsing_args.append(&mut inter_mediate_args);
						c.parsing_args = Some(parsing_args);
					} else {
						c.parsing_args = Some(inter_mediate_args);
					}
					sub_check!(c, sub, self);
					let r = sub.run(c);
					self.handle_sub_result(r)
				}
				None => match &last {
					MiddleArg::LongFlag(_, FlagValue::None)
					| MiddleArg::ShortFlag(_, FlagValue::None) => {
						// 値が設定されていないフラグが前の引数の時はそのフラグの値として扱い、次の引数をハンドリング
						inter_mediate_args.push_back(last);
						inter_mediate_args.push_back(MiddleArg::Normal(arg));
						c.args = args;
						match c.args.pop_front() {
							Some(long_flag) if p.long_flag(&long_flag) => {
								let last = p.long_middle(long_flag);
								self.assign_context(c, p, inter_mediate_args, last)
							}
							Some(short_flag) if p.flag(&short_flag) => {
								let last = p.short_middle(short_flag);
								self.assign_context(c, p, inter_mediate_args, last)
							}
							Some(arg) => match self.take_sub(&arg) {
								Some(mut sub) => {
									c.common_flags.push(self.c_flags.take());
									if let Some(mut parsing_args) = c.parsing_args {
										parsing_args.append(&mut inter_mediate_args);
										c.parsing_args = Some(parsing_args);
									} else {
										c.parsing_args = Some(inter_mediate_args);
									}
									sub_check!(c, sub, self);
									let r = sub.run(c);
									self.handle_sub_result(r)
								}
								None => {
									if let Some(mut parsing_args) = c.parsing_args {
										parsing_args.append(&mut inter_mediate_args);
										c.parsing_args = Some(parsing_args);
									} else {
										c.parsing_args = Some(inter_mediate_args);
									}
									c.common_flags.push(self.c_flags.take());
									let (mut c, non_flag_args) =
										p.parse_inter_mediate_args(&self.l_flags, c, false);
									c = p.parse_args_until_end(&self.l_flags, c);
									c.args.push_front(arg);
									if let Some(mut non_flag_args) = non_flag_args {
										non_flag_args.append(&mut c.args);
										c.args = non_flag_args;
									}
									match self.action {
										Some(action) => action(c, self),
										None => self.handle_sub_result(no_registered_error!(c)),
									}
								}
							},
							None => {
								if let Some(mut parsing_args) = c.parsing_args {
									parsing_args.append(&mut inter_mediate_args);
									c.parsing_args = Some(parsing_args);
								} else {
									c.parsing_args = Some(inter_mediate_args);
								}
								let (mut c, args) = p.parse_inter_mediate_args(&self.l_flags, c, false);

								if let Some(mut args) = args {
									args.append(&mut c.args);
									c.args = args;
								}
								match self.action {
									Some(action) => action(c, self),
									None => self.handle_sub_result(no_registered_error!(c)),
								}
							}
						}
					}
					_ => {
						inter_mediate_args.push_back(last);
						c.args = args;
						c.common_flags.push(self.c_flags.take());
						if let Some(mut parsing_args) = c.parsing_args {
							parsing_args.append(&mut inter_mediate_args);
							c.parsing_args = Some(parsing_args);
						}
						let (mut c, non_flag_args) = p.parse_inter_mediate_args(&self.l_flags, c, false);
						c = p.parse_args_until_end(&self.l_flags, c);
						c.args.push_front(arg);
						if let Some(mut non_flag_args) = non_flag_args {
							non_flag_args.append(&mut c.args);
							c.args = non_flag_args;
						}
						match self.action {
							Some(action) => action(c, self),
							None => no_registered_error!(c),
						}
					}
				},
			},
			None => {
				inter_mediate_args.push_back(last);
				c.args = args;
				if let Some(mut parsing_args) = c.parsing_args {
					parsing_args.append(&mut inter_mediate_args);
					c.parsing_args = Some(parsing_args);
				} else {
					c.parsing_args = Some(inter_mediate_args);
				}
				c.common_flags.push(self.c_flags.take());
				let (mut c, non_flag_args) = p.parse_inter_mediate_args(&self.l_flags, c, false);
				//println!("after_parse_ima:{:?}", c);
				if let Some(non_flag_args) = non_flag_args {
					//non_flag_args.append(&mut c.args);
					c.args = non_flag_args;
				}
				match self.action {
					Some(action) => action(c, self),
					None => no_registered_error!(c),
				}
			}
		}
	}

	/// Assign subcomannd's run or command's own action with no context
	/// コンテキストが生成されていないときに、run_from_args内で第一引数からサブコマンドかそうでないか分からなかった時に再帰処理を行って割り当てを行う関数
	pub fn assign_run(
		mut self,
		mut args: VecDeque<String>,
		mut inter_mediate_args: VecDeque<MiddleArg>,
		p: Parser,
		raw_args: Vec<String>,
		exe_path: String,
		last: MiddleArg,
	) -> run_result!() {
		let (next_non_flag, _args, _inter_mediate_args, last) =
			p.middle_parse(args, inter_mediate_args, last);
		inter_mediate_args = _inter_mediate_args;
		args = _args;
		match next_non_flag {
			Some(arg) => {
				match self.take_sub(&arg) {
					Some(mut sub) => {
						inter_mediate_args.push_back(last);
						let c = gen_context_for_sub_run!(
							self,
							sub,
							raw_args,
							args,
							exe_path,
							inter_mediate_args
						);
						let r = sub.run(c);
						r
					}
					None => {
						//一致するサブコマンドがなかった場合
						match &last {
							MiddleArg::LongFlag(_, FlagValue::None)
							| MiddleArg::ShortFlag(_, FlagValue::None) => {
								//フラグの値になりうる場合
								inter_mediate_args.push_back(last);
								inter_mediate_args.push_back(MiddleArg::Normal(arg));
								match args.pop_front() {
									Some(long_flag) if p.long_flag(&long_flag) => {
										let last = p.long_middle(long_flag);
										self.assign_run(args, inter_mediate_args, p, raw_args, exe_path, last)
									}
									Some(short_flag) if p.flag(&short_flag) => {
										let last = p.short_middle(short_flag);
										self.assign_run(args, inter_mediate_args, p, raw_args, exe_path, last)
									}
									Some(arg) => match self.take_sub(&arg) {
										Some(mut sub) => {
											let c = gen_context_for_sub_run!(
												self,
												sub,
												raw_args,
												args,
												exe_path,
												inter_mediate_args
											);
											let r = sub.run(c);
											r
										}
										None => {
											//サブコマンドはないのでそのままselfでaction
											let c = gen_context_for_self_action!(
												self,
												raw_args,
												args,
												exe_path,
												inter_mediate_args
											);
											let (mut c, non_flag_args) =
												p.parse_inter_mediate_args(&self.l_flags, c, false);
											c = p.parse_args_until_end(&self.l_flags, c);
											c.args.push_front(arg);
											if let Some(mut non_flag_args) = non_flag_args {
												non_flag_args.append(&mut c.args);
												c.args = non_flag_args;
											};
											match self.action {
												Some(action) => action(c, self),
												None => no_registered_error!(c),
											}
										}
									},
									None => {
										//残りのargはなし、そのままaction
										let c = gen_context_for_self_action!(
											self,
											raw_args,
											args,
											exe_path,
											inter_mediate_args
										);
										let (mut c, non_flag_args) =
											p.parse_inter_mediate_args(&self.l_flags, c, false);
										if let Some(mut non_flag_args) = non_flag_args {
											non_flag_args.append(&mut c.args);
											c.args = non_flag_args;
										}
										match self.action {
											Some(action) => action(c, self),
											None => no_registered_error!(c),
										}
									}
								}
							}
							_ => {
								//argがフラグの可能性がない
								inter_mediate_args.push_back(last);
								//inter_mediate_args.push_back(MiddleArg::Normal(arg));

								let c = gen_context_for_self_action!(
									self,
									raw_args,
									args,
									exe_path,
									inter_mediate_args
								);
								let (mut c, non_flag_args) =
									p.parse_inter_mediate_args(&self.l_flags, c, false);
								c = p.parse_args_until_end(&self.l_flags, c);
								c.args.push_front(arg);
								if let Some(mut non_flag_args) = non_flag_args {
									non_flag_args.append(&mut c.args);
									c.args = non_flag_args;
								}
								match self.action {
									Some(action) => action(c, self),
									_ => self.handle_sub_result(no_registered_error!(c)),
								}
							}
						}
					}
				}
			}
			None => {
				//argがなかった場合
				//self.actionに放り込む
				inter_mediate_args.push_back(last);
				let context =
					gen_context_for_self_action!(self, raw_args, args, exe_path, inter_mediate_args);
				let (mut c, non_flag_args) = p.parse_inter_mediate_args(&self.l_flags, context, false);
				if let Some(mut non_flag_args) = non_flag_args {
					non_flag_args.append(&mut c.args);
					c.args = non_flag_args;
				}

				match self.action {
					Some(action) => action(c, self),
					None => self.handle_sub_result(no_registered_error!(c)),
				}
			}
		}
	}

	/// Handle sub action's result (Result<ActionResult, ActionError>).
	///Implemented: at ParentActionRequest and Err
	/// アクションの結果であるResult<ActionResult, ActionError>をハンドルする関数。現在はParentActionRequestのハンドリング、もしくはエラー表示のみ
	pub fn handle_sub_result(mut self, mut req: Result<ActionResult, ActionError>) -> run_result!() {
		match req {
			done!() => {
				// Doneなら何もしないでreqを上にあげる
				req
			}
			Ok(ActionResult::ParentActionRequest(c, cmd, action)) => {
				// サブコマンドからリクエストが飛んでいた時はselfを与えてリクエストされたアクションを実行
				self.sub.push(cmd);
				action(c, self)
			}
			Err(ref mut err) => {
				if !err.printed {
					println!("error: {}", err);
				}
				err.printed = true;
				req
			}
			_ => req,
		}
	}
}
#[cfg(test)]
mod tests {
	use crate::license;

	use super::super::parser::ParseError;
	use super::super::{Flag, FlagType};
	use super::*;

	fn cnv_arg(mut v: Vec<&str>) -> Vec<String> {
		v.iter_mut().map(|s| s.to_owned()).collect()
	}

	#[test]
	fn single_run() {
		let mut arg = vec![
			"exe_path".to_string(),
			"test".to_string(),
			"test".to_string(),
		];
		let root = base_root().action(|c, _| {
			println!("test_action: {:?}", c);
			let raw_args = vec![
				"exe_path".to_string(),
				"test".to_string(),
				"test".to_string(),
			];
			let expect_args = {
				let mut vd = VecDeque::from(raw_args.clone());
				vd.pop_front();
				vd
			};
			assert_eq!(c.raw_args, raw_args);
			assert_eq!(c.args, expect_args);
			assert_eq!(c.exe_path, String::from("exe_path"));
			assert_eq!(c.routes, Vector(None));
			return done!();
		});

		let _ = root.single_run(arg.clone());

		arg.push("--common=C_after".into());
		arg.push("--local=L_after".into());
		arg.insert(1, "--common=C_before".into());
		arg.insert(1, "--local=L_before".into());
		let root = Command::with_name("root")
			.action(|c, _| {
				println!("test_action: {:?}", c);
				let raw_args: Vec<String> = vec![
					"exe_path".to_string(),
					"--local=L_before".to_string(),
					"--common=C_before".to_string(),
					"test".to_string(),
					"test".to_string(),
					"--common=C_after".to_string(),
					"--local=L_after".to_string(),
				];
				let expect_args = VecDeque::from(vec!["test".to_string(), "test".to_string()]);
				assert_eq!(c.raw_args, raw_args);
				assert_eq!(c.args, expect_args);
				assert_eq!(c.exe_path, String::from("exe_path"));
				assert_eq!(c.routes, Vector(Some(vec!["root".to_string()])));
				let l_flag_values = Vector::from(vec![
					(
						"local".to_string(),
						FlagValue::String("L_before".to_owned()),
					),
					("local".to_string(), FlagValue::String("L_after".into())),
				]);
				assert_eq!(c.local_flags_values, l_flag_values);
				let c_flag_values = Vector::from(vec![
					(
						"common".to_string(),
						FlagValue::String("C_before".to_owned()),
					),
					("common".to_string(), FlagValue::String("C_after".into())),
				]);
				assert_eq!(c.common_flags_values, c_flag_values);
				assert_eq!(c.cmd_version, "root_version".to_owned());
				assert_eq!(c.cmd_copyright, "root_copyright".to_owned());
				assert_eq!(c.cmd_license.unwrap().0, String::from("root_license"));
				done!()
			})
			.common_flag(Flag::new(
				"common",
				FlagType::default(),
				"sample common flag",
			))
			.local_flag(Flag::new("local", FlagType::default(), "sample local flag"))
			.version("root_version")
			.copyright("root_copyright")
			.license(license!(
				"root_license".into(),
				content=>"root_license_content".into()
			));

		let _ = root.single_run(arg);
	}

	#[test]
	fn run_root() {
		let arg = vec![
			"exe_path".to_string(),
			"test".to_string(),
			"test".to_string(),
			"--local".to_string(),
			"test".to_string(),
		];
		let root = Command::new()
			.action(|mut c, now| {
				println!("test_action: {:?}", c);
				let raw_args = vec![
					"exe_path".to_string(),
					"test".to_string(),
					"test".to_string(),
					"--local".to_string(),
					"test".to_string(),
				];
				let expect_args = VecDeque::from(vec!["test".to_string(), "test".to_string()]);
				assert_eq!(c.raw_args, raw_args);
				assert_eq!(c.args, expect_args);
				assert_eq!(c.exe_path, String::from("exe_path"));
				assert_eq!(
					c.get_local_flag_value_of("local", &now),
					Some(FlagValue::String("test".into()))
				);
				assert_eq!(
					c.get_flag_value_of("local", &now),
					Some(FlagValue::String("test".into()))
				);
				assert_eq!(
					c.get_flag_value_of("common", &now),
					Some(FlagValue::String(String::default()))
				);
				assert_eq!(c.routes, Vector(None));
				assert_eq!(c.cmd_version, "root_version".to_owned());
				assert_eq!(c.cmd_copyright, "root_copyright".to_owned());
				let License(inner) = c.cmd_license.take();
				let inner = inner.unwrap();
				assert_eq!(inner.0, String::from("root_license"));
				assert_eq!(inner.1(&c), String::from("root_license_content"));
				done!()
			})
			.common_flag(Flag::new(
				"common",
				FlagType::default(),
				"sample common flag",
			))
			.local_flag(Flag::new("local", FlagType::default(), "sample local flag"))
			.sub_command(Command::with_name("sub").action(|_, _| {
				println!("sub");
				done!()
			}))
			.version("root_version")
			.copyright("root_copyright")
			.license(license!(
				expr=>"root_license".into(),
				content=>"root_license_content".into()
			));
		let _ = root.run(arg);
	}

	fn base_root() -> Command {
		Command::new()
			.local_flag(Flag::new_string("local").short_alias('l'))
			.local_flag(Flag::new_bool("lafter").short_alias('a'))
			.common_flag(Flag::new_bool("common").short_alias('c'))
			.common_flag(Flag::new_string("cstr").short_alias('s'))
			.common_flag(Flag::new_bool("cafter"))
			.version("root_version")
			.copyright("root_copyright")
			.license(license!(
				"root_license".into(),
				content=>"root_license_content".into(),
			))
			.authors("root_authors")
	}

	macro_rules! assert_attrs {
		($prefix:expr, $context:expr) => {
			let prefix = String::from($prefix);
			assert_eq!($context.cmd_authors, prefix.clone() + "authors");
			assert_eq!($context.cmd_version, prefix.clone() + "version");
			assert_eq!($context.cmd_copyright, prefix.clone() + "copyright");
			assert_eq!(
				(
					$context.cmd_license.expr_unwrap(),
					$context.cmd_license.output_unwrap(&$context)
				),
				(prefix.clone() + "license", prefix + "license_content")
			);
		};
	}
	#[test]
	fn run_root_with_flag_before_normal_arg() {
		let mut arg = cnv_arg(vec!["exe_path", "--local=test"]);
		let root = base_root().sub_command(Command::with_name("sub").action(|c, _| {
			panic!("not root, in sub: {:?}", c);
		}));
		arg.push("test".into());
		let _ = root
			.clone()
			.action(|c, now| {
				println!("c: {:?}", c);
				assert_eq!(
					cnv_arg(vec!["exe_path", "--local=test", "test"]),
					c.raw_args
				);
				assert_eq!(
					c.get_flag_value_of("local", &now).unwrap(),
					FlagValue::String("test".into())
				);
				assert_eq!(c.routes, Vector(None));
				assert_attrs!("root_", c);
				done!()
			})
			.run(arg.clone());
		arg[2] = "--common".into();
		let _ = root
			.clone()
			.action(|c, cur| {
				println!("c: {:?}", c);
				assert_eq!(
					cnv_arg(vec!["exe_path", "--local=test", "--common"]),
					c.raw_args
				);
				assert_eq!(
					c.get_flag_value_of("local", &cur).unwrap(),
					FlagValue::String("test".into())
				);
				assert_eq!(
					c.get_flag_value_of("common", &cur).unwrap(),
					FlagValue::Bool(true)
				);
				assert_eq!(c.routes, Vector(None));
				assert_attrs!("root_", c);
				done!()
			})
			.run(arg.clone());

		arg.push("test".into());
		let _ = root
			.clone()
			.action(|c, cc| {
				println!("{:?}", c);
				assert_eq!(
					cnv_arg(vec!["exe_path", "--local=test", "--common", "test"]),
					c.raw_args
				);
				assert_eq!(VecDeque::from(vec!["test".to_string()]), c.args);
				assert_eq!(
					c.get_flag_value_of("local", &cc).unwrap(),
					FlagValue::String("test".into())
				);
				assert_eq!(
					c.get_flag_value_of("common", &cc).unwrap(),
					FlagValue::Bool(true)
				);
				assert_eq!(c.routes, Vector(None));
				assert_attrs!("root_", c);
				done!()
			})
			.run(arg.clone());

		println!("arg after flags");
		arg.push("arg".into());
		arg.push("ex_arg".into());
		arg.push("--lafter".into());
		arg.push("--cafter".into());
		let _ = root
			.clone()
			.action(|c, cur| {
				println!("{:?}", c);
				assert_eq!(
					cnv_arg(vec![
						"exe_path",
						"--local=test",
						"--common",
						"test",
						"arg",
						"ex_arg",
						"--lafter",
						"--cafter"
					]),
					c.raw_args
				);
				assert_eq!(
					VecDeque::from(cnv_arg(vec!["test", "arg", "ex_arg"])),
					c.args
				);
				assert_eq!(
					c.get_flag_value_of("local", &cur).unwrap(),
					FlagValue::String("test".into())
				);
				assert_eq!(
					c.get_flag_value_of("common", &cur).unwrap(),
					FlagValue::Bool(true)
				);
				assert_eq!(
					c.get_flag_value_of("cafter", &cur).unwrap(),
					FlagValue::Bool(true)
				);
				assert_eq!(
					c.get_flag_value_of("lafter", &cur).unwrap(),
					FlagValue::Bool(true)
				);
				assert_eq!(c.routes, Vector(None));
				assert_attrs!("root_", c);
				done!()
			})
			.run(arg.clone());

		arg.remove(5);
		arg.remove(4);
		arg.insert(5, "arg".into());

		let _ = root
			.clone()
			.action(|c, cur| {
				println!("{:?}", c);
				assert_eq!(
					c.raw_args,
					cnv_arg(vec![
						"exe_path",
						"--local=test",
						"--common",
						"test",
						"--lafter",
						"arg",
						"--cafter"
					])
				);
				assert_eq!(VecDeque::from(cnv_arg(vec!["test", "arg"])), c.args);
				assert_eq!(
					c.get_flag_value_of("local", &cur).unwrap(),
					FlagValue::String("test".into())
				);
				assert_eq!(
					c.get_flag_value_of("common", &cur).unwrap(),
					FlagValue::Bool(true)
				);
				assert_eq!(
					c.get_flag_value_of("cafter", &cur).unwrap(),
					FlagValue::Bool(true)
				);
				assert_eq!(
					c.get_flag_value_of("lafter", &cur).unwrap(),
					FlagValue::Bool(true)
				);
				assert_eq!(c.routes, Vector(None));
				assert_attrs!("root_", c);
				done!()
			})
			.run(arg.clone());

		arg.push("ex_arg".into());
		let _ = root
			.clone()
			.action(|c, cur| {
				println!("{:?}", c);
				assert_eq!(
					c.raw_args,
					cnv_arg(vec![
						"exe_path",
						"--local=test",
						"--common",
						"test",
						"--lafter",
						"arg",
						"--cafter",
						"ex_arg"
					])
				);
				assert_eq!(
					VecDeque::from(cnv_arg(vec!["test", "arg", "ex_arg"])),
					c.args
				);
				assert_eq!(
					c.get_flag_value_of("local", &cur).unwrap(),
					FlagValue::String("test".into())
				);
				assert_eq!(
					c.get_flag_value_of("common", &cur).unwrap(),
					FlagValue::Bool(true)
				);
				assert_eq!(
					c.get_flag_value_of("cafter", &cur).unwrap(),
					FlagValue::Bool(true)
				);
				assert_eq!(
					c.get_flag_value_of("lafter", &cur).unwrap(),
					FlagValue::Bool(true)
				);
				assert_eq!(c.routes, Vector(None));
				assert_attrs!("root_", c);
				done!()
			})
			.run(arg.clone());
		arg[4] = "-a".into();
		let _ = root
			.clone()
			.action(|c, cur| {
				println!("{:?}", c);
				assert_eq!(
					c.raw_args,
					cnv_arg(vec![
						"exe_path",
						"--local=test",
						"--common",
						"test",
						"-a",
						"arg",
						"--cafter",
						"ex_arg"
					])
				);
				assert_eq!(
					VecDeque::from(cnv_arg(vec!["test", "arg", "ex_arg"])),
					c.args
				);
				assert_eq!(
					c.get_flag_value_of("local", &cur).unwrap(),
					FlagValue::String("test".into())
				);
				assert_eq!(
					c.get_flag_value_of("common", &cur).unwrap(),
					FlagValue::Bool(true)
				);
				assert_eq!(
					c.get_flag_value_of("cafter", &cur).unwrap(),
					FlagValue::Bool(true)
				);
				assert_eq!(
					c.get_flag_value_of("lafter", &cur).unwrap(),
					FlagValue::Bool(true)
				);
				assert_eq!(c.routes, Vector(None));
				assert_attrs!("root_", c);
				done!()
			})
			.run(arg.clone());
	}

	#[test]
	fn run_node() {
		let mut arg = cnv_arg(vec![
			"exe_path", "sub", "test", "--common", "test", "--cstr", "strt", "-b", "--local",
		]);
		let root = base_root().action(|c, _| {
			println!("test_action: {:?}", c);
			panic!("not sub");
		});
		let sub = Command::with_name("sub")
			.local_flag(Flag::new_bool("bool").short_alias('b'))
			.local_flag(Flag::new_string("string").short_alias('s'))
			.common_flag(Flag::new_bool("cl"))
			.sub_command(Command::with_name("leaf").action(|c, _| {
				println!("Context: {:?}", c);
				panic!("in leaf")
			}))
			.authors("sub_authors")
			.version("sub_version")
			.copyright("sub_copyright")
			.license(license!(
				"sub_license".into(),
				content=>"sub_license_content".into(),
			));
		let _ = root
			.clone()
			.sub_command(sub.clone().action(|c, cur| {
				println!("{:?}", c);
				let raw_args = cnv_arg(vec![
					"exe_path", "sub", "test", "--common", "test", "--cstr", "strt", "-b", "--local",
				]);
				let expect_args = VecDeque::from(vec!["test".to_string(), "test".to_string()]);
				assert_eq!(c.exe_path, String::from("exe_path"));
				assert_eq!(c.raw_args, raw_args);
				assert_eq!(c.args, expect_args);
				assert_eq!(
					c.get_flag_value_of("common", &cur),
					Some(FlagValue::Bool(true))
				);
				assert_eq!(
					c.get_flag_value_of("bool", &cur).unwrap(),
					FlagValue::Bool(true)
				);
				assert_eq!(c.get_flag_value_of("commons", &cur), None);
				assert_eq!(c.get_flag_value_of("local", &cur), None);
				assert_eq!(c.routes, "sub".to_owned().into());
				assert_attrs!("sub_", c);
				done!()
			}))
			.run(arg.clone());

		println!("サブコマンド前フラグのテスト");
		arg = cnv_arg(vec!["exe_path", "--cstr=test", "-b", "sub"]);
		let _ = root
			.clone()
			.name("root")
			.sub_command(sub.clone().action(|c, cur| {
				println!("c: {:?}", c);
				assert_eq!(
					c.raw_args,
					cnv_arg(vec!["exe_path", "--cstr=test", "-b", "sub"])
				);
				assert_eq!(
					c.get_flag_value_of("cstr", &cur).unwrap(),
					FlagValue::String("test".into())
				);
				assert_eq!(
					c.get_flag_value_of("bool", &cur).unwrap(),
					FlagValue::Bool(true)
				);
				assert_eq!(
					c.routes,
					Vector(Some(vec!["root".to_string(), "sub".to_string()]))
				);
				assert_attrs!("sub_", c);
				done!()
			}))
			.run(arg.clone());

		println!("サブコマンド探しをする場合");
		arg[1] = "--cstr".into();
		arg.insert(2, "test".into());

		let _ = root
			.clone()
			.sub_command(sub.clone().action(|c, cur| {
				println!("c:{:?}", c);
				assert_eq!(
					c.raw_args,
					cnv_arg(vec!["exe_path", "--cstr", "test", "-b", "sub"])
				);
				assert_eq!(
					c.get_flag_value_of("cstr", &cur).unwrap(),
					FlagValue::String("test".into())
				);
				assert_eq!(
					c.get_flag_value_of("bool", &cur).unwrap(),
					FlagValue::Bool(true)
				);
				assert_eq!(c.routes, Vector(Some(vec!["sub".to_string()])));
				assert_attrs!("sub_", c);
				done!()
			}))
			.run(arg.clone());

		arg.remove(2);
		arg[1] = "--cstr=test".into();
		arg.push("test_arg".into());
		arg.push("--cafter".into());
		arg.push("test_arg2".into());
		arg.push("--string".into());
		arg.push("testStr".into());
		let _ = root
			.clone()
			.sub_command(sub.clone().action(|c, cur| {
				println!("{:?}", c);
				assert_eq!(
					c.raw_args,
					cnv_arg(vec![
						"exe_path",
						"--cstr=test",
						"-b",
						"sub",
						"test_arg",
						"--cafter",
						"test_arg2",
						"--string",
						"testStr"
					])
				);
				assert_eq!(c.args, cnv_arg(vec!["test_arg", "test_arg2"]));
				assert_eq!(
					c.get_flag_value_of("cstr", &cur).unwrap(),
					FlagValue::String("test".into())
				);
				assert_eq!(
					c.get_flag_value_of("bool", &cur).unwrap(),
					FlagValue::Bool(true)
				);
				assert_eq!(
					c.get_flag_value_of("cafter", &cur).unwrap(),
					FlagValue::Bool(true)
				);
				assert_eq!(c.get_local_flag_value_of("cafter", &cur), None);
				assert_eq!(
					c.get_flag_value_of("string", &cur).unwrap(),
					FlagValue::String("testStr".into())
				);
				assert_eq!(c.routes, Vector(Some(vec!["sub".to_string()])));
				assert_attrs!("sub_", c);
				done!()
			}))
			.run(arg.clone());

		println!("\r\n\r\nサブサブコマンドが存在する場合の判別系\r\n");
		arg.remove(4);
		let _ = root
			.clone()
			.sub_command(sub.clone().action(|c, cur| {
				println!("result_c: {:?}", c);
				assert_eq!(
					c.raw_args,
					cnv_arg(vec![
						"exe_path",
						"--cstr=test",
						"-b",
						"sub",
						"--cafter",
						"test_arg2",
						"--string",
						"testStr"
					])
				);
				assert_eq!(c.args, vec!["test_arg2".to_string()]);
				assert_eq!(
					c.get_common_flag_value_of("cstr").unwrap(),
					FlagValue::String("test".into())
				);
				assert_eq!(
					c.get_local_flag_value_of("string", &cur).unwrap(),
					FlagValue::String("testStr".into())
				);
				assert_eq!(
					c.get_inputted_common_flag_value_of("cafter").unwrap(),
					FlagValue::None
				);
				assert_eq!(c.routes, Vector(Some(vec!["sub".to_string()])));
				assert_attrs!("sub_", c);
				done!()
			}))
			.run(arg.clone());
		println!("\r\n\r\nサブサブコマンドが存在する場合の判別系その2\r\n");
		arg.push("ex_arg".into());
		arg[5] = "test_arg".to_owned();

		let _ = root
			.clone()
			.sub_command(sub.clone().action(|c, _| {
				println!("C: {:?}", c);
				assert_eq!(
					c.raw_args,
					cnv_arg(vec![
						"exe_path",
						"--cstr=test",
						"-b",
						"sub",
						"--cafter",
						"test_arg",
						"--string",
						"testStr",
						"ex_arg"
					])
				);
				assert_eq!(c.args, cnv_arg(vec!["test_arg", "ex_arg"]));
				assert_eq!(c.routes, Vector(Some(vec!["sub".to_string()])));
				assert_attrs!("sub_", c);
				done!()
			}))
			.run(arg.clone());
		arg[6] = "--string=testStr".into();
		arg[8] = "test_arg2".into();
		arg.remove(7);
		arg.push("test_arg3".into());
		arg.push("--common".into());
		arg.push("test_arg4".into());

		let _ = root
			.clone()
			.sub_command(sub.clone().action(|c, cur| {
				println!("C: {:?}", c);
				assert_eq!(
					c.raw_args,
					cnv_arg(vec![
						"exe_path",
						"--cstr=test",
						"-b",
						"sub",
						"--cafter",
						"test_arg",
						"--string=testStr",
						"test_arg2",
						"test_arg3",
						"--common",
						"test_arg4"
					])
				);
				assert_eq!(
					c.args,
					vec![
						"test_arg".to_owned(),
						"test_arg2".to_owned(),
						"test_arg3".to_owned(),
						"test_arg4".to_owned()
					]
				);
				assert_eq!(
					c.get_common_flag_value_of("cstr").unwrap(),
					FlagValue::String("test".into())
				);
				assert_eq!(
					c.get_local_flag_value_of("bool", &cur).unwrap(),
					FlagValue::Bool(true)
				);
				assert_eq!(
					c.get_common_flag_value_of("cafter").unwrap(),
					FlagValue::Bool(true)
				);
				assert_eq!(
					c.get_flag_value_of("string", &cur).unwrap(),
					FlagValue::String("testStr".into())
				);
				assert_eq!(c.routes, Vector(Some(vec!["sub".to_string()])));
				assert_attrs!("sub_", c);
				done!()
			}))
			.run(arg.clone());

		arg.pop();
		arg.remove(8);
		arg.remove(7);
		arg.remove(5);

		let _ = root
			.clone()
			.sub_command(sub.clone().action(|c, cur| {
				println!("c: {:?}", c);
				assert_eq!(
					c.raw_args,
					cnv_arg(vec![
						"exe_path",
						"--cstr=test",
						"-b",
						"sub",
						"--cafter",
						"--string=testStr",
						"--common"
					])
				);
				assert_eq!(c.args.len(), 0);
				assert_eq!(
					c.get_flag_value_of("cstr", &cur).unwrap(),
					FlagValue::String("test".into())
				);
				assert_eq!(
					c.get_flag_value_of("bool", &cur).unwrap(),
					FlagValue::Bool(true)
				);
				assert_eq!(
					c.get_flag_value_of("cafter", &cur).unwrap(),
					FlagValue::Bool(true)
				);
				assert_eq!(
					c.get_flag_value_of("string", &cur).unwrap(),
					FlagValue::String("testStr".into())
				);
				assert_eq!(
					c.get_flag_value_of("common", &cur).unwrap(),
					FlagValue::Bool(true)
				);
				assert_eq!(c.routes, Vector(Some(vec!["sub".to_string()])));
				assert_attrs!("sub_", c);
				done!()
			}))
			.run(arg.clone());
	}

	#[test]
	fn run_leaf() {
		let arg = vec![
			"exe_path".to_string(),
			"sub".to_string(),
			"leaf".to_string(),
			//"test".to_string(),
			"--common".to_string(),
			"test".to_string(),
			"-c".to_string(),
			"--local".to_string(),
		];
		let root = Command::new()
			.action(|c, _| {
				println!("test_action: {:?}", c);
				panic!("not sub");
			})
			.common_flag(Flag::new("common", FlagType::String, "sample common flag"))
			.common_flag(Flag::with_name("cshort").short_alias('c'))
			.local_flag(Flag::new("local", FlagType::default(), "sample local flag"))
			.sub_command(
				Command::with_name("sub")
					.action(|c, _| {
						panic!("sub: {:?}", c);
					})
					.version("sub_version")
					.copyright("sub_copyright")
					.license(license!(
						"sub_license".into(),
						content=>"root_license_content".into(),
					))
					.authors("sub_authors")
					.sub_command(
						Command::with_name("leaf")
							.action(|c, cur| {
								println!("{:?}", c);
								let raw_args = vec![
									"exe_path".to_string(),
									"sub".to_string(),
									"leaf".to_string(),
									//"test".to_string(),
									"--common".to_string(),
									"test".to_string(),
									"-c".to_string(),
									"--local".to_string(),
								];
								//let expect_args = VecDeque::from(vec!["test".to_string()]);
								let expect_args = VecDeque::new();
								assert_eq!(c.exe_path, String::from("exe_path"));
								assert_eq!(c.raw_args, raw_args);
								assert_eq!(c.args, expect_args);
								assert_eq!(
									c.get_flag_value_of("common", &cur),
									Some(FlagValue::String("test".into()))
								);
								assert_eq!(
									c.get_inputted_flag_value_of("cshort").unwrap(),
									FlagValue::None
								);
								assert_eq!(
									c.get_flag_value_of("cshort", &cur).unwrap(),
									FlagValue::String("".into())
								);
								assert_eq!(
									c.get_flag_value_of("local", &cur).unwrap(),
									FlagValue::Bool(true)
								);
								assert_eq!(c.get_common_flag_value_of("local"), None);
								assert_eq!(
									c.routes,
									Vector(Some(vec!["sub".to_string(), "leaf".to_string()]))
								);
								assert_attrs!("leaf_", c);
								done!()
							})
							.local_flag(Flag::new_bool("local").short_alias('l'))
							.version("leaf_version")
							.copyright("leaf_copyright")
							.license(license!(
								"leaf_license".into(),
								content=>"leaf_license_content".into(),
							))
							.authors("leaf_authors"),
					),
			);
		let _ = root.run(arg.clone());
	}

	#[test]
	fn run_leaf_with_flag_before_normal_flag() {
		let root = base_root().action(|c, _| {
			panic!("root action: {:?} - not leaf", c);
		});
		let sub = Command::with_name("sub")
			.local_flag(Flag::new_bool("sub_local"))
			.local_flag(Flag::new_string("sub_lstr"))
			.common_flag(Flag::new_bool("sub_common"))
			.local_flag(Flag::new_string("sub_cstr"))
			.action(|c, _| {
				panic!("sub action: {:?} - not leaf", c);
			});
		let leaf = Command::with_name("leaf")
			.common_flag(Flag::new_bool("cbool"))
			.common_flag(Flag::new_string("cs"))
			.local_flag(Flag::new_bool("lbool").short_alias('b'))
			.local_flag(Flag::new_string("lsafter").short_alias('a'))
			.local_flag(Flag::new_string("lsbefore").short_alias('s'))
			.authors("leaf_authors")
			.copyright("leaf_copyright")
			.version("leaf_version")
			.license(license!(
				"leaf_license".to_owned(),
				content=>"leaf_license_content".to_owned(),
			));

		let run_leaf: fn(Command, Command, Command, Action, Vec<String>) -> () =
			|root, sub, leaf, action, args| {
				let _ = root
					.sub_command(sub.sub_command(leaf.action(action)))
					.run(args);
			};

		let mut args = cnv_arg(vec!["exe_path", "--lbool", "sub", "--lsbefore", "leaf"]);

		run_leaf(
			root.clone().name("root"),
			sub.clone(),
			leaf.clone(),
			|c, cur| {
				println!("{:?}", c);
				assert_eq!(
					c.raw_args,
					cnv_arg(vec!["exe_path", "--lbool", "sub", "--lsbefore", "leaf"])
				);
				assert_eq!(c.args, VecDeque::new());
				assert_eq!(
					c.get_flag_value_of("lbool", &cur).unwrap(),
					FlagValue::Bool(true)
				);
				assert_eq!(
					c.get_flag_value_of("lsbefore", &cur).unwrap(),
					FlagValue::String("".into())
				);
				assert_eq!(
					c.routes,
					Vector(Some(vec![
						"root".to_string(),
						"sub".to_owned(),
						"leaf".to_owned()
					]))
				);
				assert_attrs!("leaf_", c);
				done!()
			},
			args.clone(),
		);

		args.push("arg".into());

		run_leaf(
			root.clone(),
			sub.clone(),
			leaf.clone(),
			|c, cur| {
				println!("{:?}", c);
				assert_eq!(
					c.raw_args,
					cnv_arg(vec![
						"exe_path",
						"--lbool",
						"sub",
						"--lsbefore",
						"leaf",
						"arg"
					])
				);
				assert_eq!(c.args, VecDeque::from(vec![String::from("arg")]));
				assert_eq!(
					c.get_flag_value_of("lbool", &cur).unwrap(),
					FlagValue::Bool(true)
				);
				assert_eq!(
					c.get_flag_value_of("lsbefore", &cur).unwrap(),
					FlagValue::String("".into())
				);
				assert_eq!(
					c.routes,
					Vector(Some(vec!["sub".to_owned(), "leaf".to_owned()]))
				);
				assert_attrs!("leaf_", c);
				done!()
			},
			args.clone(),
		);
		args.push("--cbool".into());
		run_leaf(
			root.clone(),
			sub.clone(),
			leaf.clone(),
			|c, cur| {
				println!("{:?}", c);
				assert_eq!(
					c.raw_args,
					cnv_arg(vec![
						"exe_path",
						"--lbool",
						"sub",
						"--lsbefore",
						"leaf",
						"arg",
						"--cbool"
					])
				);
				assert_eq!(
					c.get_flag_value_of("lbool", &cur).unwrap(),
					FlagValue::Bool(true)
				);
				assert_eq!(c.args, VecDeque::from(vec!["arg".to_string()]));
				assert_eq!(
					c.get_flag_value_of("lsbefore", &cur).unwrap(),
					FlagValue::String("".into())
				);
				assert_eq!(
					c.get_flag_value_of("cbool", &cur).unwrap(),
					FlagValue::Bool(true)
				);
				assert_eq!(
					c.routes,
					Vector(Some(vec!["sub".to_owned(), "leaf".to_owned()]))
				);
				assert_attrs!("leaf_", c);
				done!()
			},
			args.clone(),
		);
		args.pop();
		args.insert(4, "before_arg".into());

		run_leaf(
			root.clone(),
			sub.clone(),
			leaf.clone(),
			|c, cur| {
				println!("{:?}", c);
				assert_eq!(
					c.raw_args,
					cnv_arg(vec![
						"exe_path",
						"--lbool",
						"sub",
						"--lsbefore",
						"before_arg",
						"leaf",
						"arg"
					])
				);
				assert_eq!(c.args, VecDeque::from(vec!["arg".to_string()]));
				assert_eq!(c.args, VecDeque::from(vec![String::from("arg")]));
				assert_eq!(
					c.get_flag_value_of("lbool", &cur).unwrap(),
					FlagValue::Bool(true)
				);
				assert_eq!(
					c.get_flag_value_of("lsbefore", &cur).unwrap(),
					FlagValue::String("before_arg".into())
				);
				assert_eq!(
					c.routes,
					Vector(Some(vec!["sub".to_owned(), "leaf".to_owned()]))
				);
				assert_attrs!("leaf_", c);
				done!()
			},
			args.clone(),
		);
	}

	#[test]
	fn test_flag_type() {
		let arg = vec![
			"exe_path".to_string(),
			"sub".to_string(),
			"leaf".to_string(),
			"test".to_string(),
			"--common".to_string(),
			"test".to_string(),
			"-c".to_string(),
			"--local".to_string(),
			"-y".to_string(),
			"-i".to_string(),
			"111".to_string(),
			"--float".to_string(),
			"10.0".to_string(),
		];

		let leaf = Command::with_name("leaf")
			.action(|c, cur| {
				println!("sub_action: {:?}", c);
				let raw_args = vec![
					"exe_path".to_string(),
					"sub".to_string(),
					"leaf".to_string(),
					"test".to_string(),
					"--common".to_string(),
					"test".to_string(),
					"-c".to_string(),
					"--local".to_string(),
					"-y".to_string(),
					"-i".to_string(),
					"111".to_string(),
					"--float".to_string(),
					"10.0".to_string(),
				];
				let expect_args = VecDeque::from(vec!["test".to_string()]);
				assert_eq!(c.exe_path, String::from("exe_path"));
				assert_eq!(c.raw_args, raw_args);
				assert_eq!(c.args, expect_args);
				assert_eq!(
					c.get_flag_value_of("common", &cur),
					Some(FlagValue::String("test".into()))
				);
				assert_eq!(
					c.get_inputted_flag_value_of("commons"),
					Some(FlagValue::None)
				);
				assert_eq!(
					c.get_flag_value_of("commons", &cur),
					Some(FlagValue::String("".into()))
				);
				assert_eq!(c.get_flag_value_of("local", &cur), None);
				assert_eq!(c.get_inputted_common_flag_value_of("yes"), None);
				assert_eq!(
					c.get_local_flag_value_of("yes", &cur),
					Some(FlagValue::Bool(true))
				);
				assert_eq!(
					c.get_flag_value_of("yes", &cur),
					Some(FlagValue::Bool(true))
				);
				let expect_error_args = {
					let mut vd = VecDeque::new();
					vd.push_back(MiddleArg::LongFlag("local".into(), FlagValue::None));
					vd
				};
				assert_eq!(c.get_flag_value_of("int", &cur), Some(FlagValue::Int(111)));
				assert_eq!(
					c.get_flag_value_of("float", &cur),
					Some(FlagValue::Float(10.into()))
				);

				assert_eq!(
					c.error_info_list,
					Vector::from(vec![(
						MiddleArg::LongFlag("local".into(), FlagValue::None),
						ParseError::NoExistLong,
						ParseError::NoExistLong
					)])
				);
				assert_attrs!("sub_", c);
				assert_eq!(c.parsing_args.unwrap(), expect_error_args);
				done!()
			})
			.local_flag(Flag::new_bool("yes").short_alias('y'))
			.local_flag(Flag::new_int("int").short_alias('i'))
			.local_flag(Flag::new_float("float").short_alias('f'))
			.authors("sub_authors")
			.version("sub_version")
			.copyright("sub_copyright")
			.license(license!(
				"sub_license".into(),
				content=>"sub_license_content".into(),
			));
		let root = Command::new()
			.action(|c, _| {
				println!("test_action: {:?}", c);
				panic!("not sub");
			})
			.common_flag(Flag::new(
				"common",
				FlagType::default(),
				"sample common flag",
			))
			.common_flag(Flag::with_name("commons").short_alias('c'))
			.common_flag(Flag::new_string("yes").short_alias('y'))
			.local_flag(Flag::new("local", FlagType::default(), "sample local flag"))
			.sub_command(
				Command::with_name("sub")
					.action(|c, _| {
						println!("sub: {:?}", c);
						panic!("not leaf");
					})
					.sub_command(leaf),
			);
		let _ = root.run(arg.clone());
	}
}

/// Presets of Command
pub mod presets {

	use super::{Action, Command, Context, Flag, License, Vector};

	/// Preset of help function
	pub fn help<'a>(ctx: &Context, cmd: &Command) -> String {
		let mut help = String::new();
		let indent_size: usize = 3;
		let sp = String::from(" ");
		let indent: String = sp.repeat(indent_size);
		match &cmd.description {
			Some(description) => {
				help.push_str(description);
				help.push_str("\n\n");
			}
			_ => {}
		}
		help += &format!("Usage:\n{}{}\n\n", &indent, cmd.usage);

		//フラグ処理
		let l_flags: &Vector<Flag> = &cmd.l_flags;
		let c_flags: &Vector<Vector<Flag>>;
		let vec_inner: Vector<Vector<Flag>>;

		//コモンフラグが残っていた場合
		if cmd.c_flags.is_none() {
			c_flags = &ctx.common_flags;
		} else {
			vec_inner = Vector(Some(vec![cmd.c_flags.clone()]));
			c_flags = &vec_inner;
		}

		help.push_str("Flags(If exist flags have same alias and specified by user, inputted value will be interpreted as the former flag's value): \n");
		let head: String;
		let common_label;
		let name_and_alias_field_min_width: usize = 7;
		if ctx.common_flags.sum_of_length() > 0 {
			//コモンフラグが設定されている場合
			head = indent.repeat(2);
			common_label = true;
		} else {
			//設定されていない場合、ローカルフラグだけなのでラベルはいらない
			head = indent.clone();
			common_label = false;
		}

		if let Vector(Some(l_flags)) = l_flags {
			if !common_label {
				help.push_str(&indent);
				help.push_str("Local: \n");
			}
			help = l_flags.iter().fold(help, |help, l_flag| {
				l_flag.help(help + &head, name_and_alias_field_min_width + 5)
			});
		}
		let depth = c_flags.len();
		if let Vector(Some(c_flags)) = c_flags {
			if common_label {
				help = help + &indent + "Common";
			}
			let self_common_index = depth - 1;
			let route_without_root = depth > ctx.routes.len();
			let mut common_head = true;
			help = c_flags
				.iter()
				.enumerate()
				.rfold(help, |help, (index, c_flags)| {
					//コモンフラグ書き出し
					if let Vector(Some(c_flags)) = c_flags {
						let mut help = help;
						if common_label {
							if index < self_common_index {
								let mut from_owned: String;
								let from = if route_without_root {
									if index < 1 {
										let cur_path = std::path::Path::new(ctx.current());
										from_owned = cur_path
											.file_stem()
											.unwrap_or(std::ffi::OsStr::new("root"))
											.to_str()
											.unwrap_or("root")
											.to_owned();
										match cur_path.extension() {
											None => {}
											Some(val) => {
												from_owned += &format!("[.{}]", val.to_str().unwrap_or("exe"))
											}
										}

										&from_owned
									} else {
										ctx.routes.get(index - 1).unwrap()
									}
								} else {
									ctx.routes.get(index).unwrap()
								};
								if common_head {
									help += &format!("[inherited from {}]: \n", from)
								} else {
									common_head = false;
									help += &format!("{}[inherited from {}]: \n", &indent, from)
								}
							} else {
								help += &format!(
								"(common flags are available in this command and sub command{} under this): \n",
								{
									if cmd.sub.len() < 2 {
										""
									} else {
										"s"
									}
								}
								);
								common_head = false;
							}
						}

						help = c_flags.iter().fold(help, |help, c_flag| {
							c_flag.help(help + &head, name_and_alias_field_min_width)
						});
						help
					} else {
						help
					}
				});
			help += "\n";
		}

		if let Vector(Some(sub_commands)) = &cmd.sub {
			help += &format!(
				"Sub Command{}: \n",
				if sub_commands.len() < 2 { "" } else { "s" }
			);
			help = sub_commands.iter().fold(help, |help, sub_command| {
				//サブコマンドの説明出力
				let mut help = help + &indent + &sub_command.name;
				let mut name_and_alias_width = sub_command.name.len();
				if let Vector(Some(alias)) = &sub_command.alias {
					let (h_str, w) = alias
						.iter()
						.fold((help, name_and_alias_width), |(help, w), alias| {
							(help + ", " + alias, w + 2 + alias.len())
						});
					help = h_str;
					name_and_alias_width = w;
				}
				if name_and_alias_width < name_and_alias_field_min_width {
					help += &sp.repeat(name_and_alias_field_min_width - name_and_alias_width);
				}

				help = if let Some(description) = &sub_command.description {
					help + "\t" + description
				} else {
					help
				};
				help + "\n"
			});
			let loc_owned: String;
			let location: &str = {
				if cmd.name.is_empty() {
					let path = std::path::Path::new(ctx.current());
					let mut l: String = path
						.file_stem()
						.unwrap_or(std::ffi::OsStr::new("root"))
						.to_str()
						.unwrap_or("root")
						.to_owned();
					match path.extension() {
						None => {}
						Some(ext) => {
							l = l + "[." + ext.to_str().unwrap_or("exe") + "]";
						}
					}
					loc_owned = l;
					&loc_owned
				} else {
					//セルフネームがある
					if depth < 2 {
						//コモンフラグが1コマンド分しかない→現在はルートコマンド
						&cmd.name
					} else {
						loc_owned = if let Vector(Some(routes)) = &ctx.routes {
							routes.iter().rfold(
								{
									if depth > routes.len() {
										let path = std::path::Path::new(ctx.current());
										let mut l = path
											.file_stem()
											.unwrap_or(std::ffi::OsStr::new("root"))
											.to_str()
											.unwrap_or("root")
											.to_owned();
										match path.extension() {
											None => {}
											Some(val) => {
												l = l + "[." + val.to_str().unwrap_or("exe") + "]";
											}
										}
										l
									} else {
										String::new()
									}
								},
								|str, route| {
									//現在どのコマンドに対応しているか
									str + &sp + route
								},
							)
						} else {
							panic!("Routes of context should be not none under sub command.")
						};
						&loc_owned
					}
				}
			};
			help = help
				+ "\n" + &format!(
				//"See '{0} help <subcommand>' or '{0} <subcommand> --help' for more information.",
				"{0} <subcommand> --help for more information.",
				location
			);
			help += "\n";
		}

		return help;
	}

	/// Create usage preset
	pub fn usage<T: Into<String>>(name: T) -> String {
		format!("{} [SUBCOMMAND OR ARG] [OPTIONS]", name.into())
	}

	/// Create root command with base
	pub fn root_with_base<T: Into<String>>(
		name: T,
		authors: T,
		version: T,
		description: T,
		action: Option<Action>,
	) -> Command {
		Command::with_all_field(
			name.into(),
			action,
			authors.into(),
			String::default(),
			License(None),
			Some(description.into()),
			String::default(),
			Vector::default(),
			Vector::default(),
			Vector::default(),
			version.into(),
			Vector::default(),
		)
	}
}
