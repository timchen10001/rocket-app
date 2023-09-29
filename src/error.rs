use serde_json::{Value, json};

#[catch(404)]
pub fn not_found() -> Value {
    json!("Not Found!")
}