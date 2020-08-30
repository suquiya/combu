# combu(WIP)

combu is a customizable cli framework(crate name was cmb, renamed to combu). Work In Progress, and Implimenting.
The library name "combu" comes from command + 昆布(konbu, it means kelp in japanese).

combu(com + 昆布)は柔軟な設計を目標とした、カスタマイズ可能な CLI フレームワークです（一時クレートの名前が cmb だったこともありましたが、現在は combu です）。
まだまだ実装中（WIP）で、使用できる段階ではありません。

# Inspired

- [cobra](https://github.com/spf13/cobra) (Golang package for making cli)
- [seahorse](https://github.com/ksk001100/seahorse) ([A minimal CLI framework written in Rust](https://github.com/ksk001100/seahorse/blob/master/README.md))

# Features

- Unix 形式でのフラグパース
- サブコマンド（多重可能）
- No dependencies(combu depends on only std library)
- seahorse を参考にした Bool, String, Int, Float の型つきフラグ
- コモンフラグ、ローカルフラグ形式
- 独自でパース等を行いたい場合に再利用できる構造体の設定
  - 似たような CLI フレームワークを作りたいときに使用できる部品を用意
  - サブコマンド前のフラグの受付

# TODO(or Features to be implemented)

- カスタマイズできるヘルプ表示
- デフォルトのヘルプ表示の実装(Now impl)
- エラーハンドリングなどに関して、ある程度の自由度を持たせた設計
- ドキュメントコメントの整備
- テストの実装(command.rs のみ実装中)
