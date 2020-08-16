use crate::Action;

use crate::parser::MiddleArg;
use crate::Context;
use crate::Parser;
use crate::Vector;
use crate::{Flag, FlagValue};

use std::collections::VecDeque;

#[derive(Clone, Default)]
pub struct Command {
	pub name: String,
	pub action: Option<Action>,
	pub authors: String,
	pub copyright: String,
	pub description: Option<String>,
	pub usage: String,
	pub l_flags: Vector<Flag>,
	pub c_flags: Vector<Flag>,
	pub alias: Vector<String>,
	pub version: String,
	pub sub: Vector<Command>,
	pub opt_values: Vector<KeyValuePair>,
}

pub type KeyValuePair = (String, String);

impl Command {
	pub fn new() -> Command {
		Command::default()
	}

	pub fn with_name<T: Into<String>>(name: T) -> Command {
		Command {
			name: name.into(),
			action: None,
			authors: String::default(),
			copyright: String::default(),
			description: Option::default(),
			usage: String::default(),
			l_flags: Vector::default(),
			c_flags: Vector::default(),
			alias: Vector::default(),
			version: String::default(),
			sub: Vector::default(),
			opt_values: Vector::default(),
		}
	}

	#[allow(clippy::too_many_arguments)]
	pub fn build_new(
		name: String,
		action: Option<Action>,
		authors: String,
		copyright: String,
		description: Option<String>,
		usage: String,
		local_flags: Vector<Flag>,
		common_flags: Vector<Flag>,
		alias: Vector<String>,
		version: String,
		sub: Vector<Command>,
		opt_values: Vector<(String, String)>,
	) -> Command {
		Command {
			name,
			action,
			authors,
			copyright,
			description,
			usage,
			l_flags: local_flags,
			c_flags: common_flags,
			alias,
			version,
			sub,
			opt_values,
		}
	}

	pub fn run_with_auto_arg_collect(mut self) {
		//let args: Vec<String> = std::env::args().collect();
		//self.run(args);
		match &self.sub {
			Vector(None) => self.single_run(std::env::args().collect::<Vec<String>>()),
			_ => self.run(std::env::args().collect::<Vec<String>>()),
		};
	}

	pub fn single_run(&mut self, raw_args: Vec<String>) {
		println!("single_run: {:?}", raw_args);
		match self.action.take() {
			Some(action) => {
				if raw_args.len() < 2 {
					action(Context::from(raw_args))
				} else {
					let mut args = VecDeque::from(raw_args.clone());
					let current_path = args.pop_front().unwrap();
					let mut context = Context::new(
						raw_args,
						args,
						self.c_flags.take(),
						self.l_flags.take(),
						current_path,
					);
					println!("single_run_context: {:?}", context);
					context = Parser::default().parse_args_until_end(context);

					action(context)
				}
			}
			None => match self.sub {
				Vector(None) => println!("no action is registerd."),
				_ => self.run(raw_args),
			},
		}
	}

	pub fn show_help(&self) {
		//TODO: implement help function
		println!("show_help_function");
	}

	pub fn name<T: Into<String>>(mut self, name: T) -> Command {
		self.name = name.into();
		self
	}

	pub fn usage<T: Into<String>>(mut self, usage: T) -> Self {
		self.usage = usage.into();
		self
	}

	pub fn action(mut self, action: Action) -> Self {
		self.action = Some(action);
		self
	}

	pub fn authors<T: Into<String>>(mut self, authors: T) -> Self {
		self.authors = authors.into();
		self
	}

	pub fn copyright<T: Into<String>>(mut self, copyright: T) -> Self {
		self.copyright = copyright.into();
		self
	}

	pub fn local_flag(mut self, flag: Flag) -> Self {
		self.l_flags.push(flag);
		self
	}

	pub fn common_flag(mut self, flag: Flag) -> Self {
		self.c_flags.push(flag);
		self
	}

	pub fn desctiption<T: Into<String>>(mut self, description: T) -> Self {
		self.description = Some(description.into());
		self
	}

	pub fn version<T: Into<String>>(mut self, version: T) -> Self {
		self.version = version.into();
		self
	}

	pub fn sub_command(mut self, sub_command: Command) -> Self {
		self.sub.push(sub_command);
		self
	}

	pub fn alias<T: Into<String>>(mut self, a: T) -> Self {
		self.alias.push(a.into());
		self
	}

	pub fn add_opt_prop(mut self, opt_prop: KeyValuePair) -> Self {
		self.opt_values.push(opt_prop);
		self
	}

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

	///name_or_aliasに一致するサブコマンドがある場合、保持しているVectorからswap_removeで取り出して返す
	pub fn take_sub(&mut self, name_or_alias: &str) -> Option<Command> {
		match self.sub {
			Vector(None) => None,
			Vector(Some(ref mut inner)) => match inner.into_iter().position(|c| c.is(name_or_alias)) {
				None => return None,
				Some(index) => Some(inner.swap_remove(index)),
			},
		}
	}

	pub fn has_sub(&self) -> bool {
		self.sub.has_inner_vec()
	}
}

impl From<String> for Command {
	fn from(name: String) -> Self {
		Command {
			name,
			action: None,
			authors: String::default(),
			copyright: String::default(),
			description: None,
			usage: String::default(),
			l_flags: Vector::default(),
			c_flags: Vector::default(),
			alias: Vector::default(),
			version: String::default(),
			sub: Vector::default(),
			opt_values: Vector::default(),
		}
	}
}

pub trait Run<T> {
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
	pub fn run_from_args(&mut self, raw_args: Vec<String>) {
		println!("{:?}, len: {}", &raw_args, &raw_args.len());
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
					action(Context::new(
						raw_args,
						args,
						self.c_flags.take(),
						self.l_flags.take(),
						current_path,
					));
				}
				None => {
					println!("no action is registered.");
					println!("args: {:?}", raw_args);
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
					println!(
						"\r\nmiddle_result at 281 {:?}\r\n",
						(&arg, &args, &inter_mediate_args, &last_flag_arg)
					);
					if let Some(arg) = arg {
						match self.take_sub(&arg) {
							//サブコマンド合致
							Some(mut sub) => {
								inter_mediate_args.push_back(last_flag_arg);
								let context = Context::build_new(
									raw_args,
									args,
									self.c_flags.take(),
									Vector::default(),
									current_path.into(),
									Vector::default(),
									Vector::default(),
									Some(inter_mediate_args),
									Vector::default(),
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
											self.c_flags.take(),
											self.l_flags.take(),
											std::path::PathBuf::from(current_path),
											Vector::default(),
											Vector::default(),
											Some(inter_mediate_args),
											Vector::default(),
										);
										let (mut context, non_flag_arg) =
											p.parse_inter_mediate_args(context, false);
										if let Some(mut non_flag_arg) = non_flag_arg {
											non_flag_arg.append(&mut context.args);
											context.args = non_flag_arg;
										}
										match self.action {
											Some(action) => {
												action(context);
											}
											None => {
												println!("no action registered");
												self.show_help();
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
							self.c_flags.take(),
							self.l_flags.take(),
							current_path.into(),
							Vector(None),
							Vector(None),
							Some({
								inter_mediate_args.push_back(last_flag_arg);
								inter_mediate_args
							}),
							Vector(None),
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
								action(context);
							}
							_ => {
								println!("no action registered");
								self.show_help();
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
									self.c_flags.take(),
									Vector(None),
									current_path.into(),
									Vector(None),
									Vector(None),
									Some(inter_mediate_args),
									Vector(None),
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
										self.c_flags.take(),
										self.l_flags.take(),
										current_path.into(),
										Vector(None),
										Vector(None),
										Some(inter_mediate_args),
										Vector(None),
									);
									let (mut context, non_flag_args) =
										p.parse_inter_mediate_args(context, false);
									if let Some(mut non_flag_args) = non_flag_args {
										non_flag_args.append(&mut context.args);
										context.args = non_flag_args;
									}
									match self.action {
										Some(action) => {
											action(context);
										}
										None => {
											println!("no action registered");
											self.show_help();
										}
									}
								}
							},
						}
					}
				}
				Some(arg) => {
					//first arg is non-flag (normal) arg
					println!("arg: {}", &arg);

					match self.take_sub(&arg) {
						None => match self.action {
							None => println!("{} does not have its own action.", self.name),
							Some(action) => {
								args.push_front(arg);
								let mut c = Context::new(
									raw_args,
									args,
									self.c_flags.take(),
									self.l_flags.take(),
									current_path,
								);
								c = p.parse_args_until_end(c);
								action(c);
							}
						},
						Some(mut sub) => {
							println!("{}", sub.name);
							let common_flag = self.c_flags.take();
							let c = Context::new(raw_args, args, common_flag, Vector(None), current_path);
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

	pub fn run_with_context(&mut self, mut context: Context) {
		//println!("run_with_context: {:?}", context);
		if self.sub.is_none() {
			context.local_flags = self.l_flags.take();
			context.common_flags = {
				let mut taken = self.c_flags.take();
				taken.append(context.common_flags);
				taken
			};
			let p = Parser::default();
			let (mut context, non_flag_args) = p.parse_inter_mediate_args(context, true);
			if let Some(mut non_flag_args) = non_flag_args {
				non_flag_args.append(&mut context.args);
				context.args = non_flag_args;
			}
			context = p.parse_args_until_end(context);
			match self.action {
				Some(action) => {
					action(context);
				}
				None => println!("no action is registered"),
			}
		} else {
			//サブコマンドと一致するかを捜査
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
					println!("arg sub-sub: {}", &arg);
					match self.take_sub(&arg) {
						Some(mut sub) => {
							println!("{}", &sub.name);
							context.common_flags = {
								let mut taken = self.c_flags.take();
								taken.append(context.common_flags);
								taken
							};
							sub.run(context);
						}
						None => {
							context.common_flags = {
								let mut taken = self.c_flags.take();
								taken.append(context.common_flags);
								taken
							};
							context.local_flags = self.l_flags.take();
							match self.action {
								None => {
									println!("{} does not have its own action.", &self.name);
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
									action(c);
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
							context.common_flags = {
								let mut taken = self.c_flags.take();
								taken.append(context.common_flags);
								taken
							};

							let (mut context, non_flag_args) = p.parse_inter_mediate_args(context, true);
							if let Some(mut non_flag_args) = non_flag_args {
								non_flag_args.append(&mut context.args);
								context.args = non_flag_args;
							}
							action(context);
						}
						None => {
							println!("no action is registered.");
						}
					}
				}
			}
		}
	}

	pub fn assign_context(
		&mut self,
		mut c: Context,
		p: Parser,
		mut inter_mediate_args: VecDeque<MiddleArg>,
		last: MiddleArg,
	) {
		println!(
			"\r\nassign_context:::\r\nContext: {:?}, \r\ninter_mediate_args: {:?},\r\n prefix: {:?}\r\n",
			c, inter_mediate_args, last
		);
		let (next_non_flag, args, _inter_mediate_args, last) =
			p.middle_parse(c.args, inter_mediate_args, last);
		inter_mediate_args = _inter_mediate_args;
		println!("next_non_flag: {:?}", next_non_flag);
		match next_non_flag {
			Some(arg) => match self.take_sub(&arg) {
				Some(mut sub) => {
					c.args = args;
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
								Some(mut sub) => sub.run(c),
								None => match self.action {
									Some(action) => {
										if let Some(mut parsing_args) = c.parsing_args {
											parsing_args.append(&mut inter_mediate_args);
											c.parsing_args = Some(parsing_args);
										} else {
											c.parsing_args = Some(inter_mediate_args);
										}
										c.local_flags = self.l_flags.take();
										let (mut c, non_flag_args) = p.parse_inter_mediate_args(c, false);
										c = p.parse_args_until_end(c);
										c.args.push_front(arg);
										if let Some(mut non_flag_args) = non_flag_args {
											non_flag_args.append(&mut c.args);
											c.args = non_flag_args;
										}

										action(c);
									}
									None => {
										println!("no action registered.");
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
									action(c);
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
							action(c);
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
						let (mut c, non_flag_args) = p.parse_inter_mediate_args(c, false);
						if let Some(non_flag_args) = non_flag_args {
							//non_flag_args.append(&mut c.args);
							c.args = non_flag_args;
						}
						action(c);
					}
					None => {
						println!("no action is registered.");
					}
				}
			}
		}
	}

	pub fn assign_run(
		&mut self,
		mut args: VecDeque<String>,
		mut inter_mediate_args: VecDeque<MiddleArg>,
		p: Parser,
		raw_args: Vec<String>,
		current_path: String,
	) {
		println!("assign_run");
		match args.pop_front() {
			Some(long_flag) if p.long_flag(&long_flag) => {
				println!("long_flag: {}", &long_flag);
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
								self.c_flags.take(),
								None.into(),
								current_path.into(),
								None.into(),
								None.into(),
								Some(inter_mediate_args),
								None.into(),
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
												self.c_flags.take(),
												self.l_flags.take(),
												current_path.into(),
												None.into(),
												None.into(),
												Some(inter_mediate_args),
												None.into(),
											);
											let (mut context, non_flag_args) =
												p.parse_inter_mediate_args(context, false);
											if let Some(mut non_flag_args) = non_flag_args {
												non_flag_args.append(&mut context.args);
												context.args = non_flag_args;
											}

											action(context)
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
						self.c_flags.take(),
						self.l_flags.take(),
						current_path.into(),
						Vector::default(),
						Vector::default(),
						Some(inter_mediate_args),
						Vector::default(),
					);
					let (mut c, non_flag_args) = p.parse_inter_mediate_args(context, false);
					if let Some(mut non_flag_args) = non_flag_args {
						non_flag_args.append(&mut c.args);
						c.args = non_flag_args;
					}

					match self.action {
						Some(action) => {
							action(c);
						}
						None => {
							println!("no action is registered");
							self.show_help();
						}
					}
				}
			}
			Some(short_flag) if p.flag(&short_flag) => {
				println!("short_flag: {}", &short_flag);
				//そのままself.runに放り込む
				let (arg, _args, mut _inter_mediate_args, last_flag_arg) =
					p.middle_parse(args, inter_mediate_args, p.short_middle(short_flag));
				println!("next normal arg: {:?}", arg);
				args = _args;
				inter_mediate_args = _inter_mediate_args;
				if let Some(arg) = arg {
					match self.take_sub(&arg) {
						Some(mut sub) => {
							inter_mediate_args.push_back(last_flag_arg);
							sub.run(Context::build_new(
								raw_args,
								args,
								self.c_flags.take(),
								Vector::default(),
								current_path.into(),
								Vector::default(),
								Vector::default(),
								Some(inter_mediate_args),
								Vector::default(),
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
											self.c_flags.take(),
											self.l_flags.take(),
											current_path.into(),
											Vector::default(),
											Vector::default(),
											Some(inter_mediate_args),
											Vector::default(),
										);
										let (mut context, non_flag_args) =
											p.parse_inter_mediate_args(context, false);
										context = p.parse_args_until_end(context);
										if let Some(mut non_flag_args) = non_flag_args {
											non_flag_args.push_back(arg);
											non_flag_args.append(&mut context.args);
											context.args = non_flag_args;
										}
										action(context);
									}
									_ => {
										println!("no action registerd");
										self.show_help();
									}
								};
							}
						},
					}
				}
			}
			Some(arg) => {
				println!("non_flag_arg: {}", &arg);
				//次が普通の引数だった場合サブコマンドか判定
				match self.take_sub(&arg) {
					Some(mut sub) => sub.run(Context::build_new(
						raw_args,
						args,
						self.c_flags.take(),
						None.into(),
						current_path.into(),
						None.into(),
						None.into(),
						Some(inter_mediate_args),
						None.into(),
					)),
					None => {
						//サブコマンドはないのでそのままselfでaction
						inter_mediate_args.push_back(MiddleArg::Normal(arg));
						let c = Context::build_new(
							raw_args,
							args,
							self.c_flags.take(),
							self.l_flags.take(),
							current_path.into(),
							Vector(None),
							Vector(None),
							Some(inter_mediate_args),
							Vector(None),
						);

						let (mut c, non_flag_args) = p.parse_inter_mediate_args(c, false);
						c = p.parse_args_until_end(c);
						if let Some(mut non_flag_args) = non_flag_args {
							non_flag_args.append(&mut c.args);
							c.args = non_flag_args;
						}
						match self.action {
							Some(action) => {
								action(c);
							}
							None => {
								println!("no action is registered");
								self.show_help();
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
					self.c_flags.take(),
					self.l_flags.take(),
					current_path.into(),
					Vector(None),
					Vector(None),
					Some(inter_mediate_args),
					Vector(None),
				);
				match self.action {
					Some(action) => {
						let (mut context, non_flag_args) = p.parse_inter_mediate_args(context, false);
						if let Some(non_flag_args) = non_flag_args {
							context.args = non_flag_args;
						}
						action(context);
					}
					None => {
						println!("no action is registered.");
						self.show_help();
					}
				}
			}
		}
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
				assert_eq!(c.current_path, std::path::PathBuf::from("current_path"));
			})
			.common_flag(Flag::new(
				"common",
				"sample common flag",
				FlagType::default(),
			))
			.local_flag(Flag::new("local", "sample local flag", FlagType::default()));
		root.single_run(arg.clone());

		arg.push("--common=C_after".into());
		arg.push("--local=L_after".into());
		arg.insert(1, "--common=C_before".into());
		arg.insert(1, "--local=L_before".into());
		let mut root = Command::new()
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
				assert_eq!(c.current_path, std::path::PathBuf::from("current_path"));
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
			})
			.common_flag(Flag::new(
				"common",
				"sample common flag",
				FlagType::default(),
			))
			.local_flag(Flag::new("local", "sample local flag", FlagType::default()));

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
				assert_eq!(c.current_path, std::path::PathBuf::from("current_path"));
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
			})
			.common_flag(Flag::new(
				"common",
				"sample common flag",
				FlagType::default(),
			))
			.local_flag(Flag::new("local", "sample local flag", FlagType::default()))
			.sub_command(Command::with_name("sub").action(|_| println!("sub")));
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
				assert_eq!(c.current_path, std::path::PathBuf::from("current_path"));
				assert_eq!(c.raw_args, raw_args);
				assert_eq!(c.args, expect_args);
				assert_eq!(c.get_flag_value_of("common"), Some(FlagValue::Bool(true)));
				assert_eq!(c.get_flag_value_of("bool").unwrap(), FlagValue::Bool(true));
				assert_eq!(c.get_flag_value_of("commons"), None);
				assert_eq!(c.get_flag_value_of("local"), None);
			}))
			.run(arg.clone());

		println!("サブコマンド前フラグのテスト");
		arg = cnv_arg(vec!["current_path", "--cstr=test", "-b", "sub"]);
		root
			.clone()
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
			.common_flag(Flag::new("common", "sample common flag", FlagType::String))
			.common_flag(Flag::with_name("cshort").short_alias('c'))
			.local_flag(Flag::new("local", "sample local flag", FlagType::default()))
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
								assert_eq!(c.current_path, std::path::PathBuf::from("current_path"));
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
							})
							.local_flag(Flag::new_bool("local").short_alias('l')),
					),
			);
		root.run(arg.clone());
	}

	#[test]
	fn run_leaf_with_flag_before_normal_flag() {
		//
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
				assert_eq!(c.current_path, std::path::PathBuf::from("current_path"));
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
				"sample common flag",
				FlagType::default(),
			))
			.common_flag(Flag::with_name("commons").short_alias('c'))
			.common_flag(Flag::new_string("yes").short_alias('y'))
			.local_flag(Flag::new("local", "sample local flag", FlagType::default()))
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
}
