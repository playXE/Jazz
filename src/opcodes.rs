#[derive(Clone)]
pub enum Instruction
{
    ///LoadInt R(A) = B
    /// Loading integer value B to register A
    LoadInt(usize, i32),
    ///LoadLong R(A) = B
    /// Loading long value B to register A
    LoadLong(usize, i64),
    ///LoadFloat R(A) = B
    /// Loading float value B to register A
    LoadFloat(usize, f32),
    /// LoadDouble R(A) = B
    /// Loading double value B to register A
    LoadDouble(usize, f64),
    LoadObject(usize, usize),
    /// LoadConst R(A) = C(B)
    /// Load constant from object pool to register A
    LoadConst(usize, usize),
    /// LoadGlobal R(A) = G(B)
    /// Load global value B into register A
    LoadGlobal(usize, usize),
    /// LoadAt R(A) = R(B)\[C\]
    /// Load C from B and store in A
    LoadAt(usize, usize, usize),
    /// LoadSuper R(A) = R(B)C
    /// Load C from B and store in A
    LoadSuper(usize, usize, usize),
    /// Move R(A) = R(B)
    /// Move register
    Move(usize, usize),
    /// Store R(B)\[C\] = A
    /// Store A into R(B)\[C\]
    Store(usize, usize, usize),
    StoreAt(usize, usize, usize),
    /// StoreGlobal G(A) = R(B)
    /// Store global
    StoreGlobal(usize, usize),
    /// Jump IP
    Jump(usize),
    /// Jump (R(A) == false ? ip = B : continue)
    JumpF(usize, usize),
    /// Jump (R(A) == true ? ip == B : continue)
    JumpT(usize, usize),

    /// Push value from R(A) to arguments stack
    PushArg(usize),
    /// R(A) = B(Args), C - Arg count, args poped from arg stack
    Call(usize, usize, usize),

    ///Add R(A) = R(B) + R(C)
    Add(usize, usize, usize),
    ///Sub R(A) = R(B) - R(C)
    Sub(usize, usize, usize),
    ///Mul R(A) = R(B) * R(C)
    Mul(usize, usize, usize),
    ///Div R(A) = R(B) / R(C)
    Div(usize, usize, usize),
    ///Gt R(A) = R(B) > R(C)
    Gt(usize, usize, usize),
    ///Lt R(A) = R(B) < R(C)
    Lt(usize, usize, usize),
    /// Ret0
    /// return null value
    Ret0,
    /// Ret R(A)
    /// return value from R(A)
    Ret(usize),
}

use std::fmt;

impl fmt::Debug for Instruction
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        match self {
            Instruction::LoadFloat(reg, v) => write!(f, "LoadFloat R({}) = {}", reg, v),
            Instruction::LoadInt(reg, v) => write!(f, "LoadInt R({}) = {}", reg, v),
            Instruction::LoadDouble(reg, v) => write!(f, "LoadDouble R({}) = {}", reg, v),
            Instruction::LoadLong(reg, v) => write!(f, "LoadLong R({}) = {}", reg, v),
            Instruction::Ret(reg) => write!(f, "Ret R({})", reg),
            Instruction::Ret0 => write!(f, "Ret R(Null)"),
            Instruction::Jump(idx) => write!(f, "Jump ip = {}", idx),
            Instruction::JumpF(reg, idx) => write!(f, "JumpF (R({}) == false) ? ip = {}", reg, idx),
            Instruction::JumpT(reg, idx) => write!(f, "JumpT (R({}) == true ? ip = {}", reg, idx),
            Instruction::Add(reg3, reg2, reg1) => {
                write!(f, "Add R({}) = R({}) + R({})", reg3, reg2, reg1)
            }
            Instruction::Sub(reg3, reg2, reg1) => {
                write!(f, "Sub R({}) = R({}) - R({})", reg3, reg2, reg1)
            }
            Instruction::Mul(reg3, reg2, reg1) => {
                write!(f, "Sub R({}) = R({}) * R({})", reg3, reg2, reg1)
            }
            Instruction::Div(reg3, reg2, reg1) => {
                write!(f, "Div R({}) = R({}) / R({})", reg3, reg2, reg1)
            }
            Instruction::Gt(reg3, reg2, reg1) => {
                write!(f, "Gt R({}) = R({}) > R({})", reg3, reg2, reg1)
            }
            v => write!(f, "{:?}", v),
        }
    }
}

/// Stores instructions
#[derive(Clone)]
pub struct CodeBlock
{
    pub code: Vec<Instruction>,
    pub ip: usize,
}

impl fmt::Debug for CodeBlock
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        writeln!(f, "CodeBlock:");
        writeln!(f, "code:");
        for i in 0..self.code.len() {
            writeln!(f, "{:05} {:?}", i, self.code[i]);
        }
        write!(f, "")
    }
}

impl CodeBlock
{
    /// Create new instance of CodeBlock
    pub fn new(ins: Vec<Instruction>) -> CodeBlock
    {
        CodeBlock { code: ins, ip: 0 }
    }
}
