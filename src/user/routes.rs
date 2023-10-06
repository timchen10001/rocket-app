use super::repository::UserRepository;
use crate::{
    auth::BasicAuth,
    models::{CreateUserRq, UpdateUserRq},
    DbConn,
};
use rocket::{
    http::Status,
    response::status::{self, Custom},
    serde::json::{Json, Value},
};
use serde_json::json;

#[get("/")]
pub fn hello() -> Value {
    json!("Hello World\n1")
}

#[get("/user")]
pub async fn get_users(__rocket_auth: BasicAuth, con: DbConn) -> Result<Value, Custom<Value>> {
    con.run(|c| {
        UserRepository::find_all(c, 1000)
            .map(|users| json!(users))
            .map_err(|_| Custom(Status::InternalServerError, json!("Something went wrong.")))
    })
    .await
}

#[get("/user/<id>")]
pub async fn get_user(
    id: i32,
    __rocket_auth: BasicAuth,
    db: DbConn,
) -> Result<Value, Custom<Value>> {
    db.run(move |c| {
        UserRepository::find(c, id)
            .map(|user| json!(user))
            .map_err(|_| Custom(Status::NotFound, json!("User not found.")))
    })
    .await
}

#[post("/user", format = "json", data = "<new_user>")]
pub async fn create_user(db: DbConn, new_user: Json<CreateUserRq>) -> Result<Value, Custom<Value>> {
    db.run(|c| {
        UserRepository::create(c, new_user.into_inner())
            .map(|created_user| json!(created_user))
            .map_err(|_| Custom(Status::InternalServerError, json!("Create User Failed.")))
    })
    .await
}

#[put("/user/<id>", format = "json", data = "<update_user>")]
pub async fn update_user(
    id: i32,
    db: DbConn,
    __rocket_auth: BasicAuth,
    update_user: Json<UpdateUserRq>,
) -> Result<Value, Custom<Value>> {
    db.run(move |c| {
        UserRepository::save(c, id, update_user.into_inner())
            .map(|updated_user| json!(updated_user))
            .map_err(|_| Custom(Status::InternalServerError, json!("Update user failed.")))
    })
    .await
}

#[delete("/user/<id>", format = "json")]
pub async fn delete_user(
    id: i32,
    __rocket_auth: BasicAuth,
    db: DbConn,
) -> Result<status::NoContent, Custom<Value>> {
    db.run(move |c| {
        UserRepository::delete(c, id)
            .map(|_| status::NoContent)
            .map_err(|_| Custom(Status::InternalServerError, json!("Delete user failed.")))
    })
    .await
}
