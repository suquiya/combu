use combu::{cmd, done, flag, license, vector, Done, Flag, FlagType, ShowHelpRequest, Vector};
use combu::{command::*, flags};

fn main() {
	let action = |c| {
		println!("root_action: {:?}", c);
		Ok(ShowHelpRequest(c))
	};
	let sub = sub();
	let root = cmd!(
		root=>[
			action=>action,
			authors=>"suquiya",
			copyright=>"suquiya @2021",
			license=>license!("license_name".into(),content=>"license_content".into()),
			description=>"description",
			usage=>"usage",
			local_flags=>Vector(None),
			common_flags=>Vector(None),
			alias=>vector!["alias".into()],
			version=>"0.0.1",
			sub=>vector![sub],
			help=>presets::help
		]
	);
	let _ = root.run_with_auto_arg_collect();
	// flag![(test_flag=>[bool,-s,-f,--long,@"test",@def false]),];
	println!(
		"{:?}",
		flag!(test_flag=>[bool, -s,-f,--long,--long2,="test",false])
	);
	println!(
		"{:?}",
		flags!(test_flag:{bool, -s,-f,--long,--long2,="test",false},test_flag2:{bool, -a,-b,--long3,="test2",false})
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
