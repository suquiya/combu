use crate::Context;
use crate::Vector;
use crate::{Flag, FlagValue};
use std::collections::VecDeque;

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

pub fn parse_flags_until_not_flag_args_or_end_args(
    mut arg: String,
    c: Context,
) -> (Option<String>, Context) {
    let mut non_flag_arg: Option<String> = None;
    let long_flag_prefix = "--";
    let eq = '=';

    if arg.starts_with(long_flag_prefix) {
        let flag_pattern = '-';
        match arg.find(eq) {
            Some(index) => {
                let val = arg.split_off(index + 1);
                arg.pop();
                let flag_name = get_long_flag_name(&arg, flag_pattern);
                println!("{}, {}", flag_name, val);
            }
            None => {
                let flag_name = get_long_flag_name(&arg, flag_pattern);
                println!("{}", flag_name);
            }
        }
    } else {
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
    }
    /*let mut arg = args.pop_front().unwrap();
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
    }*/
    (non_flag_arg, c)
}

pub fn get_long_flag_name<'a: 'b, 'b>(arg: &'a str, flag_pattern: char) -> &'b str {
    arg.trim_start_matches(flag_pattern)
}

pub fn get_short_flag_name(arg: &str) -> &str {
    let (_, flag_name) = arg.split_at(1);
    flag_name
}
