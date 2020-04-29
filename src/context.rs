use crate::Vector;
use crate::{Flag, FlagValue};
use std::collections::VecDeque;
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub struct Context {
    pub raw_args: Vec<String>,
    pub args: VecDeque<String>,
    pub common_flags: Vector<Flag>,
    current_path: PathBuf,
    pub flags: Vector<(String, Option<FlagValue>)>,
}

impl Context {
    pub fn build_new(
        raw_args: Vec<String>,
        args: VecDeque<String>,
        common_flags: Vector<Flag>,
        current_path: PathBuf,
        flags: Vector<(String, Option<FlagValue>)>,
    ) -> Context {
        Context {
            raw_args,
            args,
            common_flags,
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

    pub fn current(&self) -> &Path {
        &self.current_path
    }

    pub fn change_current(&mut self, path: PathBuf) {
        self.current_path = path;
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
            common_flags: Vector::default(),
            current_path,
            flags: Vector::default(),
        }
    }
}
