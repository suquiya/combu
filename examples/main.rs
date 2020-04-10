use combu::command::Command;

fn main() {
    let root = Command::new();
    let arg: Vec<String> = std::env::args().collect();
    root.run_with_args(arg);
}
