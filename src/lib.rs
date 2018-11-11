//! Jazz Virtual Machine
//!
//! Jazz is a register-based virtual machine
//!
//! Jazz is still in active develop so it's not recommended to use Jazz for your purposes
//!
//!
//! Example code:
//!```rust
//! LoadInt(0,12) // Load 12 into R(0)
//! LoadInt(1,3)  // Load 13 into R(1)
//! Add(2,1,0)    // Add value from R(1) to R(0) and store result in R(2)
//! Ret(2)        // Return value from R(2)
//! ```
//!
//! Jazz is heavily inspired by [Gravity](https://marcobambini.github.io/gravity/#/) language VM
//!

#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

extern crate time;
pub mod frame;
pub mod function;
pub mod jit;
pub mod machine;
pub mod object;
pub mod object_info;
pub mod object_pool;
pub mod opcodes;
pub mod static_root;
pub mod value;
