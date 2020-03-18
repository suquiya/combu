use crate::action::Action;
use crate::Context;
use crate::Flag;

pub struct Command {
    pub name: String,
    pub description: Option<String>,
    pub usage: String,
    pub action: Action,
    pub flags: Option<Vec<Flag>>,
    pub sub: Option<Vec<Command>>,
}

impl Default for Command {
    fn default() -> Self {
        Command {
            name: String::default(),
            description: None,
            usage: String::default(),
            action: |c: &Context| println!("{:?}", c.args),
            flags: None,
            sub: None,
        }
    }
}

impl Command {
    fn new() -> Command {
        Command::default()
    }
    fn name<T: Into<String>>(mut self, name: T) -> Command {
        self.name = name.into();
        self
    }
}
