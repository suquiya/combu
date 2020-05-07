use combu::command::*;
use combu::{Flag, FlagType};

fn main() {
    let root = Command::new()
        .action(|_| println!("test_action"))
        .common_flag(Flag::new(
            "common",
            "sample common flag",
            FlagType::default(),
        ))
        .local_flag(Flag::new("local", "sample local flag", FlagType::default()));
    root.run_with_auto_arg_collect();
}
