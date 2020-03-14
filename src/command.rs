use crate::action::Action;
use crate::Flag;

pub trait Command {
    fn name<T: Into<String>>(&mut self, name: T) -> Self;
    fn is_root(self) -> bool;
    fn get_depth(self) -> i8;
    fn set_depth(&mut self, depth: i8) -> Self;
    fn action(&mut self, action: Action) -> Self;
    fn flag(&mut self, flag: Flag) -> Self;
}
