use crate::{
	action::{ActionError, ActionResult},
	parser::MiddleArg,
	take_string, Action, Context, Flag, FlagValue, Parser, Vector,
};

use std::collections::VecDeque;
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
	/// license of command
	pub license: Option<(String, String)>,
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
	///custom help fuction
	pub help: Option<HelpFunc>,
}

/// HelpFunc shows type alias for help function
pub type HelpFunc = fn(
	command: &Command,
	context: &Context,
	default_help: fn(&Command, &Context) -> String,
) -> String;

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
		Command::build_new(
			name.into(),
			action,
			authors.into(),
			String::default(),
			None,
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
			license: None,
			description: None,
			usage: String::default(),
			l_flags: Vector::default(),
			c_flags: Vector::default(),
			alias: Vector::default(),
			version: String::default(),
			sub: Vector::default(),
			help: None,
		}
	}

	/// Create new instance of Command with more options
	#[allow(clippy::too_many_arguments)]
	pub fn build_new(
		name: String,
		action: Option<Action>,
		authors: String,
		copyright: String,
		license: Option<(String, String)>,
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
			help: None,
		}
	}

	/// Run command with collecting args automatically
	pub fn run_with_auto_arg_collect(mut self) {
		//let args: Vec<String> = std::env::args().collect();
		//self.run(args);
		match &self.sub {
			Vector(None) => self.single_run(std::env::args().collect::<Vec<String>>()),
			_ => self.run(std::env::args().collect::<Vec<String>>()),
		};
	}

	/// Run command as single(do not have sub) command
	pub fn single_run(&mut self, raw_args: Vec<String>) {
		match self.action.take() {
			Some(action) => {
				if raw_args.len() < 2 {
					let current_path = raw_args[0].clone();
					let req = action(Context::new(
						raw_args,
						VecDeque::new(),
						self.c_flags.take(),
						self.l_flags.take(),
						self.derive_route_init_vector(),
						current_path,
						take_string!(self.version),
						take_string!(self.copyright),
						self.license.take(),
					));

					self.handle_action_result(req);
				} else {
					let mut args = VecDeque::from(raw_args.clone());
					let current_path = args.pop_front().unwrap();
					let mut context = Context::new(
						raw_args,
						args,
						self.c_flags.take(),
						self.l_flags.take(),
						self.derive_route_init_vector(),
						current_path,
						take_string!(self.version),
						take_string!(self.copyright),
						self.license.take(),
					);
					//println!("single_run_context: {:?}", context);
					context = Parser::default().parse_args_until_end(context);

					let req = action(context);
					self.handle_action_result(req)
				}
			}
			None => match self.sub {
				Vector(None) => {
					let mut args: VecDeque<String> = raw_args.clone().into();
					let current_path = args.pop_front().unwrap();
					let c_flags = self.c_flags.take();
					let l_flags = self.l_flags.take();
					let version = take_string!(self.version);
					let copyright = take_string!(self.copyright);
					let license = self.license.take();
					self.handle_action_result(Err(ActionError::new(
						"No action is registered.",
						Context::new(
							raw_args,
							args,
							c_flags,
							l_flags,
							self.derive_route_init_vector(),
							current_path,
							version,
							copyright,
							license,
						),
						None,
					)));
				}
				_ => self.run(raw_args),
			},
		}
	}

	/// Show command's help
	pub fn show_help(&self, c: &Context) {
		if let Some(help) = self.help {
			println!(
				"{}",
				help(&self, c, |command, context| {
					command.default_help(context)
				})
			);
		} else {
			println!("{}", self.default_help(c));
		}
	}

	/// Returuns default help as String
	pub fn default_help<'a>(&self, c: &Context) -> String {
		let mut help = String::new();
		let indent_size: usize = 3;
		let sp = String::from(" ");
		let indent: String = sp.repeat(indent_size);
		match &self.description {
			Some(description) => {
				help.push_str(description);
				help.push_str("\n\n");
			}
			_ => {}
		}
		help += &format!("Usage:\n{}{}\n\n", &indent, self.usage);

		//フラグ処理
		let l_flags: &Vector<Flag> = if c.local_flags.is_none() {
			&self.l_flags
		} else {
			&c.local_flags
		};
		let c_flags: &Vector<Vector<Flag>>;
		let vec_inner: Vector<Vector<Flag>>;

		//コモンフラグが残っていた場合
		if self.c_flags.is_none() {
			c_flags = &c.common_flags;
		} else {
			vec_inner = Vector(Some(vec![self.c_flags.clone()]));
			c_flags = &vec_inner;
		}

		//どちらのフラグもある
		//TODO: 有効なフラグを整理して表示できるようにしたい(この下のかっこ部分を消しても支障のない状態にする)

		help.push_str("Flags(If exist flags have same alias and specified by user, inputted value will be interpreted as the formaer flag's value): \n");
		let head: String;
		let common_label;
		let name_and_alias_field_min_width: usize = 7;
		if c.common_flags.sum_of_length() > 0 {
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
			let route_without_root = depth > c.routes.len();
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
										let cur_path = std::path::Path::new(c.current());
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
										c.routes.get(index - 1).unwrap()
									}
								} else {
									c.routes.get(index).unwrap()
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
									if self.sub.len() < 2 {
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

		if let Vector(Some(sub_commands)) = &self.sub {
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
				if self.name.is_empty() {
					let path = std::path::Path::new(c.current());
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
						&self.name
					} else {
						loc_owned = if let Vector(Some(routes)) = &c.routes {
							routes.iter().rfold(
								{
									if depth > routes.len() {
										let path = std::path::Path::new(c.current());
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

	/// Add a common flag to command
	pub fn common_flag(mut self, flag: Flag) -> Self {
		self.c_flags.push(flag);
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

	/// Add command's alias
	pub fn alias<T: Into<String>>(mut self, a: T) -> Self {
		self.alias.push(a.into());
		self
	}

	/// Set custom help function
	pub fn help(mut self, help_function: HelpFunc) -> Self {
		self.help = Some(help_function);
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
				None => return None,
				Some(index) => Some(inner.swap_remove(index)),
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
			license: None,
			description: None,
			usage: String::default(),
			l_flags: Vector::default(),
			c_flags: Vector::default(),
			alias: Vector::default(),
			version: String::default(),
			sub: Vector::default(),
			help: None,
		}
	}
}

/// Trait for implementation Run function.
pub trait Run<T> {
	/// run function
	fn run(&mut self, args: T);
}

impl Run<Vec<String>> for Command {
	fn run(&mut self, args: Vec<String>) {
		self.run_from_args(args);
	}
}

impl Run<Context> for Command {
	fn run(&mut self, c: Context) {
		self.run_with_context(c);
	}
}

impl Command {
	/// Run commands with raw_args
	pub fn run_from_args(&mut self, raw_args: Vec<String>) {
		if self.sub.is_none() {
			return self.single_run(raw_args);
		}
		let mut args = VecDeque::from(raw_args.clone());
		let current_path = args.pop_front().unwrap();
		let head = args.pop_front();
		if head.is_none() {
			//引数がない場合
			match self.action {
				Some(action) => {
					let req = action(Context::new(
						raw_args,
						args,
						self.c_flags.take(),
						self.l_flags.take(),
						self.derive_route_init_vector(),
						current_path,
						take_string!(self.version),
						take_string!(self.copyright),
						self.license.take(),
					));
					self.handle_action_result(req);
				}
				None => {
					println!("no action is registered.");
					let c = Context::new(
						raw_args,
						args,
						self.c_flags.take(),
						self.l_flags.take(),
						self.derive_route_init_vector(),
						current_path,
						take_string!(self.version),
						take_string!(self.copyright),
						self.license.take(),
					);
					self.show_help(&c);
					//println!("args: {:?}", raw_args);
				}
			}
		} else {
			//get before first non-flag arg with parsing flags
			let p = Parser::default();
			match head {
				Some(long_flag) if p.long_flag(&long_flag) => {
					//long flag
					let (arg, args, mut inter_mediate_args, last_flag_arg) =
						p.middle_parse(args, VecDeque::new(), p.long_middle(long_flag));
					//最初にフラグの形になっていない引数を求める
					/*println!(
						"\r\nmiddle_result at 281 {:?}\r\n",
						(&arg, &args, &inter_mediate_args, &last_flag_arg)
					);*/
					if let Some(arg) = arg {
						match self.take_sub(&arg) {
							//サブコマンド合致
							Some(mut sub) => {
								inter_mediate_args.push_back(last_flag_arg);
								let context = Context::build_new(
									raw_args,
									args,
									vec![self.c_flags.take()].into(),
									Vector::default(),
									current_path.into(),
									self.derive_route_init_vector(),
									Vector::default(),
									Vector::default(),
									Some(inter_mediate_args),
									Vector::default(),
									take_string!(self.version),
									take_string!(self.copyright),
									self.license.take(),
								);
								sub.run_with_context(context);
							}
							None =>
							//サブコマンドが合致しなかった場合
							{
								match &last_flag_arg {
									MiddleArg::LongFlag(_, FlagValue::None)
									| MiddleArg::ShortFlag(_, FlagValue::None) => {
										//検出したものがフラグの値になる可能性がある場合のハンドリング
										inter_mediate_args.push_back(last_flag_arg);
										inter_mediate_args.push_back(MiddleArg::Normal(arg));
										self.assign_run(args, inter_mediate_args, p, raw_args, current_path);
									}
									_ => {
										//フラグになる可能性がない場合
										inter_mediate_args.push_back(last_flag_arg);
										inter_mediate_args.push_back(MiddleArg::Normal(arg));
										let context = Context::build_new(
											raw_args,
											args,
											vec![self.c_flags.take()].into(),
											self.l_flags.take(),
											current_path,
											self.derive_route_init_vector(),
											Vector::default(),
											Vector::default(),
											Some(inter_mediate_args),
											Vector::default(),
											take_string!(self.version),
											take_string!(self.copyright),
											self.license.take(),
										);
										let (mut context, non_flag_arg) =
											p.parse_inter_mediate_args(context, false);
										if let Some(mut non_flag_arg) = non_flag_arg {
											non_flag_arg.append(&mut context.args);
											context.args = non_flag_arg;
										}
										match self.action {
											Some(action) => {
												self.handle_action_result(action(context));
											}
											None => {
												println!("no action registered");
												self.show_help(&context);
											}
										}
									}
								}
							}
						}
					} else {
						//Noneの場合、そのままself.actionに放り込む
						let context = Context::build_new(
							raw_args,
							args,
							vec![self.c_flags.take()].into(),
							self.l_flags.take(),
							current_path.into(),
							self.derive_route_init_vector(),
							Vector(None),
							Vector(None),
							Some({
								inter_mediate_args.push_back(last_flag_arg);
								inter_mediate_args
							}),
							Vector(None),
							take_string!(self.version),
							take_string!(self.copyright),
							self.license.take(),
						);
						match self.action {
							Some(action) => {
								let (mut context, args) = p.parse_inter_mediate_args(context, true);
								if let Some(mut args) = args {
									context.args = {
										args.append(&mut context.args);
										args
									}
								}
								self.handle_action_result(action(context));
							}
							_ => {
								println!("no action registered");
								self.show_help(&context);
							}
						}
					}
				}
				Some(short_flag) if p.flag(&short_flag) => {
					//short flag
					let (arg, args, mut inter_mediate_args, last_flag_arg) =
						p.middle_parse(args, VecDeque::new(), p.short_middle(short_flag));
					if let Some(arg) = arg {
						match self.take_sub(&arg) {
							Some(mut sub) => {
								let context = Context::build_new(
									raw_args,
									args,
									vec![self.c_flags.take()].into(),
									Vector(None),
									current_path.into(),
									self.derive_route_init_vector(),
									Vector(None),
									Vector(None),
									Some(inter_mediate_args),
									Vector(None),
									take_string!(self.version),
									take_string!(self.copyright),
									self.license.take(),
								);
								sub.run_with_context(context);
							}
							None => match &last_flag_arg {
								MiddleArg::LongFlag(_, FlagValue::None)
								| MiddleArg::ShortFlag(_, FlagValue::None) => {
									inter_mediate_args.push_back(last_flag_arg);
									inter_mediate_args.push_back(MiddleArg::Normal(arg));
									self.assign_run(args, inter_mediate_args, p, raw_args, current_path);
								}
								_ => {
									//フラグの値になる可能性がない場合（サブコマンドではなくself実行）
									inter_mediate_args.push_back(last_flag_arg);
									inter_mediate_args.push_back(MiddleArg::Normal(arg));
									let context = Context::build_new(
										raw_args,
										args,
										vec![self.c_flags.take()].into(),
										self.l_flags.take(),
										current_path.into(),
										self.derive_route_init_vector(),
										Vector::default(),
										Vector::default(),
										Some(inter_mediate_args),
										Vector::default(),
										take_string!(self.version),
										take_string!(self.copyright),
										self.license.take(),
									);
									let (mut context, non_flag_args) =
										p.parse_inter_mediate_args(context, false);
									if let Some(mut non_flag_args) = non_flag_args {
										non_flag_args.append(&mut context.args);
										context.args = non_flag_args;
									}
									match self.action {
										Some(action) => {
											self.handle_action_result(action(context));
										}
										None => {
											println!("no action registered");
											self.show_help(&context);
										}
									}
								}
							},
						}
					}
				}
				Some(arg) => {
					match self.take_sub(&arg) {
						None => match self.action {
							None => {
								let c = Context::new(
									raw_args,
									args,
									self.c_flags.take(),
									Vector::default(),
									self.derive_route_init_vector(),
									current_path,
									take_string!(self.version),
									take_string!(self.copyright),
									self.license.take(),
								);
								println!("{} does not have its own action.", self.name);
								self.show_help(&c);
							}
							Some(action) => {
								args.push_front(arg);
								let mut c = Context::new(
									raw_args,
									args,
									self.c_flags.take(),
									self.l_flags.take(),
									self.derive_route_init_vector(),
									current_path,
									take_string!(self.version),
									take_string!(self.copyright),
									self.license.take(),
								);
								c = p.parse_args_until_end(c);
								self.handle_action_result(action(c));
							}
						},
						Some(mut sub) => {
							//println!("{}", sub.name);
							let common_flag = self.c_flags.take();
							let c = Context::new(
								raw_args,
								args,
								common_flag,
								Vector(None),
								self.derive_route_init_vector(),
								current_path,
								take_string!(self.version),
								take_string!(self.copyright),
								self.license.take(),
							);
							sub.run(c);
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
	pub fn run_with_context(&mut self, mut context: Context) {
		if self.sub.is_none() {
			context.local_flags = self.l_flags.take();
			context.common_flags.push(self.c_flags.take());
			let p = Parser::default();
			let (mut context, non_flag_args) = p.parse_inter_mediate_args(context, true);
			if let Some(mut non_flag_args) = non_flag_args {
				non_flag_args.append(&mut context.args);
				context.args = non_flag_args;
			}
			context = p.parse_args_until_end(context);
			context.routes.push(self.name.clone());
			match self.action {
				Some(action) => {
					self.handle_action_result(action(context));
				}
				None => println!("no action is registered"),
			}
		} else {
			//サブコマンドと一致するかを捜査
			context.routes.push(self.name.clone());
			let p = Parser::default();
			match context.args.pop_front() {
				Some(long_flag) if p.long_flag(&long_flag) => {
					let last = p.long_middle(long_flag);
					self.assign_context(context, p, VecDeque::new(), last);
				}
				Some(short_flag) if p.flag(&short_flag) => {
					let last = p.short_middle(short_flag);
					self.assign_context(context, p, VecDeque::new(), last);
				}
				Some(arg) => {
					//println!("arg sub-sub: {}", &arg);
					match self.take_sub(&arg) {
						Some(mut sub) => {
							//println!("{}", &sub.name);
							context.common_flags.push(self.c_flags.take());
							sub.run(context);
						}
						None => {
							context.common_flags.push(self.c_flags.take());
							context.local_flags = self.l_flags.take();
							match self.action {
								None => {
									println!("{} does not have its own action.", &self.name);
									self.show_help(&context);
								}
								Some(action) => {
									let c = match p.parse_inter_mediate_args(context, true) {
										(mut context, None) => {
											context = p.parse_args_until_end(context);
											context.args.push_front(arg);
											context
										}
										(mut context, Some(mut non_flag_args)) => {
											context = p.parse_args_until_end(context);
											context.args.push_front(arg);
											non_flag_args.append(&mut context.args);
											context.args = non_flag_args;
											context
										}
									};
									self.handle_action_result(action(c));
								}
							}
						}
					}
				}
				None => {
					//サブコマンド等の処理の必要がないのでこのまま叩き込む
					match self.action {
						Some(action) => {
							context.local_flags = self.l_flags.take();
							context.common_flags.push(self.c_flags.take());

							let (mut context, non_flag_args) = p.parse_inter_mediate_args(context, true);
							if let Some(mut non_flag_args) = non_flag_args {
								non_flag_args.append(&mut context.args);
								context.args = non_flag_args;
							}
							self.handle_action_result(action(context));
						}
						None => {
							println!("no action is registered.");
						}
					}
				}
			}
		}
	}

	/// Assign context to sub command or command's own action.
	/// コンテキストのargsを見てもサブコマンド行きかコマンドでそのまま処理すればいいか分からなかった時の処理用
	pub fn assign_context(
		&mut self,
		mut c: Context,
		p: Parser,
		mut inter_mediate_args: VecDeque<MiddleArg>,
		last: MiddleArg,
	) {
		let (next_non_flag, args, _inter_mediate_args, last) =
			p.middle_parse(c.args, inter_mediate_args, last);
		inter_mediate_args = _inter_mediate_args;
		//println!("next_non_flag: {:?}", next_non_flag);
		match next_non_flag {
			Some(arg) => match self.take_sub(&arg) {
				Some(mut sub) => {
					c.common_flags.push(self.c_flags.take());
					c.args = args;
					inter_mediate_args.push_back(last);
					if let Some(mut parsing_args) = c.parsing_args {
						parsing_args.append(&mut inter_mediate_args);
						c.parsing_args = Some(parsing_args);
					} else {
						c.parsing_args = Some(inter_mediate_args);
					}

					sub.run(c);
				}
				None => match &last {
					MiddleArg::LongFlag(_, FlagValue::None)
					| MiddleArg::ShortFlag(_, FlagValue::None) => {
						inter_mediate_args.push_back(last);
						inter_mediate_args.push_back(MiddleArg::Normal(arg));
						c.args = args;
						match c.args.pop_front() {
							Some(long_flag) if p.long_flag(&long_flag) => {
								let last = p.long_middle(long_flag);
								self.assign_context(c, p, inter_mediate_args, last);
							}
							Some(short_flag) if p.flag(&short_flag) => {
								let last = p.short_middle(short_flag);
								self.assign_context(c, p, inter_mediate_args, last);
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
									sub.run(c);
								}
								None => match self.action {
									Some(action) => {
										if let Some(mut parsing_args) = c.parsing_args {
											parsing_args.append(&mut inter_mediate_args);
											c.parsing_args = Some(parsing_args);
										} else {
											c.parsing_args = Some(inter_mediate_args);
										}

										c.local_flags = self.l_flags.take();
										c.common_flags.push(self.c_flags.take());
										let (mut c, non_flag_args) = p.parse_inter_mediate_args(c, false);
										c = p.parse_args_until_end(c);
										c.args.push_front(arg);
										if let Some(mut non_flag_args) = non_flag_args {
											non_flag_args.append(&mut c.args);
											c.args = non_flag_args;
										}

										self.handle_action_result(action(c));
									}
									None => {
										println!("no action registered.");
										let err = ActionError::new("No action registered", c, None);
										self.handle_action_result(Err(err));
									}
								},
							},
							None => match self.action {
								Some(action) => {
									//println!("inter_mediate_args: {:?}\r\n", inter_mediate_args);
									if let Some(mut parsing_args) = c.parsing_args {
										parsing_args.append(&mut inter_mediate_args);
										c.parsing_args = Some(parsing_args);
									} else {
										c.parsing_args = Some(inter_mediate_args);
									}
									c.local_flags = self.l_flags.take();
									let (mut c, args) = p.parse_inter_mediate_args(c, false);

									if let Some(mut args) = args {
										args.append(&mut c.args);
										c.args = args;
									}
									self.handle_action_result(action(c));
								}
								None => {
									println!("no action registered.");
								}
							},
						}
					}
					_ => match self.action {
						Some(action) => {
							inter_mediate_args.push_back(last);
							c.args = args;
							c.common_flags.push(self.c_flags.take());
							c.local_flags = self.l_flags.take();
							if let Some(mut parsing_args) = c.parsing_args {
								parsing_args.append(&mut inter_mediate_args);
								c.parsing_args = Some(parsing_args);
							}
							let (mut c, non_flag_args) = p.parse_inter_mediate_args(c, false);
							c = p.parse_args_until_end(c);
							c.args.push_front(arg);
							if let Some(mut non_flag_args) = non_flag_args {
								non_flag_args.append(&mut c.args);
								c.args = non_flag_args;
							}
							self.handle_action_result(action(c));
						}
						None => {
							println!("no action is registered.");
						}
					},
				},
			},
			None => {
				match self.action {
					Some(action) => {
						inter_mediate_args.push_back(last);
						c.args = args;
						if let Some(mut parsing_args) = c.parsing_args {
							parsing_args.append(&mut inter_mediate_args);
							c.parsing_args = Some(parsing_args);
						} else {
							c.parsing_args = Some(inter_mediate_args);
						}
						c.common_flags.push(self.c_flags.take());
						c.local_flags = self.l_flags.take();
						let (mut c, non_flag_args) = p.parse_inter_mediate_args(c, false);
						//println!("after_parse_ima:{:?}", c);
						if let Some(non_flag_args) = non_flag_args {
							//non_flag_args.append(&mut c.args);
							c.args = non_flag_args;
						}
						self.handle_action_result(action(c));
					}
					None => {
						println!("no action is registered.");
					}
				}
			}
		}
	}

	/// Assign subcomannd's run or command's own action with no context
	/// コンテキストが生成されていないときに、run_from_args内で第一引数からサブコマンドかそうでないか分からなかった時に再帰処理を行って割り当てを行う関数
	pub fn assign_run(
		&mut self,
		mut args: VecDeque<String>,
		mut inter_mediate_args: VecDeque<MiddleArg>,
		p: Parser,
		raw_args: Vec<String>,
		current_path: String,
	) {
		//println!("assign_run");
		match args.pop_front() {
			Some(long_flag) if p.long_flag(&long_flag) => {
				//println!("long_flag: {}", &long_flag);
				let (arg, _args, mut _inter_mediate_args, last_flag_arg) =
					p.middle_parse(args, inter_mediate_args, p.long_middle(long_flag));
				args = _args;
				inter_mediate_args = _inter_mediate_args;
				if let Some(arg) = arg {
					match self.take_sub(&arg) {
						Some(mut sub) => {
							inter_mediate_args.push_back(last_flag_arg);
							sub.run(Context::build_new(
								raw_args,
								args,
								Vector::with_first_elem(self.c_flags.take()),
								Vector::default(),
								current_path.into(),
								self.derive_route_init_vector(),
								Vector::default(),
								Vector::default(),
								Some(inter_mediate_args),
								Vector::default(),
								take_string!(self.version),
								take_string!(self.copyright),
								self.license.take(),
							))
						}
						None => {
							//一致するサブコマンドがなかった場合
							match &last_flag_arg {
								MiddleArg::LongFlag(_, FlagValue::None)
								| MiddleArg::ShortFlag(_, FlagValue::None) => {
									//フラグの値になりうる場合
									inter_mediate_args.push_back(last_flag_arg);
									inter_mediate_args.push_back(MiddleArg::Normal(arg));
									self.assign_run(args, inter_mediate_args, p, raw_args, current_path);
								}
								_ => {
									inter_mediate_args.push_back(last_flag_arg);
									inter_mediate_args.push_back(MiddleArg::Normal(arg));
									match self.action {
										Some(action) => {
											//
											let context = Context::build_new(
												raw_args,
												args,
												self.c_flags.take().into(),
												self.l_flags.take(),
												current_path.into(),
												self.derive_route_init_vector(),
												Vector(None),
												Vector(None),
												Some(inter_mediate_args),
												Vector::default(),
												take_string!(self.version),
												take_string!(self.copyright),
												self.license.take(),
											);
											let (mut context, non_flag_args) =
												p.parse_inter_mediate_args(context, false);
											if let Some(mut non_flag_args) = non_flag_args {
												non_flag_args.append(&mut context.args);
												context.args = non_flag_args;
											}

											self.handle_action_result(action(context));
										}
										_ => println!("no action is registered."),
									}
								}
							}
						}
					}
				} else {
					//argがなかった場合
					//self.actionに放り込む
					inter_mediate_args.push_back(last_flag_arg);
					let context = Context::build_new(
						raw_args,
						args,
						self.c_flags.take().into(),
						self.l_flags.take(),
						current_path.into(),
						self.derive_route_init_vector(),
						Vector::default(),
						Vector::default(),
						Some(inter_mediate_args),
						Vector::default(),
						take_string!(self.version),
						take_string!(self.copyright),
						self.license.take(),
					);
					let (mut c, non_flag_args) = p.parse_inter_mediate_args(context, false);
					if let Some(mut non_flag_args) = non_flag_args {
						non_flag_args.append(&mut c.args);
						c.args = non_flag_args;
					}

					match self.action {
						Some(action) => {
							self.handle_action_result(action(c));
						}
						None => {
							println!("no action is registered");
							self.show_help(&c);
						}
					}
				}
			}
			Some(short_flag) if p.flag(&short_flag) => {
				//println!("short_flag: {}", &short_flag);
				//そのままself.runに放り込む
				let (arg, _args, mut _inter_mediate_args, last_flag_arg) =
					p.middle_parse(args, inter_mediate_args, p.short_middle(short_flag));
				//println!("next normal arg: {:?}", arg);
				args = _args;
				inter_mediate_args = _inter_mediate_args;
				if let Some(arg) = arg {
					match self.take_sub(&arg) {
						Some(mut sub) => {
							inter_mediate_args.push_back(last_flag_arg);
							sub.run(Context::build_new(
								raw_args,
								args,
								self.c_flags.take().into(),
								Vector::default(),
								current_path.into(),
								self.derive_route_init_vector(),
								Vector::default(),
								Vector::default(),
								Some(inter_mediate_args),
								Vector::default(),
								take_string!(self.version),
								take_string!(self.copyright),
								self.license.take(),
							))
						}
						None => match &last_flag_arg {
							MiddleArg::LongFlag(_, FlagValue::None)
							| MiddleArg::ShortFlag(_, FlagValue::None) => {
								inter_mediate_args.push_back(last_flag_arg);
								inter_mediate_args.push_back(MiddleArg::Normal(arg));
								self.assign_run(args, inter_mediate_args, p, raw_args, current_path);
							}
							_ => {
								match self.action {
									Some(action) => {
										inter_mediate_args.push_back(last_flag_arg);
										let context = Context::build_new(
											raw_args,
											args,
											self.c_flags.take().into(),
											self.l_flags.take(),
											current_path.into(),
											self.derive_route_init_vector(),
											Vector::default(),
											Vector::default(),
											Some(inter_mediate_args),
											Vector::default(),
											take_string!(self.version),
											take_string!(self.copyright),
											self.license.take(),
										);
										let (mut context, non_flag_args) =
											p.parse_inter_mediate_args(context, false);
										context = p.parse_args_until_end(context);
										if let Some(mut non_flag_args) = non_flag_args {
											non_flag_args.push_back(arg);
											non_flag_args.append(&mut context.args);
											context.args = non_flag_args;
										}
										self.handle_action_result(action(context));
									}
									_ => {
										inter_mediate_args.push_back(last_flag_arg);
										let context = Context::build_new(
											raw_args,
											args,
											self.c_flags.take().into(),
											self.l_flags.take(),
											current_path.into(),
											self.derive_route_init_vector(),
											Vector::default(),
											Vector::default(),
											Some(inter_mediate_args),
											Vector::default(),
											take_string!(self.version),
											take_string!(self.copyright),
											self.license.take(),
										);
										println!("no action registerd");
										self.show_help(&context);
									}
								};
							}
						},
					}
				}
			}
			Some(arg) => {
				//println!("non_flag_arg: {}", &arg);
				//次が普通の引数だった場合サブコマンドか判定
				match self.take_sub(&arg) {
					Some(mut sub) => sub.run(Context::build_new(
						raw_args,
						args,
						self.c_flags.take().into(),
						Vector::default(),
						current_path.into(),
						self.derive_route_init_vector(),
						Vector::default(),
						Vector::default(),
						Some(inter_mediate_args),
						Vector::default(),
						take_string!(self.version),
						take_string!(self.copyright),
						self.license.take(),
					)),
					None => {
						//サブコマンドはないのでそのままselfでaction
						inter_mediate_args.push_back(MiddleArg::Normal(arg));
						let c = Context::build_new(
							raw_args,
							args,
							self.c_flags.take().into(),
							self.l_flags.take(),
							current_path.into(),
							self.derive_route_init_vector(),
							Vector::default(),
							Vector::default(),
							Some(inter_mediate_args),
							Vector(None),
							take_string!(self.version),
							take_string!(self.copyright),
							self.license.take(),
						);

						let (mut c, non_flag_args) = p.parse_inter_mediate_args(c, false);
						c = p.parse_args_until_end(c);
						if let Some(mut non_flag_args) = non_flag_args {
							non_flag_args.append(&mut c.args);
							c.args = non_flag_args;
						}
						match self.action {
							Some(action) => {
								self.handle_action_result(action(c));
							}
							None => {
								println!("no action is registered");
								self.show_help(&c);
							}
						}
					}
				}
			}
			None => {
				//これで終わっている場合の判定
				let context = Context::build_new(
					raw_args,
					args, //argsを使いまわしているが、実質空
					self.c_flags.take().into(),
					self.l_flags.take(),
					current_path.into(),
					self.derive_route_init_vector(),
					Vector::default(),
					Vector::default(),
					Some(inter_mediate_args),
					Vector::default(),
					take_string!(self.version),
					take_string!(self.copyright),
					self.license.take(),
				);
				match self.action {
					Some(action) => {
						let (mut context, non_flag_args) = p.parse_inter_mediate_args(context, false);
						if let Some(non_flag_args) = non_flag_args {
							context.args = non_flag_args;
						}
						self.handle_action_result(action(context));
					}
					None => {
						println!("no action is registered.");
						self.show_help(&context);
					}
				}
			}
		}
	}

	/// Handle action's result (Result<ActionResult, ActionError>).
	///Implemented: show help / show help following show error
	/// アクションの結果であるResult<ActionResult, ActionError>をハンドルする関数。現在はhelp表示もしくはエラーを表示したのちのヘルプ表示のみ
	pub fn handle_action_result(&self, req: Result<ActionResult, ActionError>) {
		match req {
			Ok(ActionResult::ShowHelpRequest(c)) => {
				self.show_help(&c);
				//Ok(ActionResult::Done)
			}
			Err(err) => {
				println!("error: {}", err);
				self.show_help(&err.context);
				//Err(err)
			}
			_ => {
				//Doneの場合 - 何もしない
			}
		};
	}
}
#[cfg(test)]
mod tests {
	use super::super::parser::ParseError;
	use super::super::{Flag, FlagType};
	use super::*;

	fn cnv_arg(mut v: Vec<&str>) -> Vec<String> {
		v.iter_mut().map(|s| s.to_owned()).collect()
	}

	#[test]
	fn single_run() {
		let mut arg = vec![
			"current_path".to_string(),
			"test".to_string(),
			"test".to_string(),
		];
		let mut root = Command::new()
			.action(|c| {
				println!("test_action: {:?}", c);
				let raw_args = vec![
					"current_path".to_string(),
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
				assert_eq!(c.current_path, String::from("current_path"));
				assert_eq!(c.routes, Vector(None));
				return Ok(ActionResult::Done);
			})
			.common_flag(Flag::new(
				"common",
				FlagType::default(),
				"sample common flag",
			))
			.local_flag(Flag::new("local", FlagType::default(), "sample local flag"));
		root.single_run(arg.clone());

		arg.push("--common=C_after".into());
		arg.push("--local=L_after".into());
		arg.insert(1, "--common=C_before".into());
		arg.insert(1, "--local=L_before".into());
		let mut root = Command::with_name("root")
			.action(|c| {
				println!("test_action: {:?}", c);
				let raw_args: Vec<String> = vec![
					"current_path".to_string(),
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
				assert_eq!(c.current_path, String::from("current_path"));
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
				Ok(ActionResult::Done)
			})
			.common_flag(Flag::new(
				"common",
				FlagType::default(),
				"sample common flag",
			))
			.local_flag(Flag::new("local", FlagType::default(), "sample local flag"));

		root.single_run(arg);
	}

	#[test]
	fn run_root() {
		let arg = vec![
			"current_path".to_string(),
			"test".to_string(),
			"test".to_string(),
			"--local".to_string(),
			"test".to_string(),
		];
		let mut root = Command::new()
			.action(|c| {
				println!("test_action: {:?}", c);
				let raw_args = vec![
					"current_path".to_string(),
					"test".to_string(),
					"test".to_string(),
					"--local".to_string(),
					"test".to_string(),
				];
				let expect_args = VecDeque::from(vec!["test".to_string(), "test".to_string()]);
				assert_eq!(c.raw_args, raw_args);
				assert_eq!(c.args, expect_args);
				assert_eq!(c.current_path, String::from("current_path"));
				assert_eq!(
					c.get_local_flag_value_of("local"),
					Some(FlagValue::String("test".into()))
				);
				assert_eq!(
					c.get_flag_value_of("local"),
					Some(FlagValue::String("test".into()))
				);
				assert_eq!(
					c.get_flag_value_of("common"),
					Some(FlagValue::String(String::default()))
				);
				assert_eq!(c.routes, Vector(None));
				Ok(ActionResult::Done)
			})
			.common_flag(Flag::new(
				"common",
				FlagType::default(),
				"sample common flag",
			))
			.local_flag(Flag::new("local", FlagType::default(), "sample local flag"))
			.sub_command(Command::with_name("sub").action(|_| {
				println!("sub");
				Ok(ActionResult::Done)
			}));
		root.run(arg);
	}

	fn base_root() -> Command {
		Command::new()
			.local_flag(Flag::new_string("local").short_alias('l'))
			.local_flag(Flag::new_bool("lafter").short_alias('a'))
			.common_flag(Flag::new_bool("common").short_alias('c'))
			.common_flag(Flag::new_string("cstr").short_alias('s'))
			.common_flag(Flag::new_bool("cafter"))
	}
	#[test]
	fn run_root_with_flag_before_normal_arg() {
		let mut arg = cnv_arg(vec!["current_path", "--local=test"]);
		let root = base_root().sub_command(Command::with_name("sub").action(|c| {
			panic!("not root, in sub: {:?}", c);
		}));
		arg.push("test".into());
		root
			.clone()
			.action(|c| {
				println!("c: {:?}", c);
				assert_eq!(
					cnv_arg(vec!["current_path", "--local=test", "test"]),
					c.raw_args
				);
				assert_eq!(
					c.get_flag_value_of("local").unwrap(),
					FlagValue::String("test".into())
				);
				assert_eq!(c.routes, Vector(None));
				Ok(ActionResult::Done)
			})
			.run(arg.clone());
		arg[2] = "--common".into();
		root
			.clone()
			.action(|c| {
				println!("c: {:?}", c);
				assert_eq!(
					cnv_arg(vec!["current_path", "--local=test", "--common"]),
					c.raw_args
				);
				assert_eq!(
					c.get_flag_value_of("local").unwrap(),
					FlagValue::String("test".into())
				);
				assert_eq!(
					c.get_flag_value_of("common").unwrap(),
					FlagValue::Bool(true)
				);
				assert_eq!(c.routes, Vector(None));
				Ok(ActionResult::Done)
			})
			.run(arg.clone());

		arg.push("test".into());
		root
			.clone()
			.action(|c| {
				println!("{:?}", c);
				assert_eq!(
					cnv_arg(vec!["current_path", "--local=test", "--common", "test"]),
					c.raw_args
				);
				assert_eq!(VecDeque::from(vec!["test".to_string()]), c.args);
				assert_eq!(
					c.get_flag_value_of("local").unwrap(),
					FlagValue::String("test".into())
				);
				assert_eq!(
					c.get_flag_value_of("common").unwrap(),
					FlagValue::Bool(true)
				);
				assert_eq!(c.routes, Vector(None));
				Ok(ActionResult::Done)
			})
			.run(arg.clone());

		println!("arg after flags");
		arg.push("arg".into());
		arg.push("ex_arg".into());
		arg.push("--lafter".into());
		arg.push("--cafter".into());
		root
			.clone()
			.action(|c| {
				println!("{:?}", c);
				assert_eq!(
					cnv_arg(vec![
						"current_path",
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
					c.get_flag_value_of("local").unwrap(),
					FlagValue::String("test".into())
				);
				assert_eq!(
					c.get_flag_value_of("common").unwrap(),
					FlagValue::Bool(true)
				);
				assert_eq!(
					c.get_flag_value_of("cafter").unwrap(),
					FlagValue::Bool(true)
				);
				assert_eq!(
					c.get_flag_value_of("lafter").unwrap(),
					FlagValue::Bool(true)
				);
				assert_eq!(c.routes, Vector(None));
				Ok(ActionResult::Done)
			})
			.run(arg.clone());

		arg.remove(5);
		arg.remove(4);
		arg.insert(5, "arg".into());

		root
			.clone()
			.action(|c| {
				println!("{:?}", c);
				assert_eq!(
					c.raw_args,
					cnv_arg(vec![
						"current_path",
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
					c.get_flag_value_of("local").unwrap(),
					FlagValue::String("test".into())
				);
				assert_eq!(
					c.get_flag_value_of("common").unwrap(),
					FlagValue::Bool(true)
				);
				assert_eq!(
					c.get_flag_value_of("cafter").unwrap(),
					FlagValue::Bool(true)
				);
				assert_eq!(
					c.get_flag_value_of("lafter").unwrap(),
					FlagValue::Bool(true)
				);
				assert_eq!(c.routes, Vector(None));
				Ok(ActionResult::Done)
			})
			.run(arg.clone());

		arg.push("ex_arg".into());
		root
			.clone()
			.action(|c| {
				println!("{:?}", c);
				assert_eq!(
					c.raw_args,
					cnv_arg(vec![
						"current_path",
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
					c.get_flag_value_of("local").unwrap(),
					FlagValue::String("test".into())
				);
				assert_eq!(
					c.get_flag_value_of("common").unwrap(),
					FlagValue::Bool(true)
				);
				assert_eq!(
					c.get_flag_value_of("cafter").unwrap(),
					FlagValue::Bool(true)
				);
				assert_eq!(
					c.get_flag_value_of("lafter").unwrap(),
					FlagValue::Bool(true)
				);
				assert_eq!(c.routes, Vector(None));
				Ok(ActionResult::Done)
			})
			.run(arg.clone());
		arg[4] = "-a".into();
		root
			.clone()
			.action(|c| {
				println!("{:?}", c);
				assert_eq!(
					c.raw_args,
					cnv_arg(vec![
						"current_path",
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
					c.get_flag_value_of("local").unwrap(),
					FlagValue::String("test".into())
				);
				assert_eq!(
					c.get_flag_value_of("common").unwrap(),
					FlagValue::Bool(true)
				);
				assert_eq!(
					c.get_flag_value_of("cafter").unwrap(),
					FlagValue::Bool(true)
				);
				assert_eq!(
					c.get_flag_value_of("lafter").unwrap(),
					FlagValue::Bool(true)
				);
				assert_eq!(c.routes, Vector(None));
				Ok(ActionResult::Done)
			})
			.run(arg.clone());
	}

	#[test]
	fn run_node() {
		let mut arg = cnv_arg(vec![
			"current_path",
			"sub",
			"test",
			"--common",
			"test",
			"--cstr",
			"strt",
			"-b",
			"--local",
		]);
		let root = base_root().action(|c| {
			println!("test_action: {:?}", c);
			panic!("not sub");
		});
		let sub = Command::with_name("sub")
			.local_flag(Flag::new_bool("bool").short_alias('b'))
			.local_flag(Flag::new_string("string").short_alias('s'))
			.common_flag(Flag::new_bool("cl"))
			.sub_command(Command::with_name("leaf").action(|c| {
				println!("Context: {:?}", c);
				panic!("in leaf")
			}));
		root
			.clone()
			.sub_command(sub.clone().action(|c| {
				println!("{:?}", c);
				let raw_args = cnv_arg(vec![
					"current_path",
					"sub",
					"test",
					"--common",
					"test",
					"--cstr",
					"strt",
					"-b",
					"--local",
				]);
				let expect_args = VecDeque::from(vec!["test".to_string(), "test".to_string()]);
				assert_eq!(c.current_path, String::from("current_path"));
				assert_eq!(c.raw_args, raw_args);
				assert_eq!(c.args, expect_args);
				assert_eq!(c.get_flag_value_of("common"), Some(FlagValue::Bool(true)));
				assert_eq!(c.get_flag_value_of("bool").unwrap(), FlagValue::Bool(true));
				assert_eq!(c.get_flag_value_of("commons"), None);
				assert_eq!(c.get_flag_value_of("local"), None);
				assert_eq!(c.routes, "sub".to_owned().into());
				Ok(ActionResult::Done)
			}))
			.run(arg.clone());

		println!("サブコマンド前フラグのテスト");
		arg = cnv_arg(vec!["current_path", "--cstr=test", "-b", "sub"]);
		root
			.clone()
			.name("root")
			.sub_command(sub.clone().action(|c| {
				println!("c: {:?}", c);
				assert_eq!(
					c.raw_args,
					cnv_arg(vec!["current_path", "--cstr=test", "-b", "sub"])
				);
				assert_eq!(
					c.get_flag_value_of("cstr").unwrap(),
					FlagValue::String("test".into())
				);
				assert_eq!(c.get_flag_value_of("bool").unwrap(), FlagValue::Bool(true));
				assert_eq!(
					c.routes,
					Vector(Some(vec!["root".to_string(), "sub".to_string()]))
				);
				Ok(ActionResult::Done)
			}))
			.run(arg.clone());

		println!("サブコマンド探しをする場合");
		arg[1] = "--cstr".into();
		arg.insert(2, "test".into());

		root
			.clone()
			.sub_command(sub.clone().action(|c| {
				println!("c:{:?}", c);
				assert_eq!(
					c.raw_args,
					cnv_arg(vec!["current_path", "--cstr", "test", "-b", "sub"])
				);
				assert_eq!(
					c.get_flag_value_of("cstr").unwrap(),
					FlagValue::String("test".into())
				);
				assert_eq!(c.get_flag_value_of("bool").unwrap(), FlagValue::Bool(true));
				assert_eq!(c.routes, Vector(Some(vec!["sub".to_string()])));
				Ok(ActionResult::Done)
			}))
			.run(arg.clone());

		arg.remove(2);
		arg[1] = "--cstr=test".into();
		arg.push("test_arg".into());
		arg.push("--cafter".into());
		arg.push("test_arg2".into());
		arg.push("--string".into());
		arg.push("testStr".into());
		root
			.clone()
			.sub_command(sub.clone().action(|c| {
				println!("{:?}", c);
				assert_eq!(
					c.raw_args,
					cnv_arg(vec![
						"current_path",
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
					c.get_flag_value_of("cstr").unwrap(),
					FlagValue::String("test".into())
				);
				assert_eq!(c.get_flag_value_of("bool").unwrap(), FlagValue::Bool(true));
				assert_eq!(
					c.get_flag_value_of("cafter").unwrap(),
					FlagValue::Bool(true)
				);
				assert_eq!(c.get_local_flag_value_of("cafter"), None);
				assert_eq!(
					c.get_flag_value_of("string").unwrap(),
					FlagValue::String("testStr".into())
				);
				assert_eq!(c.routes, Vector(Some(vec!["sub".to_string()])));
				Ok(ActionResult::Done)
			}))
			.run(arg.clone());

		println!("\r\n\r\nサブサブコマンドが存在する場合の判別系\r\n");
		arg.remove(4);
		root
			.clone()
			.sub_command(sub.clone().action(|c| {
				println!("c: {:?}", c);
				assert_eq!(
					c.raw_args,
					cnv_arg(vec![
						"current_path",
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
					c.get_local_flag_value_of("string").unwrap(),
					FlagValue::String("testStr".into())
				);
				assert_eq!(
					c.get_inputted_common_flag_value_of("cafter").unwrap(),
					FlagValue::Bool(true)
				);
				assert_eq!(c.routes, Vector(Some(vec!["sub".to_string()])));
				Ok(ActionResult::Done)
			}))
			.run(arg.clone());
		println!("\r\n\r\nサブサブコマンドが存在する場合の判別系その2\r\n");
		arg.push("ex_arg".into());
		arg[5] = "test_arg".to_owned();

		root
			.clone()
			.sub_command(sub.clone().action(|c| {
				println!("C: {:?}", c);
				assert_eq!(
					c.raw_args,
					cnv_arg(vec![
						"current_path",
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
				Ok(ActionResult::Done)
			}))
			.run(arg.clone());
		arg[6] = "--string=testStr".into();
		arg[8] = "test_arg2".into();
		arg.remove(7);
		arg.push("test_arg3".into());
		arg.push("--common".into());
		arg.push("test_arg4".into());

		root
			.clone()
			.sub_command(sub.clone().action(|c| {
				println!("C: {:?}", c);
				assert_eq!(
					c.raw_args,
					cnv_arg(vec![
						"current_path",
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
					c.get_local_flag_value_of("bool").unwrap(),
					FlagValue::Bool(true)
				);
				assert_eq!(
					c.get_common_flag_value_of("cafter").unwrap(),
					FlagValue::Bool(true)
				);
				assert_eq!(
					c.get_flag_value_of("string").unwrap(),
					FlagValue::String("testStr".into())
				);
				assert_eq!(c.routes, Vector(Some(vec!["sub".to_string()])));
				Ok(ActionResult::Done)
			}))
			.run(arg.clone());

		arg.pop();
		arg.remove(8);
		arg.remove(7);
		arg.remove(5);

		root
			.clone()
			.sub_command(sub.clone().action(|c| {
				println!("c: {:?}", c);
				assert_eq!(
					c.raw_args,
					cnv_arg(vec![
						"current_path",
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
					c.get_flag_value_of("cstr").unwrap(),
					FlagValue::String("test".into())
				);
				assert_eq!(c.get_flag_value_of("bool").unwrap(), FlagValue::Bool(true));
				assert_eq!(
					c.get_flag_value_of("cafter").unwrap(),
					FlagValue::Bool(true)
				);
				assert_eq!(
					c.get_flag_value_of("string").unwrap(),
					FlagValue::String("testStr".into())
				);
				assert_eq!(
					c.get_flag_value_of("common").unwrap(),
					FlagValue::Bool(true)
				);
				assert_eq!(c.routes, Vector(Some(vec!["sub".to_string()])));
				Ok(ActionResult::Done)
			}))
			.run(arg.clone());
	}

	#[test]
	fn run_leaf() {
		let arg = vec![
			"current_path".to_string(),
			"sub".to_string(),
			"leaf".to_string(),
			//"test".to_string(),
			"--common".to_string(),
			"test".to_string(),
			"-c".to_string(),
			"--local".to_string(),
		];
		let mut root = Command::new()
			.action(|c| {
				println!("test_action: {:?}", c);
				panic!("not sub");
			})
			.common_flag(Flag::new("common", FlagType::String, "sample common flag"))
			.common_flag(Flag::with_name("cshort").short_alias('c'))
			.local_flag(Flag::new("local", FlagType::default(), "sample local flag"))
			.sub_command(
				Command::with_name("sub")
					.action(|c| {
						panic!("sub: {:?}", c);
					})
					.sub_command(
						Command::with_name("leaf")
							.action(|c| {
								let raw_args = vec![
									"current_path".to_string(),
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
								assert_eq!(c.current_path, String::from("current_path"));
								assert_eq!(c.raw_args, raw_args);
								assert_eq!(c.args, expect_args);
								assert_eq!(
									c.get_flag_value_of("common"),
									Some(FlagValue::String("test".into()))
								);
								assert_eq!(
									c.get_inputted_flag_value_of("cshort").unwrap(),
									FlagValue::None
								);
								assert_eq!(
									c.get_flag_value_of("cshort").unwrap(),
									FlagValue::String("".into())
								);
								assert_eq!(c.get_flag_value_of("local").unwrap(), FlagValue::Bool(true));
								assert_eq!(c.get_common_flag_value_of("local"), None);
								assert_eq!(
									c.routes,
									Vector(Some(vec!["sub".to_string(), "leaf".to_string()]))
								);
								Ok(ActionResult::Done)
							})
							.local_flag(Flag::new_bool("local").short_alias('l')),
					),
			);
		root.run(arg.clone());
	}

	#[test]
	fn run_leaf_with_flag_before_normal_flag() {
		let root = base_root().action(|c| {
			panic!("root action: {:?} - not leaf", c);
		});
		let sub = Command::with_name("sub")
			.local_flag(Flag::new_bool("sub_local"))
			.local_flag(Flag::new_string("sub_lstr"))
			.common_flag(Flag::new_bool("sub_common"))
			.local_flag(Flag::new_string("sub_cstr"))
			.action(|c| {
				panic!("sub action: {:?} - not leaf", c);
			});
		let leaf = Command::with_name("leaf")
			.common_flag(Flag::new_bool("cbool"))
			.common_flag(Flag::new_string("cs"))
			.local_flag(Flag::new_bool("lbool").short_alias('b'))
			.local_flag(Flag::new_string("lsafter").short_alias('a'))
			.local_flag(Flag::new_string("lsbefore").short_alias('s'));

		let run_leaf: fn(Command, Command, Command, Action, Vec<String>) -> () =
			|root, sub, leaf, action, args| {
				root
					.sub_command(sub.sub_command(leaf.action(action)))
					.run(args)
			};

		let mut args = cnv_arg(vec!["current_path", "--lbool", "sub", "--lsbefore", "leaf"]);

		run_leaf(
			root.clone().name("root"),
			sub.clone(),
			leaf.clone(),
			|c| {
				println!("{:?}", c);
				assert_eq!(
					c.raw_args,
					cnv_arg(vec!["current_path", "--lbool", "sub", "--lsbefore", "leaf"])
				);
				assert_eq!(c.args, VecDeque::new());
				assert_eq!(c.get_flag_value_of("lbool").unwrap(), FlagValue::Bool(true));
				assert_eq!(
					c.get_flag_value_of("lsbefore").unwrap(),
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
				Ok(ActionResult::Done)
			},
			args.clone(),
		);

		args.push("arg".into());

		run_leaf(
			root.clone(),
			sub.clone(),
			leaf.clone(),
			|c| {
				println!("{:?}", c);
				assert_eq!(
					c.raw_args,
					cnv_arg(vec![
						"current_path",
						"--lbool",
						"sub",
						"--lsbefore",
						"leaf",
						"arg"
					])
				);
				assert_eq!(c.args, VecDeque::from(vec![String::from("arg")]));
				assert_eq!(c.get_flag_value_of("lbool").unwrap(), FlagValue::Bool(true));
				assert_eq!(
					c.get_flag_value_of("lsbefore").unwrap(),
					FlagValue::String("".into())
				);
				assert_eq!(
					c.routes,
					Vector(Some(vec!["sub".to_owned(), "leaf".to_owned()]))
				);
				Ok(ActionResult::Done)
			},
			args.clone(),
		);
		args.push("--cbool".into());
		run_leaf(
			root.clone(),
			sub.clone(),
			leaf.clone(),
			|c| {
				println!("{:?}", c);
				assert_eq!(
					c.raw_args,
					cnv_arg(vec![
						"current_path",
						"--lbool",
						"sub",
						"--lsbefore",
						"leaf",
						"arg",
						"--cbool"
					])
				);
				assert_eq!(c.get_flag_value_of("lbool").unwrap(), FlagValue::Bool(true));
				assert_eq!(c.args, VecDeque::from(vec!["arg".to_string()]));
				assert_eq!(
					c.get_flag_value_of("lsbefore").unwrap(),
					FlagValue::String("".into())
				);
				assert_eq!(c.get_flag_value_of("cbool").unwrap(), FlagValue::Bool(true));
				assert_eq!(
					c.routes,
					Vector(Some(vec!["sub".to_owned(), "leaf".to_owned()]))
				);
				Ok(ActionResult::Done)
			},
			args.clone(),
		);
		args.pop();
		args.insert(4, "before_arg".into());

		run_leaf(
			root.clone(),
			sub.clone(),
			leaf.clone(),
			|c| {
				println!("{:?}", c);
				assert_eq!(
					c.raw_args,
					cnv_arg(vec![
						"current_path",
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
				assert_eq!(c.get_flag_value_of("lbool").unwrap(), FlagValue::Bool(true));
				assert_eq!(
					c.get_flag_value_of("lsbefore").unwrap(),
					FlagValue::String("before_arg".into())
				);
				assert_eq!(
					c.routes,
					Vector(Some(vec!["sub".to_owned(), "leaf".to_owned()]))
				);
				Ok(ActionResult::Done)
			},
			args.clone(),
		);
	}

	#[test]
	fn test_flag_type() {
		let arg = vec![
			"current_path".to_string(),
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
			.action(|c| {
				println!("sub_action: {:?}", c);
				let raw_args = vec![
					"current_path".to_string(),
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
				assert_eq!(c.current_path, String::from("current_path"));
				assert_eq!(c.raw_args, raw_args);
				assert_eq!(c.args, expect_args);
				assert_eq!(
					c.get_flag_value_of("common"),
					Some(FlagValue::String("test".into()))
				);
				assert_eq!(
					c.get_inputted_flag_value_of("commons"),
					Some(FlagValue::None)
				);
				assert_eq!(
					c.get_flag_value_of("commons"),
					Some(FlagValue::String("".into()))
				);
				assert_eq!(c.get_flag_value_of("local"), None);
				assert_eq!(c.get_inputted_common_flag_value_of("yes"), None);
				assert_eq!(
					c.get_local_flag_value_of("yes"),
					Some(FlagValue::Bool(true))
				);
				assert_eq!(c.get_flag_value_of("yes"), Some(FlagValue::Bool(true)));
				let expect_error_args = {
					let mut vd = VecDeque::new();
					vd.push_back(MiddleArg::LongFlag("local".into(), FlagValue::None));
					vd
				};
				assert_eq!(c.get_flag_value_of("int"), Some(FlagValue::Int(111)));
				assert_eq!(
					c.get_flag_value_of("float"),
					Some(FlagValue::Float(10.into()))
				);
				assert_eq!(c.parsing_args.unwrap(), expect_error_args);
				assert_eq!(
					c.error_info_list,
					Vector::from(vec![(
						MiddleArg::LongFlag("local".into(), FlagValue::None),
						ParseError::NoExistLong,
						ParseError::NoExistLong
					)])
				);
				Ok(ActionResult::Done)
			})
			.local_flag(Flag::new_bool("yes").short_alias('y'))
			.local_flag(Flag::new_int("int").short_alias('i'))
			.local_flag(Flag::new_float("float").short_alias('f'));
		let mut root = Command::new()
			.action(|c| {
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
					.action(|c| {
						println!("sub: {:?}", c);
						panic!("not leaf");
					})
					.sub_command(leaf),
			);
		root.run(arg.clone());
	}

	/*#[test]
	fn help_test() {
		let root = base_root()
			.usage("root usage")
			.sub_command(Command::with_name("aaa").usage("aaa usage"))
			.sub_command(Command::with_name("bbb").usage("bbb usage"));
		assert_eq!(String::new(), root.default_help());
		panic!("panic");
	}*/
}

/*mod presets {
	use super::{Command, Vector};
}*/
