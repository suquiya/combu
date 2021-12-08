# combu

[combu](https://crates.io/crates/combu) is a customizable cli framework.
The library name "combu" comes from command + 昆布(konbu, it means kelp in japanese).

combu has no dependencies(or depends on only std library).
Crate.io's page is [here](https://crates.io/crates/combu).

combu(com + 昆布)は柔軟に CLI を組み上げられることを目標とした、カスタマイズ可能な CLI フレームワークです（一時クレートの名前が cmb だったこともありましたが、現在は combu です）。

# Features

- flag parsing in Unix format (Unix 形式でのフラグパース)
- Nestable sub commmands (サブコマンド（多重可能）)
- No dependencies; combu depends on only std library (標準ライブラリ以外への依存ライブラリなし)
- Typed flag: Bool, String, Int and Float, inspired from seahorse（seahorse を参考にした Bool, String, Int, Float の型つきフラグ）
- common flag, local flag (コモンフラグ、ローカルフラグ両方を設定可能)
- flag parsing before sub command args(サブコマンド前のフラグの受付)
- 独自でパース等を行いたい場合に再利用できる構造体の設定
  - 似たような CLI フレームワークを作りたいときに使用できる部品を用意
- Useful presets (コマンド、フラグ等のプリセット)
- Return the result of run as Result<ActionResult, ActionError> (実行結果を Result に込めて実行後返却)

# Documentation

[Here](https://docs.rs/combu/)

# Installation to your project

Combu exists on crates.io.

You can use(or import) this crate like other crate that exists on crates.io.

## Edit cargo.toml manually

Add

```toml
combu="[version you want to use]"
```

to cargo.toml.

## Use cargo-edit (Recommended)

If you installed cargo-edit, exec below command under the target project:

```bash
cargo add combu
```

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
	let _ = Command::with_name("single")
		.action(act)
		.local_flag(Flag::new_bool("reverse").short_alias('r'))
		.single_run(env::args().collect::<Vec<String>>());
}

fn act(c: Context, cmd: Command) -> Result<ActionResult, ActionError> {
	let r = c.get_flag_value_of("reverse", &cmd).unwrap();

	println!(
		"{}",
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

```bash
$ cargo run --example single a b c d e
abcde
$ cargo run --example single a b c d e -r
edcba
```

## Multi (Have Sub command)

### Code

```rust
use combu::{command::presets, done, ActionError, ActionResult, Command, Context, Flag, FlagValue};

fn main() {
	let _ = root_command().run_from_args(std::env::args().collect());
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
fn call_help(c: &Context, cur_cmd: &Command) -> Result<ActionResult, ActionError> {
	println!("{}", presets::help(c, cur_cmd));
	done!()
}
fn print_args(context: Context, current_command: Command) -> Result<ActionResult, ActionError> {
	if called_help(&context, &current_command) {
		return call_help(&context, &current_command);
	}
	let r: bool =
		context.get_flag_value_of("reverse", &current_command) == Some(FlagValue::Bool(true));
	let c: bool =
		context.get_flag_value_of("by-char", &current_command) == Some(FlagValue::Bool(true));
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

fn called_help(c: &Context, cc: &Command) -> bool {
	Some(FlagValue::Bool(true)) == c.get_flag_value_of("help", cc)
}

fn add_command() -> Command {
	Command::new()
		.name("add")
		.alias("a")
		.action(add_action)
		.local_flag(Flag::new_bool("detail").short_alias('d'))
}

fn add_action(c: Context, cmd: Command) -> Result<ActionResult, ActionError> {
	if called_help(&c, &cmd) {
		return call_help(&c, &cmd);
	}
	let f = |(str, sum), num: f64| (format!("{} {} +", str, num), sum + num);
	let (mut str, sum): (String, f64) =
		if c.get_flag_value_of("reverse", &cmd) == Some(FlagValue::Bool(true)) {
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

	if c.get_flag_value_of("detail", &cmd).unwrap().is_bool_true() {
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

fn sub_action(c: Context, cmd: Command) -> Result<ActionResult, ActionError> {
	if called_help(&c, &cmd) {
		return call_help(&c, &cmd);
	}
	let f = |(str, sum), (index, num): (usize, f64)| {
		(
			format!("{} {} -", str, num),
			if index < 1 { num } else { sum - num },
		)
	};
	let filter_map_f = |arg: &String| arg.parse().ok();
	let (mut str, result): (String, f64) =
		if c.get_flag_value_of("reverse", &cmd) == Some(FlagValue::Bool(true)) {
			c.args
				.iter()
				.rev()
				.filter_map(filter_map_f)
				.enumerate()
				.fold((String::new(), 0.0), f)
		} else if c.get_flag_value_of("sort", &cmd).unwrap().is_bool_true() {
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

# TODO(or Features to be implemented)

- ドキュメントコメントを分かりやすくする(いつになるかは無期限未定)
- 必要そうなテストの実装(`command.rs` のフラグ解析テストは実装した)
- コマンド構築にあたってのプリセット実装

# License

This is licensed under [MIT LICENSE](https://github.com/suquiya/combu/blob/master/LICENSE)
