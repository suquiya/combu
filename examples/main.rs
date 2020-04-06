use combu::command::Run;
use combu::Command;

fn main() {
    let root = Command::new();
    let arg: Vec<String> = std::env::args().collect();
    root.run(arg);
}
