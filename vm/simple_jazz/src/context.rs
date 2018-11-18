use std::collections::HashMap;
use jazz::function::Function;
use jazz::opcodes::Instruction;
use jazz::value::Value;

#[derive(Debug,Clone)]
pub struct Context {
    pub builder: (),
}