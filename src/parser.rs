use crate::vector::flag::Found;
use crate::Context;
use crate::{FlagType, FlagValue};
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
                Some(long_flag) if self.long_flag(&long_flag) => {
                    inter_mediate_args.push_back(self.long_middle(long_flag));
                }
                Some(short_flag) if self.flag(&short_flag) => {
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
                    Found::Name(l_flag) => match l_flag.flag_type.get_value_from_string(after_eq) {
                        FlagValue::Invalid(after_eq) => {
                            match c.common_flags.find_long_flag(&long_flag) {
                                Found::Name(c_flag) => {
                                    match c_flag.flag_type.get_value_from_string(after_eq) {
                                        FlagValue::Invalid(after_eq) => {
                                            let flag_arg = FlagArg::Long(
                                                long_flag,
                                                FlagValue::String(after_eq),
                                            );
                                            c.error_info_list.push((
                                                flag_arg.clone(),
                                                ParseError::Invalid(l_flag.get_name_clone()),
                                                ParseError::Invalid(c_flag.get_name_clone()),
                                            ));
                                            c.parsing_flags.push(flag_arg);
                                        }
                                        val => c.common_flags_values.push((long_flag, val)),
                                    }
                                }
                                Found::Long(c_flag) => {
                                    match c_flag.flag_type.get_value_from_string(after_eq) {
                                        FlagValue::Invalid(after_eq) => {
                                            let flag_arg = FlagArg::Long(
                                                long_flag,
                                                FlagValue::String(after_eq),
                                            );
                                            c.error_info_list.push((
                                                flag_arg.clone(),
                                                ParseError::Invalid(l_flag.get_name_clone()),
                                                ParseError::Invalid(c_flag.get_name_clone()),
                                            ));

                                            c.parsing_flags.push(flag_arg);
                                        }
                                        val => c
                                            .common_flags_values
                                            .push((c_flag.get_name_clone(), val)),
                                    }
                                }
                                Found::Short(c_flag) => {
                                    let flag_arg =
                                        FlagArg::Long(long_flag, FlagValue::String(after_eq));
                                    c.error_info_list.push((
                                        flag_arg.clone(),
                                        ParseError::Invalid(l_flag.get_name_clone()),
                                        ParseError::DifferentForm(Found::Short(
                                            c_flag.get_name_clone(),
                                        )),
                                    ));
                                    c.parsing_flags.push(flag_arg);
                                }
                                _ => {
                                    let flag_arg =
                                        FlagArg::Long(long_flag, FlagValue::String(after_eq));

                                    c.error_info_list.push((
                                        flag_arg.clone(),
                                        ParseError::Invalid(l_flag.get_name_clone()),
                                        ParseError::Nohit,
                                    ));
                                    c.parsing_flags.push(flag_arg)
                                }
                            }
                        }
                        val => {
                            c.local_flags_values.push((long_flag, val));
                        }
                    },
                    Found::Long(l_flag) => match l_flag.flag_type.get_value_from_string(after_eq) {
                        FlagValue::Invalid(after_eq) => {
                            match c.common_flags.find_long_flag(&long_flag) {
                                Found::Name(c_flag) => {
                                    match c_flag.flag_type.get_value_from_string(after_eq) {
                                        FlagValue::Invalid(after_eq) => {
                                            let flag_arg = FlagArg::Long(
                                                long_flag,
                                                FlagValue::String(after_eq),
                                            );
                                            c.error_info_list.push((
                                                flag_arg.clone(),
                                                ParseError::Invalid(l_flag.get_name_clone()),
                                                ParseError::Invalid(c_flag.get_name_clone()),
                                            ));
                                            c.parsing_flags.push(flag_arg.clone());
                                        }
                                        val => c.common_flags_values.push((long_flag, val)),
                                    }
                                }
                                Found::Long(c_flag) => {
                                    match c_flag.flag_type.get_value_from_string(after_eq) {
                                        FlagValue::Invalid(after_eq) => {
                                            let flag_arg = FlagArg::Long(
                                                long_flag,
                                                FlagValue::String(after_eq),
                                            );
                                            c.error_info_list.push((
                                                flag_arg.clone(),
                                                ParseError::Invalid(l_flag.get_name_clone()),
                                                ParseError::Invalid(c_flag.get_name_clone()),
                                            ));
                                            c.parsing_flags.push(flag_arg)
                                        }
                                        val => c
                                            .common_flags_values
                                            .push((c_flag.get_name_clone(), val)),
                                    }
                                }
                                Found::Short(c_flag) => {
                                    let flag_arg =
                                        FlagArg::Long(long_flag, FlagValue::String(after_eq));
                                    c.error_info_list.push((
                                        flag_arg.clone(),
                                        ParseError::Invalid(l_flag.get_name_clone()),
                                        ParseError::DifferentForm(Found::Short(
                                            c_flag.get_name_clone(),
                                        )),
                                    ));
                                    c.parsing_flags.push(flag_arg)
                                }
                                _ => {
                                    let flag_arg =
                                        FlagArg::Long(long_flag, FlagValue::String(after_eq));
                                    c.error_info_list.push((
                                        flag_arg.clone(),
                                        ParseError::Invalid(l_flag.get_name_clone()),
                                        ParseError::Nohit,
                                    ));
                                    c.parsing_flags.push(flag_arg)
                                }
                            }
                        }
                        val => c.local_flags_values.push((l_flag.get_name_clone(), val)),
                    },
                    lhit => match c.common_flags.find_long_flag(&long_flag) {
                        Found::Name(c_flag) => {
                            match c_flag.flag_type.get_value_from_string(after_eq) {
                                FlagValue::Invalid(after_eq) => {
                                    let flag_arg =
                                        FlagArg::Long(long_flag, FlagValue::String(after_eq));
                                    c.error_info_list.push((
                                        flag_arg.clone(),
                                        match lhit {
                                            Found::Short(l_flag) => ParseError::DifferentForm(
                                                Found::Short(l_flag.get_name_clone()),
                                            ),
                                            _ => ParseError::Nohit,
                                        },
                                        ParseError::Invalid(c_flag.get_name_clone()),
                                    ));
                                    c.parsing_flags.push(flag_arg)
                                }
                                val => c.common_flags_values.push((long_flag, val)),
                            }
                        }
                        Found::Long(c_flag) => {
                            match c_flag.flag_type.get_value_from_string(after_eq) {
                                FlagValue::Invalid(after_eq) => {
                                    let flag_arg =
                                        FlagArg::Long(long_flag, FlagValue::String(after_eq));
                                    c.error_info_list.push((
                                        flag_arg.clone(),
                                        match lhit {
                                            Found::Short(l_flag) => ParseError::DifferentForm(
                                                Found::Short(l_flag.get_name_clone()),
                                            ),
                                            _ => ParseError::Nohit,
                                        },
                                        ParseError::Invalid(c_flag.get_name_clone()),
                                    ));
                                    c.parsing_flags.push(flag_arg)
                                }
                                val => c.common_flags_values.push((c_flag.get_name_clone(), val)),
                            }
                        }
                        chit => {
                            let flag_arg = FlagArg::Long(long_flag, FlagValue::String(after_eq));
                            c.error_info_list.push((
                                flag_arg.clone(),
                                match lhit {
                                    Found::Short(l_flag) => ParseError::DifferentForm(
                                        Found::Short(l_flag.get_name_clone()),
                                    ),
                                    _ => ParseError::Nohit,
                                },
                                match chit {
                                    Found::Short(c_flag) => ParseError::DifferentForm(
                                        Found::Short(c_flag.get_name_clone()),
                                    ),
                                    _ => ParseError::Nohit,
                                },
                            ));
                            c.parsing_flags.push(flag_arg)
                        }
                    },
                }
                self.parse_next_if_flag(c)
            }
            None => {
                match c.local_flags.find_long_flag(&long_flag) {
                    Found::Name(l_flag) => {
                        match c.args.pop_front() {
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
                            Some(next_arg) => {
                                match l_flag.flag_type.get_value_from_string(next_arg) {
                                    FlagValue::Invalid(next_arg) => {
                                        match l_flag.flag_type {
                                            FlagType::Bool => {
                                                c.local_flags_values
                                                    .push((long_flag, FlagValue::Bool(true)));
                                                (Some(next_arg), c)
                                            }
                                            _ => {
                                                match c.common_flags.find_long_flag(&long_flag) {
                                                    Found::Name(c_flag) => {
                                                        match c_flag
                                                            .flag_type
                                                            .get_value_from_string(next_arg)
                                                        {
                                                            FlagValue::Invalid(next_arg) => {
                                                                match c_flag.flag_type {
                                                                    FlagType::Bool => {
                                                                        c.common_flags_values.push(
                                                                            (
                                                                                long_flag,
                                                                                FlagValue::Bool(
                                                                                    true,
                                                                                ),
                                                                            ),
                                                                        );
                                                                    }
                                                                    _ => {
                                                                        c.local_flags_values.push(
                                                                            (
                                                                                long_flag,
                                                                                FlagValue::None,
                                                                            ),
                                                                        );
                                                                    }
                                                                }

                                                                (Some(next_arg), c)
                                                            }
                                                            val => {
                                                                c.common_flags_values
                                                                    .push((long_flag, val));
                                                                self.parse_next_if_flag(c)
                                                            }
                                                        }
                                                    }
                                                    Found::Long(c_flag) => {
                                                        //
                                                        match c_flag
                                                            .flag_type
                                                            .get_value_from_string(next_arg)
                                                        {
                                                            FlagValue::Invalid(next_arg) => {
                                                                c.local_flags_values.push((
                                                                    long_flag,
                                                                    l_flag
                                                                        .flag_type
                                                                        .get_value_if_no_value(),
                                                                ));
                                                                (Some(next_arg), c)
                                                            }
                                                            val => {
                                                                c.common_flags_values.push((
                                                                    c_flag.get_name_clone(),
                                                                    val,
                                                                ));
                                                                self.parse_next_if_flag(c)
                                                            }
                                                        }
                                                    }
                                                    _ => {
                                                        c.local_flags_values
                                                            .push((long_flag, FlagValue::None));
                                                        (Some(next_arg), c)
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    val => {
                                        c.local_flags_values.push((long_flag, val));
                                        self.parse_next_if_flag(c)
                                    }
                                }
                            }
                            None => {
                                c.local_flags_values
                                    .push((long_flag, l_flag.flag_type.get_value_if_no_value()));
                                (None, c)
                            }
                        }
                    }
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
                        Some(next_arg) => match l_flag.flag_type.get_value_from_string(next_arg) {
                            FlagValue::Invalid(next_arg) => {
                                match c.common_flags.find_long_flag(&long_flag) {
                                    Found::Name(c_flag) => {
                                        match c_flag.flag_type.get_value_from_string(next_arg) {
                                            FlagValue::Invalid(next_arg) => {
                                                c.local_flags_values.push((
                                                    l_flag.get_name_clone(),
                                                    FlagValue::None,
                                                ));
                                                (Some(next_arg), c)
                                            }
                                            val => {
                                                c.common_flags_values.push((long_flag, val));
                                                self.parse_next_if_flag(c)
                                            }
                                        }
                                    }
                                    Found::Long(c_flag) => {
                                        match c_flag.flag_type.get_value_from_string(next_arg) {
                                            FlagValue::Invalid(next_arg) => {
                                                c.local_flags_values.push((
                                                    l_flag.get_name_clone(),
                                                    FlagValue::None,
                                                ));
                                                (Some(next_arg), c)
                                            }
                                            val => {
                                                c.common_flags_values
                                                    .push((c_flag.get_name_clone(), val));
                                                self.parse_next_if_flag(c)
                                            }
                                        }
                                    }
                                    _ => {
                                        c.local_flags_values
                                            .push((l_flag.get_name_clone(), FlagValue::None));
                                        (Some(next_arg), c)
                                    }
                                }
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
                            Some(next_arg) => {
                                match c_flag.flag_type.get_value_from_string(next_arg) {
                                    FlagValue::Invalid(next_arg) => {
                                        c.common_flags_values.push((
                                            long_flag,
                                            c_flag.flag_type.get_value_if_no_value(),
                                        ));
                                        (Some(next_arg), c)
                                    }
                                    val => {
                                        c.common_flags_values.push((long_flag, val));
                                        self.parse_next_if_flag(c)
                                    }
                                }
                            }
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
                            Some(next_arg) => {
                                match c_flag.flag_type.get_value_from_string(next_arg) {
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
                                }
                            }
                            next_none => {
                                c.common_flags_values.push((
                                    c_flag.get_name_clone(),
                                    c_flag.flag_type.get_value_if_no_value(),
                                ));
                                (next_none, c)
                            }
                        },
                        chit => {
                            let flag_arg = FlagArg::Long(long_flag, FlagValue::None);
                            c.error_info_list.push((
                                flag_arg.clone(),
                                match lhit {
                                    Found::Short(l_flag) => ParseError::DifferentForm(
                                        Found::Short(l_flag.get_name_clone()),
                                    ),
                                    _ => ParseError::Nohit,
                                },
                                match chit {
                                    Found::Short(c_flag) => ParseError::DifferentForm(
                                        Found::Short(c_flag.get_name_clone()),
                                    ),
                                    _ => ParseError::Nohit,
                                },
                            ));
                            c.parsing_flags.push(flag_arg);
                            self.parse_next_if_flag(c)
                        }
                    },
                }
            }
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
                match c.local_flags.find_short_flag(&short_flag) {
                    Found::Short(l_flag) => match l_flag.derive_flag_value_from_string(after_eq) {
                        FlagValue::Invalid(after_eq) => {
                            match c.common_flags.find_short_flag(&short_flag) {
                                Found::Short(c_flag) => {
                                    match c_flag.derive_flag_value_from_string(after_eq) {
                                        FlagValue::Invalid(after_eq) => {
                                            let flag_arg = FlagArg::Short(
                                                short_flag,
                                                FlagValue::String(after_eq),
                                            );
                                            c.error_info_list.push((
                                                flag_arg.clone(),
                                                ParseError::Invalid(l_flag.get_name_clone()),
                                                ParseError::Invalid(c_flag.get_name_clone()),
                                            ));
                                            c.parsing_flags.push(flag_arg);
                                        }
                                        val => {
                                            c.local_flags_values
                                                .push((c_flag.get_name_clone(), val));
                                        }
                                    }
                                }
                                _ => {
                                    let flag_arg =
                                        FlagArg::Short(short_flag, FlagValue::String(after_eq));
                                    c.error_info_list.push((
                                        flag_arg.clone(),
                                        ParseError::Invalid(l_flag.get_name_clone()),
                                        ParseError::Nohit,
                                    ));
                                    c.parsing_flags.push(flag_arg);
                                }
                            }
                        }
                        val => {
                            c.local_flags_values.push((short_flag, val));
                        }
                    },
                    lhit => match c.common_flags.find_short_flag(&short_flag) {
                        Found::Short(c_flag) => {
                            match c_flag.derive_flag_value_from_string(after_eq) {
                                FlagValue::Invalid(after_eq) => {
                                    let flag_arg =
                                        FlagArg::Short(short_flag, FlagValue::String(after_eq));
                                    c.error_info_list.push((
                                        flag_arg.clone(),
                                        ParseError::Nohit,
                                        ParseError::Invalid(c_flag.get_name_clone()),
                                    ));
                                    c.parsing_flags.push(flag_arg)
                                }
                                val => {
                                    c.common_flags_values.push((c_flag.get_name_clone(), val));
                                }
                            }
                        }
                        chit => {
                            let flag_arg = FlagArg::Short(short_flag, FlagValue::String(after_eq));
                            c.error_info_list.push((
                                flag_arg.clone(),
                                match lhit {
                                    Found::Short(l_flag) => ParseError::DifferentForm(
                                        Found::Short(l_flag.get_name_clone()),
                                    ),
                                    _ => ParseError::Nohit,
                                },
                                match chit {
                                    Found::Short(c_flag) => ParseError::DifferentForm(
                                        Found::Short(c_flag.get_name_clone()),
                                    ),
                                    _ => ParseError::Nohit,
                                },
                            ));
                            c.parsing_flags.push(flag_arg);
                        }
                    },
                }
                self.parse_next_if_flag(c)
            }
            None => {
                short_flag = self.get_short_flag_name(short_flag);
                match c.local_flags.find_short_flag(&short_flag) {
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
                            FlagValue::Invalid(next_arg) => match l_flag.flag_type {
                                FlagType::Bool => {
                                    c.local_flags_values
                                        .push((l_flag.get_name_clone(), FlagValue::Bool(true)));
                                    (Some(next_arg), c)
                                }
                                _ => match c.common_flags.find_short_flag(&short_flag) {
                                    Found::Short(c_flag) => {
                                        match c_flag.derive_flag_value_from_string(next_arg) {
                                            FlagValue::Invalid(next_arg) => {
                                                match c_flag.flag_type {
                                                    FlagType::Bool => {
                                                        c.common_flags_values.push((
                                                            c_flag.get_name_clone(),
                                                            FlagValue::Bool(true),
                                                        ));
                                                    }
                                                    _ => c.local_flags_values.push((
                                                        l_flag.get_name_clone(),
                                                        FlagValue::None,
                                                    )),
                                                }
                                                (Some(next_arg), c)
                                            }
                                            val => {
                                                c.common_flags_values
                                                    .push((c_flag.get_name_clone(), val));
                                                self.parse_next_if_flag(c)
                                            }
                                        }
                                    }
                                    _ => {
                                        c.local_flags_values
                                            .push((l_flag.get_name_clone(), FlagValue::None));
                                        (Some(next_arg), c)
                                    }
                                },
                            },
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
                    lhit => match c.common_flags.find_short_flag(&short_flag) {
                        Found::Short(c_flag) => match c.args.pop_front() {
                            Some(next_long_flag) if self.long_flag(&next_long_flag) => {
                                self.parse_flags_start_with_long_flag(next_long_flag, c)
                            }
                            Some(next_short_flag) if self.flag(&next_short_flag) => {
                                self.parse_flags_start_with_short_flag(next_short_flag, c)
                            }
                            Some(next_arg) => {
                                match c_flag.derive_flag_value_from_string(next_arg) {
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
                                }
                            }
                            next_none => {
                                c.common_flags_values.push((
                                    c_flag.get_name_clone(),
                                    c_flag.derive_flag_value_if_no_value(),
                                ));
                                (next_none, c)
                            }
                        },
                        chit => {
                            let flag_arg = FlagArg::Short(short_flag, FlagValue::None);
                            c.error_info_list.push((
                                flag_arg.clone(),
                                match lhit {
                                    Found::Name(l_flag) => ParseError::DifferentForm(Found::Name(
                                        l_flag.get_name_clone(),
                                    )),
                                    Found::Long(l_flag) => ParseError::DifferentForm(Found::Long(
                                        l_flag.get_name_clone(),
                                    )),
                                    _ => ParseError::Nohit,
                                },
                                match chit {
                                    Found::Name(c_flag) => ParseError::DifferentForm(Found::Name(
                                        c_flag.get_name_clone(),
                                    )),
                                    Found::Long(c_flag) => ParseError::DifferentForm(Found::Long(
                                        c_flag.get_name_clone(),
                                    )),
                                    _ => ParseError::Nohit,
                                },
                            ));
                            c.parsing_flags.push(flag_arg);
                            self.parse_next_if_flag(c)
                        }
                    },
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
pub enum FlagArg {
    Long(String, FlagValue),
    Short(String, FlagValue),
}

impl FlagArg {
    pub fn name(&self) -> &str {
        match &self {
            FlagArg::Long(name, _) => name,
            FlagArg::Short(name, _) => name,
        }
    }

    pub fn val_if_string(&self) -> Option<&String> {
        match self {
            FlagArg::Long(_, FlagValue::String(val)) => Some(val),
            FlagArg::Short(_, FlagValue::String(val)) => Some(val),
            FlagArg::Long(_, FlagValue::Invalid(val)) => Some(val),
            FlagArg::Short(_, FlagValue::Invalid(val)) => Some(val),
            FlagArg::Long(_, _) => None,
            FlagArg::Short(_, _) => None,
        }
    }
    pub fn inner_if_string_val(&self) -> Option<(&str, &str)> {
        match &self {
            FlagArg::Long(name, FlagValue::String(val)) => Some((name, val)),
            FlagArg::Short(name, FlagValue::String(val)) => Some((name, val)),
            FlagArg::Long(name, FlagValue::Invalid(val)) => Some((name, val)),
            FlagArg::Short(name, FlagValue::Invalid(val)) => Some((name, val)),
            FlagArg::Long(_, _) => None,
            FlagArg::Short(_, _) => None,
        }
    }
}

#[derive(Debug)]
pub enum ParseError {
    Nohit,
    DifferentForm(Found<String>),
    Invalid(String),
}

pub type ErrorInfo = (FlagArg, ParseError, ParseError);

pub fn gen_error_description(err_info: &ErrorInfo) -> String {
    match err_info {
        (flag_arg, ParseError::Nohit, ParseError::Nohit) => {
            format!("{} is unknown flag.", &flag_arg.name())
        }
        (flag_arg, ParseError::Nohit, ParseError::Invalid(c_flag)) => {
            let (name, val) = flag_arg.inner_if_string_val().unwrap();
            format!(
                "The flag {}'s value {} is not valid for a common flag {}.",
                name, val, c_flag
            )
        }
        (flag_arg, ParseError::Nohit, ParseError::DifferentForm(chit)) => {
            let (c_form, c_flag_name) = get_form_str_and_name(chit);
            format!(
                "The flag {} matches {}a common flag {}. But it is specified as a {} flag.",
                flag_arg.name(),
                c_form,
                c_flag_name,
                match flag_arg {
                    FlagArg::Long(_, _) => "long",
                    FlagArg::Short(_, _) => "short",
                }
            )
        }
        (flag_arg, ParseError::Invalid(l_flag), ParseError::Nohit) => {
            let (name, val) = flag_arg.inner_if_string_val().unwrap();
            format!(
                "The flag {}'s value {} is not valid for a local flag {}.",
                name, val, l_flag
            )
        }
        (flag_arg, ParseError::DifferentForm(l_flag), ParseError::Nohit) => {
            let (l_form, l_flag_name) = get_form_str_and_name(l_flag);
            format!(
                "The flag {} matches {}a local flag {}. But it is specified as a {} flag.",
                flag_arg.name(),
                l_form,
                l_flag_name,
                match flag_arg {
                    FlagArg::Long(_, _) => "long",
                    FlagArg::Short(_, _) => "short",
                }
            )
        }
        (flag_arg, ParseError::Invalid(l_flag), ParseError::Invalid(c_flag)) => {
            let (name, val) = flag_arg.inner_if_string_val().unwrap();
            format!(
                "The flag {}'s value {} is not valid for a local flag {} and a common flag {}.",
                name, val, l_flag, c_flag
            )
        }
        (flag_arg, ParseError::Invalid(l_flag), ParseError::DifferentForm(c_flag)) => {
            let (name, val) = flag_arg.inner_if_string_val().unwrap();
            let (c_form, c_flag_name) = get_form_str_and_name(c_flag);
            format!("The flag {}'s value {} is not valid for a local flag {}.\nAnd {0} matches {}a common flag {}, but it is specified as a {} flag.",name,val,l_flag,c_form,c_flag_name,match flag_arg{FlagArg::Long(_,_)=>"long",FlagArg::Short(_,_)=>"short"})
        }
        (flag_arg, ParseError::DifferentForm(l_flag), ParseError::Invalid(c_flag)) => {
            let (name, val) = flag_arg.inner_if_string_val().unwrap();
            let (l_form, l_flag_name) = get_form_str_and_name(l_flag);
            format!("The flag {} matches {}a local flag {}, but it is specified as a {} flag.\nAnd {0}' s value {} is not valid for a common flag {}.", name, 
                l_form,l_flag_name,match flag_arg{
                FlagArg::Long(_,_)=>"long",
                FlagArg::Short(_,_)=>"short"
            },val,c_flag)
        }
        (flag_arg, ParseError::DifferentForm(l_flag), ParseError::DifferentForm(c_flag)) => {
            let (l_form, l_flag_name) = get_form_str_and_name(l_flag);
            let (c_form, c_flag_name) = get_form_str_and_name(c_flag);
            format!("The flag {} matches {}a local flag {} and {}a common flag {}, but it is specified as a {} flag.",
            flag_arg.name(),
            l_form,
            l_flag_name,
            c_form, c_flag_name, match flag_arg{
                FlagArg::Long(_,_)=>"long",
                FlagArg::Short(_,_)=>"short"
            })
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
