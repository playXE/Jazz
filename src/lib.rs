//! Jazz Virtual Machine
//!
//! Jazz is a register-based virtual machine
//!
//! Jazz is still in active develop so it's not recommended to use Jazz for your purposes
//!
//!
//! Example code:
//!```rust
//! LoadInt(0,12) // Load 12 into R(0)
//! LoadInt(1,3)  // Load 13 into R(1)
//! Add(2,1,0)    // Add value from R(1) to R(0) and store result in R(2)
//! Ret(2)        // Return value from R(2)
//! ```
//!
//! Jazz is heavily inspired by [Gravity](https://marcobambini.github.io/gravity/#/) language VM
//!

#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![warn(rust_2018_idioms)]
#![feature(test)]

pub mod frame;
pub mod function;
pub mod jit;
pub mod machine;
pub mod object;
pub mod object_info;
pub mod object_pool;
pub mod opcodes;
pub mod static_root;
pub mod value;

use time;

use self::{function::Function, opcodes::Instruction, value::Value};

extern crate test;
use self::test::Bencher;

#[bench]
fn bench(b: &mut Bencher)
{
    b.iter(|| {
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
            Instruction::LoadInt(1, 12),
            Instruction::LoadObject(2, obj),
            Instruction::Call(2, 2, 0),
            Instruction::Ret(2),
        ];

        let func2 = Function::from_instructions(code2, 0);
        let obj2 = machine.pool.allocate(Box::new(func2));
        let start = time::PreciseTime::now();

        let _value = machine.invoke(Value::Object(obj2), vec![]);
    })
}
