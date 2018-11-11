use crate::{frame::*, object_pool::ObjectPool, opcodes::*, value::Value};
use std::collections::HashMap;

///Machine that executes code
pub struct Machine
{
    pub stack: Vec<CallFrame>,
    pub pool: ObjectPool,
    pub globals: HashMap<usize, Value>,
    pub labels: HashMap<usize,usize>,
}

impl Machine
{
    pub fn new() -> Machine
    {
        Machine {
            stack: Vec::with_capacity(4096),
            pool: ObjectPool::new(),
            globals: HashMap::new(),
            labels: HashMap::new(),
        }
    }

    pub fn last_frame(&self) -> &CallFrame
    {
        self.stack.last().unwrap()
    }

    pub fn last_frame_mut(&mut self) -> &mut CallFrame
    {
        self.stack.last_mut().unwrap()
    }

    pub fn get(&mut self, rnum: usize) -> Value
    {
        self.last_frame().get(rnum)
    }

    pub fn set_this(&mut self, v: Value)
    {
        self.last_frame_mut().stack[0] = v;
    }

    pub fn set(&mut self, reg: usize, v: Value)
    {
        if reg == 0 {
            panic!("R(0) is a this value, cannot set!");
        }

        self.last_frame_mut().set(reg, v);
    }

    pub fn dispatch(&mut self)
    {
        self.last_frame_mut().ip += 1;
    }

    pub fn invoke(&mut self, callable: Value, args: Vec<Value>) -> Value
    {
        let id = match callable {
            Value::Object(id) => id,
            _ => {
                panic!("Not callable");
            }
        };

        let obj = self.pool.get(id);
        self.stack.push(CallFrame::new());
        let ret = {
            self.last_frame_mut().init_with_args(&args.as_slice());
            obj.call(self, args)
        };

        ret
    }

    pub fn branch(&mut self, idx: usize)
    {
        self.last_frame_mut().ip = idx;
    }

    pub fn run_code(&mut self, code: Vec<Instruction>) -> Value
    {
        for (idx,val) in code.iter().enumerate() {
            match val {
                Instruction::Label(id) => {
                    self.labels.insert(*id,idx);
                }
                _ => {}
            }
        }

        self.last_frame_mut().code = code;
        self.last_frame_mut().ip = 0;

        let value = self.execute_op();

        value
    }

    pub fn execute_op(&mut self) -> Value
    {
        let mut returns = false;
        let mut ret = Value::Null;

        while self.last_frame().ip < self.last_frame().code.len() {
            if returns {
                break;
            }

            let opcode = self.last_frame().code[self.last_frame().ip].clone();

            match &opcode {
                Instruction::Label(label_id) => {
                    self.labels.insert(*label_id,self.last_frame().ip);
                }

                Instruction::PushArg(reg) => {
                    let value = self.get(*reg);
                    self.last_frame_mut().arg_stack.push(value);
                }

                Instruction::LoadInt(dest, int) => {
                    self.set(*dest, Value::Int(*int));
                }

                Instruction::LoadDouble(dest, double) => {
                    self.set(*dest, Value::Double(*double));
                }

                Instruction::LoadLong(dest, long) => {
                    self.set(*dest, Value::Long(*long));
                }

                Instruction::LoadFloat(dest, float) => {
                    self.set(*dest, Value::Float(*float));
                }

                Instruction::Add(dest, r1, r2) => {
                    let (v1, v2) = (self.get(*r1), self.get(*r2));

                    let result = match (v1, v2) {
                        (Value::Int(i), Value::Int(i2)) => Value::Int(i + i2),
                        (Value::Float(f), Value::Float(f2)) => Value::Float(f + f2),
                        (Value::Long(i), Value::Long(i2)) => Value::Long(i + i2),
                        (Value::Double(f), Value::Double(f2)) => Value::Double(f + f2),
                        _ => unimplemented!(),
                    };

                    self.set(*dest, result);
                }

                Instruction::Call(dest, r2, argc) => {
                    let args = {
                        let mut temp = vec![];
                        for _ in 0..*argc {
                            temp.push(
                                self.last_frame_mut()
                                    .arg_stack
                                    .pop()
                                    .expect("Arg stack empty"),
                            );
                        }
                        temp
                    };

                    let value = self.get(*r2);

                    let v = self.invoke(value, args);
                    self.stack.pop();
                    self.set(*dest, v);
                }
                Instruction::Sub(dest, r1, r2) => {
                    let (v1, v2) = (self.get(*r1), self.get(*r2));

                    let result = match (v1, v2) {
                        (Value::Int(i), Value::Int(i2)) => Value::Int(i * i2),
                        (Value::Float(f), Value::Float(f2)) => Value::Float(f * f2),
                        (Value::Long(i), Value::Long(i2)) => Value::Long(i * i2),
                        (Value::Double(f), Value::Double(f2)) => Value::Double(f * f2),
                        _ => unimplemented!(),
                    };

                    self.set(*dest, result);
                }

                Instruction::Div(dest, r1, r2) => {
                    let (v1, v2) = (self.get(*r1), self.get(*r2));

                    let result = match (v1, v2) {
                        (Value::Int(i), Value::Int(i2)) => Value::Int(i / i2),
                        (Value::Float(f), Value::Float(f2)) => Value::Float(f / f2),
                        (Value::Long(i), Value::Long(i2)) => Value::Long(i / i2),
                        (Value::Double(f), Value::Double(f2)) => Value::Double(f / f2),
                        _ => unimplemented!(),
                    };

                    self.set(*dest, result);
                }

                Instruction::Mul(dest, r1, r2) => {
                    let (v1, v2) = (self.get(*r1), self.get(*r2));

                    let result = match (v1, v2) {
                        (Value::Int(i), Value::Int(i2)) => Value::Int(i * i2),
                        (Value::Float(f), Value::Float(f2)) => Value::Float(f * f2),
                        (Value::Long(i), Value::Long(i2)) => Value::Long(i * i2),
                        (Value::Double(f), Value::Double(f2)) => Value::Double(f * f2),
                        _ => unimplemented!(),
                    };

                    self.set(*dest, result);
                }

                Instruction::LoadObject(reg, idx) => {
                    self.set(*reg, Value::Object(*idx));
                }

                Instruction::Gt(dest, r1, r2) => {
                    let (v1, v2) = (self.get(*r1), self.get(*r2));
                    let result = match (v1, v2) {
                        (Value::Int(i), Value::Int(i2)) => Value::Bool(i > i2),
                        (Value::Long(i), Value::Long(i2)) => Value::Bool(i > i2),
                        (Value::Float(f), Value::Float(f2)) => Value::Bool(f > f2),
                        (Value::Double(f), Value::Double(f2)) => Value::Bool(f > f2),
                        _ => unimplemented!(),
                    };

                    self.set(*dest, result);
                }

                Instruction::Lt(dest, r1, r2) => {
                    let (v1, v2) = (self.get(*r1), self.get(*r2));
                    let result = match (v1, v2) {
                        (Value::Int(i), Value::Int(i2)) => Value::Bool(i < i2),
                        (Value::Long(i), Value::Long(i2)) => Value::Bool(i < i2),
                        (Value::Float(f), Value::Float(f2)) => Value::Bool(f < f2),
                        (Value::Double(f), Value::Double(f2)) => Value::Bool(f < f2),
                        _ => unimplemented!(),
                    };

                    self.set(*dest, result);
                }

                Instruction::Goto(lbl_id) => {
                    if self.labels.contains_key(lbl_id) {
                        let idx = self.labels.get(lbl_id).unwrap();
                        self.branch(*idx);
                    } else {
                        panic!("Label with id `{}` doesn't exists",lbl_id);
                    }
                }

                Instruction::GotoT(reg,lbl_id) => {
                    match self.get(*reg) {
                        Value::Bool(b) => {
                            if b {
                                if self.labels.contains_key(lbl_id) {
                                    let idx = self.labels.get(lbl_id).unwrap();
                                    self.branch(*idx);
                                } else {
                                    panic!("Label with id `{}`,doesn't exists",lbl_id)
                                }
                            }
                        }
                        _ => unimplemented!(),
                    }
                }
                Instruction::GotoF(reg,lbl_id) => {
                    match self.get(*reg) {
                        Value::Bool(b) => {
                            if !b {
                                if self.labels.contains_key(lbl_id) {
                                    let idx = self.labels.get(lbl_id).unwrap();
                                    self.branch(*idx);
                                } else {
                                    panic!("Label with id `{}`,doesn't exists",lbl_id)
                                }
                            }
                        }
                        _ => unimplemented!(),
                    }
                }

                Instruction::Jump(idx) => {
                    self.branch(*idx - 1);
                }
                Instruction::JumpT(r1, idx) => {
                    let v = self.get(*r1);
                    match v {
                        Value::Bool(b) => {
                            if b {
                                self.branch(*idx - 1);
                            }
                        }
                        _ => unimplemented!(),
                    }
                }

                Instruction::LoadGlobal(index, reg) => {
                    if self.globals.contains_key(index) {
                        let value = self.globals.get(index).unwrap();
                        self.set(*reg, *value);
                    } else {
                        panic!("No value in globals");
                    }
                }

                Instruction::StoreGlobal(index, reg) => {
                    let value = self.get(*reg);
                    self.globals.insert(*index, value);
                }

                Instruction::JumpF(r1, idx) => {
                    let v = self.get(*r1);
                    if let Value::Bool(b) = v {
                        if !b {
                            self.branch(*idx);
                        }
                    } else {
                        panic!("Expected bool");
                    }
                }

                Instruction::Move(r1, r2) => {
                    self.last_frame_mut().stack[*r1] = self.last_frame().stack[*r2];
                }

                Instruction::Ret(idx) => {
                    ret = self.get(*idx);
                    returns = true;
                }

                Instruction::Ret0 => {
                    returns = true;
                }

                _ => unimplemented!(),
            }
            self.dispatch();
        }
        ret
    }
}
