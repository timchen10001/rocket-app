use serde_json::{Value, json};

#[catch(404)]
pub fn not_found() -> Value {
    json!("Not Found!")
}

#[catch(401)]
pub fn unauthorized() -> Value {
    json!("Unauthorized!")
}