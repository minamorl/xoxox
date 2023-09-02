use sled::{Db};
use serde::{Serialize, Deserialize, de::DeserializeOwned};
use serde_json::{to_vec, from_slice};
use std::fmt::Debug;

#[derive(Debug)]
pub enum TransformationError {
    SerializationFailed(serde_json::Error),
    DeserializationFailed(serde_json::Error),
    DatabaseError(sled::Error),
    NotFound(String),
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Field<T> {
    Simple {
        field_type: String,
        value: T,
    },
    Composite {
        fields: Vec<Field<T>>,
    },
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Transformable<T> {
    pub id: String,
    pub data: Field<T>,
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
        let serialized = to_vec(&self).map_err(TransformationError::SerializationFailed)?;
        db.insert(&self.id, serialized).map_err(TransformationError::DatabaseError)?;
        Ok(())
    }

    fn load_from_db(db: &Db, id: &str) -> Result<Self, TransformationError> {
        let stored_data = db.get(id).map_err(TransformationError::DatabaseError)?
            .ok_or_else(|| TransformationError::NotFound(id.to_string()))?;
        
        let transformable: Self = from_slice(&stored_data).map_err(TransformationError::DeserializationFailed)?;
        Ok(transformable)
    }
}
