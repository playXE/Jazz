extern crate time;

extern crate jazz;
use jazz::*;

use self::opcodes::{CodeBlock, Instruction};

use self::{function::Function, value::Value};

fn main()
{
    let code = vec![Instruction::LoadInt(1, 0), Instruction::Jump(1)];

    let if_true = vec![
        Instruction::LoadInt(2, 10000000),
        Instruction::Gt(3, 2, 1),
        Instruction::JumpF(3, 2),
        Instruction::LoadInt(4, 1),
        Instruction::Add(1, 4, 1),
        Instruction::Jump(1),
    ];

    let ret = vec![Instruction::Ret(1)];

    let block = CodeBlock::new(code);
    let block2 = CodeBlock::new(if_true);
    let block3 = CodeBlock::new(ret);
    let start = time::PreciseTime::now();
    let mut machine = machine::Machine::new();
    let func = Function::from_code_blocks(vec![block, block2, block3], 0);
    let obj = machine.pool.allocate(Box::new(func));
    machine.invoke(Value::Object(obj), vec![], 1);
    let end = time::PreciseTime::now();
    println!("Result: {:?}", machine.get(1));
    println!("time: {:?}", start.to(end).num_milliseconds());
}
