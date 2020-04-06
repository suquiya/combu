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
        self.args = Vector::new(Some(args));
        self
    }
}
