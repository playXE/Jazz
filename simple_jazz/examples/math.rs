extern crate mex;
extern crate simple_jazz;

use mex::parser::ast::Node;

use simple_jazz::builder::FunctionBuilder;
use simple_jazz::opcodes::{Instruction,DebugCode};
use simple_jazz::machine::Machine;


pub fn translate(node: Node,builder: &mut FunctionBuilder) {
    match node {
        Node::Number(num) => {
            builder.dconst(num);
        }
        Node::Infix{left,right,op} => {
            translate(*right, builder);
            translate(*left, builder);

            let r1 = builder.register_pop();
            let r2 = builder.register_pop();
            let dest = builder.register_push_temp();

            let op: &str = &op;

            match op {
                "+" => {
                    builder.insert_op(Instruction::Add(dest,r1,r2));
                }
                "-" => {
                    builder.insert_op(Instruction::Sub(dest,r1,r2));
                }
                "*" => {
                    builder.insert_op(Instruction::Mul(dest,r1,r2));
                }
                "/" => {
                    builder.insert_op(Instruction::Div(dest,r1,r2));
                }
                _ => unimplemented!(),
            }
        } 
        _ => unimplemented!(),
    }
}
use simple_jazz::function::Function;

pub fn compile(node: Node) -> Function {
    let mut builder = FunctionBuilder::new(0);
    builder.regs_used[0] = true;
    translate(node, &mut builder);
    let ret = builder.register_pop();

    builder.insert_op(Instruction::Ret(ret));
    builder.create_function()

}

use simple_jazz::value::Value;
use std::io::stdin;

fn main() {
    let mut buffer = String::new();

    println!("Enter simple mathematic expression(e.g `1 + 2 - 3`)");
    stdin().read_line(&mut buffer).unwrap();

    let v = mex::parser::Parser::new(buffer).parse().unwrap();
    let v = compile(v);
    if let Function::Virtual(vf) = &v {
        println!("Generated code: \n{}",vf.code.toString());
    };

    let mut machine = Machine::new();
    let obj = Value::Object(machine.pool.allocate(Box::new(v)));

    println!("Return: {:?}",machine.invoke(obj,vec![]));


}