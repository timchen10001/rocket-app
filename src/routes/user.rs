use rocket::{
    response::status,
    serde::json::{json, Value},
};

use crate::{auth::BasicAuth, models::User, schema::users, DbConn};
use diesel::prelude::*;

#[get("/")]
pub fn hello() -> Value {
    json!("Hello World\n1")
}

#[get("/user")]
pub async fn get_users(_auth: BasicAuth, con: DbConn) -> Value {
    con.run(|c| {
        let user = users::table
            .order(users::id.desc())
            .limit(1000)
            .load::<User>(c)
            .expect("Query user failed");
        json!(user)
    })
    .await
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
