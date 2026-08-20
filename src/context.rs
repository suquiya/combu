use std::collections::VecDeque;

use crate::args::Arg;

/// Storage information for command execution.
/// This storage raw args, non-flag args, flag values, and etc.
/// コマンドからrunを通ってactionにたどり着くまでの情報およびパース結果を格納する構造体。
/// フラグの値、たどってきたルートなどを保管しています。
pub struct Context {
	/// args (almost raw, recoverable)
	pub raw_args: Vec<Arg>,
	/// args (parsed or parsing for run)
	pub args: VecDeque<Arg>,
	/// exe path
	pub exe_path: String,
}
