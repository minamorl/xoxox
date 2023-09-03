use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use std::error::Error;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TransformationError {
    InvalidType,
    InvalidValue,
    CustomError(String),
}

pub trait Applicable {
    fn apply<F>(&mut self, func: F) -> Result<&mut Self, TransformationError>
    where
        F: FnOnce(&mut Self) -> Result<(), TransformationError>;
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum JsonValue {
    Null,
    Bool(bool),
    Number(f64),
    String(String),
    Array(Vec<JsonValue>),
    Object(HashMap<String, JsonValue>),
}

impl Applicable for JsonValue {
    fn apply<F>(&mut self, func: F) -> Result<&mut Self, TransformationError>
    where
        F: FnOnce(&mut Self) -> Result<(), TransformationError>,
    {
        func(self)?;
        Ok(self)
    }
}
