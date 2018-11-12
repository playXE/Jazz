extern crate time;

extern crate jazz;
use self::opcodes::Instruction;
use jazz::*;

use self::{function::Function, value::Value};

fn main()
{
    let mut machine = machine::Machine::new();

    let factorial_code = vec![
        Instruction::Label(0),
    ];
}


