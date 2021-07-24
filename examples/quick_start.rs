use combu::{action_result, check_help, done, preset_root};
use combu::{Context, Flag};
use std::env;

fn main() {
	let _r = preset_root!(act)
		.usage(env!("CARGO_PKG_NAME").to_string() + " [args]")
		.common_flag(Flag::new_bool("help").short_alias('h'))
		.run_from_args(env::args().collect());
}

fn act(c: Context) -> action_result!() // Or use combu::{ActionResult,ActionError} and Result<ActionResult,ActionError>
{
	check_help!(c);
	println!("Hello, combu - {:?}", c.args);

	done!()
	// Or use combu::Done and Ok(Done)
}
