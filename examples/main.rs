use combu::command::*;

fn main() {
    let root = Command::new();
    root.run_with_auto_arg_collect();
}
