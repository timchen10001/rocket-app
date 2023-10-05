#[macro_use]
extern crate rocket;

mod auth;
mod error;
mod models;
mod schema;
mod user;

use error::*;
use rocket_sync_db_pools::database;
use user::routes::*;

#[database("sqlite")]
pub struct DbConn(diesel::SqliteConnection);

#[rocket::main]
async fn main() {
    let _ = rocket::build()
        .mount(
            "/",
            routes![
                hello,
                get_user,
                get_users,
                create_user,
                update_user,
                delete_user
            ],
        )
        .register(
            "/",
            catchers![not_found, unauthorized, unprocessable_entity],
        )
        .attach(DbConn::fairing())
        .launch()
        .await;
}
