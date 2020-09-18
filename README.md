# combu

[combu](https://crates.io/crates/combu) is a customizable cli framework.
The library name "combu" comes from command + 昆布(konbu, it means kelp in japanese).
Crate.io's page is [here](https://crates.io/crates/combu).

combu(com + 昆布)は柔軟な設計を目標とした、カスタマイズ可能な CLI フレームワークです（一時クレートの名前が cmb だったこともありましたが、現在は combu です）。

# Documentation

[Here](https://docs.rs/combu/)

# Installation to your project

Combu exists on crates.io.

## Edit cargo.toml manually

Add

```toml
combu="0.1.6"
```

to cargo.toml.

## Use cargo-edit (Recommended)

Exec

```bash
cargo add combu
```

under target project.

# Quick Start

```rust
use combu::{ActionError, ActionResult, Command, Context, Flag, FlagValue};
use std::env;

fn main() {
	Command::new()
		.name(env!("CARGO_PKG_NAME"))
		.authors(env!("CARGO_PKG_AUTHORS"))
		.version(env!("CARGO_PKG_VERSION"))
		.usage(env!("CARGO_PKG_NAME").to_string() + " [args]")
		.common_flag(Flag::new_bool("help").short_alias('h'))
		.action(act)
		.run_from_args(env::args().collect())
}

fn act(c: Context) -> Result<ActionResult, ActionError> {
	if Some(FlagValue::Bool(true)) == c.get_flag_value_of("help") {
		return Ok(ActionResult::ShowHelpRequest(c));
	}
	println!("Hello, combu - {:?}", c.args);
	Ok(ActionResult::Done)
}
```

If you want to run quick start as example, exec

```bash
cargo run --example quick_start
cargo run --example quick_start --help
```

# Example

## Simple (command has flags, but not has subcommand)

### Code

```rust
use combu::{ActionError, ActionResult, Command, Context, Flag, FlagValue};
use std::env;

fn main() {
	Command::with_name("single")
		.action(act)
		.local_flag(Flag::new_bool("reverse").short_alias('r'))
		.single_run(env::args().collect::<Vec<String>>());
}

fn act(c: Context) -> Result<ActionResult, ActionError> {
	let r = c.get_flag_value_of("reverse").unwrap();

	println!(
		"{:?}",
		match r {
			FlagValue::Bool(true) => {
				c.args
					.iter()
					.rev()
					.fold(String::new(), |concated, arg| concated + arg)
			}
			_ => {
				c.args
					.iter()
					.fold(String::new(), |concated, arg| concated + arg)
			}
		}
	);
	Ok(ActionResult::Done)
}

```

### Run

```
$ cargo run --example single a b c d e
abcde
$ cargo run --example single a b c d e -r
edcba
```

## Multi (Use Sub command)

### Code

```rust
use combu::{ActionError, ActionResult, Command, Context, Flag, FlagValue};

fn main() {
	root_command().run_from_args(std::env::args().collect())
}

fn root_command() -> Command {
	Command::with_name("multi")
		.common_flag(Flag::new_bool("help").short_alias('h'))
		.common_flag(Flag::new_bool("reverse").short_alias('r'))
		.local_flag(Flag::new_bool("by-char").short_alias('c'))
		.action(print_args)
		.sub_command(add_command())
		.sub_command(sub_command())
}
fn call_help(c: Context) -> Result<ActionResult, ActionError> {
	Ok(ActionResult::ShowHelpRequest(c))
}
fn print_args(context: Context) -> Result<ActionResult, ActionError> {
	if called_help(&context) {
		return call_help(context);
	}
	let r: bool = context.get_flag_value_of("reverse") == Some(FlagValue::Bool(true));
	let c: bool = context.get_flag_value_of("by-char") == Some(FlagValue::Bool(true));
	let str = {
		let str = if r && !c {
			context
				.args
				.iter()
				.rev()
				.fold(String::new(), |c, arg| c + arg)
		} else {
			context.args.iter().fold(String::new(), |c, arg| c + arg)
		};
		if c {
			str.chars().rev().collect::<String>()
		} else {
			str
		}
	};

	println!("{}", str);

	Ok(ActionResult::Done)
}

fn called_help(c: &Context) -> bool {
	Some(FlagValue::Bool(true)) == c.get_flag_value_of("help")
}

fn add_command() -> Command {
	Command::new()
		.name("add")
		.alias("a")
		.action(add_action)
		.local_flag(Flag::new_bool("detail").short_alias('d'))
}

fn add_action(c: Context) -> Result<ActionResult, ActionError> {
	if called_help(&c) {
		return call_help(c);
	}
	let f = |(str, sum), num: f64| (format!("{} {} +", str, num), sum + num);
	let (mut str, sum): (String, f64) =
		if c.get_flag_value_of("reverse") == Some(FlagValue::Bool(true)) {
			c.args
				.iter()
				.rev()
				.filter_map(|arg| arg.parse().ok())
				.fold((String::new(), 0.0), f)
		} else {
			c.args
				.iter()
				.filter_map(|arg| arg.parse().ok())
				.fold((String::new(), 0.0), f)
		};
	str.pop();
	str.pop();

	if c.get_flag_value_of("detail").unwrap().is_bool_true() {
		println!("{} = {}", str, sum);
	} else {
		println!("{}", sum);
	}
	Ok(ActionResult::Done)
}

fn sub_command() -> Command {
	Command::new()
		.name("sub")
		.alias("s")
		.action(sub_action)
		.local_flag(Flag::new_bool("sort").short_alias('s'))
}

fn sub_action(c: Context) -> Result<ActionResult, ActionError> {
	if called_help(&c) {
		return call_help(c);
	}
	let f = |(str, sum), (index, num): (usize, f64)| {
		(
			format!("{} {} -", str, num),
			if index < 1 { num } else { sum - num },
		)
	};
	let filter_map_f = |arg: &String| arg.parse().ok();
	let (mut str, result): (String, f64) =
		if c.get_flag_value_of("reverse") == Some(FlagValue::Bool(true)) {
			c.args
				.iter()
				.rev()
				.filter_map(filter_map_f)
				.enumerate()
				.fold((String::new(), 0.0), f)
		} else if c.get_flag_value_of("sort").unwrap().is_bool_true() {
			let mut fvec = c.args.iter().filter_map(filter_map_f).collect::<Vec<f64>>();
			fvec.sort_by(|a, b| a.partial_cmp(b).unwrap());
			fvec
				.iter_mut()
				.enumerate()
				.fold((String::new(), 0.0), |s, (index, fl)| f(s, (index, *fl)))
		} else {
			c.args
				.iter()
				.filter_map(filter_map_f)
				.enumerate()
				.fold((String::new(), 0.0), f)
		};
	str.pop();
	str.pop();

	println!("{} = {}", str, result);

	Ok(ActionResult::Done)
}
```

### Run

```bash
cargo run --example multi -- a 1 2 3 4 5
15
cargo run --example multi -- s 1 2 3 4 5
-13
```

# Inspired

- [cobra](https://github.com/spf13/cobra) (Golang package for making cli)
- [seahorse](https://github.com/ksk001100/seahorse) ([A minimal CLI framework written in Rust](https://github.com/ksk001100/seahorse/blob/master/README.md))
- [clap](https://github.com/clap-rs/clap)(Rust crate for making cli)

# Features

- Unix 形式でのフラグパース
- サブコマンド（多重可能）
- No dependencies(combu depends on only std library)
- seahorse を参考にした Bool, String, Int, Float の型つきフラグ
- コモンフラグ、ローカルフラグ形式
- サブコマンド前のフラグの受付
- 独自でパース等を行いたい場合に再利用できる構造体の設定
  - 似たような CLI フレームワークを作りたいときに使用できる部品を用意
- カスタマイズできるヘルプ表示(一応)
- エラーハンドリングなどに関して、ある程度の自由度を持たせた設計

# TODO(or Features to be implemented)

- ドキュメントコメントを分かりやすくする(いつになるかは無期限未定)
- テストの実装(`command.rs` のみ実装中)
- コマンド構築にあたってのプリセット実装

# License

This is licensed under [MIT LICENSE](https://github.com/suquiya/combu/blob/master/LICENSE)
