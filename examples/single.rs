use combu::{ActionError, ActionResult, Command, Context, Flag, FlagValue};
use std::env;

/*
single command example.
We can run as follows:
```bash
$ cargo run --example single a b c d e
abcde
$ cargo run --example single a b c d e -r
edcba
```
 */

fn main() {
	let _ = Command::with_name("single")
		.action(act)
		.local_flag(Flag::new_bool("reverse").short_alias('r'))
		.single_run(env::args().collect::<Vec<String>>());
}

fn act(cmd: Command, c: Context) -> Result<ActionResult, ActionError> {
	let r = c.get_flag_value_of("reverse", &cmd).unwrap();

	println!(
		"{}",
		match r {
			FlagValue::Bool(true) => {
				c.args
					.iter()
					.rev()
					.fold(String::new(), |concated, arg| concated + arg)
			}
			_ => {
				c.args
					.iter()
					.fold(String::new(), |concated, arg| concated + arg)
			}
		}
	);
	Ok(ActionResult::Done)
}
