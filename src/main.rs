#[macro_use]
extern crate rocket;

mod auth;
mod error;
mod models;
mod schema;
mod user;

// use diesel_migrations::EmbeddedMigrations;
use error::*;
use rocket::{fairing::AdHoc, Build, Rocket};
use rocket_sync_db_pools::database;
use user::routes::*;

#[database("sqlite")]
pub struct DbConn(diesel::SqliteConnection);

async fn run_db_migrations(rocket: Rocket<Build>) -> Rocket<Build> {
    use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

    const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

    DbConn::get_one(&rocket)
        .await
        .expect("Unable to retrieve connection")
        .run(|c| {
            c.run_pending_migrations(MIGRATIONS).expect("Migrations failed");
        })
        .await;

    rocket
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
        .register(
            "/",
            catchers![not_found, unauthorized, unprocessable_entity],
        )
        .attach(DbConn::fairing())
        .attach(AdHoc::on_ignite("Diesel migrations", run_db_migrations))
        .launch()
        .await;
}
