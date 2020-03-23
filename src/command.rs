use crate::action::{Action, ActionSetter};
//use crate::Context;
use crate::Flag;

pub struct Command {
    pub name: String,
    pub description: Option<String>,
    pub usage: String,
    pub action: Option<Action>,
    pub flags: Option<Vec<Flag>>,
    pub sub: Option<Vec<Command>>,
}

impl Default for Command {
    fn default() -> Self {
        Command {
            name: String::default(),
            description: None,
            usage: String::default(),
            action: None,
            flags: None,
            sub: None,
        }
    }
}

impl Command {
    pub fn new() -> Command {
        Command::default()
    }
    pub fn name<T: Into<String>>(mut self, name: T) -> Command {
        self.name = name.into();
        self
    }
}
impl ActionSetter for Command {
    fn action(mut self, action: Action) -> Command {
        self.action = Some(action);
        self
    }
}
