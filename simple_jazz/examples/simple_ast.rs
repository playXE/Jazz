extern crate simple_jazz;
use simple_jazz::builder::FunctionBuilder;
use simple_jazz::function::Function;
use simple_jazz::opcodes::Instruction;

#[derive(Debug,Clone)]
pub enum SimpleAst {
    Int(i32),
    Add(Box<SimpleAst>,Box<SimpleAst>),
    Ret(Box<SimpleAst>),
}

pub fn translate(node: SimpleAst,b: &mut FunctionBuilder) {
    match node {
        SimpleAst::Int(integer) => {
            b.emit_iconst(integer);
        }
        SimpleAst::Add(v1,v2) => {
            translate(*v1, b);
            translate(*v2,b);
            let r3 = b.register_pop();
            let r2 = b.register_pop();
            let r1 = b.register_push_temp();
            b.insert_op(Instruction::Add(r1,r2,r3));
        }
        SimpleAst::Ret(ret_a) => {
            translate(*ret_a, b);
            let r = b.register_pop();
            b.insert_op(Instruction::Ret(r));
        }
    }
}

pub fn compile(ast: Vec<SimpleAst>) -> Function {
    let mut builder = FunctionBuilder::new(0);
    builder.regs_used[0] = true; // 0 is a this value, just null for now
    for node in ast.iter() {
        translate(node.clone(), &mut builder);
    }

    builder.create_function()
}

use simple_jazz::machine;
use simple_jazz::value::Value;
use simple_jazz::opcodes::DebugCode;

fn main() {
    let ast = vec![
        SimpleAst::Ret(Box::new(SimpleAst::Add(Box::new(SimpleAst::Int(2)),Box::new(SimpleAst::Int(2)))))
    ];

    let func = compile(ast);
    if let Function::Virtual(vf) = &func {
        println!("Generated code: \n{}",vf.code.toString());
    }
    let mut machine = machine::Machine::new();
    let obj = machine.pool.allocate(Box::new(func));

    
    println!("{:?}",machine.invoke(Value::Object(obj),vec![]));
}
