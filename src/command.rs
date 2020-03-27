use crate::Action;
//use crate::Context;
use crate::utils::Vector;
use crate::Flag;

pub struct Command {
    pub name: String,
    pub desctiption: Option<String>,
    pub usage: String,
    pub action: Option<Action>,
    pub l_flags: Vector<Flag>,
    pub common_flags: Vector<Flag>,
    pub version: Option<String>,
    pub sub: Vector<Command>,
}

impl Default for Command {
    fn default() -> Self {
        Command {
            name: String::default(),
            desctiption: None,
            usage: String::default(),
            action: None,
            l_flags: Vector::default(),
            common_flags: Vector::default(),
            sub: Vector::default(),
            version: None,
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

    pub fn usage<T: Into<String>>(mut self, usage: T) -> Command {
        self.usage = usage.into();
        self
    }

    pub fn action(mut self, action: Action) -> Self {
        self.action = Some(action);
        self
    }

    pub fn local_flag(mut self, flag: Flag) -> Self {
        self.l_flags.push(flag);
        self
    }

    pub fn common_flag(mut self, flag: Flag) -> Self {
        self.common_flags.push(flag);
        self
    }

    pub fn desctiption<T: Into<String>>(mut self, description: T) -> Self {
        self.desctiption = Some(description.into());
        self
    }

    pub fn version<T: Into<String>>(mut self, version: T) -> Self {
        self.version = Some(version.into());
        self
    }

    pub fn sub_command(mut self, sub_command: Command) -> Self {
        self.sub.push(sub_command);
        self
    }
}
