use crate::Action;

use crate::Context;
use crate::Flag;
use crate::Vector;

#[derive(Default)]
pub struct Command {
    pub name: String,
    pub action: Option<Action>,
    pub authors: String,
    pub copyright: String,
    pub description: Option<String>,
    pub usage: String,
    pub l_flags: Vector<Flag>,
    pub c_flags: Vector<Flag>,
    pub version: String,
    pub sub: Vector<Command>,
    pub opt_props: Vector<KeyValuePair>,
}

pub type KeyValuePair = (String, String);

/*impl Default for Command {
    fn default() -> Self {
        Command {
            name: String::default(),
            action: None,
            authors: String::default(),
            copyright: String::default(),
            description: None,
            usage: String::default(),
            l_flags: Vector::default(),
            c_flags: Vector::default(),
            sub: Vector::default(),
            version: String::default(),
            opt_props: Vector::default(),
        }
    }
}*/

impl Command {
    pub fn new() -> Command {
        Command::default()
    }

    pub fn build_new(
        name: String,
        action: Option<Action>,
        authors: String,
        copyright: String,
        description: Option<String>,
        usage: String,
        local_flags: Vector<Flag>,
        common_flags: Vector<Flag>,
        version: String,
        sub: Vector<Command>,
        opt_props: Vector<(String, String)>,
    ) -> Command {
        Command {
            name,
            action,
            authors,
            copyright,
            description,
            usage,
            l_flags: local_flags,
            c_flags: common_flags,
            version,
            sub,
            opt_props,
        }
    }

    pub fn run_with_auto_arg_collect(self) {
        self.run(std::env::args().collect(), None);
    }

    pub fn single_run(self, args: Vec<String>) {
        match self.action {
            Some(action) => {
                action(&Context::new(args));
            }
            None => {
                self.show_help();
            }
        }
    }

    pub fn show_help(self) {}

    pub fn run(self, args: Vec<String>, c: Option<Context>) {
        match c {
            None => {}
            Some(mut c) => {}
        }
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
        self.c_flags.push(flag);
        self
    }

    pub fn desctiption<T: Into<String>>(mut self, description: T) -> Self {
        self.description = Some(description.into());
        self
    }

    pub fn version<T: Into<String>>(mut self, version: T) -> Self {
        self.version = version.into();
        self
    }

    pub fn sub_command(mut self, sub_command: Command) -> Self {
        self.sub.push(sub_command);
        self
    }

    pub fn add_opt_prop(mut self, opt_prop: KeyValuePair) -> Self {
        self.opt_props.push(opt_prop);
        self
    }
}
