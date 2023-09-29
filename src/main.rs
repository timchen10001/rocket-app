#[macro_use]
extern crate rocket;

use rocket::{
    response::status,
    serde::json::{json, Value},
};

#[get("/")]
fn hello() -> Value {
    json!("Hello World\n1")
}

#[get("/user")]
fn get_users() -> Value {
    json!([{
        "id": 1,
        "name": "Tim"
    }, {
        "id": 2,
        "name": "Amy"
    }])
}

#[get("/user/<id>")]
fn get_user(id: i32) -> Value {
    json!([{
        "id": id,
        "name": "Tim Chen",
        "email": "rustlearning@gmail.com"
    }])
}

#[post("/user", format = "json")]
fn create_user() -> Value {
    json!([{
     "id": 3,
     "name": "Tom",
     "email": "tom@gmail.com"
    }])
}

#[put("/user/<id>", format = "json")]
fn update_user(id: i32) -> Value {
    json!([{
        "id": id,
        "name": "Queen",
        "email": "queen@gmail.com"
    }])
}

#[delete("/user/<_id>", format = "json")]
fn delete_user(_id: i32) -> status::NoContent {
    status::NoContent
}

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
        .launch()
        .await;
}
