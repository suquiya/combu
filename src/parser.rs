use crate::Context;
use crate::Vector;
use crate::{CalledType, Flag, FlagType, FlagValue};

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
    let mut non_flag_arg: Option<String> = None;
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
                } /*(_, _) => match c.local_flags.find_long_flag(&flag_name) {
                      //match local_flag
                      (CalledType::Name, Some(l_flag)) => {
                          c.local_flags_values
                              .push((flag_name, l_flag.flag_type.get_value_from_str(&val)));
                      }
                      (CalledType::Long, Some(l_flag)) => c.local_flags_values.push((
                          l_flag.name.clone(),
                          l_flag.flag_type.get_value_from_str(&val),
                      )),
                      (CalledType::Short, Some(l_flag)) => {
                          println!(
                                      "The inputted flag name {} is a short form local_flag {}. It's interpreted Unknown string flag.",
                                      flag_name, l_flag.name
                                  );
                          c.unknown_flags.push((flag_name, FlagValue::String(val)));
                      }
                      (_, _) => {
                          println!("Unknown flag {} is inputted.", flag_name);
                          c.unknown_flags.push((flag_name, FlagValue::String(val)))
                      }
                  },*/
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
                        //次の引数がフラグではないとき、フラグではないとき通常引数かフラグの値かを判定
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
                            "The inputted flag name {} is a short form of {}, but it is specified as long flag.",
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
    c: Context,
) -> (Option<String>, Context) {
    let eq = "=";
    match arg.find(eq) {
        Some(index) => {
            let val = arg.split_off(index + 1);
            arg.pop();
            let (_, flag_name) = arg.split_at(1);
            println!("{}, {}", flag_name, val);
        }
        None => {
            let (_, flag_name) = arg.split_at(1);
            println!("{}", flag_name);
        }
    }
    (None, c)
}

pub fn get_long_flag_name(mut arg: String, flag_pattern: char) -> String {
    match arg.find(|c| c != flag_pattern) {
        Some(index) => arg.split_off(index),
        None => String::default(),
    }
}

pub fn get_short_flag_name(arg: &str) -> &str {
    let (_, flag_name) = arg.split_at(1);
    flag_name
}
