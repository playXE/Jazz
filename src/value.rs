///Value

#[derive(Clone, Debug, Copy)]
pub enum Value
{
    /// Integer or i32 in Rust
    Int(i32),
    /// Long or i64 in Rust
    Long(i64),
    /// Float or f32 in Rust
    Float(f32),
    /// Double or f64 in Rust
    Double(f64),
    /// Pointer to object in pool
    Object(usize),
    /// Null reference
    Null,
    /// Boolean
    Bool(bool),
}
