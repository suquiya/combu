# combu

![crates.io](https://img.shields.io/crates/v/combu)
![version](https://img.shields.io/github/v/tag/suquiya/combu)
![license](https://img.shields.io/github/license/suquiya/combu)

<div align="center">
	<img src="combu_logo.png"></img>
</div>

[combu](https://crates.io/crates/combu) is a customizable (or rather bare bones...?) cli framework.
The library name "combu" comes from command + 昆布(konbu, it means kelp in japanese).

combu has no dependencies(or depends on only std library).
Crate.io's page is [here](https://crates.io/crates/combu).

combu(com + 昆布)は柔軟に CLI を組み上げられることを目標とした、カスタマイズ可能(というより骨組みを組み上げてあるだけのような気もする)な CLI フレームワークです。
（一時クレートの名前が cmb だったこともありましたが、現在は combu です）

# Features (特徴)

- flag parsing in Unix format (Unix 形式でのフラグパース)
- Nestable sub commmands (サブコマンド（多重可能）)
- No dependencies; combu depends on only std library (標準ライブラリ以外への依存ライブラリなし)
- Typed flag: Bool, String, Int and Float, inspired from seahorse（seahorse を参考にした Bool, String, Int, Float の型つきフラグ）
- common flag, local flag (コモンフラグ、ローカルフラグ両方を設定可能)
- flag parsing before sub command args(サブコマンド前のフラグの受付)
- 独自でパース等を行いたい場合に再利用できそうな構造体の設定
  - 似たような CLI フレームワークを作りたいときに使用できる部品を用意
- Useful presets (コマンド、フラグ等のプリセット)
- Return the result of run as Result<ActionResult, ActionError> (実行結果を Result に込めて実行後返却)

# Documentation

[Here](https://docs.rs/combu/)

# Installation to your project (プロジェクトでの使用方法)

Combu exists on crates.io.
You can use(or import) this crate like other crate that exists on crates.io.

combu は crates.io に登録してありますので、他の crates.io 上のクレートと同じように使用（インポート）することが可能です。

## Edit cargo.toml manually (手動での cargo.toml への追加)

Add

```toml
combu="[version you want to use]"
```

to cargo.toml.

上記コードでバージョンを指定して、cargo.toml に追加してください。

## Use cargo-edit (Recommended) (cargo-edit でプロジェクトに追加する(推奨))

If you installed cargo-edit, exec below command under the target project:

```bash
cargo add combu
```

cargo-edit をインストールしてある場合は、上記のコマンドを実行することで使用可能です。

# Quick Start

```rust
use combu::command::presets::func::help_tablize_with_alias_dedup;
use combu::{action_result, check_error, check_help, done, preset_root, Command};
use combu::{Context, Flag};
use std::env;

fn main() {
	let _r = preset_root!(act)
		.usage(env!("CARGO_PKG_NAME").to_string() + " [args]")
		.common_flag(
			Flag::new_bool("help")
				.short_alias('h')
				.description("show help"),
		)
		.local_flag(
			Flag::new_bool("local")
				.short_alias('l')
				.description("local flag"),
		)
		.run_from_args(env::args().collect());
}

fn act(cmd: Command, c: Context) -> action_result!()
{
	check_error!(cmd, c);
	check_help!(cmd, c, help_tablize_with_alias_dedup);
	println!("Hello, combu - {:?}", c.args);

	done!()
}
```

If you want to run quick start as example, exec

```bash
cargo run --example quick_start
cargo run --example quick_start --help
```

More detail: See [quick_start.rs](examples/quick_start.rs)

# Example

## [Single (command has flags, but not has subcommand)](examples/single.rs)

## [Multi (Have Sub command)](examples/multi.rs)

# Inspired

- [cobra](https://github.com/spf13/cobra) (Golang package for making cli)
- [seahorse](https://github.com/ksk001100/seahorse) ([A minimal CLI framework written in Rust](https://github.com/ksk001100/seahorse/blob/master/README.md))
- [clap](https://github.com/clap-rs/clap)(Rust crate for making cli)

# TODO(or Features to be implemented)

- ドキュメントコメントを分かりやすくする(いつになるかは無期限未定)
- 必要そうなテストの実装(`command.rs`は済んでいる、官僚は無期限未定)
- コマンド構築にあたってのプリセット実装(主だったプリセット実装は済んでいるが、追加の可能性あり)

# CONTRIBUTING

If you want to contribute combu, please read [CONTRIBUTING.md](CONTRIBUTING.md) for checking our code of conduct, and submitting pull requests to us.

# Author(s?)

[suquiya](https://github.com/suquiya).

README Contribute: [ksk001100](https://github.com/ksk001100) contributed on [this pull request](https://github.com/suquiya/combu/pull/1).

# License

This is licensed under [MIT LICENSE](https://github.com/suquiya/combu/blob/main/LICENSE)

# Information about construct for this README.md

This readme is mainly based on [seahorse's readme](https://github.com/ksk001100/seahorse) - Copyright (c) 2019 Keisuke Toyota - licensed [MIT License](https://github.com/ksk001100/seahorse/blob/master/LICENSE).
