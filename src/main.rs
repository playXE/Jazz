extern crate time;

extern crate jazz;
use jazz::*;

use self::{
    frame::CallFrame, opcodes::{CodeBlock, Instruction}
};

fn main()
{
    let code = vec![Instruction::LoadInt(0, 0), Instruction::Jump(1)];

    let if_true = vec![
        Instruction::LoadInt(1, 1000000),
        Instruction::Gt(2, 1, 0),
        Instruction::JumpF(2, 2),
        Instruction::LoadInt(4, 1),
        Instruction::Add(0, 4, 0),
        Instruction::Jump(1),
    ];

    let ret = vec![Instruction::Ret(0)];

    let block = CodeBlock::new(code);
    let block2 = CodeBlock::new(if_true);
    let block3 = CodeBlock::new(ret);
    //let mut frame = CallFrame::new(vec![block, block2, block3]);
    let start = time::PreciseTime::now();
    let mut machine = machine::Machine::new();
    println!("{:?}", block2);
    println!("{:?}", machine.run_blocks(vec![block, block2, block3]));
    let end = time::PreciseTime::now();
    println!("time: {:?}", start.to(end).num_milliseconds());
}
