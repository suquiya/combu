use crate::Context;
use crate::Vector;
use crate::{CalledType, Flag, FlagType, FlagValue};

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

    pub fn parse_next_common_if_flags(&self, mut c: Context) -> (Option<String>, Context) {
        match c.args.pop_front() {
            Some(arg) if self.long_flag(&arg) => {
                self.parse_common_flags_starts_with_long_flag(arg, c)
            }
            Some(arg) if self.flag(&arg) => self.parse_common_flags_starts_with_short_flag(arg, c),
            non_flag => (non_flag, c),
        }
    }

    pub fn parse_common_flags_starts_with_long_flag(
        &self,
        mut arg: String,
        mut c: Context,
    ) -> (Option<String>, Context) {
        match arg.find(self.eq) {
            Some(index) => {
                let after_eq = arg.split_off(index + 1);
                arg.pop();
                let flag_name = get_long_flag_name(arg, self.flag_pattern);
                match c.common_flags.find_long_flag(&flag_name) {
                    (CalledType::Name, Some(c_flag)) => {
                        match c_flag.flag_type.get_value_from_str(&after_eq) {
                            FlagValue::None => c
                                .unknown_flags
                                .push((flag_name, FlagValue::String(after_eq))),
                            val => c.common_flags_values.push((flag_name, val)),
                        }
                    }
                    (CalledType::Long, Some(c_flag)) => {
                        match c_flag.flag_type.get_value_from_str(&after_eq) {
                            FlagValue::None => c
                                .unknown_flags
                                .push((c_flag.get_name_clone(), FlagValue::String(after_eq))),
                            val => c.common_flags_values.push((c_flag.get_name_clone(), val)),
                        }
                    }
                    (_, _) => c
                        .unknown_flags
                        .push((flag_name, FlagValue::String(after_eq))),
                }
                self.parse_next_common_if_flags(c)
            }
            None => {}
        }
    }

    pub fn parse_common_flags_starts_with_short_flag(
        &self,
        mut arg: String,
        mut c: Context,
    ) -> (Option<String>, Context) {
        (Some(arg), c)
    }
}

pub fn parse_until_end_args(mut c: Context) -> Context {
    let mut non_flag_args: Vector<String> = Vector::default();
    match c.args.pop_front() {
        None => println!("no arguments"),
        Some(mut arg) => {
            let flag_prefix = '-';
            let long_flag_prefix = "--";
            let eq = "=";
            loop {
                if arg.starts_with(flag_prefix) {
                    if arg.starts_with(long_flag_prefix) {
                        println!("long_flag");
                        match arg.find(eq) {
                            Some(index) => {
                                let val = arg.split_off(index + 1);
                                arg.pop();
                                let flag_name = arg.trim_start_matches(flag_prefix);
                                println!("{},{}", flag_name, val);
                            }
                            None => {
                                let flag_name = arg.trim_start_matches(flag_prefix);
                                println!("{}", flag_name);
                            }
                        }
                    } else {
                        println!("short_flag");
                    }
                } else {
                    println!("not_flag_arg");
                    non_flag_args.push(arg);
                }

                match c.args.pop_front() {
                    None => {
                        break;
                    }
                    Some(n) => arg = n,
                }
            }
        }
    }
    c
}

pub fn parse_flags_starts_with_long_flag(
    mut arg: String,
    mut c: Context,
) -> (Option<String>, Context) {
    let long_flag_prefix = "--";
    let eq = '=';

    let flag_pattern = '-';
    match arg.find(eq) {
        //値が明示的に指定されている場合
        Some(index) => {
            let val = arg.split_off(index + 1);
            arg.pop();
            let flag_name = get_long_flag_name(arg, flag_pattern);
            println!("{}, {}", flag_name, val);
            //match common flag
            match c.common_flags.find_long_flag(&flag_name) {
                (CalledType::Name, Some(c_flag)) => {
                    let flag_value = c_flag.flag_type.get_value_from_str(&val);
                    println!("{:?}", flag_value);
                    c.common_flags_values.push((flag_name, flag_value));
                }
                (CalledType::Long, Some(c_flag)) => {
                    let flag_value = c_flag.flag_type.get_value_from_str(&val);
                    println!("{:?}", flag_value);
                    c.common_flags_values
                        .push((c_flag.name.clone(), flag_value));
                }
                (ct, hit) => {
                    //match local_flag
                    match c.local_flags.find_long_flag(&flag_name) {
                        (CalledType::Name, Some(l_flag)) => {
                            let flag_value = l_flag.flag_type.get_value_from_str(&val);
                            println!("{:?}", flag_value);
                            c.local_flags_values.push((l_flag.name.clone(), flag_value));
                        }
                        (CalledType::Long, Some(l_flag)) => {
                            let flag_value = l_flag.flag_type.get_value_from_str(&val);
                            println!("{:?}", flag_value);
                            c.local_flags_values.push((l_flag.name.clone(), flag_value));
                        }
                        (CalledType::Short, Some(l_flag)) => {
                            println!("The inputted flag name {} is a short form of {}, but it is specified as long flag.",
                            &flag_name,
                                match (ct, hit){
                                    (CalledType::Short, Some(c_flag))=>format!("common flag {} and local flag {}", &c_flag.name, &l_flag.name),
                                    (_,_) => format!("local flag {}", &l_flag.name)
                                }
                            );

                            c.unknown_flags.push((flag_name, FlagValue::String(val)));
                        }
                        (_, _) => {
                            println!("The inputted flag name {} is unknown.", flag_name);
                            c.unknown_flags.push((flag_name, FlagValue::String(val)))
                        }
                    }
                }
            }
            //値の格納が終了したので、次の引数の解析へ
            parse_front_if_flags(c)
        }
        None => {
            //値が明示的に指定されていない
            let flag_name = get_long_flag_name(arg, flag_pattern);
            println!("{}", flag_name);
            match c.common_flags.find_long_flag(&flag_name) {
                (CalledType::Name, Some(c_flag)) => match c.args.pop_front() {
                    Some(next_arg) if next_arg.starts_with(long_flag_prefix) => {
                        match c_flag.flag_type {
                            FlagType::Bool => c
                                .common_flags_values
                                .push((flag_name, FlagValue::Bool(true))),
                            _ => c.common_flags_values.push((flag_name, FlagValue::None)),
                        }
                        parse_flags_starts_with_long_flag(next_arg, c)
                    }
                    Some(next_arg) if next_arg.starts_with(flag_pattern) => {
                        match c_flag.flag_type {
                            FlagType::Bool => c
                                .common_flags_values
                                .push((flag_name, FlagValue::Bool(true))),
                            _ => c.common_flags_values.push((flag_name, FlagValue::None)),
                        }
                        parse_flags_starts_with_short_flag(next_arg, c)
                    }
                    Some(next_arg) =>
                    //次の引数がフラグではないとき、フラグの値か通常引数かを判別
                    {
                        match &c_flag.flag_type {
                            //フラグがBoolフラグであれば、ONにしておく
                            FlagType::Bool => match FlagValue::get_bool_value_from_str(&next_arg) {
                                FlagValue::None => {
                                    c.common_flags_values
                                        .push((c_flag.name.clone(), FlagValue::Bool(true)));
                                    (Some(next_arg), c)
                                }
                                val => {
                                    c.common_flags_values.push((c_flag.name.clone(), val));
                                    parse_front_if_flags(c)
                                }
                            },
                            ft => match ft.get_value_from_str(&next_arg) {
                                FlagValue::None => (Some(next_arg), c),
                                val => {
                                    c.common_flags_values.push((flag_name, val));
                                    parse_front_if_flags(c)
                                }
                            },
                        }
                    }
                    n => (n, c),
                },
                (CalledType::Long, Some(c_flag)) => match c.args.pop_front() {
                    Some(next_arg) if next_arg.starts_with(long_flag_prefix) => {
                        match c_flag.flag_type {
                            FlagType::Bool => c
                                .common_flags_values
                                .push((flag_name, FlagValue::Bool(true))),
                            _ => c.common_flags_values.push((flag_name, FlagValue::None)),
                        }
                        parse_flags_starts_with_long_flag(next_arg, c)
                    }
                    Some(next_arg) if next_arg.starts_with(flag_pattern) => {
                        match c_flag.flag_type {
                            FlagType::Bool => c
                                .common_flags_values
                                .push((flag_name, FlagValue::Bool(true))),
                            _ => c.common_flags_values.push((flag_name, FlagValue::None)),
                        }
                        parse_flags_starts_with_short_flag(next_arg, c)
                    }
                    Some(next_arg) => {
                        //次の引数がフラグではないとき、フラグの値か通常引数かを判断
                        match &c_flag.flag_type {
                            FlagType::Bool => match FlagValue::get_bool_value_from_str(&next_arg) {
                                FlagValue::None => {
                                    c.common_flags_values
                                        .push((c_flag.name.clone(), FlagValue::Bool(true)));
                                    (Some(next_arg), c)
                                }
                                val => {
                                    c.common_flags_values.push((c_flag.name.clone(), val));
                                    parse_front_if_flags(c)
                                }
                            },
                            ft => match ft.get_value_from_str(&next_arg) {
                                FlagValue::None => (Some(next_arg), c),
                                val => {
                                    c.common_flags_values.push((c_flag.name.clone(), val));
                                    parse_front_if_flags(c)
                                }
                            },
                        }
                    }
                    n => (n, c),
                },
                (t, hit) => match c.local_flags.find_long_flag(&flag_name) {
                    (CalledType::Name, Some(l_flag)) => match c.args.pop_front() {
                        Some(next_arg) if next_arg.starts_with(long_flag_prefix) => {
                            match l_flag.flag_type {
                                FlagType::Bool => {
                                    c.local_flags_values
                                        .push((flag_name, FlagValue::Bool(true)));
                                }
                                _ => c.local_flags_values.push((flag_name, FlagValue::None)),
                            }
                            parse_flags_starts_with_long_flag(next_arg, c)
                        }
                        Some(next_arg) if next_arg.starts_with(flag_pattern) => {
                            match l_flag.flag_type {
                                FlagType::Bool => c
                                    .local_flags_values
                                    .push((l_flag.name.clone(), FlagValue::Bool(true))),

                                _ => c
                                    .local_flags_values
                                    .push((l_flag.name.clone(), FlagValue::None)),
                            }
                            parse_flags_starts_with_short_flag(next_arg, c)
                        }
                        Some(next_arg) => match l_flag.flag_type.get_value_from_str(&next_arg) {
                            FlagValue::None => (Some(next_arg), c),
                            val => {
                                c.local_flags_values.push((l_flag.name.clone(), val));
                                parse_front_if_flags(c)
                            }
                        },
                        n => (n, c),
                    },
                    (CalledType::Long, Some(l_flag)) => match c.args.pop_front() {
                        Some(next_arg) if next_arg.starts_with(long_flag_prefix) => {
                            match l_flag.flag_type {
                                FlagType::Bool => c
                                    .local_flags_values
                                    .push((l_flag.name.clone(), FlagValue::Bool(true))),
                                _ => c
                                    .local_flags_values
                                    .push((l_flag.name.clone(), FlagValue::None)),
                            }
                            parse_flags_starts_with_long_flag(next_arg, c)
                        }
                        Some(next_arg) if next_arg.starts_with(flag_pattern) => {
                            match l_flag.flag_type {
                                FlagType::Bool => c
                                    .local_flags_values
                                    .push((l_flag.name.clone(), FlagValue::Bool(true))),
                                _ => c
                                    .local_flags_values
                                    .push((l_flag.name.clone(), FlagValue::None)),
                            }
                            parse_flags_starts_with_short_flag(next_arg, c)
                        }
                        Some(next_arg) =>
                        //次の引数がフラグではないとき、通常引数かフラグの値かを判定
                        {
                            match &l_flag.flag_type {
                                FlagType::Bool => {
                                    match FlagValue::get_bool_value_from_str(&next_arg) {
                                        FlagValue::None => {
                                            c.local_flags_values
                                                .push((l_flag.name.clone(), FlagValue::Bool(true)));
                                            (Some(next_arg), c)
                                        }
                                        val => {
                                            c.local_flags_values.push((l_flag.name.clone(), val));
                                            parse_front_if_flags(c)
                                        }
                                    }
                                }
                                ft => match ft.get_value_from_str(&next_arg) {
                                    FlagValue::None => (Some(next_arg), c),
                                    val => {
                                        c.local_flags_values.push((l_flag.name.clone(), val));
                                        parse_front_if_flags(c)
                                    }
                                },
                            }
                        }
                        n => (n, c),
                    },
                    (CalledType::Short, Some(l_flag)) => {
                        println!(
                            "The inputted flag name {} is a short form of {}. But it is specified as long flag.",
                            &flag_name, match (t, hit) {
                                (CalledType::Short, Some(c_flag)) => format!(
                                    "common flag {} and local flag {}",
                                    &c_flag.name, &l_flag.name
                                ),
                                (_, _) => format!("local flag {}", &l_flag.name),
                            }
                        );

                        println!(
                            "Due to above reasons, it's interpreted unknown empty string flag."
                        );
                        c.unknown_flags
                            .push((flag_name, FlagValue::String(String::from(""))));

                        parse_front_if_flags(c)
                    }
                    (_, _) => {
                        println!("The inputted flag name {} is unknown.", flag_name);
                        c.unknown_flags
                            .push((flag_name, FlagValue::String(String::new())));
                        parse_front_if_flags(c)
                    }
                },
            }
        }
    }
}

pub fn parse_front_if_flags(mut c: Context) -> (Option<String>, Context) {
    match c.args.pop_front() {
        Some(arg) if arg.starts_with("--") => parse_flags_starts_with_short_flag(arg, c),
        Some(arg) if arg.starts_with('-') => parse_flags_starts_with_long_flag(arg, c),
        non_flag => (non_flag, c),
    }
}

pub fn parse_flags_starts_with_short_flag(
    mut arg: String,
    mut c: Context,
) -> (Option<String>, Context) {
    let eq = "=";
    match arg.find(eq) {
        Some(index) => {
            let val = arg.split_off(index + 1);
            arg.pop();
            let flag_name = get_short_flag_name(arg);
            println!("{}, {}", flag_name, val);
            match c.common_flags.find_short_flag(&flag_name) {
                (CalledType::Short, Some(c_flag)) => {
                    let flag_value = c_flag.flag_type.get_value_from_str(&val);
                    c.common_flags_values
                        .push((c_flag.get_name_clone(), flag_value));
                }
                (ct, chit) => {
                    match c.local_flags.find_short_flag(&flag_name) {
                        (CalledType::Short, Some(l_flag)) => {
                            let flag_value = l_flag.flag_type.get_value_from_str(&val);
                            c.local_flags_values
                                .push((l_flag.get_name_clone(), flag_value));
                        }
                        (lt, lhit) => match (&ct, &lt) {
                            (CalledType::None, CalledType::None) => {
                                println!("The inputted flag name {} is unknown.", &flag_name);
                            }
                            (_, _) => {
                                println!(
                                    "The inputted flag {} matches {}. But it is specified as short flag.",
                                    &flag_name,
                                    match (ct, lt) {
                                        (CalledType::None, _) => {
                                            format!("a non-short form of local flag {}",lhit.unwrap().get_name_clone())
                                        }
                                        (_, CalledType::None)=>{
                                            format!("a non-short form of common flag {}",chit.unwrap().get_name_clone())
                                        }
                                        (_,_)=>{
                                            format!("non-short forms of common flag {} and local flag {}",
                                                chit.unwrap().get_name_clone(), &lhit.unwrap().get_name_clone())
                                        }
                                    }
                                );
                                println!("Due to above reasons, it's interpreted unknown empty string flag.");
                                c.unknown_flags.push((flag_name, FlagValue::String(val)));
                            }
                        },
                    }
                }
            }
            parse_front_if_flags(c)
        }
        None => {
            let flag_name = get_short_flag_name(arg);
            println!("{}", flag_name);
            let long_flag_prefix: &str = "--";
            let short_flag_prefix = '-';
            match c.common_flags.find_short_flag(&flag_name) {
                (CalledType::Short, Some(c_flag)) => match c.args.pop_front() {
                    Some(next_arg) if next_arg.starts_with(long_flag_prefix) => {
                        match c_flag.flag_type {
                            FlagType::Bool => c
                                .common_flags_values
                                .push((c_flag.get_name_clone(), FlagValue::Bool(true))),
                            _ => c.common_flags_values.push((flag_name, FlagValue::None)),
                        }
                        parse_flags_starts_with_long_flag(next_arg, c)
                    }
                    Some(next_arg) if next_arg.starts_with(short_flag_prefix) => {
                        match c_flag.flag_type {
                            FlagType::Bool => c
                                .common_flags_values
                                .push((c_flag.get_name_clone(), FlagValue::Bool(true))),
                            _ => c.common_flags_values.push((flag_name, FlagValue::None)),
                        }
                        parse_flags_starts_with_short_flag(next_arg, c)
                    }
                    Some(next_arg) => match &c_flag.flag_type {
                        FlagType::Bool => match FlagValue::get_bool_value_from_str(&next_arg) {
                            FlagValue::None => {
                                c.common_flags_values
                                    .push((c_flag.name.clone(), FlagValue::Bool(true)));
                                (Some(next_arg), c)
                            }
                            val => {
                                c.common_flags_values.push((c_flag.name.clone(), val));
                                parse_front_if_flags(c)
                            }
                        },
                        ft => match ft.get_value_from_str(&next_arg) {
                            FlagValue::None => (Some(next_arg), c),
                            val => {
                                c.common_flags_values.push((flag_name, val));
                                parse_front_if_flags(c)
                            }
                        },
                    },
                    n => (n, c),
                },
                (ct, chit) => match c.local_flags.find_short_flag(&flag_name) {
                    (CalledType::Short, Some(l_flag)) => match c.args.pop_front() {
                        Some(next_arg) if next_arg.starts_with(long_flag_prefix) => {
                            match l_flag.flag_type {
                                FlagType::Bool => {
                                    c.local_flags_values
                                        .push((flag_name, FlagValue::Bool(true)));
                                }
                                _ => {
                                    c.local_flags_values.push((flag_name, FlagValue::None));
                                }
                            }
                            parse_flags_starts_with_short_flag(next_arg, c)
                        }
                        Some(next_arg) if next_arg.starts_with(short_flag_prefix) => {
                            match l_flag.flag_type {
                                FlagType::Bool => c
                                    .local_flags_values
                                    .push((l_flag.get_name_clone(), FlagValue::Bool(true))),
                                _ => c
                                    .local_flags_values
                                    .push((l_flag.name.clone(), FlagValue::None)),
                            }
                            parse_flags_starts_with_short_flag(next_arg, c)
                        }
                        Some(next_arg) => match l_flag.flag_type.get_value_from_str(&next_arg) {
                            FlagValue::None => (Some(next_arg), c),
                            val => {
                                c.local_flags_values.push((l_flag.name.clone(), val));
                                parse_front_if_flags(c)
                            }
                        },
                        n => (n, c),
                    },
                    (lt, lhit) => match (&ct, &lt) {
                        (CalledType::None, CalledType::None) => {
                            println!("The inputted flag name {} is unknown.", &flag_name);
                            c.unknown_flags
                                .push((flag_name, FlagValue::String(String::new())));
                            parse_front_if_flags(c)
                        }
                        (_, _) => {
                            println!("The inputted flag {} matches {}. But it is specified as short flag", &flag_name, match (ct, lt){
                                (CalledType::None, _) =>{
                                    format!("a non-short form of local flag {}", lhit.unwrap().get_name_clone())
                                }
                                (_, CalledType::None)=>{
                                    format!("a non-short form of common flag {}", chit.unwrap().get_name_clone())
                                }
                                (_, _)=>{
                                    format!("non-short forms of common flag {} and local flag {}", chit.unwrap().get_name_clone(), &lhit.unwrap().get_name_clone())
                                }
                            });
                            c.unknown_flags
                                .push((flag_name, FlagValue::String(String::new())));
                            parse_front_if_flags(c)
                        }
                    },
                },
            }
        }
    }
}

pub fn get_long_flag_name(mut arg: String, flag_pattern: char) -> String {
    match arg.find(|c| c != flag_pattern) {
        Some(index) => arg.split_off(index),
        None => String::default(),
    }
}

pub fn get_short_flag_name(mut arg: String) -> String {
    arg.split_off(1)
}
