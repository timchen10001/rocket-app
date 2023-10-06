use crate::models::{CreateUserRq, UpdateUserRq};
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

    pub fn create(c: &mut SqliteConnection, new_user: CreateUserRq) -> QueryResult<User> {
        diesel::insert_into(users::table)
            .values(new_user)
            .execute(c)?;

        let last_id = Self::last_inserted_id(c)?;
        Self::find(c, last_id)
    }

    pub fn save(c: &mut SqliteConnection, id: i32, update_user: UpdateUserRq) -> QueryResult<User> {
        diesel::update::<users::table>(users::table)
            .set((
                users::name.eq(update_user.name.to_owned().unwrap_or_default()),
                users::email.eq(update_user.email.to_owned().unwrap_or_default()),
            ))
            .filter(users::id.eq(id.to_owned()))
            .execute(c)?;

        Self::find(c, id)
    }

    pub fn delete(c: &mut SqliteConnection, id: i32) -> QueryResult<usize> {
        diesel::delete::<users::table>(users::table)
            .filter(users::id.eq(id))
            .execute(c)
    }

    fn last_inserted_id(c: &mut SqliteConnection) -> QueryResult<i32> {
        users::table
            .select(users::id)
            .order(users::id.desc())
            .first(c)
    }
}
