use crate::{machine::Machine, object_pool::ObjectPool, value::Value};
use std::any::Any;

pub trait Object: Send
{
    fn typename(&self) -> String
    {
        String::from("Object")
    }

    fn initialize(&mut self, _: &mut ObjectPool)
    {
    }

    fn call(&self, _m: &mut Machine, _args: Vec<Value>, _c_index: u8) -> Value
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
