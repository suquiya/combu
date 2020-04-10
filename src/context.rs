use crate::Flag;
use crate::Vector;
use std::path::PathBuf;

#[derive(Debug)]
pub struct Context {
    pub raw_args: Vec<String>,
    pub args: Vector<String>,
    pub common_flag: Vector<Flag>,
    pub current_path: PathBuf,
}

impl Context {
    pub fn new(
        raw_args: Vec<String>,
        args: Vector<String>,
        common_flag: Vector<Flag>,
        current_path: PathBuf,
    ) -> Context {
        Context {
            raw_args,
            args,
            common_flag,
            current_path,
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
        let current_path = match &raw_args.get(0) {
            Some(str) => PathBuf::from(str),
            None => PathBuf::new(),
        };
        Context {
            raw_args,
            args,
            common_flag: Vector::default(),
            current_path,
        }
    }
}
