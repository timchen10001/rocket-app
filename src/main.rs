#[macro_use]
extern crate rocket;

mod auth;
mod error;
mod models;
mod routes;
mod schema;

use error::*;
use rocket_sync_db_pools::database;
use routes::user::*;

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
        .register("/", catchers![not_found, unauthorized])
        .attach(DbConn::fairing())
        .launch()
        .await;
}
