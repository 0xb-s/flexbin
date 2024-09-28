use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum FieldType {
    Integer,
    Float,
    String,
    Boolean,
    Object(Schema),
    Array(Box<FieldType>),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Schema {
    pub fields: Vec<(String, FieldType)>,
}

impl Schema {
    pub fn new(fields: Vec<(String, FieldType)>) -> Self {
        Self { fields }
    }
}
