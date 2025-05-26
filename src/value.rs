
#[derive(Debug)]
pub enum ValueType {
    Number,
}

#[derive(Debug)]
pub struct Value {
    value_type: ValueType,
    value: f64,
}

impl Value {
    pub fn new(value_type: ValueType, value: f64) -> Value {
        Value { value_type, value }
    }
}
