use combu::{command::*, default_flag_value, FlagType, FlagValue, Vector};
use combu::{done, flag, flags, Done, Flag, ShowHelpRequest};

macro_rules! assert_eqs {
	($left:expr,$($right:expr),+) => {
		$(assert_eq!($left,$right);)+
	};
}
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
	let mut full = Flag {
		name: "test_flag".into(),
		description: _t.into(),
		short_alias: Vector(Some(vec!['s', 'f'])),
		long_alias: Vector(Some(vec!["long".to_owned(), "long2".to_owned()])),
		default_value: FlagValue::Bool(false),
		flag_type: FlagType::Bool,
	};
	assert_eq!(
		full,
		flag!(test_flag=>[bool,_t,-s,-f,--long,--long2,?false])
	);
	let mut s = full.clone();
	s.long_alias.take();
	assert_eq!(s, flag!(test_flag=>[bool,_t,-s,-f,?false]));
	s.short_alias.take();
	assert_eqs!(
		s,
		flag!(test_flag=>[bool,_t,false]),
		flag!(test_flag=>[bool,_t,?]),
		flag!(test_flag=>[bool,_t,?false])
	);
	println!(
		"{:?}",
		flag!(test_flag=>[bool,,-s,-f,--long,--long2,?false])
	);
	println!("{:?}", flag!(test_flag=>[bool,,-s,-f,?false]));
	println!("{:?}", flag!(test_flag=>[bool,,?false]));
	println!("{:?}", flag!(test_flag=>[bool,,false]));
	println!("{:?}", flag!(test_flag=>[bool,"aaa",?false]));
	println!("{:?}", flag!(test_flag=>[bool,"aaa",@false]));
	println!("{:?}", flag!(test_flag=>[Bool,"aaa",false]));
	println!("{:?}", flag!(test_flag=>[Bool,"aaa"]));
	assert_eqs!(
		flag!(test_flag=>[>bool,]),
		flag!(test_flag=>[>bool]),
		flag!(test_flag=>[>Bool])
	);
	println!("{:?}", flag!(test_flag=>[="desc",?false]));
	println!("{:?}", flag!(test_flag=>[="desc",-s,-f,?false]));
	println!("{:?}", flag!(test_flag=>[="desc",bool?false]));
	let _i = "donly";
	println!("{:?}", flag!(test_flag=>[_i]));
	println!("{:?}", flag!(test_flag=>[="desc bool after",bool]));
	println!("{:?}", flag!(test_flag=>[="desc",-s,-f, bool?false]));
	println!("{:?}", flag!(test_flag=>["desc",-s,-f, bool?false]));
	println!(
		"{:?}",
		flag!(test_flag=>[="desc",-s,-f,--long,--long2 bool?false])
	);
	println!(
		"{:?}",
		flag!(test_flag=>[="desc",-s,-f,--long,--long2,bool?false])
	);
	println!("{:?}", flag!(test_flag=>[="desc",-s,-f, bool]));
	println!("{:?}", flag!(test_flag=>["desc",-s,-f, bool]));
	println!(
		"{:?}",
		flag!(test_flag=>[="desc",-s,-f,--long,--long2 bool])
	);
	println!(
		"{:?}",
		flag!(test_flag=>[="desc",-s,-f,--long,--long2,bool])
	);
	println!(
		"{:?}",
		flag!(test_flag=>[="desc",-s,-f,--long,--long2,?false])
	);
	println!("{:?}", flag!(dd=>["d",?false]));
	println!(
		"{:?}",
		flag!(test_flag=>[="desc",-s,-f,--long,--long2 ?false])
	);
	println!(
		"{:?}",
		flag!(test_flag=>[-s,-f,--long,--long2,="longb",bool?false])
	);
	println!(
		"{:?}",
		flag!(test_flag=>[-s,-f,--long,--long2,="longb",bool])
	);
	println!("{:?}", flag!(test_flag["desc"]));
	println!("{:?}", flag!(test_flag["desc",b]));
	println!("{:?}", flag!(test_flag[_t]));
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
