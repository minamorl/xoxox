use std::collections::HashMap;
mod data_transformer;

use data_transformer::{DataTransformer, FieldType, Schema};
use sled::Config;

fn main() -> sled::Result<()> {
    // Initialize Sled database
    let config = Config::new().temporary(true);
    let db = config.open()?;

    // Define schemas
    let internal_representation = Schema::Complex(
        vec![
            ("field1".to_string(), Schema::Simple(FieldType::StringType)),
            ("field2".to_string(), Schema::Simple(FieldType::StringType)),
        ]
        .into_iter()
        .collect(),
    );
    let presentation = Schema::Complex(
        vec![("field3".to_string(), Schema::Simple(FieldType::StringType))]
        .into_iter()
        .collect(),
    );

    // Define transformation logic
    let transform_logic = |data: HashMap<String, String>| -> Result<HashMap<String, String>, String> {
        let field1 = data.get("field1").ok_or("Missing field1")?;
        let field2 = data.get("field2").ok_or("Missing field2")?;
        let combined = format!("{} {}", field1, field2);

        let mut transformed_data = HashMap::new();
        transformed_data.insert("field3".to_string(), combined);

        Ok(transformed_data)
    };

    // Create DataTransformer
    let transformer = DataTransformer::new(internal_representation, presentation);

    // Sample data to transform
    let mut sample_data = HashMap::new();
    sample_data.insert("field1".to_string(), "Data1".to_string());
    sample_data.insert("field2".to_string(), "Data2".to_string());

    // Perform transformation and save to Sled
    transformer.save_to_sled(&db, sample_data)?;

    // Load transformed data from Sled
    let loaded_data = transformer.load_from_sled(&db)?;

    println!("Loaded transformed data: {:?}", loaded_data);

    Ok(())
}
