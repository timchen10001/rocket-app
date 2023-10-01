use crate::schema::users;
use diesel::{AsChangeset, Queryable};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Queryable, AsChangeset)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct User {
    // #[serde(skip_deserializing)]
    pub id: i32,
    pub name: String,
    pub email: String,
    // #[serde(skip_deserializing)]
    pub created_at: String,
    // #[serde(skip_deserializing)]
    pub updated_at: String,
}
