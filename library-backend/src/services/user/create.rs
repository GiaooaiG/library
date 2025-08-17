use crate::models::{User, NewUser};
use crate::error::LibraryError;
use crate::schema::users;
use diesel::prelude::*;

pub struct UserCreate;

impl UserCreate {
    pub fn create_user(conn: &mut diesel::mysql::MysqlConnection, new_user: NewUser) -> Result<User, LibraryError> {
        // 检查用户名是否已存在
        let existing_user = users::table
            .filter(users::username.eq(&new_user.username))
            .first::<User>(conn)
            .optional()?;

        if existing_user.is_some() {
            return Err(LibraryError::UserAlreadyExists(new_user.username));
        }

        // 创建用户
        diesel::insert_into(users::table)
            .values(&new_user)
            .execute(conn)?;

        // 获取刚创建的用户
        let user = users::table
            .order(users::id.desc())
            .first(conn)?;

        Ok(user)
    }
}