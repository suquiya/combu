use crate::Flag;
use crate::Vector;

#[derive(Debug)]
pub struct Context {
    pub raw_args: Vec<String>,
    pub args: Vector<String>,
    pub common_flag: Vector<Flag>,
}

impl Context {
    pub fn new(raw_args: Vec<String>) -> Context {
        Context {
            raw_args,
            args: Vector::default(),
            common_flag: Vector::default(),
        }
    }

    pub fn root() -> Option<Context> {
        None
    }

    pub fn args(mut self, args: Vec<String>) -> Self {
        self.args = Vector::init(Some(args));
        self
    }
}
