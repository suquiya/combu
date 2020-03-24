use crate::Flag;
use crate::{Action, Actionable};

pub struct Combu {
    pub name: String,
    pub description: Option<String>,
    pub usage: String,
    pub action: Action,
    pub flags: Option<Vec<Flag>>,
    pub version: Option<String>,
    pub sub: Option<Vec<Combu>>,
}

pub trait Cmd: Name + Actionable {
    fn name<T: Into<String>>(self, name: T) -> Self;
}

impl Cmd for Combu {
    fn name<T: Into<String>>(mut self, name: T) -> Combu {
        self.name = name.into();
        self
    }
    fn name<T: Into<String>>(mut self, name: T) -> Combu {
        self.name = name.into();
        self
    }
}

pub trait Usage {
    fn usage<T: Into<String>>(self, usage: T) -> Self;
}

pub trait Name {
    fn name<T: Into<String>>(self, name: T) -> Self;
}

pub trait LocalFlag {
    fn local_flag(self, flag: Flag) -> Self;
}

pub trait CommonFlag {
    fn common_flag(self, flag: Flag) -> Self;
}

/*pub trait SubCommand {
    fn sub_command(self, sub_command: Command) -> Self;
}*/

pub trait Description {
    fn desctiption<T: Into<String>>(self, description: T) -> Self;
}

pub trait Version {
    fn version<T: Into<String>>(self, description: T) -> Self;
}

/*pub enum SubCommand {
    Sub,
    Edge,
}
pub struct Main {
    pub name: String,
    pub description: Option<String>,
    pub usage: String,
    pub action: Option<ActionFunc>,
    pub flags: Option<Vec<Flag>>,
    pub sub_command: Option<Vec<SubCommand>>,
}*/

pub struct Sub {}

pub struct Edge {}

/*impl Default for Command {
    fn default() -> Self {
        Command {
            name: String::default(),
            description: None,
            usage: String::default(),
            action: None,
            flags: None,
            sub_command: None,
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
impl Action for Command {
    fn action(mut self, action: ActionFunc) -> Command {
        self.action = Some(action);
        self
    }
}
impl Usage for Command {
    fn usage<T: Into<String>>(mut self, usage: T) -> Command {
        self.usage = usage.into();
        self
    }
}*/
