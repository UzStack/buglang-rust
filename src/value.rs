
#[derive(Debug, Clone)]
pub enum ValueType {
    Number,
}

#[derive(Debug, Clone)]
pub struct Value {
    pub value_type: ValueType,
    pub value: f64,
}

impl Value {
    pub fn new(value_type: ValueType, value: f64) -> Value {
        Value { value_type, value }
    }
}
