#[derive(Debug, Copy, Clone)]
pub enum ValueType {
    Number,
    String,
    Unit,
}

#[derive(Debug, Clone)]
pub enum Value {
    Number(f32),
    String(String),
}

impl Value {
    pub fn to_string(&self) -> String {
        match self {
            Value::Number(n) => n.to_string(),
            Value::String(s) => format!("area_memdup(\"{}\", {})", s, s.len() + 1),
        }
    }

    pub fn to_number(&self) -> f32 {
        match self {
            Value::Number(n) => *n,
            Value::String(_) => panic!("Cannot convert string to number"),
        }
    }
}