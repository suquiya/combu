use crate::{
	vector::flag::{FlagSearch, LongFound},
	Context, FlagValue,
};
use std::collections::VecDeque;

/// Struct of information for parse
pub struct Parser {
	/// flag_pattern. Default is '-'.
	pub flag_pattern: char,
	/// Long-flag prefix. Default is "--".
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

impl Parser {
	/// Creates a new Parser with flag_pattern and long_flag_prefix.
	pub fn new(flag_pattern: char, long_flag_prefix: &str) -> Parser {
		Parser {
			flag_pattern,
			long_flag_prefix: String::from(long_flag_prefix),
			eq: '=',
		}
	}

	/// Returns true if str is long-flag.
	/// ロングフラグか判定する
	pub fn long_flag(&self, str: &str) -> bool {
		str.starts_with(&self.long_flag_prefix)
	}

	/// Returns true if str is flag.
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
	pub fn parse_inter_mediate_args(
		&self,
		mut c: Context,
		flag_only: bool,
	) -> (Context, Option<VecDeque<String>>) {
		match c.parsing_args {
			None => (c, None),
			Some(mut inter_middle_args) => {
				//let _inter_middle_args = inter_middle_args.clone();
				let mut non_flag_args = VecDeque::<String>::new();
				c.parsing_args = None;
				loop {
					match inter_middle_args.pop_front() {
						Some(MiddleArg::LongFlag(long_flag, flag_val)) => {
							match c.local_flags.find_long_flag(&long_flag) {
								LongFound::Name(l_flag) => match flag_val {
									FlagValue::String(_) if l_flag.flag_type.is_string() => {
										c.local_flags_values.push((long_flag, flag_val));
									}
									FlagValue::String(val) => {
										match l_flag.derive_flag_value_from_string(val) {
											FlagValue::Invalid(val) => {
												let flag_arg = MiddleArg::LongFlag(
													long_flag.clone(),
													FlagValue::String(val),
												);
												c.error_info_list.push((
													flag_arg.clone(),
													ParseError::InvalidLong(long_flag),
													ParseError::NotParsed,
												));
												c.push_back_to_parsing_args(flag_arg);
											}
											val => {
												c.local_flags_values.push((long_flag, val));
											}
										}
									}
									FlagValue::None => match inter_middle_args.front() {
										Some(MiddleArg::Normal(_)) => {
											if let Some(MiddleArg::Normal(val)) = inter_middle_args.pop_front()
											{
												match l_flag.derive_flag_value_from_string(val) {
													FlagValue::Invalid(val) => {
														c.local_flags_values.push((
															long_flag.clone(),
															l_flag.derive_flag_value_if_no_value(),
														));
														if flag_only {
															let arg = MiddleArg::Normal(val);
															c.error_info_list.push((
																arg.clone(),
																ParseError::InvalidLong(long_flag),
																ParseError::NotParsed,
															));
															c.push_back_to_parsing_args(arg);
														} else {
															non_flag_args.push_back(val);
														}
													}
													val => {
														c.local_flags_values.push((long_flag, val));
													}
												}
											} else {
												panic!("This panic should be unreachable.");
											}
										}
										None => {
											c.local_flags_values
												.push((long_flag, l_flag.derive_flag_value_if_no_value()));
											break;
										}
										_ => {
											c.local_flags_values
												.push((long_flag, l_flag.derive_flag_value_if_no_value()));
										}
									},
									_ => {
										let flag_arg = MiddleArg::LongFlag(long_flag, flag_val);
										c.error_info_list.push((
											flag_arg.clone(),
											ParseError::InvalidLong(l_flag.get_name_clone()),
											ParseError::NotParsed,
										));
										c.push_back_to_parsing_args(flag_arg);
									}
								},
								LongFound::Long(l_flag) => match flag_val {
									FlagValue::String(_) if l_flag.flag_type.is_string() => {
										c.local_flags_values
											.push((l_flag.get_name_clone(), flag_val));
									}
									FlagValue::String(val) => {
										match l_flag.derive_flag_value_from_string(val) {
											FlagValue::Invalid(val) => {
												c.local_flags_values.push((
													l_flag.get_name_clone(),
													l_flag.derive_flag_value_if_no_value(),
												));
												let arg = MiddleArg::Normal(val);
												c.error_info_list.push((
													arg.clone(),
													ParseError::InvalidLong(l_flag.get_name_clone()),
													ParseError::NotParsed,
												));
												c.push_back_to_parsing_args(arg);
											}
											val => {
												c.local_flags_values.push((l_flag.get_name_clone(), val));
											}
										}
									}
									FlagValue::None => match inter_middle_args.front() {
										Some(MiddleArg::Normal(_)) => {
											if let Some(MiddleArg::Normal(val)) = inter_middle_args.pop_front()
											{
												match l_flag.derive_flag_value_from_string(val) {
													FlagValue::Invalid(val) => {
														c.local_flags_values.push((
															l_flag.get_name_clone(),
															l_flag.derive_flag_value_if_no_value(),
														));
														if flag_only {
															let arg = MiddleArg::Normal(val);
															c.error_info_list.push((
																arg.clone(),
																ParseError::InvalidLong(l_flag.get_name_clone()),
																ParseError::NotParsed,
															));
															c.push_back_to_parsing_args(arg);
														} else {
															non_flag_args.push_back(val);
														}
													}
													val => {
														c.local_flags_values.push((l_flag.get_name_clone(), val));
													}
												}
											} else {
												panic!("This panic should be unreachable.");
											}
										}
										_ => {
											c.local_flags_values.push((
												l_flag.get_name_clone(),
												l_flag.derive_flag_value_if_no_value(),
											));
										}
									},
									_ => {
										let flag_arg = MiddleArg::LongFlag(long_flag, flag_val);
										c.error_info_list.push((
											flag_arg.clone(),
											ParseError::InvalidLong(l_flag.get_name_clone()),
											ParseError::NotParsed,
										));
										c.push_back_to_parsing_args(flag_arg);
									}
								},
								_ => {
									match c.common_flags.find_long_flag(&long_flag) {
										LongFound::Name(c_flag) => match flag_val {
											FlagValue::String(_) if c_flag.flag_type.is_string() => {
												c.common_flags_values.push((long_flag, flag_val));
											}
											FlagValue::String(val) => {
												match c_flag.derive_flag_value_from_string(val) {
													FlagValue::Invalid(val) => {
														c.local_flags_values.push((
															long_flag,
															c_flag.derive_flag_value_if_no_value(),
														));
														let arg = MiddleArg::Normal(val);
														c.error_info_list.push((
															arg.clone(),
															ParseError::NoExistLong,
															ParseError::InvalidLong(c_flag.get_name_clone()),
														));
													}
													val => {
														c.common_flags_values.push((long_flag, val));
													}
												}
											}
											FlagValue::None => match inter_middle_args.front() {
												Some(MiddleArg::Normal(_)) => {
													if let Some(MiddleArg::Normal(val)) =
														inter_middle_args.pop_front()
													{
														match c_flag.derive_flag_value_from_string(val) {
															FlagValue::Invalid(val) => {
																c.common_flags_values.push((
																	long_flag.clone(),
																	c_flag.derive_flag_value_if_no_value(),
																));
																if flag_only {
																	let arg = MiddleArg::Normal(val);
																	c.error_info_list.push((
																		arg.clone(),
																		ParseError::InvalidLong(long_flag),
																		ParseError::NotParsed,
																	));
																	c.push_back_to_parsing_args(arg);
																} else {
																	non_flag_args.push_back(val);
																}
															}
															val => {
																c.common_flags_values.push((long_flag, val));
															}
														}
													} else {
														panic!("This panic should be unreachable.");
													}
												}
												None => {
													c.common_flags_values.push((
														long_flag,
														c_flag.derive_flag_value_if_no_value(),
													));
													break;
												}
												_ => {
													c.common_flags_values.push((
														long_flag,
														c_flag.derive_flag_value_if_no_value(),
													));
												}
											},
											_ => {
												let flag_arg = MiddleArg::LongFlag(long_flag, flag_val);
												c.error_info_list.push((
													flag_arg.clone(),
													ParseError::NoExistLong,
													ParseError::InvalidLong(c_flag.get_name_clone()),
												));
												c.push_back_to_parsing_args(flag_arg);
											}
										},
										LongFound::Long(c_flag) => {
											match flag_val {
												FlagValue::String(_) if c_flag.flag_type.is_string() => {
													c.common_flags_values
														.push((c_flag.get_name_clone(), flag_val));
												}
												FlagValue::String(val) => {
													match c_flag.derive_flag_value_from_string(val) {
														FlagValue::Invalid(val) => {
															c.common_flags_values.push((
																c_flag.get_name_clone(),
																c_flag.derive_flag_value_if_no_value(),
															));
															let arg = MiddleArg::Normal(val);
															c.error_info_list.push((
																arg.clone(),
																ParseError::NoExistLong,
																ParseError::InvalidLong(c_flag.get_name_clone()),
															));
															c.push_back_to_parsing_args(arg);
														}
														val => {
															c.common_flags_values
																.push((c_flag.get_name_clone(), val));
														}
													}
												}
												FlagValue::None => {
													match inter_middle_args.front() {
														Some(MiddleArg::Normal(_)) => {
															if let Some(MiddleArg::Normal(val)) =
																inter_middle_args.pop_front()
															{
																match c_flag.derive_flag_value_from_string(val) {
																	FlagValue::Invalid(val) => {
																		c.local_flags_values.push((
																			c_flag.get_name_clone(),
																			c_flag.derive_flag_value_if_no_value(),
																		));
																		if flag_only {
																			let arg = MiddleArg::Normal(val);
																			c.error_info_list.push((
																				arg.clone(),
																				ParseError::NoExistLong,
																				ParseError::InvalidLong(
																					c_flag.get_name_clone(),
																				),
																			));
																		} else {
																			non_flag_args.push_back(val);
																		}
																	}
																	val => {
																		c.common_flags_values
																			.push((c_flag.get_name_clone(), val));
																	}
																}
															} else {
																panic!("This panic should be unreachable.")
															}
														}
														None => {
															c.common_flags_values.push((
																c_flag.get_name_clone(),
																c_flag.derive_flag_value_if_no_value(),
															));
															break;
														}
														_ => {
															c.common_flags_values.push((
																c_flag.get_name_clone(),
																c_flag.derive_flag_value_if_no_value(),
															));
														}
													};
												}
												_ => {
													let flag_arg = MiddleArg::LongFlag(long_flag, flag_val);
													c.error_info_list.push((
														flag_arg.clone(),
														ParseError::NoExistLong,
														ParseError::InvalidLong(c_flag.get_name_clone()),
													));
													c.push_back_to_parsing_args(flag_arg);
												}
											};
										}
										_ => {
											let flag_arg = MiddleArg::LongFlag(long_flag, flag_val);
											c.error_info_list.push((
												flag_arg.clone(),
												ParseError::NoExistLong,
												ParseError::NoExistLong,
											));
											c.push_back_to_parsing_args(flag_arg);
										}
									};
								}
							}
						}
						Some(MiddleArg::ShortFlag(mut short_flag, flag_val)) => {
							match short_flag.pop() {
								Some(last) => {
									let mut i: usize = 0;
									for s in short_flag.chars() {
										match c.find_local_short_flag(&s) {
											Some(l_flag) => {
												let record = (
													l_flag.get_name_clone(),
													l_flag.derive_flag_value_if_no_value(),
												);
												c.local_flags_values.push(record);
											}
											_ => {
												match c.find_common_short_flag(&s) {
													Some(c_flag) => {
														let record = (
															c_flag.get_name_clone(),
															c_flag.derive_flag_value_if_no_value(),
														);
														c.local_flags_values.push(record);
													}
													_ => {
														c.push_back_to_parsing_args(MiddleArg::ShortFlag(
															short_flag.clone(),
															FlagValue::None,
														));
														c.error_info_list.push((
															MiddleArg::ShortFlag(
																{
																	let mut short_flag = short_flag.clone();
																	short_flag.push(last);
																	short_flag
																},
																FlagValue::None,
															),
															ParseError::NoExistShort(i),
															ParseError::NoExistShort(i),
														))
													}
												};
											}
										}
										i = i + 1;
									}
									match c.find_local_short_flag(&last) {
										Some(l_flag) => match flag_val {
											FlagValue::String(_) if l_flag.flag_type.is_string() => {
												let record = (l_flag.get_name_clone(), flag_val);
												c.local_flags_values.push(record);
											}
											FlagValue::String(val) => {
												match l_flag.derive_flag_value_from_string(val) {
													FlagValue::Invalid(val) => {
														let flag_val = FlagValue::String(val);
														let record = (
															MiddleArg::ShortFlag(
																{
																	short_flag.push(last);
																	short_flag
																},
																flag_val.clone(),
															),
															ParseError::InvalidShort(i, l_flag.get_name_clone()),
															ParseError::NotParsed,
														);
														c.error_info_list.push(record);
														c.push_back_to_parsing_args(MiddleArg::ShortFlag(
															last.to_string(),
															flag_val,
														));
													}
													val => {
														let record = (l_flag.get_name_clone(), val);
														c.local_flags_values.push(record);
													}
												}
											}
											FlagValue::None => match inter_middle_args.front() {
												Some(MiddleArg::Normal(_)) => {
													match inter_middle_args.pop_front() {
														Some(MiddleArg::Normal(val)) => {
															match l_flag.derive_flag_value_from_string(val) {
																FlagValue::Invalid(val) => {
																	let l_flag_name = l_flag.get_name_clone();
																	let l_flag_val =
																		l_flag.derive_flag_value_if_no_value();
																	if flag_only {
																		c.local_flags_values
																			.push((l_flag_name.clone(), l_flag_val));
																		let arg = MiddleArg::Normal(val);
																		let local_error =
																			ParseError::InvalidShort(i, l_flag_name);
																		c.error_info_list.push((
																			arg.clone(),
																			local_error,
																			ParseError::NotParsed,
																		));
																	} else {
																		c.local_flags_values
																			.push((l_flag_name, l_flag_val));
																		non_flag_args.push_back(val);
																	}
																}
																val => {
																	let record = (l_flag.get_name_clone(), val);
																	c.local_flags_values.push(record);
																}
															}
														}
														_ => panic!("This panic should be unreachable."),
													}
												}
												None => {
													let record = (
														l_flag.get_name_clone(),
														l_flag.derive_flag_value_if_no_value(),
													);
													c.local_flags_values.push(record);
													break;
												}
												_ => {
													let record = (
														l_flag.get_name_clone(),
														l_flag.derive_flag_value_if_no_value(),
													);
													c.local_flags_values.push(record);
												}
											},
											_ => {
												let flag_arg = MiddleArg::ShortFlag(
													{
														short_flag.push(last);
														short_flag
													},
													flag_val.clone(),
												);
												let local_error =
													ParseError::InvalidShort(i, l_flag.get_name_clone());
												c.error_info_list.push((
													flag_arg,
													local_error,
													ParseError::NotParsed,
												));
												c.push_back_to_parsing_args(MiddleArg::ShortFlag(
													last.to_string(),
													flag_val,
												));
											}
										},
										_ => {
											match c.find_common_short_flag(&last) {
												Some(c_flag) => match flag_val {
													FlagValue::String(_) if c_flag.flag_type.is_string() => {
														let record = (c_flag.get_name_clone(), flag_val);
														c.common_flags_values.push(record);
													}
													FlagValue::String(val) => {
														match c_flag.derive_flag_value_from_string(val) {
															FlagValue::Invalid(val) => {
																let flag_val = FlagValue::String(val);
																let c_flag_name = c_flag.get_name_clone();
																c.error_info_list.push((
																	MiddleArg::ShortFlag(
																		{
																			short_flag.push(last);
																			short_flag
																		},
																		flag_val.clone(),
																	),
																	ParseError::NoExistLong,
																	ParseError::InvalidShort(i, c_flag_name),
																));
																c.push_back_to_parsing_args(MiddleArg::ShortFlag(
																	last.to_string(),
																	flag_val,
																));
															}
															val => {
																let record = (c_flag.get_name_clone(), val);
																c.common_flags_values.push(record);
															}
														}
													}
													FlagValue::None => match inter_middle_args.front() {
														Some(MiddleArg::Normal(_)) => {
															if let Some(MiddleArg::Normal(val)) =
																inter_middle_args.pop_front()
															{
																match c_flag.derive_flag_value_from_string(val) {
																	FlagValue::Invalid(val) => {
																		let c_flag_name = c_flag.get_name_clone();
																		let c_flag_val =
																			c_flag.derive_flag_value_if_no_value();
																		if flag_only {
																			let arg = MiddleArg::Normal(val);
																			c.error_info_list.push((
																				arg.clone(),
																				ParseError::NoExistShort(i),
																				ParseError::InvalidShort(
																					i,
																					c_flag_name,
																				),
																			));
																			c.push_back_to_parsing_args(arg);
																		} else {
																			c.common_flags_values
																				.push((c_flag_name, c_flag_val));
																			non_flag_args.push_back(val);
																		}
																	}
																	val => {
																		let record = (c_flag.get_name_clone(), val);
																		c.common_flags_values.push(record);
																	}
																}
															} else {
																panic!("This panic should be unreachable.");
															}
														}
														None => {
															let record = (
																c_flag.get_name_clone(),
																c_flag.derive_flag_value_if_no_value(),
															);
															c.common_flags_values.push(record);
															break;
														}
														_ => {
															let record = (
																c_flag.get_name_clone(),
																c_flag.derive_flag_value_if_no_value(),
															);
															c.local_flags_values.push(record);
														}
													},
													_ => {
														let flag_arg = MiddleArg::ShortFlag(
															{
																short_flag.push(last);
																short_flag
															},
															flag_val.clone(),
														);
														let c_flag_name = c_flag.get_name_clone();
														c.error_info_list.push((
															flag_arg,
															ParseError::NoExistShort(i),
															ParseError::InvalidShort(i, c_flag_name),
														));
														c.push_back_to_parsing_args(MiddleArg::ShortFlag(
															last.to_string(),
															flag_val,
														));
													}
												},
												_ => {
													//ローカルフラグにもコモンフラグにもヒットしなかった場合
													c.error_info_list.push((
														MiddleArg::ShortFlag(
															{
																short_flag.push(last);
																short_flag
															},
															flag_val.clone(),
														),
														ParseError::NoExistShort(i),
														ParseError::NoExistShort(i),
													));
													c.push_back_to_parsing_args(MiddleArg::ShortFlag(
														last.to_string(),
														flag_val,
													));
												}
											}
										}
									}
								}
								None => {
									//フラグの値がない（-のみ）の場合
									c.error_info_list.push((
										MiddleArg::ShortFlag(String::new(), flag_val.clone()),
										ParseError::Empty,
										ParseError::Empty,
									));
									c.push_back_to_parsing_args(MiddleArg::ShortFlag(
										String::new(),
										flag_val,
									));
								}
							}
						}
						None => {
							break;
						}
						Some(val) => {
							if flag_only {
								c.error_info_list.push((
									val.clone(),
									ParseError::NotExist,
									ParseError::NotExist,
								));
								c.push_back_to_parsing_args(val);
							} else if let MiddleArg::Normal(val) = val {
								non_flag_args.push_back(val);
							} else {
								panic!("This panic should not be unreachable.");
							}
						}
					}
				}
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
	pub fn parse_middle_long_flag(
		&self,
		name_or_alias: String,
		val: FlagValue,
		c: &Context,
		mut l_flags: VecDeque<(String, FlagValue)>,
		mut c_flags: VecDeque<(String, FlagValue)>,
		mut e_list: VecDeque<ErrorInfo>,
	) -> (
		VecDeque<(String, FlagValue)>,
		VecDeque<(String, FlagValue)>,
		VecDeque<ErrorInfo>,
	) {
		match c.find_local_long_flag(&name_or_alias) {
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
						l_flags.push_front((name_or_alias, l_flag.derive_flag_value_if_no_value()));
					}
					_ => l_flags.push_front((name_or_alias, l_flag.derive_flag_value_if_no_value())),
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
					l_flags.push_front((
						l_flag.get_name_clone(),
						l_flag.derive_flag_value_if_no_value(),
					));
				}
				_ => {
					l_flags.push_front((
						l_flag.get_name_clone(),
						l_flag.derive_flag_value_if_no_value(),
					));
				}
			},
			LongFound::None => match c.find_common_long_flag(&name_or_alias) {
				LongFound::Name(c_flag) => match val {
					FlagValue::None => {
						c_flags.push_front((name_or_alias, c_flag.derive_flag_value_if_no_value()));
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
					_ => c_flags.push_front((name_or_alias, c_flag.derive_flag_value_if_no_value())),
				},
				LongFound::Long(c_flag) => match val {
					FlagValue::None => c_flags.push_front((
						c_flag.get_name_clone(),
						c_flag.derive_flag_value_if_no_value(),
					)),
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
					_ => {
						c_flags.push_front((
							c_flag.get_name_clone(),
							c_flag.derive_flag_value_if_no_value(),
						));
					}
				},
				LongFound::None => {
					e_list.push_front((
						MiddleArg::LongFlag(name_or_alias, val),
						ParseError::NoExistLong,
						ParseError::NoExistLong,
					));
				}
			},
		}
		(l_flags, c_flags, e_list)
	}

	/// Parse middle short flag
	pub fn parse_middle_short_flag(
		&self,
		mut short_alias: String,
		flag_val: FlagValue,
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
			Some(last) => match c.find_local_short_flag(&last) {
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
						l_flags.push_front((
							l_flag.get_name_clone(),
							l_flag.derive_flag_value_if_no_value(),
						));
					}
					_ => {
						l_flags.push_front((
							l_flag.get_name_clone(),
							l_flag.derive_flag_value_if_no_value(),
						));
					}
				},
				None => match c.find_common_short_flag(&last) {
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
							c_flags.push_front((
								c_flag.get_name_clone(),
								c_flag.derive_flag_value_if_no_value(),
							));
						}
						_ => {
							c_flags.push_front((
								c_flag.get_name_clone(),
								c_flag.derive_flag_value_if_no_value(),
							));
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
				panic!("not short flag included");
			}
		}

		(l_flags, c_flags, e_list)
	}

	pub fn parse_middle_normal_arg(
		&self,
		mut inter_mediate_args: VecDeque<MiddleArg>,
		mut normal_arg: String,
		mut c: Context,
		mut non_flag_args: VecDeque<String>,
		mut l_flags: VecDeque<(String, FlagValue)>,
		mut c_flags: VecDeque<(String, FlagValue)>,
		mut e_list: VecDeque<ErrorInfo>,
	) -> (
		VecDeque<MiddleArg>,
		VecDeque<(String, FlagValue)>,
		VecDeque<(String, FlagValue)>,
		VecDeque<ErrorInfo>,
	) {
		match inter_mediate_args.pop_back() {
			Some(MiddleArg::LongFlag(_, _)) => {}
			Some(MiddleArg::ShortFlag(_, _)) => {}
			Some(val) => non_flag_args.push_front(normal_arg),
			None => {}
		}
		(inter_mediate_args, l_flags, c_flags, e_list)
	}

	/// Parses args if next middle args exist.
	pub fn parse_next_if_middle_arg(
		&self,
		mut inter_mediate_args: VecDeque<MiddleArg>,
		mut non_flag_args: VecDeque<String>,
		c: &Context,
		l_flags: VecDeque<(String, FlagValue)>,
		c_flags: VecDeque<(String, FlagValue)>,
		e_list: VecDeque<ErrorInfo>,
		flag_only: bool,
	) -> (
		VecDeque<MiddleArg>,
		VecDeque<String>,
		VecDeque<(String, FlagValue)>,
		VecDeque<(String, FlagValue)>,
		VecDeque<ErrorInfo>,
	) {
		match inter_mediate_args.pop_back() {
			Some(MiddleArg::LongFlag(long_flag, flag_val)) => {
				let (l_flags, c_flags, e_list) =
					self.parse_middle_long_flag(long_flag, flag_val, &c, l_flags, c_flags, e_list);
				//(inter_mediate_args, non_flag_args, l_flags, c_flags, e_list)
				self.parse_next_if_middle_arg(
					inter_mediate_args,
					non_flag_args,
					c,
					l_flags,
					c_flags,
					e_list,
					flag_only,
				)
			}
			Some(MiddleArg::ShortFlag(short_flag, flag_val)) => {
				let (l_flags, c_flags, e_list) =
					self.parse_middle_short_flag(short_flag, flag_val, &c, l_flags, c_flags, e_list);
				self.parse_next_if_middle_arg(
					inter_mediate_args,
					non_flag_args,
					c,
					l_flags,
					c_flags,
					e_list,
					flag_only,
				)
				//(inter_mediate_args, non_flag_args, l_flags, c_flags, e_list)
			}
			Some(MiddleArg::Normal(arg)) => {
				non_flag_args.push_front(arg);
				(inter_mediate_args, non_flag_args, l_flags, c_flags, e_list)
			}
			None => (inter_mediate_args, non_flag_args, l_flags, c_flags, e_list),
		}
	}

	/// Parse args until args' end.
	pub fn parse_args_until_end(self, mut c: Context) -> Context {
		let mut non_flag_args = VecDeque::<String>::new();

		loop {
			//println!("{:?}", c);
			match c.args.pop_front() {
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
		c.args = non_flag_args;
		c
	}

	/// Parses flags start with long flag until non-flag arg appeared.
	pub fn parse_flags_start_with_long_flag(
		&self,
		mut long_flag: String,
		mut c: Context,
	) -> (Option<String>, Context) {
		long_flag = self.remove_long_flag_prefix(long_flag);
		match long_flag.find(self.eq) {
			Some(index) => {
				let after_eq = long_flag.split_off(index + 1);
				long_flag.pop();
				match c.local_flags.find_long_flag(&long_flag) {
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
					_ => match c.common_flags.find_long_flag(&long_flag) {
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
				self.parse_next_if_flag(c)
			}
			None => match c.local_flags.find_long_flag(&long_flag) {
				LongFound::Name(l_flag) => match c.args.pop_front() {
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
				LongFound::Long(l_flag) => match c.args.pop_front() {
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
				_ => match c.common_flags.find_long_flag(&long_flag) {
					LongFound::Name(c_flag) => match c.args.pop_front() {
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
					LongFound::Long(c_flag) => match c.args.pop_front() {
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
					_ => {
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

	/// Parses flags start with short flag until args appeared.
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
						let record = MiddleArg::ShortFlag(short_flag, FlagValue::String(after_eq));
						c.error_info_list
							.push((record.clone(), ParseError::Empty, ParseError::Empty));
						c.push_back_to_parsing_args(record);
						self.parse_next_if_flag(c)
					}
					Some(before_eq) => {
						let mut i = 0;
						for s in short_flag.chars() {
							match c.find_local_short_flag(&s) {
								Some(l_flag) => {
									let record = (
										l_flag.get_name_clone(),
										l_flag.derive_flag_value_if_no_value(),
									);
									c.local_flags_values.push(record);
								}
								_ => match c.find_common_short_flag(&s) {
									Some(c_flag) => {
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
						match c.find_local_short_flag(&before_eq) {
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
								Some(l_flag) => {
									let record = (
										l_flag.get_name_clone(),
										l_flag.derive_flag_value_if_no_value(),
									);
									c.local_flags_values.push(record);
								}
								_ => match c.find_common_short_flag(&s) {
									Some(c_flag) => {
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
							Some(l_flag) => match c.args.pop_front() {
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
							_ => match c.common_flags.find_short_flag(&last) {
								Some(c_flag) => match c.args.pop_front() {
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
								_ => {
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
							ParseError::NotExist,
							ParseError::NotExist,
						));
						self.parse_next_if_flag(c)
					}
				}
			}
		}
	}

	/// Parses(or assigns parse functions) args if next args.
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

/// Generates error description.
pub fn gen_error_description(err_info: &ErrorInfo) -> String {
	match err_info {
		(flag_arg, ParseError::NoExistShort(i), ParseError::NoExistShort(_)) => {
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
				"arg: {:?}, error in parse local flag: {:?}, error in parse common flag: {:?}",
				flag_arg, local_error, common_error
			)
		}
	}
}
