# combu

[combu](https://crates.io/crates/combu) is a customizable cli framework.
The library name "combu" comes from command + 昆布(konbu, it means kelp in japanese).
Crate.io's page is [here](https://crates.io/crates/combu).

combu(com + 昆布)は柔軟な設計を目標とした、カスタマイズ可能な CLI フレームワークです（一時クレートの名前が cmb だったこともありましたが、現在は combu です）。

# Documentation

[Here](https://docs.rs/combu/)

# Installation to your project

Combu exists on crates.io.
To use combu,

```toml
combu="0.1.2"
```

Or (if you installed [cargo-edit](https://crates.io/crates/cargo-edit))

```bash
cargo add combu
```

# Quick Start

# Example

## Simple (command not has subcommand)

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

## Sub command

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

# TODO(or Features to be implemented)

- エラーハンドリングなどに関して、ある程度の自由度を持たせた設計
- ドキュメントコメントの整備(now implementing...)
- テストの実装(`command.rs` のみ実装中)
- コマンド構築にあたってのプリセット実装
