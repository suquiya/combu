use combu::command::presets::func::{help, help_tablize_with_alias_dedup};
use combu::help_command_action;
use combu::{action_result, check_help, done, preset_root, Command};
use combu::{Context, Flag};
use std::env;

fn main() {
	let _r = preset_root!(act)
		.usage(env!("CARGO_PKG_NAME").to_string() + " [args]")
		.common_flag(
			Flag::new_bool("help")
				.short_alias('h')
				.description("show help"),
		)
		.local_flag(
			Flag::new_bool("local")
				.short_alias('l')
				//.alias("test")
				.description("local flag"),
		)
		.sub_command(
			Command::with_name("sub")
				.description("sub description")
				.action(sub_act)
				.usage("sub usage")
				.local_flag(Flag::new_bool("sflag").description("sub local flag")),
		)
		.sub_command(
			Command::with_name("help")
				.description("show help")
				.action(help_command_action!(help_tablize_with_alias_dedup)),
		)
		.run_from_args(env::args().collect());
}

fn act(cmd: Command, c: Context) -> action_result!() // Or use combu::{ActionResult,ActionError} and Result<ActionResult,ActionError>
{
	check_help!(c, cmd, help_tablize_with_alias_dedup);
	println!("Hello, combu - {:?}", c.args);

	done!()
	// Or use combu::Done and Ok(Done)
}

#[allow(dead_code)]
fn sub_act(cmd: Command, c: Context) -> action_result!() {
	check_help!(c, cmd, help);
	println!("sub hello, combu - {:?}", c.args);
	done!()
}
