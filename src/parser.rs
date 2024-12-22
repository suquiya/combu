use crate::{
	vector::flag::{FlagSearch, LongFound},
	Context, FlagValue,
};
use std::collections::VecDeque;

/// Struct of information for parse
pub struct Parser {
	/// flag_pattern. Default is '-'.
	pub flag_pattern: char,
	/// Long-flag pre&&fix. Default is "--".
	pub long_flag_prefix: String,
	/// equal symbol. Default is "="
	pub eq: char,
}

impl Default for Parser {
	fn default() -> Self {
		Parser {
			flag_pattern: '-',
			long_flag_prefix: String::from("--"),
			eq: '=',
		}
	}
}

impl From<char> for Parser {
	fn from(flag_pattern: char) -> Self {
		Parser {
			flag_pattern,
			long_flag_prefix: flag_pattern.to_string().repeat(2),
			eq: '=',
		}
	}
}

impl From<(char, char)> for Parser {
	fn from((flag_pattern, eq): (char, char)) -> Self {
		Parser {
			flag_pattern,
			long_flag_prefix: flag_pattern.to_string().repeat(2),
			eq,
		}
	}
}

macro_rules! str_char {
	($str:expr, $char:expr) => {{
		let mut s: String = $str.clone();
		s.push($char);
		s
	}};
}

macro_rules! arg_match {
	($self:ident, $arg:expr,long_flag=>$long_flag:ident{$($lp:tt)*}$(,)?short_flag=>$short_flag:ident{$($sp:tt)*}$(,)?non_flag=>$non_flag:ident{$($nfp:tt)*}$(,)?rest_opt=>$rest_opt:ident{$($ot:tt)*}) =>{
		match $arg {
			Some(arg) if $self.long_flag(&arg)=>{
				match arg {
					$non_flag if $non_flag.len() < 2 => {
						$($nfp)*
					}
					$long_flag=>{
						$($lp)*
					}
				}
			}
			Some(arg) if $self.flag(&arg)=>{
				match arg {
					$non_flag if $non_flag.len() < 1 =>{
						$($nfp)*
					}
					$short_flag=>{
						$($sp)*
					}
				}
			}
			$rest_opt =>{
				$($ot)*
			}
		}
	};
	($self:ident, $arg:expr,long_flag=>$long_flag:ident{$($lp:tt)*}$(,)?short_flag=>$short_flag:ident{$($sp:tt)*}$(,)?normal_arg=>$normal_arg:ident{$($ap:tt)*}$(,)?none=>{$($np:tt)*}) => {
		match $arg {
			None=>{
				$($np)*
			}
			Some(arg) if $self.long_flag(&arg)=>{
				match arg {
					$normal_arg if $normal_arg.len() < 2 => {
						$($ap)*
					}
					$long_flag=>{
						$($lp)*
					}
				}
			}
			Some(arg) if $self.flag(&arg)=>{
				match arg {
					$normal_arg if $normal_arg.len() < 1 =>{
						$($ap)*
					}
					$short_flag=>{
						$($sp)*
					}
				}
			}
			Some($normal_arg)=>{
				$($ap)*
			}
		}
	};
	($self:ident, $arg:expr,long_flag=>$long_flag:ident{$($lp:tt)*}$(,)?short_flag=>$short_flag:ident{$($sp:tt)*}$(,)?normal_arg=>$normal_arg:ident{$($ap:tt)*}$(,)?none=>$none:ident{$($np:tt)*}) => {
		match $arg {
			Some(arg) if $self.long_flag(&arg)=>{
				match arg {
					$normal_arg if $normal_arg.len() < 2 => {
						$($ap)*
					}
					$long_flag=>{
						$($lp)*
					}
				}
			}
			Some(arg) if $self.flag(&arg)=>{
				match arg {
					$normal_arg if $normal_arg.len() < 1 =>{
						$($ap)*
					}
					$short_flag=>{
						$($sp)*
					}
				}
			}
			Some($normal_arg)=>{
				$($ap)*
			}
			$none=>{
				$($np)*
			}
		}
	};
}
impl Parser {
	/// Creates a new Parser with flag_pattern and long_flag_prefix.
	pub fn new(flag_pattern: char, long_flag_prefix: &str) -> Parser {
		Parser {
			flag_pattern,
			long_flag_prefix: String::from(long_flag_prefix),
			eq: '=',
		}
	}

	/// Returns true if str has long-flag prefix (default prefix: --).
	/// ロングフラグか判定する
	pub fn long_flag(&self, str: &str) -> bool {
		str.starts_with(&self.long_flag_prefix)
	}

	/// Returns true if str has flag prefix (default prefix: -).
	/// フラグならtrueを返す
	pub fn flag(&self, str: &str) -> bool {
		str.starts_with(self.flag_pattern)
	}

	/// Builds a new Parser with all options
	pub fn with_all_field(flag_pattern: char, long_flag_prefix: String, eq: char) -> Parser {
		Parser {
			flag_pattern,
			long_flag_prefix,
			eq,
		}
	}

	/// Removes long-flag prefix from arg.
	pub fn remove_long_flag_prefix(&self, mut arg: String) -> String {
		match arg.find(|c| c != self.flag_pattern) {
			Some(index) => arg.split_off(index),
			None => String::default(),
		}
	}

	/// Gets short flag name.
	pub fn get_short_flag_name(&self, mut arg: String) -> String {
		arg.split_off(1)
	}

	/// Parses args and convert into MiddileArgs
	pub fn middle_parse(
		&self,
		mut args: VecDeque<String>,
		mut inter_mediate_args: VecDeque<MiddleArg>,
		mut last: MiddleArg,
	) -> (
		Option<String>,
		VecDeque<String>,
		VecDeque<MiddleArg>,
		MiddleArg,
	) {
		loop {
			arg_match!(
				self,
				args.pop_front(),
			long_flag=>long_flag{
				inter_mediate_args.push_back(last);
				last = self.long_middle(long_flag);
			},
			short_flag=>short_flag{
				inter_mediate_args.push_back(last);
				last = self.short_middle(short_flag);
			},
			non_flag=>arg{
				break (Some(arg), args, inter_mediate_args, last);
			},
			rest_opt=>next{
				break (next, args, inter_mediate_args, last);
			})
		}
	}

	/// Converts long_flag to MiddleArg::LongFlag.
	pub fn long_middle(&self, mut long_flag: String) -> MiddleArg {
		match &long_flag.find(self.eq) {
			Some(index) => {
				let after_eq = long_flag.split_off(index + 1);
				long_flag.pop();
				MiddleArg::LongFlag(
					self.remove_long_flag_prefix(long_flag),
					FlagValue::String(after_eq),
				)
			}
			None => MiddleArg::LongFlag(self.remove_long_flag_prefix(long_flag), FlagValue::None),
		}
	}

	/// Converts short_flag to MiddleArg::ShortFlag.
	pub fn short_middle(&self, mut short_flag: String) -> MiddleArg {
		match &short_flag.find(self.eq) {
			Some(index) => {
				let after_eq = short_flag.split_off(index + 1);
				short_flag.pop();
				MiddleArg::ShortFlag(
					self.get_short_flag_name(short_flag),
					FlagValue::String(after_eq),
				)
			}
			None => MiddleArg::ShortFlag(self.remove_long_flag_prefix(short_flag), FlagValue::None),
		}
	}

	/// Parses c's parsing_args (call inter mediate args in parsing).
	pub fn parse_inter_mediate_args<T: FlagSearch, S: FlagSearch>(
		&self,
		local_flags: &T,
		current_common_flags: &S,
		mut c: Context,
		flag_only: bool,
	) -> (Context, Option<VecDeque<String>>) {
		match c.parsing_args {
			None => (c, None),
			Some(inter_middle_args) => {
				let non_flag_args = VecDeque::<String>::new();
				c.parsing_args = None;
				let l_flags = VecDeque::new();
				let c_flags = VecDeque::new();
				let e_list = VecDeque::new();
				let (_c, _, non_flag_args, l_flags, c_flags, e_list) = self.parse_next_if_middle_arg(
					inter_middle_args,
					non_flag_args,
					local_flags,
					current_common_flags,
					c,
					l_flags,
					c_flags,
					e_list,
					flag_only,
				);
				c = _c;
				c.local_flags_values.prepend_vec(l_flags.into());
				c.common_flags_values.prepend_vec(c_flags.into());
				c.error_info_list.prepend_vec(e_list.into());
				(c, {
					if flag_only {
						None
					} else {
						Some(non_flag_args)
					}
				})
			}
		}
	}

	/// Parses middle long flag.
	pub fn parse_middle_long_flag<T: FlagSearch, S: FlagSearch>(
		&self,
		name_or_alias: String,
		val: FlagValue,
		local_flags: &T,
		current_common_flags: &S,
		c: &Context,
		mut l_flags: VecDeque<(String, FlagValue)>,
		mut c_flags: VecDeque<(String, FlagValue)>,
		mut e_list: VecDeque<ErrorInfo>,
	) -> (
		VecDeque<(String, FlagValue)>, //l_flag
		VecDeque<(String, FlagValue)>, //c_flag
		VecDeque<ErrorInfo>,           //e_list
	) {
		match local_flags.find_long_flag(&name_or_alias) {
			LongFound::Name(l_flag) => {
				match val {
					FlagValue::String(_) if l_flag.flag_type.is_string() => {
						l_flags.push_front((name_or_alias, val));
					}
					FlagValue::String(val) => match l_flag.derive_flag_value_from_string(val) {
						FlagValue::Invalid(val) => {
							let flag_arg =
								MiddleArg::LongFlag(name_or_alias.clone(), FlagValue::String(val));
							e_list.push_front((
								flag_arg,
								ParseError::InvalidLong(name_or_alias),
								ParseError::NotParsed,
							));
						}
						val => {
							l_flags.push_front((name_or_alias, val));
						}
					},
					FlagValue::None => {
						l_flags.push_front((name_or_alias, FlagValue::None));
					}
					val => l_flags.push_front((name_or_alias, val)),
				};
			}
			LongFound::Long(l_flag) => match val {
				FlagValue::String(_) if l_flag.flag_type.is_string() => {
					l_flags.push_front((l_flag.get_name_clone(), val));
				}
				FlagValue::String(val) => match l_flag.derive_flag_value_from_string(val) {
					FlagValue::Invalid(val) => e_list.push_front((
						MiddleArg::LongFlag(name_or_alias, FlagValue::String(val)),
						ParseError::InvalidLong(l_flag.get_name_clone()),
						ParseError::NotParsed,
					)),
					val => {
						l_flags.push_front((l_flag.get_name_clone(), val));
					}
				},
				FlagValue::None => {
					l_flags.push_front((l_flag.get_name_clone(), FlagValue::None));
				}
				val => {
					l_flags.push_front((l_flag.get_name_clone(), val));
				}
			},
			LongFound::None => {
				match (current_common_flags, &c.common_flags).find_long_flag(&name_or_alias) {
					LongFound::Name(c_flag) => match val {
						FlagValue::None => {
							c_flags.push_front((name_or_alias, FlagValue::None));
						}
						FlagValue::String(_) if c_flag.flag_type.is_string() => {
							c_flags.push_front((name_or_alias, val));
						}
						FlagValue::String(val) => match c_flag.derive_flag_value_from_string(val) {
							FlagValue::Invalid(val) => {
								e_list.push_front((
									MiddleArg::LongFlag(name_or_alias.clone(), FlagValue::String(val)),
									ParseError::NoExistLong,
									ParseError::InvalidLong(name_or_alias),
								));
							}
							val => {
								c_flags.push_front((name_or_alias, val));
							}
						},
						val => c_flags.push_front((name_or_alias, val)),
					},
					LongFound::Long(c_flag) => match val {
						FlagValue::None => c_flags.push_front((c_flag.get_name_clone(), FlagValue::None)),
						FlagValue::String(_) if c_flag.flag_type.is_string() => {
							c_flags.push_front((c_flag.get_name_clone(), val));
						}
						FlagValue::String(val) => match c_flag.derive_flag_value_from_string(val) {
							FlagValue::Invalid(val) => {
								e_list.push_front((
									MiddleArg::LongFlag(name_or_alias, FlagValue::String(val)),
									ParseError::NoExistLong,
									ParseError::InvalidLong(c_flag.get_name_clone()),
								));
							}
							val => {
								c_flags.push_front((c_flag.get_name_clone(), val));
							}
						},
						val => {
							c_flags.push_front((c_flag.get_name_clone(), val));
						}
					},
					LongFound::None => {
						e_list.push_front((
							MiddleArg::LongFlag(name_or_alias, val),
							ParseError::NoExistLong,
							ParseError::NoExistLong,
						));
					}
				}
			}
		}
		(l_flags, c_flags, e_list)
	}

	/// Parse middle short flag
	pub fn parse_middle_short_flag<T: FlagSearch, S: FlagSearch>(
		&self,
		mut short_alias: String,
		flag_val: FlagValue,
		local_flags: &T,
		current_common_flags: &S,
		c: &Context,
		mut l_flags: VecDeque<(String, FlagValue)>,
		mut c_flags: VecDeque<(String, FlagValue)>,
		mut e_list: VecDeque<ErrorInfo>,
	) -> (
		VecDeque<(String, FlagValue)>,
		VecDeque<(String, FlagValue)>,
		VecDeque<ErrorInfo>,
	) {
		match short_alias.pop() {
			Some(last) => match local_flags.find_short_flag(&last) {
				Some(l_flag) => match flag_val {
					FlagValue::String(_) if l_flag.flag_type.is_string() => {
						l_flags.push_front((l_flag.get_name_clone(), flag_val));
					}
					FlagValue::String(val) => match l_flag.derive_flag_value_from_string(val) {
						FlagValue::Invalid(val) => {
							let i = short_alias.len();
							e_list.push_front((
								MiddleArg::ShortFlag(
									{
										let mut s = short_alias.clone();
										s.push(last);
										s
									},
									FlagValue::String(val),
								),
								ParseError::InvalidShort(i, l_flag.get_name_clone()),
								ParseError::NotParsed,
							));
						}
						val => {
							l_flags.push_front((l_flag.get_name_clone(), val));
						}
					},
					FlagValue::None => {
						l_flags.push_front((l_flag.get_name_clone(), FlagValue::None));
					}
					val => {
						l_flags.push_front((l_flag.get_name_clone(), val));
					}
				},
				None => match (current_common_flags, &c.common_flags).find_short_flag(&last) {
					Some(c_flag) => match flag_val {
						FlagValue::String(_) if c_flag.flag_type.is_string() => {
							c_flags.push_front((c_flag.get_name_clone(), flag_val));
						}
						FlagValue::String(val) => match c_flag.derive_flag_value_from_string(val) {
							FlagValue::Invalid(val) => {
								let i = short_alias.len();
								e_list.push_front((
									MiddleArg::ShortFlag(
										{
											let mut s = short_alias.clone();
											s.push(last);
											s
										},
										FlagValue::String(val),
									),
									ParseError::NoExistShort(i),
									ParseError::InvalidShort(i, c_flag.get_name_clone()),
								));
							}
							val => {
								c_flags.push_front((c_flag.get_name_clone(), val));
							}
						},
						FlagValue::None => {
							c_flags.push_front((c_flag.get_name_clone(), FlagValue::None));
						}
						val => {
							c_flags.push_front((c_flag.get_name_clone(), val));
						}
					},
					None => e_list.push_front((
						MiddleArg::ShortFlag(str_char!(short_alias, last), flag_val),
						ParseError::NoExistShort(short_alias.len()),
						ParseError::NoExistShort(short_alias.len()),
					)),
				},
			},
			None => {
				panic!("invalid short flag");
			}
		}

		(l_flags, c_flags, e_list)
	}

	/// Parse middle normal arg
	pub fn parse_middle_normal_arg<T: FlagSearch, S: FlagSearch>(
		&self,
		mut inter_mediate_args: VecDeque<MiddleArg>,
		normal_arg: String,
		local_flags: &T,
		current_common_flags: &S,
		mut c: Context,
		mut non_flag_args: VecDeque<String>,
		mut l_flags: VecDeque<(String, FlagValue)>,
		mut c_flags: VecDeque<(String, FlagValue)>,
		mut e_list: VecDeque<ErrorInfo>,
		flag_only: bool,
	) -> (
		Context,
		VecDeque<MiddleArg>,
		VecDeque<String>,
		VecDeque<(String, FlagValue)>,
		VecDeque<(String, FlagValue)>,
		VecDeque<ErrorInfo>,
	) {
		match inter_mediate_args.pop_back() {
			//ロングフラグが前にあり、その値である可能性があるとき
			Some(MiddleArg::LongFlag(long_flag_name, FlagValue::None)) => {
				match local_flags.find_long_flag(&long_flag_name) {
					LongFound::Name(l_flag) => match l_flag.derive_flag_value_from_string(normal_arg) {
						FlagValue::Invalid(normal_arg) => {
							if flag_only {
								c = self.push_normal_arg_in_flag_only_error(c, normal_arg);
							} else {
								non_flag_args.push_front(normal_arg);
							}
							l_flags.push_front((long_flag_name, FlagValue::None));
							self.parse_next_if_middle_arg(
								inter_mediate_args,
								non_flag_args,
								local_flags,
								current_common_flags,
								c,
								l_flags,
								c_flags,
								e_list,
								flag_only,
							)
						}
						val => {
							l_flags.push_front((long_flag_name, val));
							self.parse_next_if_middle_arg(
								inter_mediate_args,
								non_flag_args,
								local_flags,
								current_common_flags,
								c,
								l_flags,
								c_flags,
								e_list,
								flag_only,
							)
						}
					},
					LongFound::Long(l_flag) => match l_flag.derive_flag_value_from_string(normal_arg) {
						FlagValue::Invalid(normal_arg) => {
							l_flags.push_front((l_flag.get_name_clone(), FlagValue::None));
							if flag_only {
								c = self.push_normal_arg_in_flag_only_error(c, normal_arg);
							} else {
								non_flag_args.push_front(normal_arg);
							}
							self.parse_next_if_middle_arg(
								inter_mediate_args,
								non_flag_args,
								local_flags,
								current_common_flags,
								c,
								l_flags,
								c_flags,
								e_list,
								flag_only,
							)
						}
						val => {
							l_flags.push_front((l_flag.get_name_clone(), val));
							self.parse_next_if_middle_arg(
								inter_mediate_args,
								non_flag_args,
								local_flags,
								current_common_flags,
								c,
								l_flags,
								c_flags,
								e_list,
								flag_only,
							)
						}
					},
					LongFound::None => {
						match (current_common_flags, &c.common_flags).find_long_flag(&long_flag_name) {
							LongFound::Name(c_flag) => {
								match c_flag.derive_flag_value_from_string(normal_arg) {
									FlagValue::Invalid(normal_arg) => {
										if flag_only {
											c = self.push_normal_arg_in_flag_only_error(c, normal_arg);
										} else {
											non_flag_args.push_front(normal_arg);
										}
										c_flags.push_front((long_flag_name, FlagValue::None));
										self.parse_next_if_middle_arg(
											inter_mediate_args,
											non_flag_args,
											local_flags,
											current_common_flags,
											c,
											l_flags,
											c_flags,
											e_list,
											flag_only,
										)
									}
									val => {
										c_flags.push_front((long_flag_name, val));
										self.parse_next_if_middle_arg(
											inter_mediate_args,
											non_flag_args,
											local_flags,
											current_common_flags,
											c,
											l_flags,
											c_flags,
											e_list,
											flag_only,
										)
									}
								}
							}
							LongFound::Long(c_flag) => {
								match c_flag.derive_flag_value_from_string(normal_arg) {
									FlagValue::Invalid(normal_arg) => {
										if flag_only {
											non_flag_args.push_front(normal_arg);
										} else {
											c_flags.push_front((c_flag.get_name_clone(), FlagValue::None));
										}
										self.parse_next_if_middle_arg(
											inter_mediate_args,
											non_flag_args,
											local_flags,
											current_common_flags,
											c,
											l_flags,
											c_flags,
											e_list,
											flag_only,
										)
									}
									val => {
										c_flags.push_front((c_flag.get_name_clone(), val));
										self.parse_next_if_middle_arg(
											inter_mediate_args,
											non_flag_args,
											local_flags,
											current_common_flags,
											c,
											l_flags,
											c_flags,
											e_list,
											flag_only,
										)
									}
								}
							}
							LongFound::None => {
								non_flag_args.push_front(normal_arg);
								e_list.push_front((
									MiddleArg::LongFlag(long_flag_name, FlagValue::None),
									ParseError::NoExistLong,
									ParseError::NoExistLong,
								));
								self.parse_next_if_middle_arg(
									inter_mediate_args,
									non_flag_args,
									local_flags,
									current_common_flags,
									c,
									l_flags,
									c_flags,
									e_list,
									flag_only,
								)
							}
						}
					}
				}
			}
			//ロングフラグが前にあり、その引数である可能性がないとき
			Some(MiddleArg::LongFlag(name_or_alias, val)) => {
				if flag_only {
					c = self.push_normal_arg_in_flag_only_error(c, normal_arg);
				} else {
					non_flag_args.push_front(normal_arg);
				}
				let (l_flags, c_flags, e_list) = self.parse_middle_long_flag(
					name_or_alias,
					val,
					local_flags,
					current_common_flags,
					&c,
					l_flags,
					c_flags,
					e_list,
				);
				self.parse_next_if_middle_arg(
					inter_mediate_args,
					non_flag_args,
					local_flags,
					current_common_flags,
					c,
					l_flags,
					c_flags,
					e_list,
					flag_only,
				)
			}
			//ショートフラグの引数である可能性があるとき
			Some(MiddleArg::ShortFlag(mut short_str, FlagValue::None)) => {
				let short_alias = short_str.pop();
				if let Some(short_alias) = short_alias {
					match local_flags.find_short_flag(&short_alias) {
						Some(l_flag) => match l_flag.derive_flag_value_from_string(normal_arg) {
							FlagValue::Invalid(normal_arg) => {
								non_flag_args.push_front(normal_arg);
								l_flags.push_front((l_flag.get_name_clone(), FlagValue::None));
							}
							val => {
								l_flags.push_front((l_flag.get_name_clone(), val));
							}
						},
						None => {
							match (current_common_flags, &c.common_flags).find_short_flag(&short_alias) {
								Some(c_flag) => match c_flag.derive_flag_value_from_string(normal_arg) {
									FlagValue::Invalid(normal_arg) => {
										non_flag_args.push_front(normal_arg);
										c_flags.push_front((c_flag.get_name_clone(), FlagValue::None));
									}
									val => {
										c_flags.push_front((c_flag.get_name_clone(), val));
									}
								},
								None => {
									non_flag_args.push_front(normal_arg);
									let i = short_str.len() - 1;
									e_list.push_back((
										MiddleArg::ShortFlag(short_str, FlagValue::None),
										ParseError::NoExistShort(i),
										ParseError::NoExistShort(i),
									));
								}
							}
						}
					};

					self.parse_next_if_middle_arg(
						inter_mediate_args,
						non_flag_args,
						local_flags,
						current_common_flags,
						c,
						l_flags,
						c_flags,
						e_list,
						flag_only,
					)
				} else {
					panic!("short alias is not existed")
				}
			}
			Some(MiddleArg::ShortFlag(short_str, val)) => {
				let (l_flags, c_flags, e_list) = self.parse_middle_short_flag(
					short_str,
					val,
					local_flags,
					current_common_flags,
					&c,
					l_flags,
					c_flags,
					e_list,
				);
				self.parse_next_if_middle_arg(
					inter_mediate_args,
					non_flag_args,
					local_flags,
					current_common_flags,
					c,
					l_flags,
					c_flags,
					e_list,
					flag_only,
				)
			}
			Some(MiddleArg::Normal(prev_arg)) => {
				if flag_only {
					c = self.push_normal_arg_in_flag_only_error(c, normal_arg);
				} else {
					non_flag_args.push_front(normal_arg);
				}

				self.parse_middle_normal_arg(
					inter_mediate_args,
					prev_arg,
					local_flags,
					current_common_flags,
					c,
					non_flag_args,
					l_flags,
					c_flags,
					e_list,
					flag_only,
				)
			}
			None => {
				non_flag_args.push_front(normal_arg);
				(
					c,
					inter_mediate_args,
					non_flag_args,
					l_flags,
					c_flags,
					e_list,
				)
			}
		}
	}

	fn push_normal_arg_in_flag_only_error(&self, mut c: Context, normal_arg: String) -> Context {
		let val = MiddleArg::Normal(normal_arg);
		c.error_info_list
			.push((val.clone(), ParseError::NotExist, ParseError::NotExist));
		c.push_front_to_parsing_args(val);
		c
	}

	/// Parses args if next middle args exist.
	pub fn parse_next_if_middle_arg<T: FlagSearch, S: FlagSearch>(
		&self,
		mut inter_mediate_args: VecDeque<MiddleArg>,
		non_flag_args: VecDeque<String>,
		local_flags: &T,
		current_common_flags: &S,
		c: Context,
		l_flags: VecDeque<(String, FlagValue)>,
		c_flags: VecDeque<(String, FlagValue)>,
		e_list: VecDeque<ErrorInfo>,
		flag_only: bool,
	) -> (
		Context,
		VecDeque<MiddleArg>,           //inter_mediate_args
		VecDeque<String>,              //non_flag_args
		VecDeque<(String, FlagValue)>, //l_flags
		VecDeque<(String, FlagValue)>, //c_flags
		VecDeque<ErrorInfo>,           //e_list
	) {
		match inter_mediate_args.pop_back() {
			Some(MiddleArg::LongFlag(long_flag, flag_val)) => {
				let (l_flags, c_flags, e_list) = self.parse_middle_long_flag(
					long_flag,
					flag_val,
					local_flags,
					current_common_flags,
					&c,
					l_flags,
					c_flags,
					e_list,
				);
				self.parse_next_if_middle_arg(
					inter_mediate_args,
					non_flag_args,
					local_flags,
					current_common_flags,
					c,
					l_flags,
					c_flags,
					e_list,
					flag_only,
				)
			}
			Some(MiddleArg::ShortFlag(short_flag, flag_val)) => {
				let (l_flags, c_flags, e_list) = self.parse_middle_short_flag(
					short_flag,
					flag_val,
					local_flags,
					current_common_flags,
					&c,
					l_flags,
					c_flags,
					e_list,
				);
				self.parse_next_if_middle_arg(
					inter_mediate_args,
					non_flag_args,
					local_flags,
					current_common_flags,
					c,
					l_flags,
					c_flags,
					e_list,
					flag_only,
				)
			}
			Some(MiddleArg::Normal(arg)) => self.parse_middle_normal_arg(
				inter_mediate_args,
				arg,
				local_flags,
				current_common_flags,
				c,
				non_flag_args,
				l_flags,
				c_flags,
				e_list,
				flag_only,
			),
			None => (
				c,
				inter_mediate_args,
				non_flag_args,
				l_flags,
				c_flags,
				e_list,
			),
		}
	}

	/// Parse args until args' end.
	pub fn parse_args_until_end<T: FlagSearch, S: FlagSearch>(
		self,
		local_flags: &T,
		current_common_flags: &S,
		mut c: Context,
	) -> Context {
		let mut non_flag_args = VecDeque::<String>::new();
		loop {
			arg_match!(self, c.args.pop_front(),
					long_flag=>long_flag{
						let (next, _c) = self.parse_flags_start_with_long_flag(
							long_flag,
							local_flags,
							current_common_flags,
							c,
						);
						c = _c;
						if let Some(arg) = next {
							non_flag_args.push_back(arg);
						} else {
							break;
						}
					},
					short_flag=>short_flag{
						let (next, _c) = self.parse_flags_start_with_short_flag(
							short_flag,
							local_flags,
							current_common_flags,
							c,
						);
						c = _c;
						if let Some(arg) = next {
							non_flag_args.push_back(arg);
						} else {
							break;
						}
					},
					normal_arg=>arg{
						non_flag_args.push_back(arg);
					},
					none=>{
						break;
					}
			)
		}
		c.args = non_flag_args;
		c
	}

	/// Parses flags start with long flag until non-flag arg appeared.
	pub fn parse_flags_start_with_long_flag<T: FlagSearch, S: FlagSearch>(
		&self,
		mut long_flag: String,
		local_flags: &T,
		current_common_flags: &S,
		mut c: Context,
	) -> (Option<String>, Context) {
		long_flag = self.remove_long_flag_prefix(long_flag);
		match long_flag.find(self.eq) {
			Some(index) => {
				let after_eq = long_flag.split_off(index + 1);
				long_flag.pop();
				match local_flags.find_long_flag(&long_flag) {
					LongFound::Name(l_flag) => match l_flag.derive_flag_value_from_string(after_eq) {
						FlagValue::Invalid(after_eq) => {
							let flag_arg =
								MiddleArg::LongFlag(long_flag.clone(), FlagValue::Invalid(after_eq));
							c.push_back_to_parsing_args(flag_arg.clone());
							c.error_info_list.push((
								flag_arg,
								ParseError::InvalidLong(long_flag),
								ParseError::NotParsed,
							));
						}
						val => {
							c.local_flags_values.push((long_flag, val));
						}
					},
					LongFound::Long(l_flag) => match l_flag.flag_type.get_value_from_string(after_eq) {
						FlagValue::Invalid(after_eq) => {
							let flag_arg = MiddleArg::LongFlag(long_flag, FlagValue::Invalid(after_eq));
							let l_flag_name = l_flag.get_name_clone();
							c.error_info_list.push((
								flag_arg.clone(),
								ParseError::InvalidLong(l_flag_name),
								ParseError::NotParsed,
							));
							c.push_back_to_parsing_args(flag_arg);
						}
						val => {
							let l_flag = l_flag.get_name_clone();
							c.local_flags_values.push((l_flag, val));
						}
					},
					_ => match (current_common_flags, &c.common_flags).find_long_flag(&long_flag) {
						LongFound::Name(c_flag) => match c_flag.flag_type.get_value_from_string(after_eq)
						{
							FlagValue::Invalid(after_eq) => {
								let flag_arg = MiddleArg::LongFlag(long_flag, FlagValue::String(after_eq));
								c.error_info_list.push((
									flag_arg.clone(),
									ParseError::NoExistLong,
									ParseError::InvalidLong(c_flag.get_name_clone()),
								));
								c.push_back_to_parsing_args(flag_arg)
							}
							val => c.common_flags_values.push((long_flag, val)),
						},
						LongFound::Long(c_flag) => match c_flag.flag_type.get_value_from_string(after_eq)
						{
							FlagValue::Invalid(after_eq) => {
								let flag_arg = MiddleArg::LongFlag(long_flag, FlagValue::String(after_eq));
								c.error_info_list.push((
									flag_arg.clone(),
									ParseError::NoExistLong,
									ParseError::InvalidLong(c_flag.get_name_clone()),
								));
								c.push_back_to_parsing_args(flag_arg)
							}
							val => c.common_flags_values.push((c_flag.get_name_clone(), val)),
						},
						_ => {
							let flag_arg = MiddleArg::LongFlag(long_flag, FlagValue::String(after_eq));
							c.error_info_list.push((
								flag_arg.clone(),
								ParseError::NoExistLong,
								ParseError::NoExistLong,
							));
							c.push_back_to_parsing_args(flag_arg)
						}
					},
				}
				self.parse_next_if_flag(local_flags, current_common_flags, c)
			}
			None => match local_flags.find_long_flag(&long_flag) {
				LongFound::Name(l_flag) => {
					arg_match!(self, c.args.pop_front(),long_flag=>next_long_flag{
						c.local_flags_values.push((long_flag, FlagValue::None));
						self.parse_flags_start_with_long_flag(
							next_long_flag,
							local_flags,
							current_common_flags,
							c,
						)
					},
					short_flag=>next_short_flag{
						c.local_flags_values.push((long_flag, FlagValue::None));
						self.parse_flags_start_with_short_flag(
							next_short_flag,
							local_flags,
							current_common_flags,
							c,
						)
					},
					normal_arg=>next_arg{
						match l_flag.derive_flag_value_from_string(next_arg) {
							FlagValue::Invalid(next_arg) => {
								c.local_flags_values.push((long_flag, FlagValue::None));
								(Some(next_arg), c)
							}
							val => {
								c.local_flags_values.push((long_flag, val));
								self.parse_next_if_flag(local_flags, current_common_flags, c)
							}
						}
					},
					none=>{
						c.local_flags_values.push((long_flag, FlagValue::None));
						(None, c)
					})
				}
				LongFound::Long(l_flag) => arg_match!(
					self,c.args.pop_front(),
					long_flag=>next_long_flag{
						c.local_flags_values
						.push((l_flag.get_name_clone(), FlagValue::None));
						self.parse_flags_start_with_long_flag(
							next_long_flag,
							local_flags,
							current_common_flags,
							c,
						)
					}
					short_flag=>next_short_flag{
						c.local_flags_values
						.push((l_flag.get_name_clone(), FlagValue::None));
						self.parse_flags_start_with_short_flag(
							next_short_flag,
							local_flags,
							current_common_flags,
							c,
						)
					}
					normal_arg=>next_arg{
						match l_flag.derive_flag_value_from_string(next_arg) {
							FlagValue::Invalid(next_arg) => {
								c.local_flags_values
								.push((l_flag.get_name_clone(), FlagValue::None));
								(Some(next_arg), c)
							}
							val => {
								c.local_flags_values.push((long_flag, val));
								self.parse_next_if_flag(local_flags, current_common_flags, c)
							}
						}
					}
					none=>{
						c.local_flags_values
						.push((l_flag.get_name_clone(), FlagValue::None));
						(None, c)
					}
				),
				_ => match (current_common_flags, &c.common_flags).find_long_flag(&long_flag) {
					LongFound::Name(c_flag) => {
						arg_match!(self,c.args.pop_front(), long_flag=>next_long_flag{
								c.common_flags_values.push((long_flag, FlagValue::None));
								self.parse_flags_start_with_long_flag(
									next_long_flag,
									local_flags,
									current_common_flags,
									c,
								)
							},
							short_flag=>next_short_flag{
								c.common_flags_values.push((long_flag, FlagValue::None));
								self.parse_flags_start_with_short_flag(
									next_short_flag,
									local_flags,
									current_common_flags,
									c,
								)
							}
							normal_arg=>next_arg{
								match c_flag.flag_type.get_value_from_string(next_arg) {
								FlagValue::Invalid(next_arg) => {
									c.common_flags_values.push((long_flag, FlagValue::None));
									(Some(next_arg), c)
								}
								val => {
									c.common_flags_values.push((long_flag, val));
									self.parse_next_if_flag(local_flags, current_common_flags, c)
								}
							}
							}
							none=>next_none{
								c.common_flags_values.push((long_flag, FlagValue::None));
								(next_none, c)
							}
						)
					}
					LongFound::Long(c_flag) => {
						arg_match!(self,c.args.pop_front(),
							long_flag=>next_long_flag{
								c.common_flags_values
										.push((c_flag.get_name_clone(), FlagValue::None));
									self.parse_flags_start_with_long_flag(
										next_long_flag,
										local_flags,
										current_common_flags,
										c,
									)
							},
							short_flag=>next_short_flag{
								c.common_flags_values
										.push((c_flag.get_name_clone(), FlagValue::None));
									self.parse_flags_start_with_short_flag(
										next_short_flag,
										local_flags,
										current_common_flags,
										c,
									)
							},
							normal_arg=>next_arg{
								match c_flag.flag_type.get_value_from_string(next_arg) {
									FlagValue::Invalid(next_arg) => {
										c.common_flags_values
											.push((c_flag.get_name_clone(), FlagValue::None));
										(Some(next_arg), c)
									}
									val => {
										c.common_flags_values.push((c_flag.get_name_clone(), val));
										self.parse_next_if_flag(local_flags, current_common_flags, c)
									}
								}
							},
						none=>next_none{
							c.common_flags_values
										.push((c_flag.get_name_clone(), FlagValue::None));
									(next_none, c)
						})
					}
					_ => {
						let flag_arg = MiddleArg::LongFlag(long_flag, FlagValue::None);
						c.error_info_list.push((
							flag_arg.clone(),
							ParseError::NoExistLong,
							ParseError::NoExistLong,
						));
						c.push_back_to_parsing_args(flag_arg);
						self.parse_next_if_flag(local_flags, current_common_flags, c)
					}
				},
			},
		}
	}

	/// Parses flags start with short flag until args appeared.
	pub fn parse_flags_start_with_short_flag<T: FlagSearch, S: FlagSearch>(
		&self,
		mut short_flag: String,
		local_flags: &T,
		current_common_flags: &S,
		mut c: Context,
	) -> (Option<String>, Context) {
		match short_flag.find(self.eq) {
			Some(index) => {
				let after_eq = short_flag.split_off(index + 1);
				short_flag.pop();
				short_flag = self.get_short_flag_name(short_flag);
				match short_flag.pop() {
					None => {
						let record = MiddleArg::ShortFlag(short_flag, FlagValue::String(after_eq));
						c.error_info_list
							.push((record.clone(), ParseError::Empty, ParseError::Empty));
						c.push_back_to_parsing_args(record);
						self.parse_next_if_flag(local_flags, current_common_flags, c)
					}
					Some(before_eq) => {
						let mut i = 0;
						for s in short_flag.chars() {
							match local_flags.find_short_flag(&s) {
								Some(l_flag) => {
									let record = (l_flag.get_name_clone(), FlagValue::None);
									c.local_flags_values.push(record);
								}
								_ => match (current_common_flags, &c.common_flags).find_short_flag(&s) {
									Some(c_flag) => {
										let record = (c_flag.get_name_clone(), FlagValue::None);
										c.common_flags_values.push(record);
									}
									_ => {
										c.push_back_to_parsing_args(MiddleArg::ShortFlag(
											s.to_string(),
											FlagValue::None,
										));
										let mut short_flag = short_flag.clone();
										short_flag.push(before_eq);

										c.error_info_list.push((
											MiddleArg::ShortFlag(short_flag, FlagValue::None),
											ParseError::NoExistShort(i),
											ParseError::NoExistShort(i),
										));
									}
								},
							}
							i = i + 1;
						}
						//最後のフラグと値を処理
						match local_flags.find_short_flag(&before_eq) {
							Some(l_flag) => {
								match l_flag.derive_flag_value_from_string(after_eq) {
									FlagValue::Invalid(after_eq) => {
										let l_flag = l_flag.get_name_clone();
										c.error_info_list.push((
											MiddleArg::ShortFlag(
												{
													short_flag.push(before_eq);
													short_flag
												},
												FlagValue::String(after_eq.clone()),
											),
											ParseError::InvalidShort(i, l_flag),
											ParseError::None,
										));
										c.push_back_to_parsing_args(MiddleArg::ShortFlag(
											before_eq.to_string(),
											FlagValue::String(after_eq),
										));
									}
									val => {
										let l_flag = l_flag.get_name_clone();
										c.local_flags_values.push((l_flag, val));
									}
								};
							}
							_ => {
								//ローカルにヒットしなかった場合
								match c.common_flags.find_short_flag(&before_eq) {
									Some(c_flag) => match c_flag.derive_flag_value_from_string(after_eq) {
										FlagValue::Invalid(after_eq) => {
											c.error_info_list.push((
												MiddleArg::ShortFlag(
													{
														short_flag.push(before_eq);
														short_flag
													},
													FlagValue::String(after_eq.clone()),
												),
												ParseError::NoExistShort(i),
												ParseError::InvalidShort(i, c_flag.get_name_clone()),
											));
											c.push_back_to_parsing_args(MiddleArg::ShortFlag(
												before_eq.to_string(),
												FlagValue::String(after_eq),
											));
										}
										val => {
											c.local_flags_values.push((c_flag.get_name_clone(), val));
										}
									},
									_ => {
										let f_val = FlagValue::String(after_eq);
										c.error_info_list.push((
											MiddleArg::ShortFlag(
												{
													short_flag.push(before_eq);
													short_flag
												},
												f_val.clone(),
											),
											ParseError::NoExistShort(i),
											ParseError::NoExistShort(i),
										));
										c.push_back_to_parsing_args(MiddleArg::ShortFlag(
											before_eq.to_string(),
											f_val,
										))
									}
								}
							}
						}
						self.parse_next_if_flag(local_flags, current_common_flags, c)
					}
				}
			}
			None => {
				short_flag = self.get_short_flag_name(short_flag);
				let mut i: Index = 0;
				let last = short_flag.pop();
				match last {
					Some(last) => {
						for s in short_flag.chars() {
							match local_flags.find_short_flag(&s) {
								Some(l_flag) => {
									let record = (l_flag.get_name_clone(), FlagValue::None);
									c.local_flags_values.push(record);
								}
								_ => match (current_common_flags, &c.common_flags).find_short_flag(&s) {
									Some(c_flag) => {
										let record = (c_flag.get_name_clone(), FlagValue::None);
										c.common_flags_values.push(record);
									}
									_ => {
										c.push_back_to_parsing_args(MiddleArg::ShortFlag(
											s.to_string(),
											FlagValue::None,
										));
										c.error_info_list.push((
											MiddleArg::ShortFlag(short_flag.clone(), FlagValue::None),
											ParseError::NoExistShort(i),
											ParseError::NoExistShort(i),
										));
									}
								},
							};
							i = i + 1;
						}
						//最後の１フラグを処理
						match local_flags.find_short_flag(&last) {
							Some(l_flag) => {
								arg_match!(self,c.args.pop_front(),
										long_flag=>next_long_flag{
											c.local_flags_values
													.push((l_flag.get_name_clone(), FlagValue::None));
												self.parse_flags_start_with_long_flag(
													next_long_flag,
													local_flags,
													current_common_flags,
													c,
												)
										},
									short_flag=>next_short_flag{
										c.local_flags_values
													.push((l_flag.get_name_clone(), FlagValue::None));
												self.parse_flags_start_with_short_flag(
													next_short_flag,
													local_flags,
													current_common_flags,
													c,
												)
									},
									normal_arg=>next_arg{
										match l_flag.derive_flag_value_from_string(next_arg) {
												FlagValue::Invalid(next_arg) => {
													//
													c.local_flags_values
														.push((l_flag.get_name_clone(), FlagValue::None));
													(Some(next_arg), c)
												}
												val => {
													c.local_flags_values.push((l_flag.get_name_clone(), val));
													self.parse_next_if_flag(local_flags, current_common_flags, c)
												}
											}
									},
								none=>next_none{
									c.local_flags_values
													.push((l_flag.get_name_clone(), FlagValue::None));
												(next_none, c)
								})
							}
							_ => match (current_common_flags, &c.common_flags).find_short_flag(&last) {
								Some(c_flag) => arg_match!(self, c.args.pop_front(),
								long_flag=>next_long_flag {
									c.common_flags_values
									.push((c_flag.get_name_clone(), FlagValue::None));
									self.parse_flags_start_with_long_flag(
										next_long_flag,
										local_flags,
										current_common_flags,
										c,
									)
								},
								short_flag=>next_short_flag{
									c.common_flags_values
									.push((c_flag.get_name_clone(), FlagValue::None));
									self.parse_flags_start_with_short_flag(
										next_short_flag,
										local_flags,
										current_common_flags,
										c,
									)
								},
								normal_arg=>next_arg{
									match c_flag.derive_flag_value_from_string(next_arg) {
										FlagValue::Invalid(next_arg) => {
											c.common_flags_values
											.push((c_flag.get_name_clone(), FlagValue::None));
											(Some(next_arg), c)
										}
										val => {
											c.common_flags_values.push((c_flag.get_name_clone(), val));
											self.parse_next_if_flag(local_flags, current_common_flags, c)
										}
									}
								},
									none=>next_none{
										c.common_flags_values
											.push((c_flag.get_name_clone(), FlagValue::None));
										(next_none, c)
									}
								),
								_ => {
									let flag_arg = MiddleArg::ShortFlag(short_flag, FlagValue::None);
									c.error_info_list.push((
										flag_arg.clone(),
										ParseError::NoExistShort(i),
										ParseError::NoExistShort(i),
									));
									c.push_back_to_parsing_args(flag_arg);
									self.parse_next_if_flag(local_flags, current_common_flags, c)
								}
							},
						}
					}
					None => {
						c.push_back_to_parsing_args(MiddleArg::ShortFlag(
							short_flag.clone(),
							FlagValue::String(String::new()),
						));
						c.error_info_list.push((
							MiddleArg::ShortFlag(short_flag, FlagValue::String(String::new())),
							ParseError::NotExist,
							ParseError::NotExist,
						));
						self.parse_next_if_flag(local_flags, current_common_flags, c)
					}
				}
			}
		}
	}

	/// Parses(or assigns parse functions) args if next args.
	pub fn parse_next_if_flag<T: FlagSearch, S: FlagSearch>(
		&self,
		local_flags: &T,
		current_common_flags: &S,
		mut c: Context,
	) -> (Option<String>, Context) {
		arg_match!(self,
		c.args.pop_front(),
		long_flag=>long_flag{
			self.parse_flags_start_with_long_flag(
					long_flag,
					local_flags,
					current_common_flags,
					c,
				)
		},
		short_flag=>short_flag{
			self.parse_flags_start_with_short_flag(
					short_flag,
					local_flags,
					current_common_flags,
					c,
				)
		},
		non_flag=>non_flag{
			(Some(non_flag),c)
		},
		rest_opt=>val{
			(val,c)
		})
	}
}

/// Enum for middle result in parsing
#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub enum MiddleArg {
	/// Variant shows a normal arg.
	Normal(String),
	/// Variant shows a long flag.
	LongFlag(String, FlagValue),
	/// Variant shows a short flag.
	ShortFlag(String, FlagValue),
}

impl MiddleArg {
	/// Gets &self's name (if a non-flag arg, returns content).
	pub fn name(&self) -> &str {
		match &self {
			MiddleArg::LongFlag(name, _) => name,
			MiddleArg::ShortFlag(name, _) => name,
			MiddleArg::Normal(name) => name,
		}
	}

	/// Gets flag value storaged in &self if FlagValue::String
	pub fn val_if_string(&self) -> Option<&String> {
		match self {
			MiddleArg::LongFlag(_, FlagValue::String(val)) => Some(val),
			MiddleArg::ShortFlag(_, FlagValue::String(val)) => Some(val),
			MiddleArg::LongFlag(_, FlagValue::Invalid(val)) => Some(val),
			MiddleArg::ShortFlag(_, FlagValue::Invalid(val)) => Some(val),
			_ => None,
			/*MiddleArg::LongFlag(_, _) => None,
			MiddleArg::ShortFlag(_, _) => None,
			MiddleArg::Normal(_) => None,*/
		}
	}

	/// Gets inner of Variant if string value.
	pub fn inner_if_string_val(&self) -> Option<(&str, &str)> {
		match &self {
			MiddleArg::LongFlag(name, FlagValue::String(val)) => Some((name, val)),
			MiddleArg::ShortFlag(name, FlagValue::String(val)) => Some((name, val)),
			MiddleArg::LongFlag(name, FlagValue::Invalid(val)) => Some((name, val)),
			MiddleArg::ShortFlag(name, FlagValue::Invalid(val)) => Some((name, val)),
			_ => None,
			/*MiddleArg::LongFlag(_, _) => None,
			MiddleArg::ShortFlag(_, _) => None,
			MiddleArg::Normal(_) => None,*/
		}
	}

	/// Sets value to MiddileArg self.
	pub fn set_val(mut self, value: FlagValue) -> Self {
		match self {
			MiddleArg::LongFlag(_, ref mut val) => {
				(*val) = value;
				self
			}
			MiddleArg::ShortFlag(_, ref mut val) => {
				(*val) = value;
				self
			}
			MiddleArg::Normal(ref mut val) => {
				(*val) = value.get_string();
				self
			}
		}
	}

	/// Gets the form of MiddleArg as str value.
	pub fn get_flag_type_str<'a>(&self) -> &'a str {
		match self {
			MiddleArg::LongFlag(_, _) => "long",
			MiddleArg::ShortFlag(_, _) => "short",
			MiddleArg::Normal(_) => "normal",
		}
	}
}

type Index = usize;
/// ParseError shows error in parsing
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum ParseError {
	/// Variant shows that corresponding long flag do not exist.
	NoExistLong,
	/// Variant shows that corresponding short flag do not exist.
	NoExistShort(Index),
	/// Variant shows invalid short flag.
	InvalidShort(Index, String),
	/// Variant shows invalid short flag.
	InvalidLong(String),
	/// Shows not exist flag.
	NotExist,
	/// Shows not parsed.
	NotParsed,
	/// Shows empty
	Empty,
	/// Shows no error.
	None,
}

/// Type of error information
pub type ErrorInfo = (MiddleArg, ParseError, ParseError);

/// Presets for output Error info
pub mod preset {
	use super::{ErrorInfo, MiddleArg, ParseError};

	/// Generates error description.
	pub fn gen_error_description(err_info: &ErrorInfo) -> String {
		let mut description = String::from("Parse error: ");
		match &err_info.0 {
			MiddleArg::Normal(name) => {
				description.push_str("arg:");
				description.push_str(&name);
			}
			MiddleArg::LongFlag(name, _) => {
				description.push_str("flag: --");
				description.push_str(&name)
			}
			MiddleArg::ShortFlag(name, _) => {
				description.push_str("short flag: -");
				description.push_str(&name);
			}
		}
		description.push_str(".\n");
		match err_info {
			(flag_arg, ParseError::NoExistShort(i), ParseError::NoExistShort(_)) => {
				let name = flag_arg.name();
				description = format!(
					"{}The short flag {} is an unknown short flag.",
					description,
					match name.len() {
						1 => {
							let mut s = String::from("-");
							s.push(name.chars().nth(*i).unwrap());
							s
						}
						_ => {
							let mut s = String::from("\"");
							s.push(name.chars().nth(*i).unwrap());
							s.push('"');
							s.push_str(" in ");
							s.push_str(name);
							s
						}
					},
				);
			}
			(flag_arg, ParseError::NoExistLong, ParseError::NoExistLong) => {
				description = format!("The flag --{} is an unknown flag.", flag_arg.name());
			}
			(flag_arg, ParseError::NoExistShort(_), ParseError::InvalidShort(i, c_flag)) => {
				let (name, val) = flag_arg.inner_if_string_val().unwrap();
				description = format!(
					"{}The value of short flag {} in {} - {} is not valid for a common flag {}.",
					description,
					name.chars().nth(*i).unwrap(),
					name,
					val,
					c_flag
				);
			}
			(flag_arg, ParseError::NoExistLong, ParseError::InvalidLong(chit)) => {
				let (name, val) = flag_arg.inner_if_string_val().unwrap();
				description = format!(
					"{}The flag {} matches a common flag {}. But its value {} is invalid for {}.",
					description,
					name,
					chit,
					val,
					flag_arg.get_flag_type_str()
				);
			}
			(flag_arg, ParseError::InvalidShort(i, l_flag), _) => {
				let (name, val) = flag_arg.inner_if_string_val().unwrap();
				description = format!(
					"{}The flag {}{}'s value {} is not valid for a local flag {}.",
					description,
					name.chars().nth(*i).unwrap(),
					{
						if name.chars().count() < 2 {
							""
						} else {
							name
						}
					},
					val,
					l_flag
				);
			}
			(_, _, _) => {}
		};
		description
	}
}
