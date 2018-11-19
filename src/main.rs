extern crate jazz;
extern crate jazz_vm;
extern crate structopt;
use jazz::{
    parser::{lex, parse},
    Compiler,
};
use jazz_vm::machine::Machine;

use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
pub struct Options {
    #[structopt(name = "FILE", parse(from_os_str))]
    file: Option<PathBuf>,
    #[structopt(short = "d", long = "debug")]
    debug: bool,
}

fn main() {
    let mut src = String::new();

    let ops = Options::from_args();

    if let Some(path) = ops.file {
        File::open(path).unwrap().read_to_string(&mut src).unwrap();
    } else {
        panic!("You should enter file path");
    }

    let lex = lex(&src);
    let parsed = parse(&mut lex.peekable()).unwrap();
    let mut machine = Machine::new();
    let mut cmpl = Compiler::new(&mut machine, 0, ops.debug);
    cmpl.compile(parsed);
}
