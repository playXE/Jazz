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

    fn call(&self, m: &mut Machine, args: Vec<Value>) -> Value
    {
        Value::Null
    }

    fn store_at(&self, m: &mut Machine, args: Vec<Value>, rindex: usize)
    {
        panic!("Cannot store_at");
    }

    fn load_at(&self, m: &mut Machine, args: Vec<Value>, rindex: usize)
    {
        panic!("Cannot load_at");
    }

    fn as_any(&self) -> &Any;

    fn as_any_mut(&mut self) -> &mut Any;

    fn get_children(&self) -> Vec<usize>;
}
