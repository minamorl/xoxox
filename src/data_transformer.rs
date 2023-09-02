use sled::{Db};
use serde::{Serialize, Deserialize, from_slice, to_vec, de::DeserializeOwned};
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

#[derive(Serialize, Deserialize, Debug)]
pub struct Transformable<T: Serialize + DeserializeOwned> {
    pub id: String,
    pub data: T,
}

pub trait Transform<Y> {
    fn transform(&self) -> Result<Y, TransformationError>;
}

pub trait Persistable {
    fn save_to_db(&self, db: &Db) -> Result<(), TransformationError>;
    fn load_from_db(db: &Db, id: &str) -> Result<Self, TransformationError>
    where
        Self: Sized;
}

impl<T: Serialize + DeserializeOwned> Persistable for Transformable<T> {
    fn save_to_db(&self, db: &Db) -> Result<(), TransformationError> {
        let serialized = to_vec(&self.data).map_err(TransformationError::SerializationFailed)?;
        db.insert(&self.id, serialized).map_err(TransformationError::DatabaseError)?;
        Ok(())
    }

    fn load_from_db(db: &Db, id: &str) -> Result<Self, TransformationError> {
        let stored_data = db.get(id).map_err(TransformationError::DatabaseError)?
            .ok_or_else(|| TransformationError::NotFound(id.to_string()))?;
        
        let data: T = from_slice(&stored_data).map_err(TransformationError::DeserializationFailed)?;
        Ok(Self { id: id.to_string(), data })
    }
}
