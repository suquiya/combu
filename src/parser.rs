use crate::Vector;
use crate::{Flag, FlagValue};
use std::collections::VecDeque;

pub fn parse_until_end_args(
    mut args: VecDeque<String>,
    common_flag: Vector<Flag>,
    local_flags: Vector<Flag>,
) {
    let mut non_flag_args: Vector<String> = Vector::default();
    match args.pop_front() {
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

                match args.pop_front() {
                    None => {
                        break;
                    }
                    Some(n) => arg = n,
                }
            }
        }
    }
}

pub fn parse_until_not_flag_args_or_end_args(
    mut args: VecDeque<String>,
    common_flags: Vector<Flag>,
    local_flags: Vector<Flag>,
) -> (
    Option<String>,
    Vector<(String, Option<FlagValue>)>,
    Vector<(String, Option<FlagValue>)>,
) {
    let mut non_flag_arg: Option<String>;
    let flag_prefix = '-';
    let long_flag_prefix = "--";
    let eq = "=";
    let mut arg = args.pop_front().unwrap();
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
            non_flag_arg = Some(arg);
            break;
        }

        match args.pop_front() {
            None => {
                non_flag_arg = None;
                break;
            }
            Some(n) => arg = n,
        }
    }
    (non_flag_arg, Vector(None), Vector(None))
}
