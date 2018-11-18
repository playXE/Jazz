use crate::{machine::Machine, object::Object, opcodes::*, value::Value};
use std::any::Any;

#[derive(Debug)]
pub enum Function
{
    Virtual(VirtualFunction),
    Native(NativeFunction),
}

impl Clone for Function {
    fn clone(&self) -> Function {
        match self {
            Function::Virtual(vf) => {
                Function::Virtual(vf.clone())
            }
            Function::Native(_) => panic!("Cannot clone native func"),
        }
    }
}

impl crate::object::ObjectAddon for Function {
    fn as_function(&self) -> &Function {
        self
    }
}

impl Object for Function
{
    fn as_any(&self) -> &dyn Any
    {
        self as &dyn Any
    }

    fn as_any_mut(&mut self) -> &mut dyn Any
    {
        self as &mut dyn Any
    }

    /// Get Object Id's(Used for GC) W.I.P
    fn get_children(&self) -> Vec<usize>
    {
        vec![]
    }

    fn load_at(&self, m: &mut Machine, _args: Vec<Value>, dest: usize)
    {
        let _this = _args[0];
        let val = if let Value::Object(id) = &_args[1] {
            m.pool.get(*id)
        } else {
            panic!("Exptected object")
        };

        let fname: &str = &val.to_string(m);

        match fname {
            "disassemble" => {
                let code = if let Function::Virtual(vf) = self {
                    vf.code.toString()
                } else {
                    "<native function>".to_string()
                };
                let obj = m.pool.allocate(Box::new(code));
                let code = vec![Instruction::LoadObject(1, obj), Instruction::Ret(1)];
                let func = Function::from(code);
                let obj = m.pool.allocate(Box::new(func));
                m.set(dest, Value::Object(obj));
            }
            f => panic!("Unknown field `{}`", f),
        }
    }

    /// Call object
    fn call(&self, m: &mut Machine, args: Vec<Value>, c_index: u8) -> Value
    {
        if c_index == 0 {
            let ret = match self {
                Function::Virtual(ref vf) => {
                    let func = vf.clone();
                    for i in 0..args.len() {
                        m.last_frame_mut().stack[i] = args[i];
                    }
                    let code = func.code;
                    m.run_code(code.clone())
                }

                Function::Native(nv) => nv.invoke(m, args),
            };
            return ret;
        } else {
            panic!("Function expect CALL idnex,found `{}`", c_index);
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

    pub fn from_native(f: Box<dyn Fn(&mut Machine, Vec<Value>) -> Value + Send>) -> Function
    {
        Function::Native(NativeFunction(f))
    }
}

impl From<Vec<Instruction>> for Function
{
    fn from(f: Vec<Instruction>) -> Function
    {
        Function::Virtual(VirtualFunction { code: f, argc: 0 })
    }
}

pub struct NativeFunction(Box<dyn Fn(&mut Machine, Vec<Value>) -> Value + Send>);

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
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        write!(f, "<native function>")
    }
}
