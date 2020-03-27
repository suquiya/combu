use crate::Action;
//use crate::Context;
use crate::Flag;
use crate::Vector;

pub struct Command {
    pub name: String,
    pub description: Option<String>,
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
            description: None,
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
        Command::build_new(
            String::default(),
            None,
            String::default(),
            None,
            Vector::default(),
            Vector::default(),
            None,
            Vector::default(),
        )
    }

    pub fn build_new(
        name: String,
        description: Option<String>,
        usage: String,
        action: Option<Action>,
        l_flags: Vector<Flag>,
        common_flags: Vector<Flag>,
        version: Option<String>,
        sub: Vector<Command>,
    ) -> Command {
        Command {
            name,
            description,
            usage,
            action,
            l_flags,
            common_flags,
            version,
            sub,
        }
    }
}

impl Command {
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
        self.description = Some(description.into());
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
