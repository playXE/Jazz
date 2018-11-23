use float_duration;
use crate::{builtins::*, class::Class};
use jazz_vm::{function::Function, machine::Machine, value::Value};
use std::{
    cell::{RefCell, UnsafeCell}, collections::HashMap
};
use self::float_duration::FloatDuration;
use std::time::Instant;

pub fn time(m: &mut Machine,_: Vec<Value>) -> Value {
    let now = Instant::now();
    let duration = FloatDuration::from_std(now.elapsed());
    
    let str = format!("{}",duration);
    let obj = Value::Object(m.pool.allocate(Box::new(str)));
    return obj;
}

pub fn system_class(m: &mut Machine) -> Class
{
    let mut fields = HashMap::new();
    let f = Function::from_native(Box::new(print));
    fields.insert(
        "print".to_owned(),
        Value::Object(m.pool.allocate(Box::new(f))),
    );
    fields.insert(
        "readln".to_owned(),
        Value::Object(m.pool.allocate(Box::new(Function::from_native(Box::new(readln)))))
    ); 
    fields.insert(
        "time".to_owned(),
        Value::Object(m.pool.allocate(Box::new(Function::from_native(Box::new(time)))))
    );
    Class {
        name: String::from("System"),
        fields: UnsafeCell::new(fields),
    }
}
