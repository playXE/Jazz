extern crate time;

extern crate jazz;
use self::opcodes::Instruction;
use jazz::*;

use self::{function::Function, value::Value};

fn main()
{
    let mut machine = machine::Machine::new();
    let code = vec![
        /*Instruction::LoadInt(1, 0),
        Instruction::LoadInt(2, 1000000),
        Instruction::LoadInt(4, 1),
        Instruction::Label(0),
        Instruction::Gt(3, 2, 1),
        Instruction::GotoT(3, 1),
        Instruction::Ret(1),
        Instruction::Label(1),
        Instruction::Add(1, 4, 1),
        Instruction::Goto(0),*/
        Instruction::Add(2, 1, 2),
        Instruction::Ret(2),
    ];

    let func = Function::from_instructions(code, 0);
    let obj = machine.pool.allocate(Box::new(func));

    let code2 = vec![
        Instruction::LoadObject(2, obj), // load function to R(2)
        Instruction::LoadInt(1, 3),
        Instruction::PushArg(1),
        Instruction::LoadInt(1, 12),
        Instruction::PushArg(1),
        Instruction::Move(3, 2), // Load function to R(3) and push to arguments stack as `this` value
        Instruction::PushArg(3),
        Instruction::Call(2, 2, 2), // Call Function from R(2) with argc 2 and set R(2) to return value
        Instruction::Ret(2),
    ];

    let func2 = Function::from_instructions(code2, 0);
    let obj2 = machine.pool.allocate(Box::new(func2));
    let _value = machine.invoke(Value::Object(obj2), vec![]);
    println!("{:?}", _value);
}
