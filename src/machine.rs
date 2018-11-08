use crate::{frame::*, opcodes::*, value::Value};

///Machine that executes code
pub struct Machine
{
    stack: CallStack,
}

impl Machine
{
    pub fn new() -> Machine
    {
        Machine {
            stack: CallStack::new(4096),
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

    pub fn set(&mut self, reg: usize, v: Value)
    {
        self.last_frame_mut().set(reg, v);
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
                    _ => unimplemented!(),
                };

                self.set(*dest, result);
                None
            }

            Instruction::Gt(dest, r1, r2) => {
                let (v1, v2) = (self.get(*r1), self.get(*r2));
                let result = match (v1, v2) {
                    (Value::Int(i), Value::Int(i2)) => Value::Bool(i > i2),
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
