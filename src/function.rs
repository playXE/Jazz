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

    /// Get Object Id's(Used for GC) W.I.P
    fn get_children(&self) -> Vec<usize>
    {
        vec![]
    }

    /// Call object
    fn call(&self, m: &mut Machine, args: Vec<Value>) -> Value
    {
        match self {
            Function::Virtual(ref vf) => {
                let func = vf.clone();

                for i in 0..args.len() {
                    m.last_frame_mut().stack[i] = args[i];
                }

                m.run_code(func.code)
            }

            Function::Native(nv) => nv.invoke(m, args),
        }
    }
}

#[derive(Clone, Debug)]
pub struct VirtualFunction
{
    pub code: Vec<Instruction>,
    pub argc: usize,
}

impl Function
{
    pub fn from_instructions(code: Vec<Instruction>, args: usize) -> Function
    {
        Function::Virtual(VirtualFunction {
            code: code,
            argc: args,
        })
    }

    pub fn from_native(f: NativeFunction) -> Function
    {
        Function::Native(f)
    }
}

impl From<Vec<Instruction>> for Function
{
    fn from(f: Vec<Instruction>) -> Function
    {
        Function::Virtual(VirtualFunction { code: f, argc: 0 })
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
