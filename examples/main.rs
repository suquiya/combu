use combu::command::*;
use combu::{ActionResult, Flag, FlagType};

fn main() {
	let root = Command::new()
		.action(|c| {
			println!("root_action: {:?}", c);
			Ok(ActionResult::ShowHelpRequest(c))
		})
		.usage("root usage")
		.desctiption("example main")
		.common_flag(Flag::new("common", FlagType::default(), "Sample common flag").short_alias('c'))
		.local_flag(Flag::new("local", FlagType::default(), "Sample local flag").short_alias('l'))
		.local_flag(Flag::new("f", FlagType::Bool, "sss"))
		.local_flag(Flag::new("local2", FlagType::Bool, "sample2"))
		.sub_command(
			Command::with_name("sublong")
				.desctiption("sub command")
				.action(|c| {
					println!("sub_action: {:?}", c);
					Ok(ActionResult::ShowHelpRequest(c))
				})
				.alias("s")
				.common_flag(Flag::with_name("scommon"))
				.sub_command(Command::with_name("leaf").action(|c| {
					println!("leaf_action: {:?}", c);
					println!("common: {:?}", c.get_flag_value_of("common"));
					Ok(ActionResult::Done)
				})),
		)
		.sub_command(Command::with_name("sub2").desctiption("sub command 2"))
		.sub_command(Command::with_name("t").desctiption("test desc"));
	root.run_with_auto_arg_collect();
}
