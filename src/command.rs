use crate::{
	action::{ActionError, ActionErrorKind::NoActionRegistered, ActionResult},
	done,
	parser::MiddleArg,
	Action, Context, Flag, FlagValue, Parser, Vector,
};

use core::mem::swap;
use std::{collections::VecDeque, fmt::Debug};

///The struct for command information store and command execution
///This can be root and edge
///コマンドの情報格納＆実行用構造体
#[derive(Clone, Default, Debug)]
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
	($command:expr,$context:expr) => {
		Err(ActionError::without_related_error(
			"no action is registered.".into(),
			NoActionRegistered,
			$command,
			$context,
		))
	};
}

macro_rules! check_sub {
	($sub:expr, $self:expr) => {
		check_sub_field!($sub, $self, authors);
		check_sub_field!($sub, $self, version);
		check_sub_field!($sub, $self, copyright);
		check_sub_field!($sub, $self, license: License);
	};
}

macro_rules! check_sub_field {
	($sub: expr, $self:expr, $field: ident) => {
		if $sub.$field.is_empty() {
			swap(&mut $sub.$field, &mut $self.$field)
		}
	};
	($sub:expr, $self:expr,$field:ident :Option,) => {
		if $sub.$field.is_none() {
			swap(&mut $sub.$field, &mut $self.$field)
		}
	};
	($sub:expr, $self:expr,$field:ident :License) => {
		if $sub.$field.is_none() {
			swap(&mut $sub.$field, &mut $self.$field)
		}
	};
}

/// LicenseFunc shows type alias for license function
pub type LicenseFunc = fn(command: &Command, context: &Context) -> String;

#[derive(Clone)]
/// License shows license information
pub struct License(pub Option<(String, LicenseFunc)>);

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
	pub fn new(inner: Option<(String, LicenseFunc)>) -> Self {
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
	pub fn output(&self, cmd: &Command, ctx: &Context) -> Option<String> {
		match self {
			License(Some((_, outputter))) => Some(outputter(cmd, ctx)),
			License(None) => None,
		}
	}
	/// Returns (long) expression - detail of license. If self is License(None), this function may panic.
	pub fn output_unwrap(&self, cmd: &Command, ctx: &Context) -> String {
		match self {
			License(Some((_, outputter))) => outputter(cmd, ctx),
			_ => panic!("called `License::expr_unwrap()` on a `None`value"),
		}
	}
	/// Returns function which outputs (long) expression (or detail of license) with wrapping Option.
	pub fn output_fn(&self) -> Option<LicenseFunc> {
		match self {
			License(Some((_, outputter))) => Some(*outputter),
			License(None) => None,
		}
	}
	/// Returns function of (long) expression (or detail of license). If self is License(None), this will panic.
	pub fn output_fn_unwrap(&self) -> LicenseFunc {
		match self {
			License(Some((_, outputter))) => *outputter,
			_ => panic!("called `License::output_fn_wrap` on a `None` value"),
		}
	}
	/// Returns unwrap inner
	pub fn unwrap(self) -> (String, LicenseFunc) {
		let License(inner) = self;
		inner.unwrap()
	}
}

macro_rules! gen_context_for_self_action {
	($raw_args:expr) => {{
		let mut args = VecDeque::from($raw_args.clone());
		let exe_path = args.pop_front().unwrap();
		gen_context_for_self_action!($raw_args, args, exe_path)
	}};
	($raw_args:expr,$args:expr,$exe_path:expr) => {
		Context::new($raw_args, $args, Vector(None), Vector(None), $exe_path)
	};
	($raw_args:expr,$args:expr,$exe_path:expr, $inter_mediate_args:expr) => {
		Context::with_all_field(
			$raw_args,
			$args,
			Vector(None),
			$exe_path,
			Vector(None),
			Vector::default(),
			Vector::default(),
			Some($inter_mediate_args),
			Vector(None),
		)
	};
}

macro_rules! gen_context_for_sub_run {
	($self:expr,$raw_args:expr,$args:expr,$exe_path:expr) => {
		gen_context_for_sub_run!(inner, $self, $raw_args, $args, $exe_path, None)
	};
	($self:expr,$raw_args:expr,$args:expr,$exe_path:expr, $inter_mediate_args: expr) => {
		gen_context_for_sub_run!(
			inner,
			$self,
			$raw_args,
			$args,
			$exe_path,
			Some($inter_mediate_args)
		)
	};
	(inner,$self:expr, $raw_args:expr,$args:expr,$exe_path:expr,$inter_mediate_args:expr) => {
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
					action(self, gen_context_for_self_action!(raw_args))
				} else {
					let mut context = gen_context_for_self_action!(raw_args);
					//println!("single_run_context: {:?}", context);
					context =
						Parser::default().parse_args_until_end(&self.l_flags, &self.c_flags, context);
					action(self, context)
				}
			}
			None => match self.sub {
				Vector(None) => {
					let c = gen_context_for_self_action!(raw_args);
					no_registered_error!(self, c)
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
	pub fn description<T: Into<String>>(mut self, description: T) -> Self {
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
			Vector(Some(ref mut inner)) => match inner.iter().position(|c| c.is(name_or_alias)) {
				None => None,
				Some(index) => Some(inner.swap_remove(index)),
			},
		}
	}

	/// Gets sub command mutable reference matches name_or_alias.
	pub fn get_mut_sub(&mut self, name_or_alias: &str) -> Option<&mut Command> {
		match self.sub {
			Vector(None) => None,
			Vector(Some(ref mut inner)) => match inner.iter().position(|c| c.is(name_or_alias)) {
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
			let c = gen_context_for_self_action!(raw_args, args, exe_path);
			match self.action {
				Some(action) => action(self, c),
				None => no_registered_error!(self, c),
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
							let mut c = gen_context_for_self_action!(raw_args, args, exe_path);
							match self.action {
								None => no_registered_error!(self, c),
								Some(action) => {
									c = p.parse_args_until_end(&self.l_flags, &self.c_flags, c);
									action(self, c)
								}
							}
						}
						Some(sub) => {
							// サブコマンドがヒットしたとき
							let c = gen_context_for_sub_run!(self, raw_args, args, exe_path);
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
			let p = Parser::default();
			let (mut context, non_flag_args) =
				p.parse_inter_mediate_args(&self.l_flags, &self.c_flags, context, true);
			if let Some(mut non_flag_args) = non_flag_args {
				non_flag_args.append(&mut context.args);
				context.args = non_flag_args;
			}
			context = p.parse_args_until_end(&self.l_flags, &self.c_flags, context);
			match self.action {
				Some(action) => action(self, context),
				None => no_registered_error!(self, context),
			}
		} else {
			//サブコマンドと一致するかを捜査
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
							check_sub!(sub, self);
							context.routes.push(self.name.clone());
							let r = sub.run(context);
							self.handle_sub_result(r)
						}
						None => match self.action {
							None => no_registered_error!(self, context),
							Some(action) => {
								let c = match p.parse_inter_mediate_args(
									&self.l_flags,
									&self.c_flags,
									context,
									true,
								) {
									(mut context, None) => {
										context =
											p.parse_args_until_end(&self.l_flags, &self.c_flags, context);
										context.args.push_front(arg);
										context
									}
									(mut context, Some(mut non_flag_args)) => {
										context =
											p.parse_args_until_end(&self.l_flags, &self.c_flags, context);
										context.args.push_front(arg);
										non_flag_args.append(&mut context.args);
										context.args = non_flag_args;
										context
									}
								};
								action(self, c)
							}
						},
					}
				}
				None => {
					//サブコマンド等の処理の必要がないのでこのまま叩き込む
					match self.action {
						Some(action) => {
							let (mut context, non_flag_args) =
								p.parse_inter_mediate_args(&self.l_flags, &self.c_flags, context, true);
							if let Some(mut non_flag_args) = non_flag_args {
								non_flag_args.append(&mut context.args);
								context.args = non_flag_args;
							}
							action(self, context)
						}
						None => {
							let (mut context, non_flag_args) =
								p.parse_inter_mediate_args(&self.l_flags, &self.c_flags, context, true);
							if let Some(mut non_flag_args) = non_flag_args {
								non_flag_args.append(&mut context.args);
								context.args = non_flag_args;
							}
							no_registered_error!(self, context)
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
					check_sub!(sub, self);
					c.routes.push(self.name.clone());
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
									c.routes.push(self.name.clone());
									check_sub!(sub, self);
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
									let (mut c, non_flag_args) =
										p.parse_inter_mediate_args(&self.l_flags, &self.c_flags, c, false);
									c = p.parse_args_until_end(&self.l_flags, &self.c_flags, c);
									c.args.push_front(arg);
									if let Some(mut non_flag_args) = non_flag_args {
										non_flag_args.append(&mut c.args);
										c.args = non_flag_args;
									}
									match self.action {
										Some(action) => action(self, c),
										None => no_registered_error!(self, c),
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
								let (mut c, args) =
									p.parse_inter_mediate_args(&self.l_flags, &self.c_flags, c, false);

								if let Some(mut args) = args {
									args.append(&mut c.args);
									c.args = args;
								}
								match self.action {
									Some(action) => action(self, c),
									None => no_registered_error!(self, c),
								}
							}
						}
					}
					_ => {
						inter_mediate_args.push_back(last);
						c.args = args;
						if let Some(mut parsing_args) = c.parsing_args {
							parsing_args.append(&mut inter_mediate_args);
							c.parsing_args = Some(parsing_args);
						}
						let (mut c, non_flag_args) =
							p.parse_inter_mediate_args(&self.l_flags, &self.c_flags, c, false);
						c = p.parse_args_until_end(&self.l_flags, &self.c_flags, c);
						c.args.push_front(arg);
						if let Some(mut non_flag_args) = non_flag_args {
							non_flag_args.append(&mut c.args);
							c.args = non_flag_args;
						}
						match self.action {
							Some(action) => action(self, c),
							None => no_registered_error!(self, c),
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
				let (mut c, non_flag_args) =
					p.parse_inter_mediate_args(&self.l_flags, &self.c_flags, c, false);
				//println!("after_parse_ima:{:?}", c);
				if let Some(non_flag_args) = non_flag_args {
					//non_flag_args.append(&mut c.args);
					c.args = non_flag_args;
				}
				match self.action {
					Some(action) => action(self, c),
					None => no_registered_error!(self, c),
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
					Some(sub) => {
						inter_mediate_args.push_back(last);
						let c =
							gen_context_for_sub_run!(self, raw_args, args, exe_path, inter_mediate_args);
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
										Some(sub) => {
											let c = gen_context_for_sub_run!(
												self,
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
												raw_args,
												args,
												exe_path,
												inter_mediate_args
											);
											let (mut c, non_flag_args) = p.parse_inter_mediate_args(
												&self.l_flags,
												&self.c_flags,
												c,
												false,
											);
											c = p.parse_args_until_end(&self.l_flags, &self.c_flags, c);
											c.args.push_front(arg);
											if let Some(mut non_flag_args) = non_flag_args {
												non_flag_args.append(&mut c.args);
												c.args = non_flag_args;
											};
											match self.action {
												Some(action) => action(self, c),
												None => no_registered_error!(self, c),
											}
										}
									},
									None => {
										//残りのargはなし、そのままaction
										let c = gen_context_for_self_action!(
											raw_args,
											args,
											exe_path,
											inter_mediate_args
										);
										let (mut c, non_flag_args) =
											p.parse_inter_mediate_args(&self.l_flags, &self.c_flags, c, false);
										if let Some(mut non_flag_args) = non_flag_args {
											non_flag_args.append(&mut c.args);
											c.args = non_flag_args;
										}
										match self.action {
											Some(action) => action(self, c),
											None => no_registered_error!(self, c),
										}
									}
								}
							}
							_ => {
								//argがフラグの可能性がない
								inter_mediate_args.push_back(last);
								//inter_mediate_args.push_back(MiddleArg::Normal(arg));

								let c = gen_context_for_self_action!(
									raw_args,
									args,
									exe_path,
									inter_mediate_args
								);
								let (mut c, non_flag_args) =
									p.parse_inter_mediate_args(&self.l_flags, &self.c_flags, c, false);
								c = p.parse_args_until_end(&self.l_flags, &self.c_flags, c);
								c.args.push_front(arg);
								if let Some(mut non_flag_args) = non_flag_args {
									non_flag_args.append(&mut c.args);
									c.args = non_flag_args;
								}
								match self.action {
									Some(action) => action(self, c),
									_ => no_registered_error!(self, c),
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
					gen_context_for_self_action!(raw_args, args, exe_path, inter_mediate_args);
				let (mut c, non_flag_args) =
					p.parse_inter_mediate_args(&self.l_flags, &self.c_flags, context, false);
				if let Some(mut non_flag_args) = non_flag_args {
					non_flag_args.append(&mut c.args);
					c.args = non_flag_args;
				}

				match self.action {
					Some(action) => action(self, c),
					None => no_registered_error!(self, c),
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
			Ok(ActionResult::ParentActionRequest(mut c, mut sub, action)) => {
				// サブコマンドからリクエストが飛んでいた時はselfを与えてリクエストされたアクションを実行
				c.routes.pop(); //ルートをさかのぼる
				self.c_flags = c.common_flags.remove_last(); //コモンフラグを戻す
				check_sub!(self, sub); // authors, version, copyright, licenseを戻す
				self.sub.push(sub); //サブコマンドを親コマンドの末尾に戻す
				action(self, c)
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
	fn test_single_run() {
		let mut arg = vec![
			"exe_path".to_string(),
			"test".to_string(),
			"test".to_string(),
		];
		let root = base_root().action(|_, c| {
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
			.action(|cmd, c| {
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
				assert_eq!(c.routes, Vector(None));
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
				assert_eq!(cmd.version, "root_version".to_owned());
				assert_eq!(cmd.copyright, "root_copyright".to_owned());
				assert_eq!(cmd.license.unwrap().0, String::from("root_license"));
				done!()
			})
			.common_flag(Flag::new("common", FlagType::String, "sample common flag"))
			.local_flag(Flag::new("local", FlagType::String, "sample local flag"))
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
			.action(|mut cmd, c| {
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
					c.get_local_flag_value_of("local", &cmd),
					Some(FlagValue::String("test".into()))
				);
				assert_eq!(
					c.get_flag_value_of("local", &cmd),
					Some(FlagValue::String("test".into()))
				);
				assert_eq!(
					c.get_flag_value_of("common", &cmd),
					Some(FlagValue::String(String::default()))
				);
				assert_eq!(c.routes, Vector(None));
				assert_eq!(cmd.version, "root_version".to_owned());
				assert_eq!(cmd.copyright, "root_copyright".to_owned());
				let License(inner) = cmd.license.take();
				let inner = inner.unwrap();
				assert_eq!(inner.0, String::from("root_license"));
				assert_eq!(inner.1(&cmd, &c), String::from("root_license_content"));
				done!()
			})
			.common_flag(Flag::new("common", FlagType::String, "sample common flag"))
			.local_flag(Flag::new("local", FlagType::String, "sample local flag"))
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
		($prefix:expr, $context:expr,$command:expr) => {
			let prefix = String::from($prefix);
			assert_eq!($command.authors, prefix.clone() + "authors");
			assert_eq!($command.version, prefix.clone() + "version");
			assert_eq!($command.copyright, prefix.clone() + "copyright");
			assert_eq!(
				(
					$command.license.expr_unwrap(),
					$command.license.output_unwrap(&$command, &$context)
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
			.action(|cmd, c| {
				println!("c: {:?}", c);
				assert_eq!(
					cnv_arg(vec!["exe_path", "--local=test", "test"]),
					c.raw_args
				);
				assert_eq!(
					c.get_flag_value_of("local", &cmd).unwrap(),
					FlagValue::String("test".into())
				);
				assert_eq!(c.routes, Vector(None));
				assert_attrs!("root_", c, cmd);
				done!()
			})
			.run(arg.clone());
		arg[2] = "--common".into();
		let _ = root
			.clone()
			.action(|cur_cmd, c| {
				println!("c: {:?}", c);
				assert_eq!(
					cnv_arg(vec!["exe_path", "--local=test", "--common"]),
					c.raw_args
				);
				assert_eq!(
					c.get_flag_value_of("local", &cur_cmd).unwrap(),
					FlagValue::String("test".into())
				);
				assert_eq!(
					c.get_flag_value_of("common", &cur_cmd).unwrap(),
					FlagValue::Bool(true)
				);
				assert_eq!(c.routes, Vector(None));
				assert_attrs!("root_", c, cur_cmd);
				done!()
			})
			.run(arg.clone());

		arg.push("test".into());
		let _ = root
			.clone()
			.action(|cc, c| {
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
				assert_attrs!("root_", c, cc);
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
			.action(|cur, c| {
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
				assert_attrs!("root_", c, cur);
				done!()
			})
			.run(arg.clone());

		arg.remove(5);
		arg.remove(4);
		arg.insert(5, "arg".into());

		let _ = root
			.clone()
			.action(|cur_cmd, c| {
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
					c.get_flag_value_of("local", &cur_cmd).unwrap(),
					FlagValue::String("test".into())
				);
				assert_eq!(
					c.get_flag_value_of("common", &cur_cmd).unwrap(),
					FlagValue::Bool(true)
				);
				assert_eq!(
					c.get_flag_value_of("cafter", &cur_cmd).unwrap(),
					FlagValue::Bool(true)
				);
				assert_eq!(
					c.get_flag_value_of("lafter", &cur_cmd).unwrap(),
					FlagValue::Bool(true)
				);
				assert_eq!(c.routes, Vector(None));
				assert_attrs!("root_", c, cur_cmd);
				done!()
			})
			.run(arg.clone());

		arg.push("ex_arg".into());
		let _ = root
			.clone()
			.action(|cmd, c| {
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
					c.get_flag_value_of("local", &cmd).unwrap(),
					FlagValue::String("test".into())
				);
				assert_eq!(
					c.get_flag_value_of("common", &cmd).unwrap(),
					FlagValue::Bool(true)
				);
				assert_eq!(
					c.get_flag_value_of("cafter", &cmd).unwrap(),
					FlagValue::Bool(true)
				);
				assert_eq!(
					c.get_flag_value_of("lafter", &cmd).unwrap(),
					FlagValue::Bool(true)
				);
				assert_eq!(c.routes, Vector(None));
				assert_attrs!("root_", c, cmd);
				done!()
			})
			.run(arg.clone());
		arg[4] = "-a".into();
		let _ = root
			.clone()
			.action(|cmd, c| {
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
					c.get_flag_value_of("local", &cmd).unwrap(),
					FlagValue::String("test".into())
				);
				assert_eq!(
					c.get_flag_value_of("common", &cmd).unwrap(),
					FlagValue::Bool(true)
				);
				assert_eq!(
					c.get_flag_value_of("cafter", &cmd).unwrap(),
					FlagValue::Bool(true)
				);
				assert_eq!(
					c.get_flag_value_of("lafter", &cmd).unwrap(),
					FlagValue::Bool(true)
				);
				assert_eq!(c.routes, Vector(None));
				assert_attrs!("root_", c, cmd);
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
			.sub_command(sub.clone().action(|cmd, c| {
				println!("{:?}", c);
				let raw_args = cnv_arg(vec![
					"exe_path", "sub", "test", "--common", "test", "--cstr", "strt", "-b", "--local",
				]);
				let expect_args = VecDeque::from(vec!["test".to_string(), "test".to_string()]);
				assert_eq!(c.exe_path, String::from("exe_path"));
				assert_eq!(c.raw_args, raw_args);
				assert_eq!(c.args, expect_args);
				assert_eq!(
					c.get_flag_value_of("common", &cmd),
					Some(FlagValue::Bool(true))
				);
				assert_eq!(
					c.get_flag_value_of("bool", &cmd).unwrap(),
					FlagValue::Bool(true)
				);
				assert_eq!(c.get_flag_value_of("commons", &cmd), None);
				assert_eq!(c.get_flag_value_of("local", &cmd), None);
				assert_eq!(c.routes, Vector(None));
				assert_attrs!("sub_", c, cmd);
				done!()
			}))
			.run(arg.clone());

		println!("サブコマンド前フラグのテスト");
		arg = cnv_arg(vec!["exe_path", "--cstr=test", "-b", "sub"]);
		let _ = root
			.clone()
			.name("root")
			.sub_command(sub.clone().action(|cmd, c| {
				println!("c: {:?}", c);
				assert_eq!(
					c.raw_args,
					cnv_arg(vec!["exe_path", "--cstr=test", "-b", "sub"])
				);
				assert_eq!(
					c.get_flag_value_of("cstr", &cmd).unwrap(),
					FlagValue::String("test".into())
				);
				assert_eq!(
					c.get_flag_value_of("bool", &cmd).unwrap(),
					FlagValue::Bool(true)
				);
				assert_eq!(c.routes, Vector(Some(vec!["root".to_string()])));
				assert_attrs!("sub_", c, cmd);
				done!()
			}))
			.run(arg.clone());

		println!("サブコマンド探しをする場合");
		arg[1] = "--cstr".into();
		arg.insert(2, "test".into());

		let _ = root
			.clone()
			.sub_command(sub.clone().action(|cmd, c| {
				println!("c:{:?}", c);
				assert_eq!(
					c.raw_args,
					cnv_arg(vec!["exe_path", "--cstr", "test", "-b", "sub"])
				);
				assert_eq!(
					c.get_flag_value_of("cstr", &cmd).unwrap(),
					FlagValue::String("test".into())
				);
				assert_eq!(
					c.get_flag_value_of("bool", &cmd).unwrap(),
					FlagValue::Bool(true)
				);
				assert_eq!(c.routes, Vector(None));
				assert_attrs!("sub_", c, cmd);
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
			.sub_command(sub.clone().action(|cmd, c| {
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
					c.get_flag_value_of("cstr", &cmd).unwrap(),
					FlagValue::String("test".into())
				);
				assert_eq!(
					c.get_flag_value_of("bool", &cmd).unwrap(),
					FlagValue::Bool(true)
				);
				assert_eq!(
					c.get_flag_value_of("cafter", &cmd).unwrap(),
					FlagValue::Bool(true)
				);
				assert_eq!(c.get_local_flag_value_of("cafter", &cmd), None);
				assert_eq!(
					c.get_flag_value_of("string", &cmd).unwrap(),
					FlagValue::String("testStr".into())
				);
				assert_eq!(c.routes, Vector(None));
				assert_attrs!("sub_", c, cmd);
				done!()
			}))
			.run(arg.clone());

		println!("\r\n\r\nサブサブコマンドが存在する場合の判別系\r\n");
		arg.remove(4);
		let _ = root
			.clone()
			.sub_command(sub.clone().action(|cmd, c| {
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
					c.get_common_flag_value_of("cstr", &cmd).unwrap(),
					FlagValue::String("test".into())
				);
				assert_eq!(
					c.get_local_flag_value_of("string", &cmd).unwrap(),
					FlagValue::String("testStr".into())
				);
				assert_eq!(
					c.get_inputted_common_flag_value_of("cafter").unwrap(),
					FlagValue::None
				);
				assert_eq!(c.routes, Vector(None));
				assert_attrs!("sub_", c, cmd);
				done!()
			}))
			.run(arg.clone());
		println!("\r\n\r\nサブサブコマンドが存在する場合の判別系その2\r\n");
		arg.push("ex_arg".into());
		arg[5] = "test_arg".to_owned();

		let _ = root
			.clone()
			.sub_command(sub.clone().action(|cmd, c| {
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
				assert_eq!(c.routes, Vector(None));
				assert_attrs!("sub_", c, cmd);
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
			.sub_command(sub.clone().action(|cmd, c| {
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
					c.get_common_flag_value_of("cstr", &cmd).unwrap(),
					FlagValue::String("test".into())
				);
				assert_eq!(
					c.get_local_flag_value_of("bool", &cmd).unwrap(),
					FlagValue::Bool(true)
				);
				assert_eq!(
					c.get_common_flag_value_of("cafter", &cmd).unwrap(),
					FlagValue::Bool(true)
				);
				assert_eq!(
					c.get_flag_value_of("string", &cmd).unwrap(),
					FlagValue::String("testStr".into())
				);
				assert_eq!(c.routes, Vector(None));
				assert_attrs!("sub_", c, cmd);
				done!()
			}))
			.run(arg.clone());

		arg.pop();
		arg.remove(8);
		arg.remove(7);
		arg.remove(5);

		let _ = root
			.clone()
			.sub_command(sub.clone().action(|cmd, c| {
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
					c.get_flag_value_of("cstr", &cmd).unwrap(),
					FlagValue::String("test".into())
				);
				assert_eq!(
					c.get_flag_value_of("bool", &cmd).unwrap(),
					FlagValue::Bool(true)
				);
				assert_eq!(
					c.get_flag_value_of("cafter", &cmd).unwrap(),
					FlagValue::Bool(true)
				);
				assert_eq!(
					c.get_flag_value_of("string", &cmd).unwrap(),
					FlagValue::String("testStr".into())
				);
				assert_eq!(
					c.get_flag_value_of("common", &cmd).unwrap(),
					FlagValue::Bool(true)
				);
				assert_eq!(c.routes, Vector(None));
				assert_attrs!("sub_", c, cmd);
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
			.common_flag(Flag::new_string("cshort").short_alias('c'))
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
							.action(|cmd, c| {
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
									c.get_flag_value_of("common", &cmd),
									Some(FlagValue::String("test".into()))
								);
								assert_eq!(
									c.get_inputted_flag_value_of("cshort").unwrap(),
									FlagValue::None
								);
								assert_eq!(
									c.get_flag_value_of("cshort", &cmd).unwrap(),
									FlagValue::String("".into())
								);
								assert_eq!(
									c.get_flag_value_of("local", &cmd).unwrap(),
									FlagValue::Bool(true)
								);
								assert_eq!(c.get_common_flag_value_of("local", &cmd), None);
								assert_eq!(c.routes, Vector(Some(vec!["sub".to_string()])));
								assert_attrs!("leaf_", c, cmd);
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
			.common_flag(Flag::new_bool("cbool").short_alias('o'))
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
			|cmd, c| {
				println!("{:?}", c);
				assert_eq!(
					c.raw_args,
					cnv_arg(vec!["exe_path", "--lbool", "sub", "--lsbefore", "leaf"])
				);
				assert_eq!(c.args, VecDeque::new());
				assert_eq!(
					c.get_flag_value_of("lbool", &cmd).unwrap(),
					FlagValue::Bool(true)
				);
				assert_eq!(
					c.get_flag_value_of("lsbefore", &cmd).unwrap(),
					FlagValue::String("".into())
				);
				assert_eq!(
					c.routes,
					Vector(Some(vec!["root".to_string(), "sub".to_owned(),]))
				);
				assert_attrs!("leaf_", c, cmd);
				done!()
			},
			args.clone(),
		);

		args.push("arg".into());

		run_leaf(
			root.clone(),
			sub.clone(),
			leaf.clone(),
			|cmd, c| {
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
					c.get_flag_value_of("lbool", &cmd).unwrap(),
					FlagValue::Bool(true)
				);
				assert_eq!(
					c.get_flag_value_of("lsbefore", &cmd).unwrap(),
					FlagValue::String("".into())
				);
				assert_eq!(c.routes, Vector(Some(vec!["sub".to_owned()])));
				assert_attrs!("leaf_", c, cmd);
				done!()
			},
			args.clone(),
		);
		args.push("-o".into());
		run_leaf(
			root.clone(),
			sub.clone(),
			leaf.clone(),
			|cmd, c| {
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
						"-o"
					])
				);
				assert_eq!(
					c.get_flag_value_of("lbool", &cmd).unwrap(),
					FlagValue::Bool(true)
				);
				assert_eq!(c.args, VecDeque::from(vec!["arg".to_string()]));
				assert_eq!(
					c.get_flag_value_of("lsbefore", &cmd).unwrap(),
					FlagValue::String("".into())
				);
				assert_eq!(
					c.get_flag_value_of("cbool", &cmd).unwrap(),
					FlagValue::Bool(true)
				);
				assert_eq!(c.routes, Vector(Some(vec!["sub".to_owned()])));
				assert_attrs!("leaf_", c, cmd);
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
			|cmd, c| {
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
					c.get_flag_value_of("lbool", &cmd).unwrap(),
					FlagValue::Bool(true)
				);
				assert_eq!(
					c.get_flag_value_of("lsbefore", &cmd).unwrap(),
					FlagValue::String("before_arg".into())
				);
				assert_eq!(c.routes, Vector(Some(vec!["sub".to_owned()])));
				assert_attrs!("leaf_", c, cmd);
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
			.action(|cmd, c| {
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
				let expect_args = VecDeque::from(vec!["test".to_string(), "test".to_owned()]);
				assert_eq!(c.exe_path, String::from("exe_path"));
				assert_eq!(c.raw_args, raw_args);
				assert_eq!(c.args, expect_args);
				assert_eq!(
					c.get_flag_value_of("common", &cmd),
					Some(FlagValue::Bool(true))
				);
				assert_eq!(
					c.get_inputted_flag_value_of("commons"),
					Some(FlagValue::None)
				);
				assert_eq!(
					c.get_flag_value_of("commons", &cmd),
					Some(FlagValue::String("".into()))
				);
				assert_eq!(c.get_flag_value_of("local", &cmd), None);
				assert_eq!(c.get_inputted_common_flag_value_of("yes"), None);
				assert_eq!(
					c.get_local_flag_value_of("yes", &cmd),
					Some(FlagValue::Bool(true))
				);
				assert_eq!(
					c.get_flag_value_of("yes", &cmd),
					Some(FlagValue::Bool(true))
				);
				let expect_error_args = {
					let mut vd = VecDeque::new();
					vd.push_back(MiddleArg::LongFlag("local".into(), FlagValue::None));
					vd
				};
				assert_eq!(c.get_flag_value_of("int", &cmd), Some(FlagValue::Int(111)));
				assert_eq!(
					c.get_flag_value_of("float", &cmd),
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
				assert_attrs!("sub_", c, cmd);
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
			.common_flag(Flag::new_string("commons").short_alias('c'))
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

	use crate::default_usage;
	use crate::Vector;

	use super::{Action, Command, License};

	/// Create usage preset
	pub fn usage<T: Into<String>>(name: T) -> String {
		default_usage!(name as var)
	}

	/// Create root command with base
	pub fn root_with_base<T: Into<String>>(
		name: T,
		authors: T,
		version: T,
		description: T,
		action: Option<Action>,
	) -> Command {
		let name = name.into();
		let usage = default_usage!(nameString=>name.clone());
		Command::with_all_field(
			name,
			action,
			authors.into(),
			String::default(),
			License(None),
			Some(description.into()),
			usage,
			Vector::default(),
			Vector::default(),
			Vector::default(),
			version.into(),
			Vector::default(),
		)
	}

	/// Create version command preset
	pub fn preset_version() -> Command {
		Command::with_all_field(
			String::from("version"),
			Some(func::version_print),
			String::default(),
			String::default(),
			License::default(),
			Some(String::from("show version")),
			String::default(),
			Vector::default(),
			Vector::default(),
			Vector::default(),
			String::default(),
			Vector::default(),
		)
	}

	/// function presets for command construction.
	pub mod func {
		use crate::{action_result, FlagType};

		use super::super::{Command, Context, Flag, Vector};
		use std::cmp::max;

		/// Preset of version command action
		pub fn version_print(cmd: Command, ctx: Context) -> action_result!() {
			crate::check_help!(ctx, cmd, help_tablize_with_alias_dedup);
			println!("{}", cmd.version);
			crate::done!()
		}

		macro_rules! _add_help_with_flag_dudup {
			($help:ident,$iter:expr,$nl_list:ident,$s_list:ident,$suffix:ident,$name_and_alias_min_width:ident,$sp:ident,$indent:ident) => {
				for f in $iter {
					let mut all_dup = true;
					let first_help_width = $help.len();
					if let Vector(Some(short_alias)) = &f.short_alias {
						for s in short_alias.iter() {
							if !$s_list.contains(&s) {
								if (!all_dup) {
									$help.push(',');
								}
								all_dup = false;
								$help.push_str("-");
								$help.push(*s);
								$s_list.push(s);
							}
						}
					} else {
						$help.push_str(&$indent);
					}
					if !$nl_list.contains(&&f.name) {
						$help.push_str(if all_dup { " --" } else { ", --" });
						all_dup = false;
						$help.push_str(&f.name);
						$nl_list.push(&f.name);
					}
					if let Vector(Some(long_alias)) = &f.long_alias {
						for long in long_alias.iter() {
							if !$nl_list.contains(&long) {
								$nl_list.push(long);
								if all_dup {
									$help.push_str(" --");
									all_dup = false;
								} else {
									$help.push_str(", --");
								}
								$help.push_str(long);
							}
						}
					}
					if all_dup {
						$help.truncate(first_help_width);
					} else {
						let name_and_alias_width = $help.len() - first_help_width;
						if name_and_alias_width < $name_and_alias_min_width {
							$help.push_str(
								$sp.repeat($name_and_alias_min_width - name_and_alias_width)
									.as_str(),
							);
						}
						$help.push('\t');
						$help.push_str(&f.description);
						$help.push_str(&$suffix);
						$help.push('\n');
					}
				}
			};
		}

		// Add help for this flag to append_to. name_and_alias_min_width means min width of name and alias' field.
		// Flagに対するヘルプをappend_toに追加する。nameとalias表示部分のずれをある程度吸収できるようにその部分の最小幅をname_and_alias_min_widthで指定する
		fn flag_help_simple(
			flag: &Flag,
			append_to: String,
			name_and_alias_min_width: usize,
		) -> String {
			let mut help = append_to;
			let first_help_width = help.len();

			if let Vector(Some(short_alias)) = &flag.short_alias {
				help = short_alias
					.iter()
					.fold(help, |help, s| format!("{}-{},", help, s));
			} else {
				help += "   ";
			}
			help = help + " --" + &flag.name;
			if let Vector(Some(long_alias)) = &flag.long_alias {
				help = long_alias.iter().fold(help, |help, l| {
					//ロングフラグ出力
					format!("{}, --{}", help, l)
				})
			};
			help = add_type_suffix(help, &flag.flag_type);
			let name_and_alias_width = help.len() - first_help_width;

			if name_and_alias_width < name_and_alias_min_width {
				help += &" ".repeat(name_and_alias_min_width - name_and_alias_width);
			}

			help + "\t" + &flag.description + "\n"
		}
		/// Preset of help function(compact version)
		pub fn help_with_alias_dedup(cmd: &Command, ctx: &Context) -> String {
			let mut help = String::new();
			let indent_size: usize = 3;
			let sp = String::from(" ");
			let indent = sp.repeat(indent_size);
			if let Some(description) = &cmd.description {
				help.push_str(description);
				help.push_str("\n\n");
			}
			help += &format!("Usage:\n{}{}\n", &indent, cmd.usage);
			let name_and_alias_min_width = 12;
			let routes = ctx.routes.clone();
			let mut routes: Vec<String> = if routes.len() < ctx.depth() {
				let mut routes: Vec<String> = routes.into();
				routes.insert(
					0,
					std::path::Path::new(ctx.exe_path())
						.file_stem()
						.unwrap_or(std::ffi::OsStr::new("root"))
						.to_str()
						.unwrap_or("root")
						.to_owned(),
				);
				routes
			} else {
				routes.into()
			};
			if cmd.l_flags.has_at_least_one()
				|| cmd.c_flags.has_at_least_one()
				|| ctx.common_flags.has_at_least_one()
			{
				help.push_str("\nFlags: \n");

				let mut nl_list = Vec::<&String>::new();
				let mut s_list = Vec::<&char>::new();

				if let Vector(Some(l_flags)) = &cmd.l_flags {
					let mut i = l_flags.iter().rev();
					if let Some(f) = i.next() {
						// ローカルフラグ出力
						help = flag_help_simple(f, help, name_and_alias_min_width);
						nl_list.push(&f.name);
						if let Vector(Some(la)) = &f.long_alias {
							let mut la = la.iter().collect();
							nl_list.append(&mut la);
						};
						if let Vector(Some(sa)) = &f.short_alias {
							let mut sa = sa.iter().collect();
							s_list.append(&mut sa)
						}
						let emp_str = String::new();
						_add_help_with_flag_dudup!(
							help,
							i,
							nl_list,
							s_list,
							emp_str,
							name_and_alias_min_width,
							sp,
							indent
						)
					}
				}

				// コモンフラグ出力
				// まず現在のコマンドのコモンフラグ出力
				if let Vector(Some(c_flags)) = &cmd.c_flags {
					let suffix = if cmd.sub.len() > 0 {
						format!(
							"[also available in sub command{} under here]",
							(if cmd.sub.len() < 2 { "" } else { "s" })
						)
					} else {
						String::new()
					};
					_add_help_with_flag_dudup!(
						help,
						c_flags.iter().rev(),
						nl_list,
						s_list,
						suffix,
						name_and_alias_min_width,
						sp,
						indent
					);
				}

				// コモンフラグ出力(contextに取り込まれているフラグ)
				if let Vector(Some(cfs)) = &ctx.common_flags {
					for (c_index, c_flags) in cfs.iter().enumerate().rev() {
						if let Vector(Some(c_flags)) = c_flags {
							let suffix = match routes.get(c_index) {
								Some(cmd_name) => sp.clone() + "(inherited from " + cmd_name + ")",
								None => String::new(),
							};

							_add_help_with_flag_dudup!(
								help,
								c_flags.iter().rev(),
								nl_list,
								s_list,
								suffix,
								name_and_alias_min_width,
								sp,
								indent
							);
						}
					}
				}
			}

			// サブコマンド出力
			if let Vector(Some(sub)) = &cmd.sub {
				let mut iter = sub.iter();
				if let Some(sub_command) = iter.next() {
					help.push_str("Sub Command");
					if sub.len() > 1 {
						help.push('s');
					}
					help.push_str(": \n");
					// 最初のフラグ情報追加
					help = help + &indent + &sub_command.name;
					let mut na_list = vec![&sub_command.name];
					let mut name_and_alias_width = sub_command.name.len();
					if let Vector(Some(alias)) = &sub_command.alias {
						let mut a = alias.iter().collect();
						na_list.append(&mut a);
						for a in alias {
							help = help + ", " + a;
							name_and_alias_width += a.len() + 2;
						}
					}
					if name_and_alias_width < name_and_alias_min_width {
						help = help + &sp.repeat(name_and_alias_min_width - name_and_alias_width);
					}
					if let Some(description) = &sub_command.description {
						help = help + "\t" + description
					}
					help += "\n";

					for sub_cmd in iter {
						let mut all_dup = true;
						let help_first_width = help.len();
						if !na_list.contains(&&sub_cmd.name) {
							na_list.push(&sub_cmd.name);
							help = help + &indent + &sub_cmd.name;
							all_dup = false;
						}
						if let Vector(Some(alias)) = &sub_cmd.alias {
							for a in alias {
								if !na_list.contains(&a) {
									na_list.push(a);
									if all_dup {
										help = help + &indent + a;
									} else {
										help = help + ", " + a;
									}
									all_dup = false;
								}
							}
						}
						if !all_dup {
							let name_and_alias_width = help_first_width - help.len();
							if name_and_alias_width < name_and_alias_min_width {
								help += &sp.repeat(name_and_alias_min_width - name_and_alias_width);
							}
							if let Some(description) = &sub_cmd.description {
								help = help + "\t" + description;
							}
							help += "\n"
						}
					}
				}
				if routes.len() < 2 && !cmd.name.is_empty() {
					routes[0] = cmd.name.clone();
				}
				let exe_suffix = std::env::consts::EXE_SUFFIX;
				if !exe_suffix.is_empty() {
					routes[0].push_str("[");
					routes[0].push_str(exe_suffix);
					routes[0].push_str("]")
				}
				help = help + &routes.join(" ") + "<subcommand> --help for more information.";
				help += "\n";
			}
			return help;
		}

		/// Add type suffix for flag help
		pub fn add_type_suffix(to: String, ft: &FlagType) -> String {
			match &ft {
				FlagType::Bool => to,
				FlagType::String => to + " <string>",
				FlagType::Int => to + " <int>",
				FlagType::Float => to + " <float>",
			}
		}

		/// Preset of help function
		pub fn help(cmd: &Command, ctx: &Context) -> String {
			let mut help = String::new();
			let indent_size: usize = 3;
			let sp = String::from(" ");
			let indent: String = sp.repeat(indent_size);
			if let Some(description) = &cmd.description {
				help.push_str(description);
				help.push_str("\n\n");
			}
			help += &format!("Usage:\n{}{}\n\n", &indent, cmd.usage);

			//フラグ処理
			let l_flags: &Vector<Flag> = &cmd.l_flags;
			let ctx_c_flags: &Vector<Vector<Flag>> = &ctx.common_flags;

			help.push_str("Flags(If exist flags have same alias and specified by user, inputted value will be interpreted as the former flag's value): \n");
			let head: String;
			let cl_label;
			let name_and_alias_field_min_width: usize = 7;
			if (ctx_c_flags.sum_of_length() + cmd.c_flags.len()) > 0 && cmd.l_flags.has_at_least_one()
			{
				//コモンフラグとローカルフラグ両方が設定されている場合
				head = indent.repeat(2);
				cl_label = true;
			} else {
				//設定されていない場合、ローカルフラグもしくはコモンフラグだけなのでラベルはいらない
				head = indent.clone();
				cl_label = false;
			}

			if let Vector(Some(l_flags)) = l_flags {
				if cl_label {
					help.push_str(&indent);
					help.push_str("[Local]: \n");
				}
				help = l_flags.iter().rfold(help, |help, l_flag| {
					flag_help_simple(l_flag, help + &head, name_and_alias_field_min_width + 10)
				});
			}
			let depth = ctx.depth();
			let mut common_head = true;
			if let Vector(Some(c_flags)) = &cmd.c_flags {
				if cl_label {
					help = help + &indent + "[Common" + &format!("(common flags are available in this command and sub command{} under this command)]: \n", (if cmd.sub.len()<2{""}else{"s"}));
				}

				for cf in c_flags {
					help = flag_help_simple(cf, help + &head, name_and_alias_field_min_width)
				}

				common_head = false;
			}
			if let Vector(Some(c_flags)) = ctx_c_flags {
				let route_without_root = depth > ctx.routes.len();
				if cl_label && common_head {
					help = help + &indent + "Common ";
				}
				help = c_flags
					.iter()
					.enumerate()
					.rfold(help, |help, (index, c_flags)| -> String {
						//コモンフラグ書き出し
						if let Vector(Some(c_flags)) = c_flags {
							let mut help = help;
							if cl_label {
								let mut from_owned: String;
								let from = if route_without_root {
									if index < 1 {
										let cur_path = std::path::Path::new(ctx.exe_path());
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
							}

							help = c_flags.iter().rfold(help, |help, c_flag| -> String {
								flag_help_simple(c_flag, help + &head, name_and_alias_field_min_width)
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
						let path = std::path::Path::new(ctx.exe_path());
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
						if depth < 1 {
							//コモンフラグが1コマンド分しかない→現在はルートコマンド
							&cmd.name
						} else {
							loc_owned = if let Vector(Some(routes)) = &ctx.routes {
								routes.iter().rfold(
									{
										if depth > routes.len() {
											let path = std::path::Path::new(ctx.exe_path());
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
				help =
					help + "\n" + &format!("{0} <subcommand> --help for more information.", location);
				help += "\n";
			}

			return help;
		}

		/// Preset of flag help function (tablize)
		pub fn flag_help_tablize(
			append_to: String,
			f: &Flag,
			sp: &String,
			s_max_num: usize, //最大ショートエイリアス数
			nl_width: usize,
			pre_d_space: &String,
		) -> String {
			let mut help = append_to;
			// short_alias出力
			help = help + &sp.repeat((s_max_num - f.short_alias.len()) * 4);
			if let Vector(Some(short_alias)) = &f.short_alias {
				for s in short_alias {
					help.push_str("-");
					help.push(*s);
					help.push_str(", ");
				}
			}
			let prev_help_len = help.len();
			help.push_str("--");
			help.push_str(&f.name);
			if let Vector(Some(long_alias)) = &f.long_alias {
				for l in long_alias {
					help.push_str(", --");
					help.push_str(l);
				}
			}
			help = add_type_suffix(help, &f.flag_type);
			let _nl_width = help.len() - prev_help_len;
			if _nl_width < nl_width {
				help.push_str(&sp.repeat(nl_width - _nl_width));
			}
			help.push_str(&pre_d_space);
			help.push_str(&f.description);
			help.push('\n');

			help
		}

		/// Preset of help function (tablize)
		pub fn help_tablize(cmd: &Command, ctx: &Context) -> String {
			let mut help = String::new();
			let indent_size: usize = 3;
			let sp = String::from(" ");
			let indent: String = sp.repeat(indent_size);
			if let Some(description) = &cmd.description {
				help.push_str(description);
				help.push_str("\n\n");
			}
			help = help + "Usage:\n" + &indent + &cmd.usage + "\n\n";

			if &cmd.l_flags.len() + &cmd.c_flags.len() + &ctx.common_flags.sum_of_length() > 0 {
				// フラグが存在するとき
				help.push_str("Flags(If exist flags have same alias and specified by user, inputted value will be interpreted as the former flag's value): \n");

				let nl_width = |flag: &Flag| match &flag.long_alias {
					Vector(None) => flag.name.len() + flag_type_suffix_len(&flag.flag_type),
					Vector(Some(long_aliases)) => {
						long_aliases.iter().fold(
							flag.name.len() + flag_type_suffix_len(&flag.flag_type),
							|width, long_alias| width + long_alias.len(),
						) + long_aliases.len() * 4
					}
				};

				let max_calc =
					|flags: &Vector<Flag>, s_width_max: &mut usize, nl_width_max: &mut usize| {
						if let Vector(Some(flags)) = flags {
							for f in flags {
								*s_width_max = max(*s_width_max, f.short_alias.len());
								*nl_width_max = max(*nl_width_max, nl_width(f));
							}
						}
					};
				// フラグ出力
				let l_flags = &cmd.l_flags;
				let c_flags = &cmd.c_flags;
				let ctx_c_flags = &ctx.common_flags;
				// short_aliasの幅とlong_aliasの幅計算
				// short_aliasとlong_aliasを調べてmax_widthを出す
				let mut s_width_max: usize = 1; //文字幅が決まっているので文字数を記録
				let mut nl_width_max: usize = 8;
				max_calc(l_flags, &mut s_width_max, &mut nl_width_max);
				max_calc(c_flags, &mut s_width_max, &mut nl_width_max);
				if let Vector(Some(ctx_c_flags)) = ctx_c_flags {
					for ccf in ctx_c_flags {
						max_calc(ccf, &mut s_width_max, &mut nl_width_max);
					}
				}

				let head: String;
				let cl_label: bool;
				if cmd.l_flags.has_at_least_one() {
					// ローカルがある場合、区別用のラベル表示する
					head = indent.repeat(2);
					cl_label = true;
				} else {
					head = indent.clone();
					cl_label = false;
				}

				let gap = sp.repeat(2);
				if let Vector(Some(l_flags)) = l_flags {
					if cl_label {
						help.push_str(&indent);
						help.push_str("[Local]: \n");
					}
					for l in l_flags.iter().rev() {
						help.push_str(&head);
						help = flag_help_tablize(help, l, &sp, s_width_max, nl_width_max, &gap);
					}
				}

				if let Vector(Some(c_flags)) = c_flags {
					if cl_label {
						help.push_str(&indent);
						help.push_str("[Common (available in this command and sub command");
						if cmd.sub.len() > 1 {
							help.push('s');
						}
						help.push_str(" under this command)]: \n");
					}
					for c in c_flags.iter().rev() {
						help.push_str(&head);
						help = flag_help_tablize(help, c, &sp, s_width_max, nl_width_max, &gap)
					}
				}

				if let Vector(Some(ctx_c_flags)) = ctx_c_flags {
					let route_without_root = ctx.depth() > ctx.routes.len();
					for (index, cc_flags) in ctx_c_flags.iter().enumerate().rev() {
						if let Vector(Some(cc_flags)) = cc_flags {
							help.push_str("[Common, inherited from ");
							if route_without_root {
								if index < 1 {
									help.push_str(&root_str(ctx.exe_path()))
								} else {
									match ctx.routes.get(index - 1) {
										Some(val) => help.push_str(val),
										None => help.push_str("unknown"),
									}
								}
							} else {
								match ctx.routes.get(index) {
									Some(val) => help.push_str(val),
									None => help.push_str("unknown"),
								}
							}
							help.push_str("]: \n");
							for c in cc_flags {
								help.push_str(&head);
								help = flag_help_tablize(help, c, &sp, s_width_max, nl_width_max, &gap);
							}
						}
					}
				}
			}

			if let Vector(Some(sub_commands)) = &cmd.sub {
				help = help + "Sub Command";
				if sub_commands.len() > 1 {
					help.push('s');
				}
				help = help + ": \n";
				let mut na_max_width: usize = 10;
				for sc in sub_commands {
					match &sc.alias {
						Vector(None) => na_max_width = max(na_max_width, sc.name.len()),
						Vector(Some(alias)) => {
							na_max_width = max(
								na_max_width,
								alias
									.iter()
									.fold(sc.name.len() + 2 * alias.len(), |sum, a| sum + a.len()),
							);
						}
					}
				}

				na_max_width += 3;

				for sc in sub_commands {
					let help_pref_len = help.len();
					help = help + &sc.name;
					if let Vector(Some(alias)) = &sc.alias {
						help = alias.iter().fold(help, |help, a| help + ", " + a)
					}
					let sp_num = na_max_width - help.len() + help_pref_len;
					help = help + &sp.repeat(sp_num);
					if let Some(description) = &sc.description {
						help.push_str(description);
					}
					help.push('\n');
				}

				help.push_str("\nSee '");
				if ctx.depth() > 0 {
					if ctx.depth() > ctx.routes.len() {
						help.push_str(&root_str(ctx.exe_path()));
						help.push_str(&sp);
					}
					if let Vector(Some(routes)) = &ctx.routes {
						for route in routes {
							help.push_str(route);
							help.push_str(&sp);
						}
					}
					help.push_str(&cmd.name);
					help.push_str(" <subcommand> --help' for more information");
				} else {
					let root = if cmd.name.is_empty() {
						root_str(&ctx.exe_path())
					} else {
						cmd.name.clone()
					};

					help.push_str(&root);
					help.push_str("<subcommand> --help' for more information.")
				}
			}

			help
		}

		macro_rules! _flag_tablize_dedup {
			($iter:expr,$nl_col_width:ident,$s_col_width:ident,$nl_list:ident,$s_list:ident,$s_columns:ident,$nl_columns:ident) => {
				for f in $iter {
					let mut alias_exist = false;
					if let Vector(Some(sa)) = &f.short_alias {
						let mut dedup_s = Vec::<&char>::new();
						for s in sa.iter() {
							if !$s_list.contains(&s) {
								dedup_s.push(s);
								$s_list.push(s);
								alias_exist = true;
							}
						}
						$s_col_width = max(dedup_s.len(), $s_col_width);
						$s_columns.push_back(dedup_s);
					}
					let mut dedup_nl = Vec::<&String>::new();
					let mut nl_width;
					if !$nl_list.contains(&&f.name) {
						$nl_list.push(&f.name);
						dedup_nl.push(&f.name);
						nl_width = f.name.len();
						alias_exist = true;
					} else {
						nl_width = 0;
					}
					if let Vector(Some(long_alias)) = &f.long_alias {
						for la in long_alias {
							if !$nl_list.contains(&la) {
								$nl_list.push(la);
								dedup_nl.push(la);
								nl_width += la.len();
								alias_exist = true;
							}
						}
					}
					if alias_exist {
						match dedup_nl.len() {
							1 => {
								nl_width += flag_type_suffix_len(&f.flag_type);
							}
							x if x > 1 => {
								nl_width += match &f.flag_type {
									FlagType::Bool => x * 4,
									FlagType::String => x * 4 + 9,
									FlagType::Int => x * 4 + 6,
									FlagType::Float => x * 4 + 8,
								};
							}
							_ => {}
						}
					}
					$nl_columns.push_back(dedup_nl);
					$nl_col_width = max(nl_width, $nl_col_width);
				}
			};
		}

		fn add_short_flags_str(append_to: &mut String, s_list: Vec<&char>) {
			append_to.push('-');
			let mut si = s_list.into_iter();
			append_to.push(*si.next().unwrap());
			for s in si {
				append_to.push_str(", -");
				append_to.push(*s);
			}
		}
		fn flag_type_suffix_len(ft: &FlagType) -> usize {
			match &ft {
				FlagType::Bool => 2,
				FlagType::String => 11, // 2 + " <string>"
				FlagType::Int => 8,     // 2 + " <int>"
				FlagType::Float => 10,  // 2 + " <float>"
			}
		}

		fn add_long_flags_str_to_prev_flags(
			append_to: &mut String,
			nl_iter: std::vec::IntoIter<&String>,
		) {
			for nl in nl_iter {
				append_to.push_str(", --");
				append_to.push_str(nl);
			}
		}

		fn add_long_flags_str(append_to: &mut String, mut nl_iter: std::vec::IntoIter<&String>) {
			append_to.push_str("--");
			append_to.push_str(nl_iter.next().unwrap());
			add_long_flags_str_to_prev_flags(append_to, nl_iter);
		}

		fn add_flags_help_str(
			mut append_to: String,
			flags: &Vec<Flag>,
			s_columns: &mut std::collections::VecDeque<Vec<&char>>,
			nl_columns: &mut std::collections::VecDeque<Vec<&String>>,
			s_col_width: usize,
			nl_col_width: usize,
			gap_width: usize,
			suffix: &str,
			prefix: &str,
			sp: &String,
		) -> String {
			for f in flags.iter().rev() {
				append_to.push_str(prefix);
				let s_list = s_columns.pop_front().unwrap();
				let nl_list = nl_columns.pop_front().unwrap();
				if s_list.is_empty() {
					if !nl_list.is_empty() {
						append_to.push_str(&sp.repeat(s_col_width));
						let prev_help_len = append_to.len();
						add_long_flags_str(&mut append_to, nl_list.into_iter());
						append_to = add_type_suffix(append_to, &f.flag_type);
						let nl_len = append_to.len() - prev_help_len;
						append_to = append_to
							+ &sp.repeat(nl_col_width - nl_len + gap_width)
							+ &f.description + suffix;
					}
				} else {
					append_to = append_to + &sp.repeat(s_col_width - (s_list.len() * 4));
					add_short_flags_str(&mut append_to, s_list);
					if nl_list.is_empty() {
						append_to = add_type_suffix(append_to, &f.flag_type)
							+ &sp.repeat(4 + nl_col_width)
							+ &f.description + suffix;
					} else {
						let prev_help_len = append_to.len();
						add_long_flags_str_to_prev_flags(&mut append_to, nl_list.into_iter());
						append_to = add_type_suffix(append_to, &f.flag_type);
						let nl_len = append_to.len() - prev_help_len - 2;
						append_to = append_to
							+ &sp.repeat(nl_col_width - nl_len + gap_width)
							+ &f.description + suffix;
					}
				}
			}
			append_to
		}

		/// Preset of help function (tablize) with deleted duplication
		pub fn help_tablize_with_alias_dedup(cmd: &Command, ctx: &Context) -> String {
			let mut help = String::new();
			let indent_size = 3;
			let sp = String::from(" ");
			let indent: String = sp.repeat(indent_size);
			if let Some(description) = &cmd.description {
				help.push_str(description);
				help.push_str("\n\n");
			}
			help = help + "Usage:\n" + &indent + &cmd.usage + "\n";

			let flag_num = cmd.l_flags.len() + cmd.c_flags.len() + ctx.common_flags.sum_of_length();
			if flag_num > 0 {
				let mut nl_col_width = 5;
				let mut s_col_width = 1;
				help.push_str("\nFlags: \n");

				let mut nl_list = Vec::<&String>::new();
				let mut s_list = Vec::<&char>::new();
				let mut s_columns = std::collections::VecDeque::<Vec<&char>>::with_capacity(flag_num);
				let mut nl_columns =
					std::collections::VecDeque::<Vec<&String>>::with_capacity(flag_num);
				if let Vector(Some(l_flags)) = &cmd.l_flags {
					let mut l = l_flags.iter().rev();
					if let Some(f) = l.next() {
						nl_list.push(&f.name);
						let mut nl_width = f.name.len() + 2;
						if let Vector(Some(la)) = &f.long_alias {
							let mut la: Vec<&String> = la.iter().collect();
							nl_width += 4 * la.len();
							for l in la.iter() {
								nl_width += l.len();
							}
							nl_list.append(&mut la);
						}
						nl_col_width = max(nl_width, nl_col_width);
						nl_columns.push_back(nl_list.clone());
						if let Vector(Some(sa)) = &f.short_alias {
							let mut sa: Vec<&char> = sa.iter().collect();
							s_col_width = max(sa.len(), s_col_width);
							s_list.append(&mut sa);
						}
						s_columns.push_back(s_list.clone());
						_flag_tablize_dedup!(
							l,
							nl_col_width,
							s_col_width,
							nl_list,
							s_list,
							s_columns,
							nl_columns
						);
					}
				}
				if let Vector(Some(c_flags)) = &cmd.c_flags {
					_flag_tablize_dedup!(
						c_flags.iter().rev(),
						nl_col_width,
						s_col_width,
						nl_list,
						s_list,
						s_columns,
						nl_columns
					);
				}
				if let Vector(Some(cfs)) = &ctx.common_flags {
					for c_flags in cfs.iter().rev() {
						if let Vector(Some(c_flags)) = c_flags {
							_flag_tablize_dedup!(
								c_flags.iter().rev(),
								nl_col_width,
								s_col_width,
								nl_list,
								s_list,
								s_columns,
								nl_columns
							)
						}
					}
				}
				drop(s_list);
				drop(nl_list);
				// help出力
				s_col_width = s_col_width * 4;
				let gap_width = 3;
				if let Vector(Some(l_flags)) = &cmd.l_flags {
					let suffix = "\n";
					help = add_flags_help_str(
						help,
						l_flags,
						&mut s_columns,
						&mut nl_columns,
						s_col_width,
						nl_col_width,
						gap_width,
						suffix,
						&indent,
						&sp,
					)
				}

				if let Vector(Some(c_flags)) = &cmd.c_flags {
					let suffix = match &cmd.sub {
						Vector(Some(subs)) if subs.len() > 1 => {
							" [common: also available in sub commands under here]\n"
						}
						Vector(Some(subs)) if subs.len() > 0 => {
							" [common: also available in sub command under here]\n"
						}
						_ => "\n",
					};
					help = add_flags_help_str(
						help,
						c_flags,
						&mut s_columns,
						&mut nl_columns,
						s_col_width,
						nl_col_width,
						gap_width,
						suffix,
						&indent,
						&sp,
					)
				}

				if let Vector(Some(c_flags_list)) = &ctx.common_flags {
					let route_without_root = ctx.depth() > ctx.routes.len();
					for (index, c_flags) in c_flags_list.into_iter().enumerate().rev() {
						if let Vector(Some(c_flags)) = c_flags {
							let mut suffix = sp.clone() + "[common: inherited from ";
							if route_without_root {
								if index < 1 {
									suffix.push_str(&root_str(ctx.exe_path()))
								} else {
									match ctx.routes.get(index - 1) {
										Some(val) => suffix.push_str(val),
										None => suffix.push_str("unknown"),
									}
								}
							} else {
								match ctx.routes.get(index) {
									Some(val) => suffix.push_str(val),
									None => suffix.push_str("unknown"),
								}
							}
							suffix.push_str("]\n");

							help = add_flags_help_str(
								help,
								c_flags,
								&mut s_columns,
								&mut nl_columns,
								s_col_width,
								nl_col_width,
								gap_width,
								&suffix,
								&indent,
								&sp,
							);
						}
					}
				}
			}

			if let Vector(Some(sub_commands)) = &cmd.sub {
				help = help + "\nSub Command";
				if sub_commands.len() > 1 {
					help.push('s');
				}
				help = help + ": \n";

				// サブコマンド名の列挙最大長算出
				let mut na_max_width: usize = 12;
				for sc in sub_commands {
					match &sc.alias {
						Vector(None) => na_max_width = max(na_max_width, sc.name.len()),
						Vector(Some(alias)) => {
							na_max_width = max(
								na_max_width,
								alias
									.iter()
									.fold(sc.name.len() + 2 * alias.len(), |sum, a| sum + a.len()),
							);
						}
					}
				}
				na_max_width += 3;

				for sc in sub_commands {
					let help_pref_len = help.len();
					help = help + &indent + &sc.name;
					if let Vector(Some(alias)) = &sc.alias {
						for a in alias {
							help = help + a;
						}
					}
					let sp_num = na_max_width + help_pref_len - help.len();
					help = help + &sp.repeat(sp_num);
					if let Some(description) = &sc.description {
						help.push_str(description);
					}
					help.push('\n');
				}

				help.push_str("\nSee '");
				if ctx.depth() > 0 {
					if ctx.depth() > ctx.routes.len() {
						help.push_str(&root_str(ctx.exe_path()));
						help.push_str(&sp);
					}
					if let Vector(Some(routes)) = &ctx.routes {
						for route in routes {
							help.push_str(route);
							help.push_str(&sp);
						}
					}
					help.push_str(&cmd.name);
				} else {
					let root = if cmd.name.is_empty() {
						root_str(&ctx.exe_path())
					} else {
						cmd.name.clone()
					};
					help.push_str(&root);
				}
				help.push_str(" <subcommand> --help' for more information");
			}

			help
		}

		/// Get root path as string for help
		pub fn root_str(exe_path: &str) -> String {
			let exe_path = std::path::Path::new(exe_path);
			let mut root_string = exe_path
				.file_stem()
				.unwrap_or(std::ffi::OsStr::new("root"))
				.to_str()
				.unwrap_or("root")
				.to_owned();
			if let Some(val) = exe_path.extension() {
				match val.to_str() {
					Some(val) => root_string = root_string + "[." + val + "]",
					None => match std::env::consts::EXE_SUFFIX {
						"" => {}
						val => root_string = root_string + "[" + val + "]",
					},
				}
			}

			root_string
		}
	}
}
