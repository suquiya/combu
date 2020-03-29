use crate::Flag;

pub struct Context {
    pub args: Vec<String>,
    pub common_flag: Vec<Flag>,
}

impl Context {
    pub fn new(args: Vec<String>) -> Context {
        Context {
            args,
            common_flag: vec![],
        }
    }
}
