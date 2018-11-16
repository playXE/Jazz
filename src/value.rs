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

use crate::{machine::Machine, object::ObjectAddon};

impl ObjectAddon for Value
{
    fn to_double(&self, m: &mut Machine) -> f64
    {
        match self {
            Value::Double(d) => d.clone(),
            Value::Float(f) => f.clone() as f64,
            Value::Bool(b) => {
                if *b {
                    return 1.0;
                } else {
                    return 0.0;
                }
            }
            Value::Int(i) => *i as f64,
            Value::Long(i) => *i as f64,
            Value::Null => 0.0,
            Value::Object(id) => {
                let obj = m.pool.get(*id);
                obj.to_double(m)
            }
        }
    }
}
