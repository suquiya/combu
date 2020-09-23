use combu::command::*;
use combu::{done, Done, Flag, FlagType, ShowHelpRequest};

fn main() {
	let root = Command::new()
		.action(|c| {
			println!("root_action: {:?}", c);
			Ok(ShowHelpRequest(c))
		})
		.version("root_version")
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
					Ok(ShowHelpRequest(c))
				})
				.alias("s")
				.version("sublong_version")
				.common_flag(Flag::with_name("scommon"))
				.sub_command(Command::with_name("leaf").action(|c| {
					println!("leaf_action: {:?}", c);
					println!("common: {:?}", c.get_flag_value_of("common"));
					Ok(Done)
				}))
				.sub_command(
					Command::with_name("help")
						.action(|c| {
							println!("send help req: {:?}", c);
							/*Ok(combu::ActionResult::ShowOtherHelpRequest(
								c,
								1,
								Vector(None),
							))*/
							done!()
						})
						.version("leaf_version"),
				),
		)
		.sub_command(Command::with_name("sub2").desctiption("sub command 2"))
		.sub_command(Command::with_name("t").desctiption("test desc"));
	let _ = root.run_with_auto_arg_collect();
}
