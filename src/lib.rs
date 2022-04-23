#![warn(missing_docs)]
#![forbid(unsafe_code)]
#![warn(rust_2018_idioms)]
#![warn(clippy::missing_docs_in_private_items)]

/*! [combu](https://crates.io/crates/combu) is a customizable cli framework crate.
The library name "combu" comes from command + 昆布(konbu, it means kelp in japanese).

combu has no dependencies(or depends on only std library).
Crate.io's page is [here](https://crates.io/crates/combu).

To know more about this crate, please read README(You can read [file](../README.md) OR [Github](https://github.com/suquiya/combu)) first.

combu(com + 昆布)は柔軟に CLI を組み上げられることを目標とした、カスタマイズ可能な CLI フレームワークです（一時クレートの名前が cmb だったこともありましたが、現在は combu です）。

もう少し詳しく知りたい場合、まず初めにREADME(ファイルは[こちら](../README.md)、Githubのページは[こちら](https://github.com/suquiya/combu))をお読みください。

 */

/// action is a module about Action
pub mod action;
/// command is a module about command
pub mod command;

// hook is erased.
// /// hook for command execution
// pub mod hook;

mod context;
/// flag is a module about flag
pub mod flag;
/// parser is a module about command args parser
pub mod parser;
/// vector is a moudle about vector
pub mod vector;

pub use action::{
	Action, ActionError, ActionResult, ActionResult::Done, ActionResult::ParentActionRequest,
};
pub use command::Command;
pub use context::Context;
pub use flag::{Flag, FlagType, FlagValue};
// pub use hook::Hook;
pub use parser::Parser;
pub use vector::Vector;

/// Macros for combu
pub mod pub_macros;
