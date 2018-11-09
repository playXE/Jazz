use crate::{frame::*, object_pool::ObjectPool, opcodes::*, value::Value};
use std::collections::HashMap;

///Machine that executes code
pub struct Machine
{
    pub stack: CallStack,
    pub pool: ObjectPool,
    pub globals: HashMap<usize, Value>,
}

impl Machine
{
    pub fn new() -> Machine
    {
        Machine {
            stack: CallStack::new(4096),
            pool: ObjectPool::new(),
            globals: HashMap::new(),
        }
    }

    pub fn last_frame(&self) -> &CallFrame
    {
        self.stack.frames.last().unwrap()
    }

    pub fn last_frame_mut(&mut self) -> &mut CallFrame
    {
        self.stack.frames.last_mut().unwrap()
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

    pub fn invoke(&mut self, callable: Value, args: Vec<Value>, dest: usize)
    {
        let id = match callable {
            Value::Object(id) => id,
            _ => {
                panic!("Not callable");
            }
        };

        let obj = self.pool.get(id);

        let ret = {
            let frame = self.stack.top_mut();
            frame.init_with_args(&args.as_slice());

            obj.call(self, args)
        };

        self.stack.pop();
        self.set(dest, ret);
    }

    pub fn branch(&mut self, idx: usize)
    {
        self.last_frame_mut().bp = idx;
        self.last_frame_mut().blocks[idx].ip = 0;
    }

    pub fn fetch_opcode(&mut self) -> Instruction
    {
        self.last_frame_mut().fetch_opcode()
    }

    pub fn run_blocks(&mut self, blocks: Vec<CodeBlock>) -> Value
    {
        self.last_frame_mut().blocks = blocks;
        self.last_frame_mut().bp = 0;
        let mut old_bp = self.last_frame().bp;
        let value = loop {
            if old_bp != self.last_frame().bp {
                old_bp = self.last_frame().bp;
            }

            let ret = self.execute_op();

            match ret {
                Some(val) => break val,
                None => {}
            }
        };

        value
    }

    pub fn execute_op(&mut self) -> Option<Value>
    {
        let opcode = self.fetch_opcode();
        match &opcode {
            Instruction::PushArg(reg) => {
                let value = self.get(*reg);
                self.last_frame_mut().arg_stack.push(value);
                None
            }

            Instruction::LoadInt(dest, int) => {
                self.set(*dest, Value::Int(*int));
                None
            }

            Instruction::LoadFloat(dest, float) => {
                self.set(*dest, Value::Float(*float));
                None
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
                None
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

                self.invoke(value, args, *dest);
                None
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
                None
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
                None
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
                None
            }

            Instruction::LoadObject(reg, idx) => {
                self.set(*reg, Value::Object(*idx));
                None
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
                None
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
                None
            }

            Instruction::Jump(idx) => {
                self.branch(*idx);
                None
            }
            Instruction::JumpT(r1, idx) => {
                let v = self.get(*r1);
                match v {
                    Value::Bool(b) => {
                        if b {
                            self.branch(*idx);
                        }
                    }
                    _ => unimplemented!(),
                }

                None
            }

            Instruction::LoadGlobal(index, reg) => {
                if self.globals.contains_key(index) {
                    let value = self.globals.get(index).unwrap();
                    self.set(*reg, *value);
                    None
                } else {
                    panic!("No value in globals");
                }
            }

            Instruction::StoreGlobal(index, reg) => {
                let value = self.get(*reg);
                self.globals.insert(*index, value);
                None
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
                None
            }

            Instruction::Move(r1, r2) => {
                self.last_frame_mut().stack[*r1] = self.last_frame().stack[*r2];

                None
            }

            Instruction::Ret(idx) => {
                return Some(self.get(*idx));
            }

            Instruction::Ret0 => return Some(Value::Null),

            _ => unimplemented!(),
        }
    }
}
