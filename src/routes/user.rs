use rocket::{
    response::status,
    serde::json::{json, Json, Value},
};

use crate::{
    auth::BasicAuth,
    models::{CreateUserRq, User},
    schema::users,
    DbConn,
};
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

#[post("/user", format = "json", data = "<new_user>")]
pub async fn create_user(db: DbConn, new_user: Json<CreateUserRq>) -> Value {
    db.run(|c| {
        let inserted_user = diesel::insert_into(users::table)
            .values(new_user.into_inner())
            .execute(c)
            .expect("DB error inserting");
        json!(inserted_user)
    })
    .await
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
