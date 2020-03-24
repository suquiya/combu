use crate::Context;

//Action Type for Command
pub type Action = fn(&Context);

pub trait Actionable {
    fn action(self, action: Action) -> Self;
}
