use combu::{command::*, default_flag_value};
use combu::{done, flag, flags, Done, Flag, ShowHelpRequest};

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
	// flag![(test_flag=>[bool,-s,-f,--long,@"test",@def false]),];
	let _t = "test";
	println!(
		"{:?}",
		flag!(test_flag=>[bool,_t,-s,-f,--long,--long2,?false])
	);
	println!("{:?}", flag!(test_flag=>[bool,_t,-s,-f,?false]));
	println!("{:?}", flag!(test_flag=>[bool,_t,?false]));
	println!("{:?}", flag!(test_flag=>[bool,_t,false]));
	println!("{:?}", flag!(test_flag=>[bool,"aaa",?false]));
	println!("{:?}", flag!(test_flag=>[bool,"aaa",@false]));
	println!("{:?}", flag!(test_flag=>[bool,"aaa",false]));
	println!("{:?}", flag!(test_flag=>[bool,"aaa"]));
	println!("{:?}", flag!(test_flag=>[str,"aaa","aaa"]));
	println!("{:?}", flag!(test_flag=>[bool,_t,--long,--long2,?false]));
	println!(
		"{:?}",
		flag!(test_flag=>[bool,"desc",-s,-f,--long,--long2,?false])
	);
	println!(
		"{:?}",
		flag!(test_flag[bool,-s,-f,--long,--long2,=_t,?false])
	);
	println!(
		"{:?}",
		flag!(test_flag=>[bool,-s,-f,--long,--long2,="desc",?false])
	);
	println!("{:?}", flag!(test_flag=>[bool,-s,-f,="desc",?false]));
	println!("{:?}", flag!(test_flag=>[bool,,="desc",?false]));
	println!("{:?}", flag!(test_flag=>[bool,,="desc",false]));
	println!(
		"{:?}",
		flag!(test_flag[bool,-s,-f,--long,--long2,_t,?false])
	);
	println!(
		"{:?}",
		flags!(test_flag{bool, -s,-f,--long,--long2,="test",false},test_flag2:{bool, -a,-b,--long3,="test2",false},)
	);
	println!("{:?}", default_flag_value!(bool));
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
