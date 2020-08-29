use combu::command::*;
use combu::{ActionResult, Flag, FlagType};

fn main() {
	let root = Command::new()
		.action(|c| {
			println!("root_action: {:?}", c);
			Ok(ActionResult::ShowHelpRequest(c))
		})
		.usage("root usage")
		.common_flag(Flag::new(
			"common",
			"sample common flag",
			FlagType::default(),
		))
		.local_flag(Flag::new("local", "sample local flag", FlagType::default()))
		.sub_command(
			Command::with_name("sub")
				.action(|c| {
					println!("sub_action: {:?}", c);
					Ok(ActionResult::Done)
				})
				.sub_command(Command::with_name("leaf").action(|c| {
					println!("leaf_action: {:?}", c);
					println!("common: {:?}", c.get_flag_value_of("common"));
					Ok(ActionResult::Done)
				})),
		)
		.sub_command(Command::with_name("sub2"));
	root.run_with_auto_arg_collect();
}
