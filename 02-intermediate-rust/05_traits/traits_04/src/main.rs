// Write a program that creates a trait called Serializable with methods for serializing and deserializing data, and implement it for a struct representing a user.

use std::fs;
use anyhow::Result;

use serde::{Serialize, Deserialize};

const DB_FILE_PATH: &str = "traits_04/db.json";

trait Serializable {
    fn serialize_user(&self) -> Result<()>;
    fn deserialize_user(&self) -> Result<()>;
}

#[derive(Debug, Serialize, Deserialize)]
struct User {
    name: String,
    username: String,
    email: String,
    password: String
}

impl Serializable for User {
    fn deserialize_user(&self) -> Result<()> {
        let db_contents = fs::read_to_string(DB_FILE_PATH)?;
        let user: User = serde_json::from_str(&db_contents)?;
        println!("{:?}", user);
        Ok(())
    }

    fn serialize_user(&self) -> Result<()> {
        let serialized_user: String = serde_json::to_string(self)?;
        fs::write(DB_FILE_PATH, serialized_user)?;
        Ok(())
    }
}

fn main() -> Result<()> {
    let user: User = User {
        name: "Radha Swami".to_string(),
        username: "radha".to_string(),
        email: "radhaswami@gmail.com".to_string(),
        password: "@Haregobinda4567#".to_string()
    };

    user.serialize_user()?;
    user.deserialize_user()?;

    Ok(())
}
