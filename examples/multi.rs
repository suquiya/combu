use combu::{command::presets, done, ActionError, ActionResult, Command, Context, Flag, FlagValue};
/*
multi command example.
We can run as follows:
```bash
cargo run --example multi -- a 1 2 3 4 5
15
cargo run --example multi -- s 1 2 3 4 5
-13
```
 */
fn main() {
	let _ = root_command().run_from_args(std::env::args().collect());
}

fn root_command() -> Command {
	Command::with_name("multi")
		.description("root command sample: arg printer")
		.usage(presets::usage("multi"))
		.common_flag(Flag::new_bool("help").short_alias('h'))
		.common_flag(Flag::new_bool("reverse").short_alias('r'))
		.local_flag(
			Flag::new_bool("by-char")
				.short_alias('c')
				.description("process at char units"),
		)
		.action(print_args)
		.sub_command(add_command())
		.sub_command(sub_command())
}
fn call_help(c: &Context, cur_cmd: &Command) -> Result<ActionResult, ActionError> {
	println!("{}", presets::func::help(cur_cmd, c));
	done!()
}
fn print_args(current_command: Command, context: Context) -> Result<ActionResult, ActionError> {
	if called_help(&context, &current_command) {
		return call_help(&context, &current_command);
	}
	let r: bool =
		context.get_flag_value_of("reverse", &current_command) == Some(FlagValue::Bool(true));
	let c: bool =
		context.get_flag_value_of("by-char", &current_command) == Some(FlagValue::Bool(true));
	let str = {
		let str = if r && !c {
			context
				.args
				.iter()
				.rev()
				.fold(String::new(), |c, arg| c + arg)
		} else {
			context.args.iter().fold(String::new(), |c, arg| c + arg)
		};
		if c {
			str.chars().rev().collect::<String>()
		} else {
			str
		}
	};

	println!("{}", str);

	Ok(ActionResult::Done)
}

fn called_help(c: &Context, cc: &Command) -> bool {
	Some(FlagValue::Bool(true)) == c.get_flag_value_of("help", cc)
}

fn add_command() -> Command {
	Command::new()
		.name("add")
		.alias("a")
		.action(add_action)
		.local_flag(Flag::new_bool("detail").short_alias('d'))
}

fn add_action(cmd: Command, c: Context) -> Result<ActionResult, ActionError> {
	if called_help(&c, &cmd) {
		return call_help(&c, &cmd);
	}
	let f = |(str, sum), num: f64| (format!("{} {} +", str, num), sum + num);
	let (mut str, sum): (String, f64) =
		if c.get_flag_value_of("reverse", &cmd) == Some(FlagValue::Bool(true)) {
			c.args
				.iter()
				.rev()
				.filter_map(|arg| arg.parse().ok())
				.fold((String::new(), 0.0), f)
		} else {
			c.args
				.iter()
				.filter_map(|arg| arg.parse().ok())
				.fold((String::new(), 0.0), f)
		};
	str.pop();
	str.pop();

	if c.get_flag_value_of("detail", &cmd).unwrap().is_bool_true() {
		println!("{} = {}", str, sum);
	} else {
		println!("{}", sum);
	}
	Ok(ActionResult::Done)
}

fn sub_command() -> Command {
	Command::new()
		.name("sub")
		.alias("s")
		.action(sub_action)
		.local_flag(Flag::new_bool("sort").short_alias('s'))
}

fn sub_action(cmd: Command, c: Context) -> Result<ActionResult, ActionError> {
	if called_help(&c, &cmd) {
		return call_help(&c, &cmd);
	}
	let f = |(str, sum), (index, num): (usize, f64)| {
		(
			format!("{} {} -", str, num),
			if index < 1 { num } else { sum - num },
		)
	};
	let filter_map_f = |arg: &String| arg.parse().ok();
	let (mut str, result): (String, f64) =
		if c.get_flag_value_of("reverse", &cmd) == Some(FlagValue::Bool(true)) {
			c.args
				.iter()
				.rev()
				.filter_map(filter_map_f)
				.enumerate()
				.fold((String::new(), 0.0), f)
		} else if c.get_flag_value_of("sort", &cmd).unwrap().is_bool_true() {
			let mut fvec = c.args.iter().filter_map(filter_map_f).collect::<Vec<f64>>();
			fvec.sort_by(|a, b| a.partial_cmp(b).unwrap());
			fvec
				.iter_mut()
				.enumerate()
				.fold((String::new(), 0.0), |s, (index, fl)| f(s, (index, *fl)))
		} else {
			c.args
				.iter()
				.filter_map(filter_map_f)
				.enumerate()
				.fold((String::new(), 0.0), f)
		};
	str.pop();
	str.pop();

	println!("{} = {}", str, result);

	Ok(ActionResult::Done)
}
