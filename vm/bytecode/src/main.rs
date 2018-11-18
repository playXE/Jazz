extern crate jazz_bytecode;
extern crate jazz;
use jazz_bytecode::opcode::*;
use jazz_bytecode::parser::Parser;
use jazz_bytecode::encode;
use jazz_bytecode::assembler::Assembler;
use jazz::opcodes::Instruction;

fn main() {
    let mut assembler = Assembler::new(vec![
        Instruction::LoadInt(1,12),
        Instruction::Move(1,2),
    ]);

    assembler.translate();

    println!("{:?}",assembler.code);

    let mut parser = Parser::new(&assembler.code);
    let code = parser.parse();
    println!("{:?}",code);

}
