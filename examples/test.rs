use combu::command::*;
use combu::{done, Done, Flag};

fn main() {
	/*let action = |c| {
		println!("root_action: {:?}", c);
		Ok(ShowHelpRequest(c))
	};*/
	let _sub = sub();
	/*let r = cmd!(root:
	[
		|c| {
			println!("root_action: {:?}", c);
			Ok(ShowHelpRequest(c))
		},
		<...,
		@[2021,suquiya],
		+("license_name",content=>"license_content"),
		="description",
		:"usage",
		l#flags!(local=>{bool, -l,--long,="test",false},),
		c#Vector(None),
		&vector!["alias".into()],
		n "0.0.1",
		|vector![sub],
		?presets::help,
	]
	);*/

	//local2:{bool, -a,--long2,="test2",false}

	//let _ = r.run_with_auto_arg_collect();
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
