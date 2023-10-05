use crate::{models::User, schema::users};
use diesel::prelude::*;
use diesel::SqliteConnection;

pub struct UserRepository;

impl UserRepository {
    pub fn find(c: &mut SqliteConnection, id: i32) -> QueryResult<User> {
        users::table.find(id).get_result::<User>(c)
    }
    pub fn find_all(c: &mut SqliteConnection, limit: i64) -> QueryResult<Vec<User>> {
        users::table
            .order(users::id.desc())
            .limit(limit)
            .load::<User>(c)
    }
}
