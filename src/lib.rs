mod action;
pub mod command;
mod context;
pub mod parser;
//mod experiment;
mod flag;
pub mod vector;

pub use action::Action;
pub use command::Command;
pub use context::Context;
pub use flag::{Flag, FlagType, FlagValue};
pub use parser::Parser;
pub use vector::Vector;

/*#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}*/
