#![feature(duration_as_u128)]
pub mod ircode;
pub mod parser;

extern crate jazz_vm;
extern crate time;
extern crate float_duration;
use float_duration::{TimePoint};
use std::time::{Instant};

use std::collections::HashMap;

use self::ircode::FunctionBuilder;
use self::parser::{Expr, FnDef, Op, Stmt};
use jazz_vm::function::Function;
use jazz_vm::machine::Machine;
use jazz_vm::opcodes::DebugCode;
use jazz_vm::opcodes::Instruction;
use jazz_vm::value::Value;


pub fn print(m: &mut Machine,args: Vec<Value>) -> Value {
    
    for i in 1..args.len() {
        match args[i] {
            Value::Int(i) => print!("{}",i),
            Value::Long(l) => print!("{}",l),
            Value::Float(f) => print!("{}",f),
            Value::Double(d) => print!("{}",d),
            Value::Bool(b) => print!("{}",b),
            Value::Null => print!("null"),
            Value::Object(id) => {
                let stri = m.pool.get(id).to_string(m);
                println!("{}",stri);
            }
        }
    }
    println!("");
    Value::Null
}

pub struct Compiler<'a> {
    pub machine: &'a mut Machine,
    pub builder: FunctionBuilder,
    pub gp: usize,
    pub globals: HashMap<String, usize>,
    pub debug: bool,
}

impl<'a> Compiler<'a> {
    pub fn new(m: &'a mut Machine, argc: usize, debug: bool) -> Compiler<'a> {
        let mut gp = 0;
        gp += 1;
        let id = m.pool.allocate(Box::new(Function::from_native(Box::new(print))));
        let mut globals = HashMap::new();
        globals.insert("print".to_string(), gp);
        m.globals.insert(gp,Value::Object(id));

        Compiler {
            machine: m,
            builder: FunctionBuilder::new(argc),
            globals: globals,
            gp,
            debug,
        }
    }
    pub fn translate_stmt(&mut self, s: Stmt) {
        match s {
            Stmt::If(condition, then) => {
                let then = *then;
                self.translate_expr(*condition);
                let cond_reg = self.builder.register_pop();
                let end_label = self.builder.new_label();

                self.builder
                    .push_op(Instruction::GotoF(cond_reg, end_label));
                self.translate_stmt(then);
                self.builder.label_here(end_label);
            }

            Stmt::IfElse(condition, if_true, if_false) => {
                let if_true = *if_true;
                let if_false = *if_false;

                self.translate_expr(*condition);
                let cond_reg = self.builder.register_pop();
                let if_false_label = self.builder.new_label();

                self.builder
                    .push_op(Instruction::GotoF(cond_reg, if_false_label));
                self.translate_stmt(if_true);
                self.builder.label_here(if_false_label);
                self.translate_stmt(if_false);
            }

            Stmt::While(condition, block) => {
                let block = *block;

                let check_label = self.builder.new_label();
                let end_label = self.builder.new_label();

                self.builder.label_here(check_label);
                self.translate_expr(*condition);
                let reg = self.builder.register_pop();
                self.builder.push_op(Instruction::GotoF(reg, end_label));
                self.translate_stmt(block);
                self.builder.push_op(Instruction::Goto(check_label));
                self.builder.label_here(end_label);
            }

            Stmt::Var(ref name, ref expr) => {
                let name = name.to_string();
                let expr = expr.clone();

                if expr.is_some() {
                    self.translate_expr(*expr.unwrap().clone());
                    let r = self.builder.register_pop();
                    self.builder.new_local(name, r);
                }
            }
            Stmt::Return => {
                self.builder.push_op(Instruction::Ret0);
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
            v => panic!("{:?}", v),
        }
    }

    pub fn compile(&mut self, f: Vec<FnDef>) -> Value {
        for fun in f.iter() {
            self.gp += 1;
            let name = if let Expr::Identifier(ref n) = &*fun.name.clone() {
                n.to_string()
            } else {
                "main".to_string()
            };
            self.machine
                .globals
                .insert(self.gp, Value::Int(0));
        
            self.globals.insert(name.clone(), self.gp);
            let builder = FunctionBuilder::new(fun.params.len());
            self.builder = builder;
            for param in &fun.params {
                let reg = self.builder.register_first_temp_available();
                
                self.builder.new_local(param.to_string(), reg);
            }
            self.translate_stmt(*fun.clone().body);

            let code = self.builder.get_insts();

            if self.debug {
                println!("function `{}` code: ", name);
                println!("{}", code.toString());
            }

            let function = Function::from_instructions(code, fun.params.len());
            let func = self.machine.pool.allocate(Box::new(function));
            self.machine.globals.insert(self.gp, Value::Object(func));
            self.globals.insert(name,self.gp);
        }

        let main = self.globals.get("main").unwrap().clone();
        let main = self.machine.globals.get(&main).unwrap();
        let start = Instant::now();
        let ret = self.machine.invoke(*main, vec![]);
        let end = Instant::now();
        println!(
            "RESULT: {:?} (in {})",
            ret,
            end.float_duration_since(start).unwrap()
        );
        ret
    }

    pub fn translate_expr(&mut self, expr: Expr) {
        match expr {
            Expr::IntConst(int) => {
                self.builder.int_const(int as i32);
            }
            Expr::FloatConst(float) => {
                self.builder.float_const(float as f32);
            }

            Expr::FnCall(ref fname, ref args) => {
                let args = args.clone();
            
                for arg in args.iter() {
                    self.translate_expr(arg.clone());
                    let r = self.builder.register_pop();
                    self.builder.push_op(Instruction::PushArg(r));
                }

                self.builder.int_const(0); // emit this value for function
                let r = self.builder.register_pop();
                self.builder.push_op(Instruction::PushArg(r));
                let dest = self.builder.register_push_temp();
                let fptr = if !self.globals.contains_key(fname) {
                    let r = self.builder.get_local(fname);
                    r
                } else {
                    let idx = self.globals.get(fname).unwrap();
                    let register = self.builder.register_first_temp_available();
                    self.builder
                        .push_op(Instruction::LoadObject(register, *idx));
                    register
                };
                self.builder
                    .push_op(Instruction::Call(dest, fptr, args.len()));
            }

            Expr::Op(op, e1, e2) => {
                let e1 = *e1;
                let e2 = *e2;

                self.translate_expr(e1);
                self.translate_expr(e2);
                let r2 = self.builder.register_pop();
                let r1 = self.builder.register_pop();
                let r3 = self.builder.register_push_temp();
                match op {
                    Op::Add => self.builder.push_op(Instruction::Add(r3, r1, r2)),
                    Op::Sub => self.builder.push_op(Instruction::Sub(r3, r1, r2)),
                    Op::Mul => self.builder.push_op(Instruction::Mul(r3, r1, r2)),
                    Op::Div => self.builder.push_op(Instruction::Div(r3, r1, r2)),
                    Op::Eq => self.builder.push_op(Instruction::Eq(r3, r1, r2)),
                    Op::Gt => self.builder.push_op(Instruction::Gt(r3, r1, r2)),
                    Op::Lt => self.builder.push_op(Instruction::Lt(r3, r1, r2)),
                    Op::Le => self.builder.push_op(Instruction::Le(r3, r1, r2)),
                    Op::Ge => self.builder.push_op(Instruction::Ge(r3, r1, r2)),
                    Op::And => self.builder.push_op(Instruction::And(r3,r1,r2)),
                    Op::Or => self.builder.push_op(Instruction::Or(r3,r1,r2)),
                    Op::BitOr => self.builder.push_op(Instruction::BitOr(r3,r1,r2)),
                    Op::BitAnd =>  self.builder.push_op(Instruction::BitAnd(r3,r1,r2)),
                    Op::BitXor =>  self.builder.push_op(Instruction::BitXor(r3,r1,r2)),
                    Op::Shl => self.builder.push_op(Instruction::Shl(r3,r1,r2)),
                    Op::Shr => self.builder.push_op(Instruction::Shr(r3,r1,r2)),
                    _ => unimplemented!(),
                }
            }

            Expr::StringConst(ref s) => {
                let obj = self.machine.pool.allocate(Box::new(s.to_string()));
                let r = self.builder.register_push_temp();
                self.builder.push_op(Instruction::LoadObject(r, obj));
            }

            Expr::Identifier(ref name) => {
                if !self.globals.contains_key(name) {
                    let r = self.builder.get_local(name);
                    let r2 = self.builder.register_push_temp();
                    self.builder.push_op(Instruction::Move(r2, r));
                } else {
                    let idx = self.globals.get(name).unwrap();
                    let register = self.builder.register_push_temp();
                    self.builder
                        .push_op(Instruction::LoadObject(register, *idx));
                }
            }

            Expr::Assignment(e1, e2) => {
                let e1 = *e1;
                let e2 = *e2;
                self.translate_expr(e2);
                if let Expr::Identifier(ref name) = e1 {
                    let r1 = self.builder.get_local(name);
                    let r2 = self.builder.register_pop();
                    self.builder.push_op(Instruction::Move(r1, r2));
                }
            }
            Expr::False => {
                let reg = self.builder.register_push_temp();
                self.builder.push_op(Instruction::LoadBool(reg,false));
            }
            Expr::True => {
                let reg = self.builder.register_push_temp();
                self.builder.push_op(Instruction::LoadBool(reg,true));
            }
            _ => unimplemented!(),
        }
    }
}
