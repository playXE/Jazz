use crate::{machine::Machine, object_pool::ObjectPool, value::Value};
use std::any::Any;

pub trait Object: Send + ObjectAddon
{
    fn typename(&self) -> String
    {
        String::from("Object")
    }

    fn initialize(&mut self, _: &mut ObjectPool)
    {
    }

    fn call(&self, _m: &mut Machine, _args: Vec<Value>) -> Value
    {
        Value::Null
    }

    fn store_at(&self, _m: &mut Machine, _args: Vec<Value>, _rindex: usize)
    {
        panic!("Cannot store_at");
    }

    fn load_at(&self, _m: &mut Machine, _args: Vec<Value>, _rindex: usize)
    {
        panic!("Cannot load_at");
    }

    fn as_any(&self) -> &dyn Any;

    fn as_any_mut(&mut self) -> &mut dyn Any;

    fn get_children(&self) -> Vec<usize>;
}

pub trait ObjectAddon
{
    fn to_String(&self, _: &mut Machine) -> String
    {
        String::new()
    }

    fn as_bytes(&self,_: &mut Machine) -> Vec<u8> {
        Vec::new()
    }

    fn to_int(&self, _: &mut Machine) -> i32
    {
        0
    }

    fn to_long(&self, _: &mut Machine) -> i64
    {
        0
    }

    fn to_float(&self, _: &mut Machine) -> f32
    {
        0.0
    }
    fn to_double(&self, _: &mut Machine) -> f64
    {
        0.0
    }

    fn as_function(&self) -> &crate::function::Function {
        panic!()
    }
}
