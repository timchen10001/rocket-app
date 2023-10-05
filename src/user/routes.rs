use crate::{
    auth::BasicAuth,
    models::{CreateUserRq, UpdateUserRq, User},
    schema::users,
    DbConn,
};
use diesel::prelude::*;
use rocket::{
    response::status,
    serde::json::{json, Json, Value},
};

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
pub async fn get_user(id: i32, _auth: BasicAuth, db: DbConn) -> Value {
    db.run(move |c| {
        let user = users::table
            .find(id)
            .get_result::<User>(c)
            .expect("DB error querying");
        json!(user)
    })
    .await
}

#[post("/user", format = "json", data = "<new_user>")]
pub async fn create_user(db: DbConn, new_user: Json<CreateUserRq>) -> Value {
    db.run(|c| {
        match diesel::insert_into(users::table)
            .values(new_user.into_inner())
            .execute(c)
            .expect("DB error inserting")
        {
            1 => json!("OK"),
            0 => json!("USER NOT FOUND"),
            _ => json!("Unexpected Error"),
        }
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
        match diesel::update(users::table.find(id))
            .set((
                users::name.eq(update_user.name.to_owned().unwrap_or_default()),
                users::email.eq(update_user.email.to_owned().unwrap_or_default()),
            ))
            .execute(c)
            .expect("DB error updating")
        {
            1 => json!("OK"),
            0 => json!("NOT FOUND"),
            _ => json!("Unexpected Error"),
        }
    })
    .await
}

#[delete("/user/<id>", format = "json")]
pub async fn delete_user(id: i32, _auth: BasicAuth, db: DbConn) -> status::NoContent {
    db.run(move |c| {
        diesel::delete(users::table.find(id))
            .execute(c)
            .expect("DB error deleting");
        status::NoContent
    })
    .await
}
