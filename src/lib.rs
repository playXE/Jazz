#![feature(duration_as_u128)]
#![warn(rust_2018_idioms)]

pub mod ircode;
pub mod parser;
pub mod builtins;
use self::builtins::*;

use float_duration::TimePoint;
use std::time::Instant;

use std::collections::HashMap;

use self::ircode::FunctionBuilder;
use self::parser::{Expr, Global, Op, Stmt};
use jazz_vm::function::Function;
use jazz_vm::machine::Machine;
use jazz_vm::opcodes::DebugCode;
use jazz_vm::opcodes::Instruction;
use jazz_vm::value::Value;
use jazz_vm::object::ObjectAddon;

pub struct Compiler<'a> {
    pub machine: &'a mut Machine,
    pub builder: FunctionBuilder,
    pub gp: usize,
    pub globals: HashMap<String, usize>,
    pub debug: bool,
}

impl<'a> Compiler<'a> {
    pub fn new(m: &'a mut Machine, argc: usize, debug: bool) -> Compiler<'a> {
        let mut compiler = Compiler {
            machine: m,
            builder: FunctionBuilder::new(argc),
            globals: HashMap::new(),
            gp: 0,
            debug,
        };
        compiler.register_funs();
        compiler
    }

    pub fn register_funs(&mut self) {
        self.gp += 1;

        let id = self
            .machine
            .pool
            .allocate(Box::new(Function::from_native(Box::new(print))));
        self.globals.insert("print".to_string(), self.gp);
        self.machine.globals.insert(self.gp, Value::Object(id));
        self.gp += 1;
        let id = self.machine.pool.allocate(Box::new(Function::from_native(Box::new(readln))));
        self.globals.insert("readln".to_string(),self.gp);
        self.machine.globals.insert(self.gp,Value::Object(id));

        self.gp += 1;
        let id = self.machine.pool.allocate(Box::new(Function::from_native(Box::new(new_array))));
        self.globals.insert("__new_array__".to_string(),self.gp);
        self.machine.globals.insert(self.gp,Value::Object(id));

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


            Stmt::For(value,condition,expr,block) => {
                let check_label = self.builder.new_label();
                let end_label = self.builder.new_label();

                self.translate_stmt(*value);

                self.builder.label_here(check_label);
                self.translate_expr(*condition);
                let reg = self.builder.register_pop();
                self.builder.push_op(Instruction::GotoF(reg,end_label));
                self.translate_stmt(*block);
                self.translate_expr(*expr);
                self.builder.push_op(Instruction::Goto(check_label));
                self.builder.label_here(end_label);
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

    pub fn compile(&mut self, globals: Vec<Global>) -> Value {
        for global in globals.iter() {
            self.gp += 1;

            if let Global::FnDefenition(ref fun) = &global {
                let name = if let Expr::Identifier(ref n) = &*fun.name {
                    n.to_string()
                } else {
                    "<unknown>".to_string()
                };
                self.machine.globals.insert(self.gp, Value::Int(0));

                self.globals.insert(name.clone(), self.gp);
            }


            if let Global::FnDefenition(ref fun) = global {
                let name = if let Expr::Identifier(ref n) = &*fun.name.clone() {
                    n.to_string()
                } else {
                    "main".to_string()
                };

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
                self.globals.insert(name, self.gp);
            }
        }

        let main = self.globals.get("main").expect("main not found").clone();
        let main = self.machine.globals.get(&main).expect("main not found");
        let start = Instant::now();
        let ret = self.machine.invoke(*main, vec![]);
        let end = Instant::now();
        println!(
            "RESULT: {} (in {})",
            ret.to_String(self.machine),
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


                let dest = self.builder.register_push_temp();
                let fptr = if !self.globals.contains_key(fname) {
                    let r = self.builder.get_local(fname);
                    r
                } else {
                    let idx = self.globals.get(fname).expect(&format!("Function not found `{}`",fname));
                    let register = self.builder.register_first_temp_available();
                    self.builder.push_op(Instruction::LoadGlobal(*idx,register));
                    register
                };
                self.builder.push_op(Instruction::PushArg(fptr));
                self.builder
                    .push_op(Instruction::Call(dest, fptr, args.len()));
            }

            Expr::Array(arr_expr) => {
                for expr in arr_expr.iter() {
                    self.translate_expr(expr.clone());
                    let r = self.builder.register_pop();
                    self.builder.push_op(Instruction::PushArg(r));
                }



                let id = self.globals.get("__new_array__").unwrap();
                let reg = self.builder.register_push_temp();
                self.builder.push_op(Instruction::LoadGlobal(*id,reg));
                self.builder.push_op(Instruction::PushArg(reg));
                let dest = self.builder.register_push_temp();
                self.builder.push_op(Instruction::Call(dest,reg,arr_expr.len()));


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
                    Op::And => self.builder.push_op(Instruction::And(r3, r1, r2)),
                    Op::Or => self.builder.push_op(Instruction::Or(r3, r1, r2)),
                    Op::BitOr => self.builder.push_op(Instruction::BitOr(r3, r1, r2)),
                    Op::BitAnd => self.builder.push_op(Instruction::BitAnd(r3, r1, r2)),
                    Op::BitXor => self.builder.push_op(Instruction::BitXor(r3, r1, r2)),
                    Op::Shl => self.builder.push_op(Instruction::Shl(r3, r1, r2)),
                    Op::Shr => self.builder.push_op(Instruction::Shr(r3, r1, r2)),
                    _ => unimplemented!(),
                }
            }

            Expr::StringConst(ref s) => {

                let r = self.builder.register_push_temp();
                self.builder.push_op(Instruction::LoadString(r,s.to_string()));
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
                    if self.globals.contains_key(name) {
                        let id = self.globals.get(name).unwrap();
                        let r = self.builder.register_pop();
                        self.builder.push_op(Instruction::StoreGlobal(*id,r));
                    } else {
                        let r1 = self.builder.get_local(name);
                        let r2 = self.builder.register_pop();
                        self.builder.push_op(Instruction::Move(r1, r2));
                    }
                }
            }
            Expr::False => {
                let reg = self.builder.register_push_temp();
                self.builder.push_op(Instruction::LoadBool(reg, false));
            }
            Expr::True => {
                let reg = self.builder.register_push_temp();
                self.builder.push_op(Instruction::LoadBool(reg, true));
            }
            _ => unimplemented!(),
        }
    }
}
