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
		//println!("{:?}", args);
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
						&current_path,
					);
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

	pub fn show_help(self) {}

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

	pub fn take_sub(&mut self, name_or_alias: &str) -> Option<Command> {
		match self.sub.take() {
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
		let mut args = VecDeque::from(raw_args.clone());
		let current_path = args.pop_front().unwrap();
		let head = args.pop_front();
		if head.is_none() {
			//引数がない場合
			match self.action {
				Some(action) => {
					action(Context::from(raw_args));
				}
				None => {
					println!("args: {:?}", raw_args);
				}
			}
		} else {
			//get before first non-flag arg with parsing flags
			let p = Parser::default();
			match head {
				Some(long_flag) if p.long_flag(&long_flag) => {
					//long flag
					let (arg, mut args, mut inter_mediate_args, last_flag_arg) =
						p.middle_parse(args, VecDeque::new(), p.long_middle(long_flag));
					//最初にフラグの形になっていない引数を求める
					if let Some(arg) = arg {
						match self.take_sub(&arg) {
							//サブコマンド合致
							Some(mut sub) => {
								let context = Context::build_new(
									raw_args,
									args,
									self.c_flags.take(),
									Vector::default(),
									std::path::PathBuf::from(current_path),
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
									MiddleArg::LongFlag(name, FlagValue::None) => {
										//検出したものがフラグの値になる可能性がある場合のハンドリング

										let mut non_flag_args = {
											let mut vd = VecDeque::new();
											vd.push_back(arg);
											vd
										};
										loop {
											//前のループ、あるいはループに入る前にサブコマンドが特定できなかった場合
											match args.pop_front() {
												Some(long_flag) if p.long_flag(&long_flag) => {
													let (arg, _args, inter_mediate_args, last_flag_arg) = p
														.middle_parse(
															args,
															VecDeque::new(),
															p.long_middle(long_flag),
														);
													args = _args;
													if let Some(arg) = arg {
														//
													} else {
														//最後になったのでself.runに放り込む
														let context = Context::build_new(
															raw_args,
															non_flag_args,
															self.c_flags.take(),
															None.into(),
															current_path.into(),
															None.into(),
															None.into(),
															Some(inter_mediate_args),
															None.into(),
														);
														break self.run_with_context(context);
													}
												}
												Some(short_flag) if p.flag(&short_flag) => {
													//
												}
												Some(arg) => {
													//次の引数を判定
													match self.take_sub(&arg) {
														Some(mut sub) => {
															inter_mediate_args.push_back(
																last_flag_arg.set_val(FlagValue::String(arg)),
															);
															let context = Context::build_new(
																raw_args,
																args,
																self.c_flags.take(),
																Vector::default(),
																std::path::PathBuf::from(current_path),
																Vector::default(),
																Vector::default(),
																Some(inter_mediate_args),
																Vector::default(),
															);
															break sub.run(context);
														}
														None => {
															//
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

															if let Some(action) = self.action {
																break action(context);
															}
															break;
														}
													}
												}
												None => {
													//
												}
											}
										}
									}
									MiddleArg::ShortFlag(name, FlagValue::None) => {
										//
									}
									_ => {
										//
										inter_mediate_args.push_back(last_flag_arg);
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
										self.run_with_context(context);
									}
								}
							}
						}
					} else {
						//Noneの場合、そのままself.runに放り込む
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

						self.run(context);
					}
				}
				Some(short_flag) if p.flag(&short_flag) => {
					//short flag
					p.middle_parse(args, VecDeque::new(), p.short_middle(short_flag));
				}
				Some(arg) => {
					//non-flag (normal) arg
					println!("arg: {}", &arg);
					//let common_flag = self.c_flags.take();
					match self.take_sub(&arg) {
						None => match self.action {
							None => println!("{} does not have its own action.", self.name),
							Some(action) => {
								let c = Context::new(
									raw_args,
									args,
									self.c_flags.take(),
									self.l_flags.take(),
									&current_path,
								);
								action(c);
							}
						},
						Some(mut sub) => {
							let common_flag = self.c_flags.take();
							let c = Context::new(raw_args, args, common_flag, Vector(None), &current_path);
							sub.run(c);
						}
					}
				}
				_ => {
					panic!("unexpected error");
				}
			}
		}
	}

	pub fn run_with_context(&mut self, context: Context) {
		println!("{:?}", context);
	}
}
