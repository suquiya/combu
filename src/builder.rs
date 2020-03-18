use crate::Action;
use crate::Flag;

pub struct Command {
    pub name: String,
    pub description: Option<String>,
    pub usage: String,
    pub action: Action,
    pub flags: Option<Vec<Flag>>,
}
