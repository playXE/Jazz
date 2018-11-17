extern crate jazzc;
extern crate jazz;
use jazzc::{parser::{lex,parse},Compiler};
use jazz::machine::Machine;

fn main() {
    let lex = lex("
    function main() {
        let a = 2.5;
        let b = 2.5;
        a = a + b;
        b = a * 2.0;
        return b;
    }");
    let parsed = parse(&mut lex.peekable()).unwrap();
    let mut machine = Machine::new();
    let mut cmpl = Compiler::new(&mut machine,0);
    cmpl.compile(parsed);    
}


