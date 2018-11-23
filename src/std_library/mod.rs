use crate::{builtins::*, class::Class};
use jazz_vm::{function::Function, machine::Machine, value::Value};
use std::{
    cell::{RefCell, UnsafeCell}, collections::HashMap
};

pub fn system_class(m: &mut Machine) -> Class
{
    let mut fields = HashMap::new();
    let f = Function::from_native(Box::new(print));
    fields.insert(
        "print".to_owned(),
        Value::Object(m.pool.allocate(Box::new(f))),
    );
    Class {
        name: String::from("System"),
        fields: UnsafeCell::new(fields),
        is_inited: RefCell::new(true),
    }
}
