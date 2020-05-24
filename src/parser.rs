use crate::Context;
use crate::Vector;
use crate::{CalledType, FlagType, FlagValue};
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
        mut inter_mediate_args: VecDeque<FlagArg>,
    ) -> (Option<String>, VecDeque<String>, VecDeque<FlagArg>) {
        loop {
            match args.pop_front() {
                Some(mut long_flag) if self.long_flag(&long_flag) => {
                    inter_mediate_args.push_back(self.long_middle(long_flag));
                }
                Some(mut short_flag) if self.flag(&short_flag) => {
                    inter_mediate_args.push_back(self.short_middle(short_flag));
                }
                next => {
                    break (next, args, inter_mediate_args);
                }
            }
        }
    }

    pub fn long_middle(&self, mut long_flag: String) -> FlagArg {
        match &long_flag.find(self.eq) {
            Some(index) => {
                let after_eq = long_flag.split_off(index + 1);
                long_flag.pop();
                FlagArg::Long(
                    self.get_long_flag_name(long_flag),
                    FlagValue::String(after_eq),
                )
            }
            None => FlagArg::Long(self.get_long_flag_name(long_flag), FlagValue::None),
        }
    }

    pub fn short_middle(&self, mut short_flag: String) -> FlagArg {
        match &short_flag.find(self.eq) {
            Some(index) => {
                let after_eq = short_flag.split_off(index + 1);
                short_flag.pop();
                FlagArg::Short(
                    self.get_short_flag_name(short_flag),
                    FlagValue::String(after_eq),
                )
            }
            None => FlagArg::Short(self.get_long_flag_name(short_flag), FlagValue::None),
        }
    }

    pub fn parse_args_until_end(self, mut c: Context) -> Context {
        let mut non_flag_args = VecDeque::<String>::new();

        loop {
            match c.args.pop_back() {
                None => {
                    break;
                }
                Some(mut long_flag) if self.long_flag(&long_flag) => {}
                Some(mut short_flag) if self.flag(&short_flag) => {}
                Some(mut arg) => {
                    non_flag_args.push_back(arg);
                }
            }
        }
        c
    }

    pub fn parse_long_flag(&self, mut long_flag: String, c: Context) -> (Option<String>, Context) {
        match long_flag.find(self.eq) {
            Some(index) => {
                let after_eq = long_flag.split_off(index);
                long_flag.pop();
                long_flag = self.get_long_flag_name(long_flag);

                match c.local_flags.find_short_flag(&long_flag) {
                    (CalledType::Name, Some(l_flag)) => {}
                    (CalledType::Long, Some(l_flag)) => {}
                    (ctype, chit) => {}
                }
            }
            None => {}
        };

        (None, c)
    }
}

pub enum FlagArg {
    Long(String, FlagValue),
    Short(String, FlagValue),
}
