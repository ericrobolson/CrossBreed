use super::command::*;

pub trait Controller {}

pub struct CbController {
    command_stack: MacroCommand,
}
