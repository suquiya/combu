use combu::{cmd, done, flag, vector, Done, Flag, FlagType, ShowHelpRequest, Vector};
use combu::{command::*, flags};

fn main() {
	let action = |c| {
		println!("root_action: {:?}", c);
		Ok(ShowHelpRequest(c))
	};
	let sub = sub();
	let r = cmd!(root=>
	[
		action,
		<"suquiya",
		@"suquiya @2021",
		#("license_name",content=>"license_content"),
		="description",
		~"usage",
		l#flags!(local=>{bool, -l,--long,="test",false},),
		c#Vector(None),
		&vector!["alias".into()],
		n "0.0.1",
		|vector![sub],
		?presets::help,
		]
	);

	//local2:{bool, -a,--long2,="test2",false}

	let _ = r.run_with_auto_arg_collect();
	// flag![(test_flag=>[bool,-s,-f,--long,@"test",@def false]),];
	println!(
		"{:?}",
		flag!(test_flag[bool, -s,-f,--long,--long2,="test",false])
	);
	println!(
		"{:?}",
		flags!(test_flag{bool, -s,-f,--long,--long2,="test",false},test_flag2:{bool, -a,-b,--long3,="test2",false},)
	);
}

fn sub() -> Command {
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
		)
}
