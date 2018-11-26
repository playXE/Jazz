#![feature(duration_as_u128)]
#![warn(rust_2018_idioms)]

pub mod builtins;
pub mod class;
pub mod compiler;
pub mod ircode;
pub mod parser;
pub mod std_library;
pub use self::compiler::Compiler;
