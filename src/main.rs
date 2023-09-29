mod user;
mod error;

#[macro_use]
extern crate rocket;

use user::*;
use error::*;


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
        .register("/", catchers![not_found])
        .launch()
        .await;
}
