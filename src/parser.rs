use crate::Context;
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
                    (CalledType::Name, Some(l_flag)) => {
                        match l_flag.flag_type.get_value_from_string(after_eq) {
                            FlagValue::Invalid(after_eq) => {
                                match c.common_flags.find_long_flag(&long_flag) {
                                    (CalledType::Name, Some(c_flag)) => {
                                        match c_flag.flag_type.get_value_from_string(after_eq) {
                                            FlagValue::Invalid(after_eq) => {
                                                /*println!(
                                                    "The flag {}'s value is invalid for . So, it is interpreted as an unknown flag.",
                                                    &long_flag
                                                );*/

                                                c.parsing_flags.push(FlagArg::Long(
                                                    long_flag,
                                                    FlagValue::String(after_eq),
                                                ))
                                            }
                                            val => c.common_flags_values.push((long_flag, val)),
                                        }
                                    }
                                    (CalledType::Long, Some(c_flag)) => {
                                        match c_flag.flag_type.get_value_from_string(after_eq) {
                                            FlagValue::Invalid(after_eq) => {
                                                println!("The flag {} is a name of local flag and a long form of {}, but it has invalid value.\nDue to above reason, it is interpreted as an unknown flag.",&long_flag,&c_flag.name);
                                                c.parsing_flags.push(FlagArg::Long(
                                                    c_flag.get_name_clone(),
                                                    FlagValue::String(after_eq),
                                                ))
                                            }
                                            val => c
                                                .common_flags_values
                                                .push((c_flag.get_name_clone(), val)),
                                        }
                                    }
                                    (CalledType::Short, Some(c_flag)) => {
                                        println!("The flag {} is a name of a local flag {} and a short form of a common flag {}, but it has invalid value as local flag {} and it is specified long flag.", &long_flag,&long_flag, &c_flag.name, &long_flag);
                                        println!("Due to above reason, the flag {} is interpreted as unknown flag.",&long_flag);
                                        c.parsing_flags.push(FlagArg::Long(
                                            long_flag,
                                            FlagValue::String(after_eq),
                                        ))
                                    }
                                    (_, _) => {
                                        println!("The flag {} is a name of a local flag {0}, but its value is invalid.\nDue to above reason, {0} is interpreted unknown flag.", long_flag);
                                        c.parsing_flags.push(FlagArg::Long(
                                            long_flag,
                                            FlagValue::String(after_eq),
                                        ))
                                    }
                                }
                            }
                            val => {
                                c.local_flags_values.push((long_flag, val));
                            }
                        }
                    }
                    (CalledType::Long, Some(l_flag)) => {
                        match l_flag.flag_type.get_value_from_string(after_eq) {
                            FlagValue::Invalid(after_eq) => {
                                match c.common_flags.find_long_flag(&long_flag) {
                                    (CalledType::Name, Some(c_flag)) => {
                                        match c_flag.flag_type.get_value_from_string(after_eq) {
                                            FlagValue::Invalid(after_eq) => {
                                                c.parsing_flags.push(FlagArg::Long(
                                                    long_flag,
                                                    FlagValue::String(after_eq),
                                                ))
                                            }
                                            val => c.common_flags_values.push((long_flag, val)),
                                        }
                                    }
                                    (CalledType::Long, Some(c_flag)) => {
                                        match c_flag.flag_type.get_value_from_string(after_eq) {
                                            FlagValue::Invalid(after_eq) => {
                                                c.parsing_flags.push(FlagArg::Long(
                                                    long_flag,
                                                    FlagValue::String(after_eq),
                                                ))
                                            }
                                            val => c
                                                .common_flags_values
                                                .push((c_flag.get_name_clone(), val)),
                                        }
                                    }
                                    (CalledType::Short, Some(c_flag)) => {
                                        println!(
                                            "The flag {} is a long form of a local flag {} and a short form of a common flag {}. But, it has invalid value as {1} and it is specified as long flag.", &long_flag, &l_flag.name,&c_flag.name
                                        );
                                        println!("Due to above reason, {} is interpreted as an unknown flag.", &long_flag);
                                        c.parsing_flags.push(FlagArg::Long(
                                            long_flag,
                                            FlagValue::String(after_eq),
                                        ))
                                    }
                                    (_, _) => {
                                        println!("The flag {} is a long form of a local flag {}, but its value \"{}\" is invalid.\nDue to above reason, {0} is inter_mediate_args as an unknown flag.",&long_flag, &l_flag.name, &after_eq);
                                        c.parsing_flags.push(FlagArg::Long(
                                            long_flag,
                                            FlagValue::String(after_eq),
                                        ))
                                    }
                                }
                            }
                            val => c.local_flags_values.push((l_flag.get_name_clone(), val)),
                        }
                    }
                    (ltype, lhit) => match c.common_flags.find_long_flag(&long_flag) {
                        (CalledType::Name, Some(c_flag)) => {
                            match c_flag.flag_type.get_value_from_string(after_eq) {
                                FlagValue::Invalid(after_eq) => c
                                    .parsing_flags
                                    .push(FlagArg::Long(long_flag, FlagValue::String(after_eq))),
                                val => c.common_flags_values.push((long_flag, val)),
                            }
                        }
                        (CalledType::Long, Some(c_flag)) => {
                            match c_flag.flag_type.get_value_from_string(after_eq) {
                                FlagValue::Invalid(after_eq) => c
                                    .parsing_flags
                                    .push(FlagArg::Long(long_flag, FlagValue::String(after_eq))),
                                val => c.common_flags_values.push((c_flag.get_name_clone(), val)),
                            }
                        }
                        (ctype, chit) => match (ltype, ctype) {
                            (CalledType::None, CalledType::None) => {
                                println!("The flag {} is an unknown flag.", &long_flag);
                                c.parsing_flags
                                    .push(FlagArg::Long(long_flag, FlagValue::String(after_eq)));
                            }
                            (CalledType::None, _) => {
                                println!("The flag {} is a short form of a common flag {}. But it is specified as a long flag. \n Due to above reason, it is interpreted as an unknown flag.", &long_flag, &chit.unwrap().name);
                                c.parsing_flags
                                    .push(FlagArg::Long(long_flag, FlagValue::String(after_eq)));
                            }
                            (_, CalledType::None) => {
                                println!("The flag {} is a short form of a local flag {}. But it is specified as a long flag.\nDue to above reason, it is interpreted as an unknown flag.", &long_flag, lhit.unwrap().get_name_clone());
                                c.parsing_flags
                                    .push(FlagArg::Long(long_flag, FlagValue::String(after_eq)));
                            }
                            (_, _) => {
                                println!("The flag {} is short forms of a local flag {} and common flag {}. But it is specified as a long flag.\nDur to above reason, it is interpreted as an unknown flag.", &long_flag, &lhit.unwrap().name, &chit.unwrap().name);
                                c.parsing_flags
                                    .push(FlagArg::Long(long_flag, FlagValue::String(after_eq)));
                            }
                        },
                    },
                }
                self.parse_next_if_flag(c)
            }
            None => {
                match c.local_flags.find_long_flag(&long_flag) {
                    (CalledType::Name, Some(l_flag)) => {
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
                                        //
                                        match l_flag.flag_type {
                                            FlagType::Bool => {
                                                c.local_flags_values
                                                    .push((long_flag, FlagValue::Bool(true)));
                                                (Some(next_arg), c)
                                            }
                                            _ => {
                                                match c.common_flags.find_long_flag(&long_flag) {
                                                    (CalledType::Name, Some(c_flag)) => {
                                                        //
                                                        match c_flag
                                                            .flag_type
                                                            .get_value_from_string(next_arg)
                                                        {
                                                            FlagValue::Invalid(next_arg) => {
                                                                //
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
                                                    (CalledType::Long, Some(c_flag)) => {
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
                                                    (_, _) => {
                                                        //
                                                        println!("{} is unknown flag.", &long_flag);
                                                        c.parsing_flags.push(FlagArg::Long(
                                                            long_flag,
                                                            FlagValue::None,
                                                        ));
                                                        (Some(next_arg), c)
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    val => {
                                        //
                                        c.local_flags_values.push((long_flag, val));
                                        self.parse_next_if_flag(c)
                                    }
                                }
                            }
                            None => {
                                /*match l_flag.flag_type {
                                    FlagType::Bool => c
                                        .local_flags_values
                                        .push((long_flag, FlagValue::Bool(true))),
                                    _ => {
                                        c.local_flags_values.push((long_flag, FlagValue::None));
                                    }
                                }*/
                                c.local_flags_values
                                    .push((long_flag, l_flag.flag_type.get_value_if_no_value()));
                                (None, c)
                            }
                        }
                    }
                    (CalledType::Long, Some(l_flag)) => {
                        match c.args.pop_front() {
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
                            Some(next_arg) => {
                                match l_flag.flag_type.get_value_from_string(next_arg) {
                                    FlagValue::Invalid(next_arg) => {
                                        match c.common_flags.find_long_flag(&long_flag) {
                                            (CalledType::Name, Some(c_flag)) => {
                                                match c_flag
                                                    .flag_type
                                                    .get_value_from_string(next_arg)
                                                {
                                                    FlagValue::Invalid(next_arg) => {
                                                        //
                                                        c.local_flags_values.push((
                                                            l_flag.get_name_clone(),
                                                            FlagValue::None,
                                                        ));
                                                        (Some(next_arg), c)
                                                    }
                                                    val => {
                                                        //
                                                        c.common_flags_values
                                                            .push((long_flag, val));
                                                        self.parse_next_if_flag(c)
                                                    }
                                                }
                                            }
                                            (CalledType::Long, Some(c_flag)) => {
                                                match c_flag
                                                    .flag_type
                                                    .get_value_from_string(next_arg)
                                                {
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
                                            (_ctype, _chit) => {
                                                c.local_flags_values.push((
                                                    l_flag.get_name_clone(),
                                                    FlagValue::None,
                                                ));
                                                (Some(next_arg), c)
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
                                c.local_flags_values.push((
                                    l_flag.get_name_clone(),
                                    l_flag.flag_type.get_value_if_no_value(),
                                ));
                                (None, c)
                            }
                        }
                    }
                    (ltype, lhit) => {
                        //
                        match c.common_flags.find_long_flag(&long_flag) {
                            (CalledType::Name, Some(c_flag)) => {
                                // TODO: エラーメッセージの表示
                                match c.args.pop_front() {
                                    Some(next_long_flag) if self.long_flag(&next_long_flag) => {
                                        //
                                        c.common_flags_values.push((
                                            long_flag,
                                            c_flag.flag_type.get_value_if_no_value(),
                                        ));
                                        self.parse_flags_start_with_long_flag(next_long_flag, c)
                                    }
                                    Some(next_short_flag) if self.flag(&next_short_flag) => {
                                        //
                                        c.common_flags_values.push((
                                            long_flag,
                                            c_flag.flag_type.get_value_if_no_value(),
                                        ));
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
                                        //
                                        c.common_flags_values.push((
                                            long_flag,
                                            c_flag.flag_type.get_value_if_no_value(),
                                        ));
                                        (next_none, c)
                                    }
                                }
                            }
                            (CalledType::Long, Some(c_flag)) => match c.args.pop_front() {
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
                                            c.common_flags_values
                                                .push((c_flag.get_name_clone(), val));
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
                            (ctype, chit) => {
                                match (ltype, ctype) {
                                    (CalledType::None, CalledType::None) => {
                                        println!("The flag {} is an unknown flag.", &long_flag);
                                    }
                                    (CalledType::Short, CalledType::None) => {
                                        println!("The flag {} is a short form of a local flag {}. But {0} is specified as a long flag.\nDue to above reason, {0} is interpreted as an unknown flag.", &long_flag,&lhit.unwrap().name);
                                    }
                                    (CalledType::None, CalledType::Short) => {
                                        //
                                        println!("The flag {} is a short form of a common flag {}. But {0} is specified as a long flag.\nDue to above reason, {0} is interpreted as an unkown flag.", &long_flag, &lhit.unwrap().name);
                                    }
                                    (_, _) => {
                                        //
                                        println!("The flag {} matches short forms of a local flag {} and a common flag {}. But {0} is specified as long flag.\nDue to above reason, {0} is interpreted as an unknown flag.", &long_flag, &lhit.unwrap().name, &chit.unwrap().name);
                                    }
                                }
                                c.parsing_flags
                                    .push(FlagArg::Long(long_flag, FlagValue::None));
                                self.parse_next_if_flag(c)
                            }
                        }
                    }
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
                    (CalledType::Short, Some(l_flag)) => {
                        //
                        match l_flag.derive_flag_value_from_string(after_eq) {
                            FlagValue::Invalid(after_eq) => {
                                match c.common_flags.find_short_flag(&short_flag) {
                                    (CalledType::Short, Some(c_flag)) => {
                                        //
                                        match c_flag.derive_flag_value_from_string(after_eq) {
                                            FlagValue::Invalid(after_eq) => {
                                                c.parsing_flags.push(FlagArg::Short(
                                                    short_flag,
                                                    FlagValue::String(after_eq),
                                                ));
                                            }
                                            val => {
                                                c.local_flags_values
                                                    .push((c_flag.get_name_clone(), val));
                                            }
                                        }
                                    }
                                    (_, _) => {
                                        println!("{} is match a short form of a local flag {}, but its value is invalid.\nDue to above reason, {0} is interpreted as an unknown flag.", &short_flag, &l_flag.name);
                                        c.parsing_flags.push(FlagArg::Short(
                                            short_flag,
                                            FlagValue::String(after_eq),
                                        ));
                                    }
                                }
                            }
                            val => {
                                c.local_flags_values.push((short_flag, val));
                            }
                        }
                    }
                    (ltype, lhit) => {
                        match c.common_flags.find_short_flag(&short_flag) {
                            (CalledType::Short, Some(c_flag)) => {
                                match c_flag.derive_flag_value_from_string(after_eq) {
                                    FlagValue::Invalid(after_eq) => {
                                        //
                                        c.parsing_flags.push(FlagArg::Short(
                                            short_flag,
                                            FlagValue::String(after_eq),
                                        ))
                                    }
                                    val => {
                                        c.common_flags_values.push((c_flag.get_name_clone(), val));
                                    }
                                }
                            }
                            (ctype, chit) => {
                                println!("The flag {} is not any short forms of local and common flags.\nDue to above reason, it is interpreted as an unknown flag.", &short_flag);
                                c.parsing_flags
                                    .push(FlagArg::Short(short_flag, FlagValue::String(after_eq)));
                            }
                        }
                    }
                }
                self.parse_next_if_flag(c)
            }
            None => {
                //
                short_flag = self.get_short_flag_name(short_flag);
                match c.local_flags.find_short_flag(&short_flag) {
                    (CalledType::Short, Some(l_flag)) => {
                        match c.args.pop_front() {
                            Some(next_long_flag) if self.long_flag(&next_long_flag) => {
                                //
                                c.local_flags_values.push((
                                    l_flag.get_name_clone(),
                                    l_flag.derive_flag_value_if_no_value(),
                                ));
                                self.parse_flags_start_with_long_flag(next_long_flag, c)
                            }
                            Some(next_short_flag) if self.flag(&next_short_flag) => {
                                //
                                c.local_flags_values.push((
                                    l_flag.get_name_clone(),
                                    l_flag.derive_flag_value_if_no_value(),
                                ));
                                self.parse_flags_start_with_short_flag(next_short_flag, c)
                            }
                            Some(next_arg) => {
                                //
                                match l_flag.derive_flag_value_from_string(next_arg) {
                                    FlagValue::Invalid(next_arg) => {
                                        match l_flag.flag_type {
                                            FlagType::Bool => {
                                                //
                                                c.local_flags_values.push((
                                                    l_flag.get_name_clone(),
                                                    FlagValue::Bool(true),
                                                ));
                                                (Some(next_arg), c)
                                            }
                                            _ => {
                                                match c.common_flags.find_short_flag(&short_flag) {
                                                    (CalledType::Short, Some(c_flag)) => {
                                                        //
                                                        match c_flag
                                                            .derive_flag_value_from_string(next_arg)
                                                        {
                                                            FlagValue::Invalid(next_arg) => {
                                                                match c_flag.flag_type {
                                                                    FlagType::Bool => {
                                                                        c.common_flags_values.push(
                                                                            (
                                                                                c_flag
                                                                                    .get_name_clone(
                                                                                    ),
                                                                                FlagValue::Bool(
                                                                                    true,
                                                                                ),
                                                                            ),
                                                                        );
                                                                    }
                                                                    _ => {
                                                                        c.local_flags_values.push((
                                                                            l_flag.get_name_clone(),
                                                                            FlagValue::None,
                                                                        ))
                                                                    }
                                                                }
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
                                                    (ctype, chit) => {
                                                        c.local_flags_values.push((
                                                            l_flag.get_name_clone(),
                                                            FlagValue::None,
                                                        ));
                                                        (Some(next_arg), c)
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    val => {
                                        c.local_flags_values.push((l_flag.get_name_clone(), val));
                                        self.parse_next_if_flag(c)
                                    }
                                }
                            }
                            next_none => {
                                c.local_flags_values.push((
                                    l_flag.get_name_clone(),
                                    l_flag.derive_flag_value_if_no_value(),
                                ));
                                (next_none, c)
                            }
                        }
                    }
                    (ltype, lhit) => {
                        //
                        match c.common_flags.find_short_flag(&short_flag) {
                            (CalledType::Short, Some(c_flag)) => match c.args.pop_front() {
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
                                            c.common_flags_values
                                                .push((c_flag.get_name_clone(), val));
                                            self.parse_next_if_flag(c)
                                        }
                                    }
                                }
                                next_none => {
                                    //
                                    c.common_flags_values.push((
                                        c_flag.get_name_clone(),
                                        c_flag.derive_flag_value_if_no_value(),
                                    ));
                                    (next_none, c)
                                }
                            },
                            (_, _) => {
                                c.parsing_flags
                                    .push(FlagArg::Short(short_flag, FlagValue::None));
                                self.parse_next_if_flag(c)
                            }
                        }
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

#[derive(Debug)]
pub enum FlagArg {
    Long(String, FlagValue),
    Short(String, FlagValue),
}

#[derive(Debug)]
pub enum ParseError {
    Nohit,
    DifferentForm(String),
    Invalid(String),
}

/*#[derive(Debug)]
pub struct ErrorInfo {
    pub flag_arg: FlagValue,
    pub local: ParseError,
    pub common: ParseError,
    pub description: String,
}*/
