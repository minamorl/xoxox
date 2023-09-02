use serde::{Serialize, Deserialize};
use std::fmt::Debug;

#[derive(Debug)]
pub enum TransformationError {
    SerializationFailed(serde_json::Error),
    DeserializationFailed(serde_json::Error),
    DatabaseError(sled::Error),
    NotFound(String),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Field<T> {
    pub field_type: String,
    pub value: T,
}

impl<T> Field<T> {
    pub fn new(field_type: String, value: T) -> Self {
        Self { field_type, value }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Transformable<T> {
    pub data: T,
}

pub trait Transform<Y> {
    fn transform(&self) -> Result<Y, TransformationError>;
}
