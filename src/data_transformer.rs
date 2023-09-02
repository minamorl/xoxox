
use sled::{Db, IVec};
use serde::{Serialize, Deserialize};
use serde_json::{to_vec};

// Transformation Error Types
pub enum TransformationError {
    SerializationFailed,
    DeserializationFailed,
}

// Generic Field
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

// Generic Transformable Data
#[derive(Serialize, Deserialize)]
pub struct Transformable<T> {
    pub data: T,
}

// Data Transformation Trait
pub trait DataTransformer: Sized {
    fn to_sled(&self, db: &Db) -> Result<(), TransformationError>;
    fn from_sled(db: &Db, key: &str) -> Result<Self, TransformationError>;
}


// High-order function for data transformation
pub fn transform_data<T, F>(data: T, transform_fn: F) -> Result<T, TransformationError>
where
    F: Fn(T) -> Result<T, TransformationError>,
{
    transform_fn(data)
}

// Serialization and Deserialization using Sled and Serde
pub fn save_to_sled<T: Serialize>(db: &Db, key: &str, data: &T) -> Result<(), TransformationError> {
    let serialized_data = to_vec(data).map_err(|_| TransformationError::SerializationFailed)?;
    db.insert(key, serialized_data).map_err(|_| TransformationError::SerializationFailed)?;
    Ok(())
}

pub fn load_from_sled<T: Deserialize<'static>>(db: &Db, key: &str) -> Result<T, TransformationError> {
    let stored_data = db.get(key)
        .map_err(|_| TransformationError::DeserializationFailed)?
        .ok_or(TransformationError::NotFound(key.to_string()))?;

    let deserialized_data: T = from_slice(stored_data.as_ref())
        .map_err(|_| TransformationError::DeserializationFailed)?;

    Ok(deserialized_data)
}
