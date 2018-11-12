# Jazz Virtual Machine
[![license](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/playXE/Jazz/blob/master/LICENSE)
[![Docs Status](https://docs.rs/jazz/badge.svg)](https://docs.rs/jazz)


Jazz is a register-based virtual machine

VM is still in active develop so it's not recommended to use Jazz for your purposes

# Example code:
```rust
LoadInt(0,12) // Load 12 into R(0)
LoadInt(1,3)  // Load 3 into R(1)
Add(2,1,0)    // Add value from R(1) to R(0) and store result in R(2)
Ret(2)        // Return value from R(2)
 ```

Jazz is heavily inspired by [Gravity](https://marcobambini.github.io/gravity/#/) language VM
