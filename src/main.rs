mod data_transformer;

use data_transformer::{persist, Field, Transformable, DataTransformer};
use sled::Db;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
struct Message {
    sender_id: Field<i32>,
    text: Field<String>,
}

fn main() -> sled::Result<()> {
    let db = sled::open("demo_db")?;

    let message = Message {
        sender_id: Field::new("IntegerField".to_string(), 1),
        text: Field::new("StringField".to_string(), "Hello, world!".to_string()),
    };

    // Save to sled
    message.to_sled(&db)?;

    // Load from sled
    let loaded_message: Message = Message::from_sled(&db, "message")?;

    println!("Loaded Message: {:?}", loaded_message);

    Ok(())
}
