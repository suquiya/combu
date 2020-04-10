use crate::Flag;
use crate::Vector;

#[derive(Debug)]
pub struct Context {
    pub raw_args: Vec<String>,
    pub args: Vector<String>,
    pub common_flag: Vector<Flag>,
}

impl Context {
    pub fn new(raw_args: Vec<String>, args: Vector<String>, common_flag: Vector<Flag>) -> Context {
        Context {
            raw_args,
            args,
            common_flag,
        }
    }

    pub fn root() -> Option<Context> {
        None
    }

    pub fn args(mut self, args: Vec<String>) -> Self {
        self.args.init(Some(args));
        self
    }
}

impl From<Vec<String>> for Context {
    fn from(raw_args: Vec<String>) -> Context {
        let args = Vector::from(&raw_args);
        Context {
            raw_args,
            args,
            common_flag: Vector::default(),
        }
    }
}
