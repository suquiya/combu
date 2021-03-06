use combu::{command::*, FlagType, FlagValue, Vector};
use combu::{done, flag, Done, Flag, ShowHelpRequest};

macro_rules! assert_eqs {
	($left:expr,$($right:expr),+$(,)?) => {
		$(assert_eq!($left,$right);)+
		//println!("OK: {:?}",$left);
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
	let _t_string = String::from(_t);
	let full = Flag {
		name: "test_flag".into(),
		description: _t.into(),
		short_alias: Vector(Some(vec!['s', 'f'])),
		long_alias: Vector(Some(vec!["long".to_owned(), "long2".to_owned()])),
		default_value: FlagValue::Bool(false),
		flag_type: FlagType::Bool,
	};
	let _flag_name = String::from("test_flag");
	let _flag_name2 = _flag_name.clone();
	let _flag_name3 = _flag_name.clone();
	assert_eqs!(
		full.clone(),
		flag!(->String::from("test_flag")=>[
				String::from(_t),
				Vector(Some(vec!['s', 'f'])),
				Vector(Some(vec!["long".to_owned(), "long2".to_owned()])),
				FlagType::Bool,
				FlagValue::Bool(false)
			]
		),
		flag!(@->String::from("test_flag")=>[
				String::from(_t),
				Vector(Some(vec!['s', 'f'])),
				Vector(Some(vec!["long".to_owned(), "long2".to_owned()])),
				FlagType::Bool,
				FlagValue::Bool(false)
			]
		),
		flag!(->[String::from("test_flag")]=>[
				String::from(_t),
				Vector(Some(vec!['s', 'f'])),
				Vector(Some(vec!["long".to_owned(), "long2".to_owned()])),
				FlagType::Bool,
				FlagValue::Bool(false)
			]
		),
		flag!([->String::from("test_flag")]=>[
				String::from(_t),
				Vector(Some(vec!['s', 'f'])),
				Vector(Some(vec!["long".to_owned(), "long2".to_owned()])),
				FlagType::Bool,
				FlagValue::Bool(false)
			]
		),
		flag!([->String::from("test_flag")][
				String::from(_t),
				Vector(Some(vec!['s', 'f'])),
				Vector(Some(vec!["long".to_owned(), "long2".to_owned()])),
				FlagType::Bool,
				FlagValue::Bool(false)
			]
		),
		flag!(&String::from("test_flag")=>[
				String::from(_t),
				Vector(Some(vec!['s', 'f'])),
				Vector(Some(vec!["long".to_owned(), "long2".to_owned()])),
				FlagType::Bool,
				FlagValue::Bool(false)
			]
		),
		flag!("test_flag"=>[
			String::from(_t),
			Vector(Some(vec!['s', 'f'])),
			Vector(Some(vec!["long".to_owned(), "long2".to_owned()])),
			FlagType::Bool,
			FlagValue::Bool(false)
		]),
		flag!(&_flag_name=>[
			String::from(_t),
			Vector(Some(vec!['s', 'f'])),
			Vector(Some(vec!["long".to_owned(), "long2".to_owned()])),
			FlagType::Bool,
			FlagValue::Bool(false)
		]),
		flag!(
			&_flag_name2 = [
				String::from(_t),
				Vector(Some(vec!['s', 'f'])),
				Vector(Some(vec!["long".to_owned(), "long2".to_owned()])),
				FlagType::Bool,
				FlagValue::Bool(false)
			]
		),
		flag!(
			&_flag_name3[
				String::from(_t),
				Vector(Some(vec!['s', 'f'])),
				Vector(Some(vec!["long".to_owned(), "long2".to_owned()])),
				FlagType::Bool,
				FlagValue::Bool(false)
			]
		),
		flag!(test_flag=>[
			String::from(_t),
			Vector(Some(vec!['s', 'f'])),
			Vector(Some(vec!["long".to_owned(), "long2".to_owned()])),
			FlagType::Bool,
			FlagValue::Bool(false)
		]),
		flag!(test_flag[
			String::from(_t),
			Vector(Some(vec!['s', 'f'])),
			Vector(Some(vec!["long".to_owned(), "long2".to_owned()])),
			FlagType::Bool,
			FlagValue::Bool(false)
		]),
		flag!(
			[test_flag][
				String::from(_t),
				Vector(Some(vec!['s', 'f'])),
				Vector(Some(vec!["long".to_owned(), "long2".to_owned()])),
				FlagType::Bool,
				FlagValue::Bool(false)
			]
		),
		flag!(
			[test_flag]=>[
				String::from(_t),
				Vector(Some(vec!['s', 'f'])),
				Vector(Some(vec!["long".to_owned(), "long2".to_owned()])),
				FlagType::Bool,
				FlagValue::Bool(false)
			]
		),
		flag!(
			[test_flag] = [
				String::from(_t),
				Vector(Some(vec!['s', 'f'])),
				Vector(Some(vec!["long".to_owned(), "long2".to_owned()])),
				FlagType::Bool,
				FlagValue::Bool(false)
			]
		)
	);
	let _f = String::from("test_flag");
	assert_eqs!(
		Flag::with_name("test_flag"),
		flag!(->String::from("test_flag")),
		flag!([->String::from("test_flag")]),
		flag!(->[String::from("test_flag")]),
		flag!("test_flag"),
		flag!(test_flag),
		flag!(->_f),
	);
	assert_eqs!(
		full,
		flag!(test_flag=>[
			bool,
			_t,
			Vector(Some(vec!['s', 'f'])),
			Vector(Some(vec!["long".to_owned(), "long2".to_owned()])),
			?false]),
		flag!(test_flag=>[
			bool,
			_t,
			[s, f],
			Vector(Some(vec!["long".to_owned(), "long2".to_owned()])),
			?false]),
		flag!(test_flag=>[
			bool,
			_t,
			[s f],
			Vector(Some(vec!["long".to_owned(), "long2".to_owned()])),
			?false])
	);
	// let mut s = full.clone();
	// s.long_alias.take();
	// assert_eq!(s, flag!(test_flag=>[bool,_t,-s,-f,?false]));
	// s.short_alias.take();
	// assert_eqs!(
	// 	s,
	// 	flag!(test_flag=>[bool,_t,false]),
	// 	flag!(test_flag=>[bool,_t,?]),
	// 	flag!(test_flag=>[bool,_t,?false])
	// );
	// assert_eq!(
	// 	full.clone().description(""),
	// 	flag!(test_flag=>[bool,,-s,-f,--long,--long2,?false])
	// );
	// println!("{:?}", flag!(test_flag=>[bool,,-s,-f,?false]));
	// println!("{:?}", flag!(test_flag=>[bool,"aaa",?false]));
	// println!("{:?}", flag!(test_flag=>[bool,"aaa",@false]));
	// println!("{:?}", flag!(test_flag=>[Bool,"aaa",false]));
	// println!("{:?}", flag!(test_flag=>[Bool,"aaa"]));
	// assert_eqs!(
	// 	Flag::new_bool("test_flag"),
	// 	flag!(test_flag=>[>bool,]),
	// 	flag!(test_flag=>[>bool]),
	// 	flag!(test_flag=>[>Bool]),
	// 	flag!(test_flag=>[bool,,?false]),
	// 	flag!(test_flag=>[bool,,false])
	// );
	// println!("{:?}", flag!(test_flag=>[="desc",?false]));
	// println!("{:?}", flag!(test_flag=>[="desc",bool?false]));
	// let _i = "donly";
	// println!("{:?}", flag!(test_flag=>[_i]));
	// println!("{:?}", flag!(test_flag=>[="desc bool after",bool]));
	// println!("{:?}", flag!(test_flag=>[="desc",-s,-f, bool?false]));
	// println!("{:?}", flag!(test_flag=>["desc",-s,-f, bool?false]));
	// assert_eqs!(
	// 	{ full.clone().description("desc") },
	// 	flag!(test_flag=>[="desc",-s,-f,--long,--long2 bool?false]),
	// 	flag!(test_flag=>[="desc",-s,-f,--long,--long2,bool?false]),
	// 	flag!(test_flag=>[="desc",-s,-f,--long,--long2 bool]),
	// 	flag!(test_flag=>[="desc",-s,-f,--long,--long2,bool]),
	// 	flag!(test_flag:[="desc",-s,-f,--long,--long2,?false]),
	// 	flag!(test_flag=[="desc",-s,-f,--long,--long2 ?false]),
	// 	flag!(test_flag[-s,-f,--long,--long2,="desc",bool?false]),
	// 	flag!(test_flag=>[-s,-f,--long,--long2,="desc",bool])
	// );
	// assert_eqs!(
	// 	Flag::new("test_flag", FlagType::Bool, "desc")
	// 		.short_alias('s')
	// 		.short_alias('f'),
	// 	flag!(test_flag=>[="desc",-s,-f, bool]),
	// 	flag!(test_flag=>["desc",-s,-f, bool]),
	// 	flag!(test_flag=>[="desc",-s,-f,?false])
	// );
	// println!("{:?}", flag!(test_flag=>["desc",?false]));
	// println!("{:?}", flag!(test_flag["desc"]));
	// println!("{:?}", flag!(test_flag["desc",b]));
	// println!("{:?}", flag!(test_flag[_t]));
	// println!("{:?}", flag!(test_flag=>[str,"aaa","aaa"]));
	// println!("{:?}", flag!(test_flag=>[bool,_t,--long,--long2,?false]));
	// println!(
	// 	"{:?}",
	// 	flag!(test_flag=>[bool,"desc",-s,-f,--long,--long2,?false])
	// );
	// println!(
	// 	"{:?}",
	// 	flag!(test_flag[bool,-s,-f,--long,--long2,=_t,?false])
	// );
	// println!(
	// 	"{:?}",
	// 	flag!(test_flag=>[bool,-s,-f,--long,--long2,="desc",?false])
	// );
	// println!("{:?}", flag!(test_flag=>[bool,-s,-f,="desc",?false]));
	// println!("{:?}", flag!(test_flag=>[bool,,="desc",?false]));
	// println!("{:?}", flag!(test_flag=>[bool,,="desc",false]));
	// println!(
	// 	"{:?}",
	// 	flag!(test_flag[bool,-s,-f,--long,--long2,_t,?false])
	// );
	// println!(
	// 	"{:?}",
	// 	flags!(
	//			*test_flag{bool, -s,-f,--long,--long2,="test",false},
	//			@[String::from("test_flag")]=>{bool, -s,-f,--long,--long2,="test",false},
	//			[test_flag2]=>{bool, -a,-b,--long3,="test2",false},)
	// 	);
	// println!("{:?}", default_flag_value!(bool));
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
