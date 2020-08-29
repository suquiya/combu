use crate::parser::{ErrorInfo, MiddleArg};
use crate::vector::flag::{FlagSearch, LongFound};
use crate::Vector;
use crate::{Flag, FlagValue};
use std::collections::VecDeque;
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub struct Context {
	pub raw_args: Vec<String>,
	pub args: VecDeque<String>,
	pub common_flags: Vector<Vector<Flag>>,
	pub routes: Vector<String>,
	pub local_flags: Vector<Flag>,
	pub current_path: PathBuf,
	pub common_flags_values: Vector<(String, FlagValue)>,
	pub local_flags_values: Vector<(String, FlagValue)>,
	pub parsing_args: Option<VecDeque<MiddleArg>>,
	pub error_info_list: Vector<ErrorInfo>,
}

impl Context {
	pub fn new<T: Into<PathBuf>>(
		raw_args: Vec<String>,
		args: VecDeque<String>,
		common_flags: Vector<Flag>,
		local_flags: Vector<Flag>,
		current_path: T,
	) -> Context {
		Context {
			raw_args,
			args,
			common_flags: Vector(Some(vec![common_flags])),
			routes: Vector::default(),
			local_flags,
			current_path: current_path.into(),
			common_flags_values: Vector::default(),
			local_flags_values: Vector::default(),
			parsing_args: None,
			error_info_list: Vector::default(),
		}
	}
	pub fn build_new(
		raw_args: Vec<String>,
		args: VecDeque<String>,
		common_flags: Vector<Vector<Flag>>,
		local_flags: Vector<Flag>,
		current_path: PathBuf,
		routes: Vector<String>,
		common_flags_values: Vector<(String, FlagValue)>,
		local_flags_values: Vector<(String, FlagValue)>,
		parsing_args: Option<VecDeque<MiddleArg>>,
		error_info_list: Vector<ErrorInfo>,
	) -> Context {
		Context {
			raw_args,
			args,
			common_flags,
			routes,
			local_flags,
			current_path,
			common_flags_values,
			local_flags_values,
			parsing_args,
			error_info_list,
		}
	}

	pub fn root<'a>() -> Option<Context> {
		None
	}

	pub fn args(mut self, args: VecDeque<String>) -> Self {
		self.args = args;
		self
	}

	#[inline]
	pub fn next_arg(&mut self) -> Option<String> {
		self.args.pop_front()
	}

	pub fn current(&self) -> &Path {
		&self.current_path
	}

	pub fn change_current(mut self, path: PathBuf) {
		self.current_path = path;
	}

	#[inline]
	pub fn find_local_long_flag(&self, name_or_alias: &str) -> LongFound<&Flag> {
		self.local_flags.find_long_flag(name_or_alias)
	}

	#[inline]
	pub fn find_local_short_flag(&self, short_alias: &char) -> Option<&Flag> {
		self.local_flags.find_short_flag(short_alias)
	}

	#[inline]
	pub fn find_common_long_flag(&self, name_or_alias: &str) -> LongFound<&Flag> {
		self.common_flags.find_long_flag(name_or_alias)
	}

	#[inline]
	pub fn find_common_short_flag(&self, short_alias: &char) -> Option<&Flag> {
		self.common_flags.find_short_flag(short_alias)
	}

	pub fn push_back_to_parsing_args(&mut self, middle_arg: MiddleArg) {
		match self.parsing_args {
			None => {
				self.parsing_args = Some({
					let mut inner = VecDeque::new();
					inner.push_back(middle_arg);
					inner
				})
			}
			Some(ref mut vd) => (*vd).push_back(middle_arg),
		}
	}

	pub fn take_flag_value_of(&mut self, flag_name: &str) -> Option<FlagValue> {
		match self.take_local_flag_value_of(flag_name) {
			None => self.take_common_flag_value_of(flag_name),
			val => val,
		}
	}

	pub fn take_inputted_flag_value_of(&mut self, flag_name: &str) -> Option<FlagValue> {
		match self.take_inputted_local_flag_value_of(flag_name) {
			None => self.take_inputted_common_flag_value_of(flag_name),
			val => val,
		}
	}

	pub fn take_local_flag_value_of(&mut self, flag_name: &str) -> Option<FlagValue> {
		match self.take_inputted_common_flag_value_of(flag_name) {
			None => match self.local_flags.find(flag_name) {
				None => None,
				Some(f) => Some(f.default_value.clone()),
			},
			val => val,
		}
	}

	pub fn take_common_flag_value_of(&mut self, flag_name: &str) -> Option<FlagValue> {
		match self.take_inputted_common_flag_value_of(flag_name) {
			None => match self.common_flags.find(flag_name) {
				None => None,
				Some(f) => Some(f.default_value.clone()),
			},
			val => val,
		}
	}

	pub fn take_inputted_local_flag_value_of(&mut self, flag_name: &str) -> Option<FlagValue> {
		match self.local_flags_values {
			Vector(None) => None,
			Vector(Some(ref mut local)) => {
				match local.iter().position(|(name, _)| name == flag_name) {
					Some(index) => {
						let (_, val) = local.remove(index);
						Some(val)
					}
					None => None,
				}
			}
		}
	}

	pub fn take_inputted_common_flag_value_of(&mut self, flag_name: &str) -> Option<FlagValue> {
		match self.common_flags_values {
			Vector(None) => None,
			Vector(Some(ref mut common)) => {
				match common.iter().position(|(name, _)| name == flag_name) {
					Some(index) => {
						let (_, val) = common.remove(index);
						Some(val)
					}
					None => None,
				}
			}
		}
	}

	pub fn get_flag_value_of(&self, flag_name: &str) -> Option<FlagValue> {
		match self.get_local_flag_value_of(flag_name) {
			None => self.get_common_flag_value_of(flag_name),
			flag_val => flag_val,
		}
	}

	pub fn get_inputted_flag_value_of(&self, flag_name: &str) -> Option<FlagValue> {
		match self.get_inputted_local_flag_value_of(flag_name) {
			None => self.get_inputted_common_flag_value_of(flag_name),
			flag_val => flag_val,
		}
	}

	pub fn get_common_flag_value_of(&self, flag_name: &str) -> Option<FlagValue> {
		match self.get_inputted_common_flag_value_of(flag_name) {
			None | Some(FlagValue::None) => match self.common_flags.find(flag_name) {
				Some(f) => Some(f.default_value.clone()),
				None => None,
			},
			val => val,
		}
	}

	pub fn get_local_flag_value_of(&self, flag_name: &str) -> Option<FlagValue> {
		match self.get_inputted_local_flag_value_of(flag_name) {
			None | Some(FlagValue::None) => match self.local_flags.find(flag_name) {
				Some(f) => Some(f.default_value.clone()),
				None => None,
			},
			val => val,
		}
	}

	pub fn get_inputted_local_flag_value_of(&self, flag_name: &str) -> Option<FlagValue> {
		match &self.local_flags_values {
			Vector(None) => None,
			Vector(Some(local)) => match local.iter().find(|(name, _)| name == flag_name) {
				None => None,
				Some((_, flag_val)) => Some(flag_val.to_owned()),
			},
		}
	}

	pub fn get_inputted_common_flag_value_of(&self, flag_name: &str) -> Option<FlagValue> {
		match &self.common_flags_values {
			Vector(None) => None,
			Vector(Some(common)) => match common.iter().find(|(name, _)| name == flag_name) {
				None => None,
				Some((_, flag_val)) => Some(flag_val.to_owned()),
			},
		}
	}
}

impl<'a> From<Vec<String>> for Context {
	fn from(raw_args: Vec<String>) -> Context {
		let args = VecDeque::from(raw_args.clone());
		let current_path = match raw_args.get(0) {
			Some(str) => PathBuf::from(str),
			None => PathBuf::new(),
		};
		Context {
			raw_args,
			args,
			common_flags: Vector::default(),
			local_flags: Vector::default(),
			routes: Vector::default(),
			current_path,
			common_flags_values: Vector::default(),
			local_flags_values: Vector::default(),
			parsing_args: None,
			error_info_list: Vector::default(),
		}
	}
}
