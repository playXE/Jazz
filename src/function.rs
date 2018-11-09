use crate::{
    frame::CallFrame, machine::Machine, object::Object, object_pool::ObjectPool, opcodes::*, value::Value
};
use std::any::Any;

#[derive(Debug)]
pub enum Function
{
    Virtual(VirtualFunction),
    Native(NativeFunction),
}

impl Object for Function
{
    fn as_any(&self) -> &Any
    {
        self as &Any
    }

    fn as_any_mut(&mut self) -> &mut Any
    {
        self as &mut Any
    }

    fn get_children(&self) -> Vec<usize>
    {
        vec![]
    }

    fn call(&self, m: &mut Machine, args: Vec<Value>) -> Value
    {
        match self {
            Function::Virtual(ref vf) => {
                let func = vf.clone();

                for i in 0..args.len() {
                    m.last_frame_mut().stack[i] = args[i];
                }

                m.run_blocks(func.code_blocks)
            }

            Function::Native(nv) => nv.invoke(m, args),
        }
    }
}

#[derive(Clone, Debug)]
pub struct VirtualFunction
{
    pub code_blocks: Vec<CodeBlock>,
    pub argc: usize,
}

impl Function
{
    pub fn from_code_blocks(blocks: Vec<CodeBlock>, args: usize) -> Function
    {
        Function::Virtual(VirtualFunction {
            code_blocks: blocks,
            argc: args,
        })
    }

    pub fn from_native(f: NativeFunction) -> Function
    {
        Function::Native(f)
    }
}

pub struct NativeFunction(Box<Fn(&mut Machine, Vec<Value>) -> Value + Send>);

impl NativeFunction
{
    pub fn invoke(&self, m: &mut Machine, args: Vec<Value>) -> Value
    {
        self.0(m, args)
    }
}

use std::fmt;

impl fmt::Debug for NativeFunction
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        write!(f, "<native function>")
    }
}
