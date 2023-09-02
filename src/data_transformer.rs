
use sled::{Config, Db, IVec};
use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use serde_json::{to_vec, from_slice};
pub enum FieldType {
    StringType,
    // Add other types as needed
}

pub enum Schema {
    Simple(FieldType),
    Complex(HashMap<String, Schema>),
}

pub struct DataTransformer {
    from: Schema,
    to: Schema,
}

impl DataTransformer {
    pub fn new(from: Schema, to: Schema) -> Self {
        Self { from, to }
    }

    pub fn transform(&self, data: HashMap<String, String>) -> Result<HashMap<String, String>, String> {
        // Insert your transformation logic here
        // For example, combine "field1" and "field2" to produce "field3"
        Ok(data)  // Placeholder
    }

    pub fn get_schema_names(&self, schema: &Schema, prefix: String) -> Vec<String> {
        let mut keys = Vec::new();
        match schema {
            Schema::Simple(_) => keys.push(prefix),
            Schema::Complex(map) => {
                for (key, value) in map {
                    let new_prefix = if prefix.is_empty() {
                        key.clone()
                    } else {
                        format!("{}.{}", prefix, key)
                    };
                    keys.extend(self.get_schema_names(value, new_prefix));
                }
            }
        }
        keys
    }

    // New method to save transformed data to Sled
    pub fn save_to_sled(&self, db: &Db, data: HashMap<String, String>) -> sled::Result<()> {
        let transformed_data = match self.transform(data) {
            Ok(data) => data,
            Err(_) => return Err(sled::Error::Unsupported("Transformation failed".into())),
        };
        let serialized_data = to_vec(&transformed_data).map_err(|_| sled::Error::Unsupported("Serialization failed".into()))?;
        db.insert("transformed_data", serialized_data)?;
        Ok(())
    }

    // New method to load and deserialize data from Sled
    pub fn load_from_sled(&self, db: &Db) -> sled::Result<HashMap<String, String>> {
        let stored_data = db.get("transformed_data")?
            .ok_or_else(|| sled::Error::CollectionNotFound(IVec::from("transformed_data not found".as_bytes().to_vec())))?;
        let deserialized_data: HashMap<String, String> = from_slice(&stored_data).map_err(|_| sled::Error::Unsupported("Deserialization failed".into()))?;
        Ok(deserialized_data)
    }
}

#[derive(Serialize, Deserialize)]
pub struct Data {
    field1: String,
    field2: String,
}
