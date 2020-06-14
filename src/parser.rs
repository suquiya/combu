use crate::vector::flag::Found;
use crate::Context;
use crate::FlagValue;
use std::collections::VecDeque;

pub struct Parser {
	pub flag_pattern: char,
	pub long_flag_prefix: String,
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

impl Parser {
	pub fn new(flag_pattern: char, long_flag_prefix: &str) -> Parser {
		Parser {
			flag_pattern,
			long_flag_prefix: String::from(long_flag_prefix),
			eq: '=',
		}
	}

	pub fn long_flag(&self, str: &str) -> bool {
		str.starts_with(&self.long_flag_prefix)
	}

	pub fn flag(&self, str: &str) -> bool {
		str.starts_with(self.flag_pattern)
	}

	pub fn build_new(flag_pattern: char, long_flag_prefix: String, eq: char) -> Parser {
		Parser {
			flag_pattern,
			long_flag_prefix,
			eq,
		}
	}

	pub fn get_long_flag_name(&self, mut arg: String) -> String {
		match arg.find(|c| c != self.flag_pattern) {
			Some(index) => arg.split_off(index),
			None => String::default(),
		}
	}

	pub fn get_short_flag_name(&self, mut arg: String) -> String {
		arg.split_off(1)
	}

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
			match args.pop_front() {
				Some(long_flag) if self.long_flag(&long_flag) => {
					inter_mediate_args.push_back(last);
					last = self.long_middle(long_flag);
				}
				Some(short_flag) if self.flag(&short_flag) => {
					inter_mediate_args.push_back(last);
					last = self.short_middle(short_flag);
				}
				next => {
					break (next, args, inter_mediate_args, last);
				}
			}
		}
	}

	pub fn long_middle(&self, mut long_flag: String) -> MiddleArg {
		match &long_flag.find(self.eq) {
			Some(index) => {
				let after_eq = long_flag.split_off(index + 1);
				long_flag.pop();
				MiddleArg::LongFlag(
					self.get_long_flag_name(long_flag),
					FlagValue::String(after_eq),
				)
			}
			None => MiddleArg::LongFlag(self.get_long_flag_name(long_flag), FlagValue::None),
		}
	}

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
			None => MiddleArg::ShortFlag(self.get_long_flag_name(short_flag), FlagValue::None),
		}
	}

	pub fn parse_args_until_end(self, mut c: Context) -> Context {
		let mut non_flag_args = VecDeque::<String>::new();

		loop {
			match c.next_arg() {
				None => {
					break;
				}
				Some(long_flag) if self.long_flag(&long_flag) => {
					let (next, _c) = self.parse_flags_start_with_long_flag(long_flag, c);
					c = _c;
					if let Some(arg) = next {
						non_flag_args.push_back(arg);
					} else {
						break;
					}
				}
				Some(short_flag) if self.flag(&short_flag) => {
					let (next, _c) = self.parse_flags_start_with_short_flag(short_flag, c);
					c = _c;
					if let Some(arg) = next {
						non_flag_args.push_back(arg);
					} else {
						break;
					}
				}
				Some(arg) => {
					non_flag_args.push_back(arg);
				}
			}
		}
		c
	}

	pub fn parse_flags_start_with_long_flag(
		&self,
		mut long_flag: String,
		mut c: Context,
	) -> (Option<String>, Context) {
		match long_flag.find(self.eq) {
			Some(index) => {
				let after_eq = long_flag.split_off(index + 1);
				long_flag.pop();
				long_flag = self.get_long_flag_name(long_flag);

				match c.local_flags.find_long_flag(&long_flag) {
					Found::Name(l_flag) => match l_flag.derive_flag_value_from_string(after_eq) {
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
					Found::Long(l_flag) => match l_flag.flag_type.get_value_from_string(after_eq) {
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
					lhit => match c.common_flags.find_long_flag(&long_flag) {
						Found::Name(c_flag) => match c_flag.flag_type.get_value_from_string(after_eq) {
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
						Found::Long(c_flag) => match c_flag.flag_type.get_value_from_string(after_eq) {
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
						chit => {
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
				self.parse_next_if_flag(c)
			}
			None => match c.local_flags.find_long_flag(&long_flag) {
				Found::Name(l_flag) => match c.args.pop_front() {
					Some(next_long_flag) if self.long_flag(&next_long_flag) => {
						c.local_flags_values
							.push((long_flag, l_flag.flag_type.get_value_if_no_value()));
						self.parse_flags_start_with_long_flag(next_long_flag, c)
					}
					Some(next_short_flag) if self.flag(&next_short_flag) => {
						c.local_flags_values
							.push((long_flag, l_flag.flag_type.get_value_if_no_value()));
						self.parse_flags_start_with_short_flag(next_short_flag, c)
					}
					Some(next_arg) => match l_flag.derive_flag_value_from_string(next_arg) {
						FlagValue::Invalid(next_arg) => {
							c.local_flags_values
								.push((long_flag, FlagValue::Bool(true)));
							(Some(next_arg), c)
						}
						val => {
							c.local_flags_values.push((long_flag, val));
							self.parse_next_if_flag(c)
						}
					},
					None => {
						c.local_flags_values
							.push((long_flag, l_flag.flag_type.get_value_if_no_value()));
						(None, c)
					}
				},
				Found::Long(l_flag) => match c.args.pop_front() {
					Some(next_long_flag) if self.long_flag(&next_long_flag) => {
						c.local_flags_values.push((
							l_flag.get_name_clone(),
							l_flag.flag_type.get_value_if_no_value(),
						));
						self.parse_flags_start_with_long_flag(next_long_flag, c)
					}
					Some(next_short_flag) if self.flag(&next_short_flag) => {
						c.local_flags_values.push((
							l_flag.get_name_clone(),
							l_flag.flag_type.get_value_if_no_value(),
						));
						self.parse_flags_start_with_short_flag(next_short_flag, c)
					}
					Some(next_arg) => match l_flag.derive_flag_value_from_string(next_arg) {
						FlagValue::Invalid(next_arg) => {
							c.local_flags_values.push((
								l_flag.get_name_clone(),
								l_flag.derive_flag_value_if_no_value(),
							));
							(Some(next_arg), c)
						}
						val => {
							c.local_flags_values.push((long_flag, val));
							self.parse_next_if_flag(c)
						}
					},
					None => {
						c.local_flags_values.push((
							l_flag.get_name_clone(),
							l_flag.flag_type.get_value_if_no_value(),
						));
						(None, c)
					}
				},
				lhit => match c.common_flags.find_long_flag(&long_flag) {
					Found::Name(c_flag) => match c.args.pop_front() {
						Some(next_long_flag) if self.long_flag(&next_long_flag) => {
							c.common_flags_values
								.push((long_flag, c_flag.flag_type.get_value_if_no_value()));
							self.parse_flags_start_with_long_flag(next_long_flag, c)
						}
						Some(next_short_flag) if self.flag(&next_short_flag) => {
							c.common_flags_values
								.push((long_flag, c_flag.flag_type.get_value_if_no_value()));
							self.parse_flags_start_with_short_flag(next_short_flag, c)
						}
						Some(next_arg) => match c_flag.flag_type.get_value_from_string(next_arg) {
							FlagValue::Invalid(next_arg) => {
								c.common_flags_values
									.push((long_flag, c_flag.flag_type.get_value_if_no_value()));
								(Some(next_arg), c)
							}
							val => {
								c.common_flags_values.push((long_flag, val));
								self.parse_next_if_flag(c)
							}
						},
						next_none => {
							c.common_flags_values
								.push((long_flag, c_flag.flag_type.get_value_if_no_value()));
							(next_none, c)
						}
					},
					Found::Long(c_flag) => match c.args.pop_front() {
						Some(next_long_flag) if self.long_flag(&next_long_flag) => {
							c.common_flags_values.push((
								c_flag.get_name_clone(),
								c_flag.derive_flag_value_if_no_value(),
							));
							self.parse_flags_start_with_long_flag(next_long_flag, c)
						}
						Some(next_short_flag) if self.flag(&next_short_flag) => {
							c.common_flags_values.push((
								c_flag.get_name_clone(),
								c_flag.derive_flag_value_if_no_value(),
							));
							self.parse_flags_start_with_short_flag(next_short_flag, c)
						}
						Some(next_arg) => match c_flag.flag_type.get_value_from_string(next_arg) {
							FlagValue::Invalid(next_arg) => {
								c.common_flags_values.push((
									c_flag.get_name_clone(),
									c_flag.derive_flag_value_if_no_value(),
								));
								(Some(next_arg), c)
							}
							val => {
								c.common_flags_values.push((c_flag.get_name_clone(), val));
								self.parse_next_if_flag(c)
							}
						},
						next_none => {
							c.common_flags_values.push((
								c_flag.get_name_clone(),
								c_flag.flag_type.get_value_if_no_value(),
							));
							(next_none, c)
						}
					},
					chit => {
						let flag_arg = MiddleArg::LongFlag(long_flag, FlagValue::None);
						c.error_info_list.push((
							flag_arg.clone(),
							ParseError::NoExistLong,
							ParseError::NoExistLong,
						));
						c.push_back_to_parsing_args(flag_arg);
						self.parse_next_if_flag(c)
					}
				},
			},
		}
	}

	pub fn parse_flags_start_with_short_flag(
		&self,
		mut short_flag: String,
		mut c: Context,
	) -> (Option<String>, Context) {
		match short_flag.find(self.eq) {
			Some(index) => {
				let after_eq = short_flag.split_off(index + 1);
				short_flag.pop();
				short_flag = self.get_short_flag_name(short_flag);
				match short_flag.pop() {
					None => {
						c.error_info_list.push((
							MiddleArg::ShortFlag(short_flag, FlagValue::String(after_eq)),
							ParseError::Empty,
							ParseError::Empty,
						));
						self.parse_next_if_flag(c)
					}
					Some(before_eq) => {
						let mut i = 0;
						for s in short_flag.chars() {
							match c.find_local_short_flag(&s) {
								Found::Short(l_flag) => {
									let record = (
										l_flag.get_name_clone(),
										l_flag.derive_flag_value_if_no_value(),
									);
									c.local_flags_values.push(record);
								}
								_ => match c.find_common_short_flag(&s) {
									Found::Short(c_flag) => {
										let record = (
											c_flag.get_name_clone(),
											c_flag.derive_flag_value_if_no_value(),
										);
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
											MiddleArg::ShortFlag(short_flag.clone(), FlagValue::None),
											ParseError::NoExistShort(i),
											ParseError::NoExistShort(i),
										))
									}
								},
							}
							i = i + 1;
						}
						//最後のフラグと値を処理
						match c.find_local_short_flag(&before_eq) {
							Found::Short(l_flag) => {
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
									Found::Short(c_flag) => {
										match c_flag.derive_flag_value_from_string(after_eq) {
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
										}
									}
									_ => {
										c.error_info_list.push((
											MiddleArg::ShortFlag(short_flag, FlagValue::String(after_eq)),
											ParseError::NoExistShort(i),
											ParseError::NoExistShort(i),
										));
									}
								}
							}
						}
						self.parse_next_if_flag(c)
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
							match c.find_local_short_flag(&s) {
								Found::Short(l_flag) => {
									let record = (
										l_flag.get_name_clone(),
										l_flag.derive_flag_value_if_no_value(),
									);
									c.local_flags_values.push(record);
								}
								_ => match c.find_common_short_flag(&s) {
									Found::Short(c_flag) => {
										let record = (
											c_flag.get_name_clone(),
											c_flag.derive_flag_value_if_no_value(),
										);
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
						match c.local_flags.find_short_flag(&last) {
							Found::Short(l_flag) => match c.args.pop_front() {
								Some(next_long_flag) if self.long_flag(&next_long_flag) => {
									c.local_flags_values.push((
										l_flag.get_name_clone(),
										l_flag.derive_flag_value_if_no_value(),
									));
									self.parse_flags_start_with_long_flag(next_long_flag, c)
								}
								Some(next_short_flag) if self.flag(&next_short_flag) => {
									c.local_flags_values.push((
										l_flag.get_name_clone(),
										l_flag.derive_flag_value_if_no_value(),
									));
									self.parse_flags_start_with_short_flag(next_short_flag, c)
								}
								Some(next_arg) => match l_flag.derive_flag_value_from_string(next_arg) {
									FlagValue::Invalid(next_arg) => {
										//
										c.local_flags_values.push((
											l_flag.get_name_clone(),
											l_flag.derive_flag_value_if_no_value(),
										));
										(Some(next_arg), c)
									}
									val => {
										c.local_flags_values.push((l_flag.get_name_clone(), val));
										self.parse_next_if_flag(c)
									}
								},
								next_none => {
									c.local_flags_values.push((
										l_flag.get_name_clone(),
										l_flag.derive_flag_value_if_no_value(),
									));
									(next_none, c)
								}
							},
							lhit => match c.common_flags.find_short_flag(&last) {
								Found::Short(c_flag) => match c.args.pop_front() {
									Some(next_long_flag) if self.long_flag(&next_long_flag) => {
										self.parse_flags_start_with_long_flag(next_long_flag, c)
									}
									Some(next_short_flag) if self.flag(&next_short_flag) => {
										self.parse_flags_start_with_short_flag(next_short_flag, c)
									}
									Some(next_arg) => match c_flag.derive_flag_value_from_string(next_arg) {
										FlagValue::Invalid(next_arg) => {
											c.common_flags_values.push((
												c_flag.get_name_clone(),
												c_flag.derive_flag_value_if_no_value(),
											));
											(Some(next_arg), c)
										}
										val => {
											c.common_flags_values.push((c_flag.get_name_clone(), val));
											self.parse_next_if_flag(c)
										}
									},
									next_none => {
										c.common_flags_values.push((
											c_flag.get_name_clone(),
											c_flag.derive_flag_value_if_no_value(),
										));
										(next_none, c)
									}
								},
								chit => {
									let flag_arg = MiddleArg::ShortFlag(short_flag, FlagValue::None);
									c.error_info_list.push((
										flag_arg.clone(),
										ParseError::NoExistShort(i),
										ParseError::NoExistShort(i),
									));
									c.push_back_to_parsing_args(flag_arg);
									self.parse_next_if_flag(c)
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
							ParseError::NoExist,
							ParseError::NoExist,
						));
						self.parse_next_if_flag(c)
					}
				}
			}
		}
	}

	pub fn parse_next_if_flag(&self, mut c: Context) -> (Option<String>, Context) {
		match c.args.pop_front() {
			Some(long_flag) if self.long_flag(&long_flag) => {
				self.parse_flags_start_with_long_flag(long_flag, c)
			}
			Some(short_flag) if self.flag(&short_flag) => {
				self.parse_flags_start_with_short_flag(short_flag, c)
			}
			val => (val, c),
		}
	}
}

#[derive(Debug, Clone)]
pub enum MiddleArg {
	Normal(String),
	LongFlag(String, FlagValue),
	ShortFlag(String, FlagValue),
}

impl MiddleArg {
	pub fn name(&self) -> &str {
		match &self {
			MiddleArg::LongFlag(name, _) => name,
			MiddleArg::ShortFlag(name, _) => name,
			MiddleArg::Normal(name) => name,
		}
	}

	pub fn val_if_string(&self) -> Option<&String> {
		match self {
			MiddleArg::LongFlag(_, FlagValue::String(val)) => Some(val),
			MiddleArg::ShortFlag(_, FlagValue::String(val)) => Some(val),
			MiddleArg::LongFlag(_, FlagValue::Invalid(val)) => Some(val),
			MiddleArg::ShortFlag(_, FlagValue::Invalid(val)) => Some(val),
			MiddleArg::LongFlag(_, _) => None,
			MiddleArg::ShortFlag(_, _) => None,
			MiddleArg::Normal(_) => None,
		}
	}
	pub fn inner_if_string_val(&self) -> Option<(&str, &str)> {
		match &self {
			MiddleArg::LongFlag(name, FlagValue::String(val)) => Some((name, val)),
			MiddleArg::ShortFlag(name, FlagValue::String(val)) => Some((name, val)),
			MiddleArg::LongFlag(name, FlagValue::Invalid(val)) => Some((name, val)),
			MiddleArg::ShortFlag(name, FlagValue::Invalid(val)) => Some((name, val)),
			MiddleArg::LongFlag(_, _) => None,
			MiddleArg::ShortFlag(_, _) => None,
			MiddleArg::Normal(_) => None,
		}
	}

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

	pub fn get_flag_type_str<'a>(&self) -> &'a str {
		match self {
			MiddleArg::LongFlag(_, _) => "long",
			MiddleArg::ShortFlag(_, _) => "short",
			MiddleArg::Normal(_) => "non",
		}
	}
}

type Index = usize;
#[derive(Debug)]
pub enum ParseError {
	NoExistLong,
	NoExistShort(Index),
	//DifferentForm(Found<String>),
	InvalidShort(Index, String),
	InvalidLong(String),
	NoExist,
	NotParsed,
	Empty,
	None,
}

pub type ErrorInfo = (MiddleArg, ParseError, ParseError);

pub fn gen_error_description(err_info: &ErrorInfo) -> String {
	match err_info {
		(flag_arg, ParseError::NoExistShort(i), ParseError::NoExistShort(j)) => {
			let name = flag_arg.name();
			format!(
				"The short flag {} in {} is an unknown short flag.",
				name.chars().nth(*i).unwrap(),
				name
			)
		}
		(flag_arg, ParseError::NoExistLong, ParseError::NoExistLong) => {
			format!("The flag {} is an unknown flag.", flag_arg.name())
		}
		(flag_arg, ParseError::NoExistShort(_), ParseError::InvalidShort(i, c_flag)) => {
			let (name, val) = flag_arg.inner_if_string_val().unwrap();
			format!(
				"The value of short flag {} in {} - {} is not valid for a common flag {}.",
				name.chars().nth(*i).unwrap(),
				name,
				val,
				c_flag
			)
		}
		(flag_arg, ParseError::NoExistLong, ParseError::InvalidLong(chit)) => {
			let (name, val) = flag_arg.inner_if_string_val().unwrap();
			format!(
				"The flag {} matches a common flag {}. But its value {} is invalid for {}.",
				name,
				chit,
				val,
				flag_arg.get_flag_type_str()
			)
		}
		(flag_arg, ParseError::InvalidShort(i, l_flag), _) => {
			let (name, val) = flag_arg.inner_if_string_val().unwrap();
			format!(
				"The flag {}{}'s value {} is not valid for a local flag {}.",
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
			)
		}
		(flag_arg, local_error, common_error) => {
			//どれでもない状況になったとき
			format!(
				"flag: {:?}, error in parse local flag: {:?}, error in parse common flag: {:?}",
				flag_arg, local_error, common_error
			)
		}
	}
}

pub(crate) fn get_form_str_and_name<'a, 'f: 'a>(f: &'f Found<String>) -> (&'a str, &'a str) {
	match f {
		Found::Name(f_name) => ("", f_name),
		Found::Long(f_name) => ("a long form of", f_name),
		Found::Short(f_name) => ("a short form of ", f_name),
		Found::None => ("not any form of", "unknown - error"),
	}
}
