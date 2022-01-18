use combu::command::presets::func::help_with_alias_dedup;
use combu::{check_help, Action};
use combu::{cmd, command::*};
use combu::{done, Done, Flag};

fn main() {
	let _sub = sub();
	let act: Action = |c, cmd| {
		check_help!(c, cmd, help_with_alias_dedup);
		println!("root_action: {:?}", c);
		done!()
	};
	let r = cmd!(root
	[
		>act
		<from_crate,
		@"suquiya copyright",
		@"test_license","test_license_fn",
		="test_command",
		usage:"test_usage",
		l~{tlf[="test_local_flag" -l >bool?false]},
		c~{tcf[="test_common_flag" -c >bool?false]},
		&alias,
		&alias2,
		n "0.0.1",
		+ _sub.clone(),
	]
	);

	let _ = r.run_with_auto_arg_collect();
}

fn sub() -> Command {
	Command::with_name("sublong")
		.desctiption("sub command")
		.action(|c, _| {
			println!("sub_action: {:?}", c);
			Ok(Done)
		})
		.alias("s")
		.version("sublong_version")
		.common_flag(Flag::with_name("scommon"))
		.sub_command(Command::with_name("leaf").action(|c, cc| {
			println!("leaf_action: {:?}", c);
			println!("common: {:?}", c.get_flag_value_of("common", &cc));
			Ok(Done)
		}))
		.sub_command(
			Command::with_name("help")
				.action(|c, _| {
					println!("send help req: {:?}", c);
					done!()
				})
				.version("leaf_version"),
		)
}
