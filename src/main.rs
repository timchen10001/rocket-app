mod auth;
mod error;
mod user;

#[macro_use]
extern crate rocket;

use error::*;
use user::*;

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
        .launch()
        .await;
}
