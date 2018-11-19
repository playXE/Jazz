use jazz_vm::machine::Machine;
use jazz_vm::object::ObjectAddon;
use jazz_vm::value::Value;




use std::io::{stdout,stdin};
use std::io::Write;

pub fn print(m: &mut Machine, args: Vec<Value>) -> Value {
    let mut stdout = stdout();
    let mut args = args.clone();

    for i in 1..args.len() {
        let bytes = args[args.len() - i].as_bytes(m);

        stdout.write(&bytes).expect("Failed to unwrap");
    }
    print!("\n");
    Value::Null
}

pub fn readln(m: &mut Machine,_args: Vec<Value>) -> Value {
    let mut buffer = String::new();
    stdin().read_line(&mut buffer).unwrap();
    let obj = Value::Object(m.pool.allocate(Box::new(buffer)));
    obj
}