extern crate time;

extern crate jazz;
use self::opcodes::Instruction;
use jazz::*;

use self::{function::Function, value::Value};

fn main()
{
    let mut machine = machine::Machine::new();
    let code = vec![
        Instruction::LoadInt(1, 0),
        Instruction::LoadInt(2, 1000000),
        Instruction::LoadInt(4, 1),
        Instruction::Label(0),
        Instruction::Gt(3, 2, 1),
        Instruction::GotoT(3, 1),
        Instruction::Ret(1),
        Instruction::Label(1),
        Instruction::Add(1, 4, 1),
        Instruction::Goto(0),
    ];
    let func = Function::from_instructions(code, 0);
    let obj = machine.pool.allocate(Box::new(func));

    let code2 = vec![
        Instruction::LoadInt(1,12),
        Instruction::LoadObject(2, obj),
        Instruction::Call(2, 2, 0),
        Instruction::Ret(2),
    ];

    let func2 = Function::from_instructions(code2, 0);
    let obj2 = machine.pool.allocate(Box::new(func2));
    let start = time::PreciseTime::now();

    let _value = machine.invoke(Value::Object(obj2), vec![]);
    let end = time::PreciseTime::now();
    println!("Result: {:?}", _value);
    let time = start.to(end).num_milliseconds();
    println!("time: {:?}", time);
}
