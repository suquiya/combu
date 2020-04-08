use combu::command::*;

fn main() {
    let root = Command::new();
    let arg: Vec<String> = std::env::args().collect();
    root.run(arg);
}
