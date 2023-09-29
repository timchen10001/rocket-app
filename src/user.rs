use rocket::{
    response::status,
    serde::json::{json, Value},
};

#[get("/")]
pub fn hello() -> Value {
    json!("Hello World\n1")
}

#[get("/user")]
pub fn get_users() -> Value {
    json!([{
        "id": 1,
        "name": "Tim"
    }, {
        "id": 2,
        "name": "Amy"
    }])
}

#[get("/user/<id>")]
pub fn get_user(id: i32) -> Value {
    json!([{
        "id": id,
        "name": "Tim Chen",
        "email": "rustlearning@gmail.com"
    }])
}

#[post("/user", format = "json")]
pub fn create_user() -> Value {
    json!([{
     "id": 3,
     "name": "Tom",
     "email": "tom@gmail.com"
    }])
}

#[put("/user/<id>", format = "json")]
pub fn update_user(id: i32) -> Value {
    json!([{
        "id": id,
        "name": "Queen",
        "email": "queen@gmail.com"
    }])
}

#[delete("/user/<_id>", format = "json")]
pub fn delete_user(_id: i32) -> status::NoContent {
    status::NoContent
}
