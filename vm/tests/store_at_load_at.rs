extern crate time;

extern crate jazz_vm;

use jazz_vm::*;

use std::collections::HashMap;

use std::cell::RefCell;

#[derive(Debug, Clone)]
pub struct TestObject
{
    pub flds: RefCell<HashMap<usize, Value>>,
    pub inited: bool,
}

impl TestObject
{
    pub fn new() -> TestObject
    {
        TestObject {
            flds: RefCell::new(HashMap::new()),
            inited: false,
        }
    }
}

use self::{
    machine::Machine, object::{Object, ObjectAddon}, object_pool::ObjectPool, value::Value
};
use std::any::Any;

impl ObjectAddon for TestObject {}

impl Object for TestObject
{
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

    fn initialize(&mut self, _: &mut ObjectPool)
    {
    }

    fn load_at(&self, m: &mut Machine, args: Vec<Value>, dest: usize)
    {
        let _this = args[0];
        let idx = args[1];

        if let Value::Int(i) = idx {
            let flds = self.flds.borrow();
            let f = flds.get(&(i as usize)).expect("Field doesn't exists");

            m.set(dest, *f);
        } else if let Value::Long(i) = idx {
            let flds = self.flds.borrow();
            let f = flds.get(&(i as usize)).expect("Field doesn't exists");

            m.set(dest, *f);
        } else {
            panic!("Expected Int or Long value as index");
        }
    }

    fn store_at(&self, _m: &mut Machine, args: Vec<Value>, _rindex: usize)
    {
        let _this = args[0];
        let idx = args[1];
        let val = args[2];
        let mut flds = self.flds.borrow_mut();
        if let Value::Int(i) = idx {
            flds.insert(i as usize, val);
        } else if let Value::Long(i) = idx {
            flds.insert(i as usize, val);
        } else {
            panic!("Expected Int or Long value as index");
        }
    }
}

use self::opcodes::Instruction::*;
#[test]
fn load_at_and_store_at()
{
    let mut m = Machine::new();

    let obj = m.pool.allocate(Box::new(TestObject::new()));

    let code = vec![
        LoadObject(1, obj),
        LoadInt(2, 1),
        LoadFloat(3, 2.6),
        StoreAt(3, 1, 2),
        PushArg(1),
        LoadAt(4, 1, 2),
        Ret(4),
    ];

    let func = self::function::Function::from(code);
    let func = m.pool.allocate(Box::new(func));

    let value = m.invoke(Value::Object(func), vec![]);

    if let Value::Float(f) = value {
        assert_eq!(f, 2.6);
    } else {
        panic!("");
    }
}
