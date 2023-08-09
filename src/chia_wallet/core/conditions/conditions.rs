use crate::program_utils::program::Program;

pub trait Condition {
    fn program(&self) -> Program;
}
