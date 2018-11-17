pub mod parser;
pub mod ircode;


extern crate jazz;
use self::parser::{lex,parse};
use self::ircode::FunctionBuilder;
use jazz::machine::Machine;
use self::parser::{Expr,FnDef,Stmt,Op};
use jazz::opcodes::Instruction;
use std::collections::HashMap;

use jazz::function::Function;
use jazz::value::Value;
use jazz::opcodes::DebugCode;

pub struct Compiler<'a> {
    pub machine: &'a mut Machine,
    pub builder: FunctionBuilder,
    pub compiled: HashMap<String,Value>
}

impl<'a> Compiler<'a> {
    pub fn new(m: &'a mut Machine,argc: usize) -> Compiler<'a> {
        Compiler {
            machine: m,
            builder: FunctionBuilder::new(argc),
            compiled: HashMap::new(),
        }
    }
    pub fn translate_stmt(&mut self,s: Stmt) {
        match s {
            Stmt::Var(ref name, ref expr) => {
                let name = name.to_string();
                let expr = expr.clone();

                if expr.is_some() {
                    self.translate_expr(*expr.unwrap().clone());
                    let r = self.builder.register_pop();
                    self.builder.new_local(name,r);
                }
            }
            Stmt::ReturnWithVal(val) => {
                self.translate_expr(*val);
                let r = self.builder.register_pop();
                self.builder.push_op(Instruction::Ret(r));
            }

            Stmt::Block(body) => {
                for stmt in body.iter() {
                    self.translate_stmt(stmt.clone());
                }
            }
            Stmt::Expr(expr) => {
                self.translate_expr(*expr.clone());
            }
            v => panic!("{:?}",v)
        }
    }

    pub fn compile(&mut self,f: Vec<FnDef>) {
        for fun in f.iter() {
            let name = if let Expr::Identifier(ref n) = &*fun.name.clone() {
                n.to_string()
            } else {
                "main".to_string()
            };
            let builder = FunctionBuilder::new(fun.params.len());

            self.builder = builder;

            self.translate_stmt(*fun.clone().body);
            
            let code = self.builder.get_insts();


            if &name == "main" {
                println!("Main code: \n{}",code.toString());
            }
            let fun = Function::from_instructions(code, fun.params.len());
            let fun = Value::Object(self.machine.pool.allocate(Box::new(fun)));
            
            self.compiled.insert(name,fun);
        }

        let main = self.compiled.get("main").unwrap().clone();
        let ret = self.machine.invoke(main,vec![]);
        println!("{:?}",ret);

    }


    pub fn translate_expr(&mut self,expr: Expr) {
        match expr {
            Expr::IntConst(int) => {
                self.builder.int_const(int as i32);
            }
            Expr::FloatConst(float) => {
                self.builder.float_const(float as f32);
            }
            Expr::Op(op,e1,e2) => {
                let e1 = *e1;
                let e2 = *e2;

                self.translate_expr(e1);
                self.translate_expr(e2);
                let r2 = self.builder.register_pop();
                let r1 = self.builder.register_pop();
                let r3 = self.builder.register_push_temp();
                match op {
                    Op::Add => {
                        self.builder.push_op(Instruction::Add(r3,r1,r2))
                    }
                    Op::Sub => {
                        self.builder.push_op(Instruction::Sub(r3,r1,r2))
                    }
                    Op::Mul => {
                        self.builder.push_op(Instruction::Mul(r3,r1,r2))
                    }
                    Op::Div => {
                        self.builder.push_op(Instruction::Div(r3,r1,r2))
                    }
                    Op::Eq => {
                        self.builder.push_op(Instruction::Eq(r3,r1,r2))
                    }
                    Op::Gt => {
                        self.builder.push_op(Instruction::Gt(r3,r1,r2))
                    }
                    Op::Lt => {
                        self.builder.push_op(Instruction::Lt(r3,r1,r2))
                    }
                    Op::Le => {
                        self.builder.push_op(Instruction::Le(r3,r1,r2))
                    }
                    Op::Ge => {
                        self.builder.push_op(Instruction::Ge(r3,r1,r2))
                    }
                    _ => unimplemented!(),
                }
            }


            Expr::Identifier(ref name) => {
                let r = self.builder.get_local(name);
                let r2 = self.builder.register_push_temp();
                self.builder.push_op(Instruction::Move(r2,r));
            }

            Expr::Assignment(e1,e2) => {
                let e1 = *e1;
                let e2 = *e2;
                self.translate_expr(e2);
                if let Expr::Identifier(ref name) = e1 {
                    let r1 = self.builder.get_local(name);
                    let r2 = self.builder.register_pop();
                    self.builder.push_op(Instruction::Move(r1,r2));
                } 
                

            }
            _ => unimplemented!(),
        }
    }
}