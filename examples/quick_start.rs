use combu::command::presets::func::{help, help_tablize_with_alias_dedup};
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
		/* If you want to use help subcommand,uncomment this block and add preset_help_command to use.
		.sub_command(preset_help_command!(help_tablize_with_alias_dedup))
		*/
		/* If you want to use subcommand, uncomment this block, then remove this line and the line above sub_act function.
		.sub_command(
			Command::with_name("sub")
				.desctiption("sub description")
				.action(sub_act)
				.usage("sub usage")
				.local_flag(Flag::new_bool("sflag").description("sub local flag")),
		)*/
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
