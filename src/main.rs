use sled::open;
mod data_transformer;
use data_transformer::*;

fn main() -> Result<(), TransformationError> {
    // Open a sled database
    let db = sled::open("my_db").map_err(TransformationError::DatabaseError)?;

    // Create simple and composite fields
    let simple_field = Field::Simple {
        field_type: "Name".to_string(),
        value: "John".to_string(),
    };

    let composite_field = Field::Composite {
        fields: vec![
            Field::Simple {
                field_type: "Age".to_string(),
                value: "30".to_string(),
            },
            Field::Simple {
                field_type: "Address".to_string(),
                value: "123 Main St".to_string(),
            },
        ],
    };

    // Create a Transformable instance
    let transformable = Transformable {
        id: "person1".to_string(),
        data: composite_field,
    };

    // Save the Transformable instance to the database
    transformable.save_to_db(&db)?;

    // Load the Transformable instance from the database
    let loaded_transformable: Transformable<String> = Transformable::load_from_db(&db, "person1")?;

    // Display the loaded Transformable instance
    println!("Loaded Transformable: {:?}", loaded_transformable);

    Ok(())
}
