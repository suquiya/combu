mod action;
mod app;
pub mod command;
mod context;
//mod experiment;
mod flag;

pub use action::Action;
pub use action::Actionable;
pub use command::Command;
pub use context::Context;
pub use flag::Flag;
/*#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}*/
