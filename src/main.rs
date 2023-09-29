#[macro_use]
extern crate rocket;

#[get("/")]
fn hello() -> &'static str {
    "Hello World\n1"
}

#[rocket::main]
async fn main() {
    let _ = rocket::build().mount("/", routes![hello]).launch().await;
}
