use super::repository::UserRepository;
use crate::{
    auth::BasicAuth,
    models::{CreateUserRq, UpdateUserRq},
    DbConn,
};
use rocket::{
    response::status,
    serde::json::{Json, Value},
};
use serde_json::json;

#[get("/")]
pub fn hello() -> Value {
    json!("Hello World\n1")
}

#[get("/user")]
pub async fn get_users(_auth: BasicAuth, con: DbConn) -> Value {
    con.run(|c| {
        let user = UserRepository::find_all(c, 1000).expect("Query user failed");
        json!(user)
    })
    .await
}

#[get("/user/<id>")]
pub async fn get_user(id: i32, _auth: BasicAuth, db: DbConn) -> Value {
    db.run(move |c| {
        let user = UserRepository::find(c, id).expect("DB error querying");
        json!(user)
    })
    .await
}

#[post("/user", format = "json", data = "<new_user>")]
pub async fn create_user(db: DbConn, new_user: Json<CreateUserRq>) -> Value {
    db.run(|c| {
        let created_user =
            UserRepository::create(c, new_user.into_inner()).expect("DB error inserting");
        json!(created_user)
    })
    .await
}

#[put("/user/<id>", format = "json", data = "<update_user>")]
pub async fn update_user(
    id: i32,
    db: DbConn,
    _auth: BasicAuth,
    update_user: Json<UpdateUserRq>,
) -> Value {
    db.run(move |c| {
        let user =
            UserRepository::save(c, id, update_user.into_inner()).expect("DB error updating");
        json!(user)
    })
    .await
}

#[delete("/user/<id>", format = "json")]
pub async fn delete_user(id: i32, _auth: BasicAuth, db: DbConn) -> status::NoContent {
    db.run(move |c| {
        UserRepository::delete(c, id).expect("DB error deleting");
        status::NoContent
    })
    .await
}
