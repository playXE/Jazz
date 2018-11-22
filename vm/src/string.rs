use crate::{machine::Machine, object::*, object_pool::ObjectPool};

impl ObjectAddon for String
{
    fn to_String(&self, _: &mut Machine) -> String
    {
        self.clone()
    }

    fn to_double(&self, _: &mut Machine) -> f64
    {
        let f = self.parse::<f64>().unwrap();
        f
    }

    fn to_float(&self, _: &mut Machine) -> f32
    {
        let f = self.parse::<f32>().unwrap();
        f
    }
    fn to_int(&self, _: &mut Machine) -> i32
    {
        let f = self.parse::<i32>().unwrap();
        f
    }
    fn to_long(&self, _: &mut Machine) -> i64
    {
        let f = self.parse::<i64>().unwrap();
        f
    }
}

use std::any::Any;

impl Object for String
{
    fn typename(&self) -> String
    {
        String::from("String")
    }

    fn initialize(&mut self, _: &mut ObjectPool)
    {
    }
    fn get_children(&self) -> Vec<usize>
    {
        vec![]
    }
    fn as_any(&self) -> &dyn Any
    {
        self as &dyn Any
    }
    fn as_any_mut(&mut self) -> &mut dyn Any
    {
        self as &mut dyn Any
    }
}
