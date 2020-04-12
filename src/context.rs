use crate::Vector;
use crate::{Flag, FlagType, FlagValue};
use std::collections::VecDeque;
use std::path::PathBuf;

#[derive(Debug)]
pub struct Context {
    pub raw_args: Vec<String>,
    pub args: VecDeque<String>,
    pub parsed_common: Vector<Flag>,
    pub current_path: PathBuf,
    pub flags: Vector<(String, Option<FlagValue>)>,
}

impl Context {
    pub fn build_new(
        raw_args: Vec<String>,
        args: VecDeque<String>,
        parsed_common: Vector<Flag>,
        current_path: PathBuf,
        flags: Vector<(String, Option<FlagValue>)>,
    ) -> Context {
        Context {
            raw_args,
            args,
            parsed_common,
            current_path,
            flags,
        }
    }

    pub fn root() -> Option<Context> {
        None
    }

    pub fn args(mut self, args: VecDeque<String>) -> Self {
        self.args = args;
        self
    }
}

impl From<Vec<String>> for Context {
    fn from(raw_args: Vec<String>) -> Context {
        let args = VecDeque::from(raw_args.clone());
        let current_path = match &raw_args.get(0) {
            Some(str) => PathBuf::from(str),
            None => PathBuf::new(),
        };
        Context {
            raw_args,
            args,
            parsed_common: Vector::default(),
            current_path,
            flags: Vector::default(),
        }
    }
}
