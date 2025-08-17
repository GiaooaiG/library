use crate::models::User;
use crate::error::LibraryError;
use crate::schema::users;
use diesel::prelude::*;

pub struct UserQuery;

impl UserQuery {
    pub fn get_user_by_username(conn: &mut diesel::mysql::MysqlConnection, username: &str) -> Result<User, LibraryError> {
        let user = users::table
            .filter(users::username.eq(username))
            .first::<User>(conn)
            .optional()?
            .ok_or(LibraryError::UserNotFound)?;

        Ok(user)
    }

    /// 获取所有用户（管理员功能）
    pub fn get_all_users(
        conn: &mut diesel::mysql::MysqlConnection,
        limit: i64,
        offset: i64,
    ) -> Result<(Vec<User>, i64), LibraryError> {
        use crate::schema::users::dsl::*;

        // 获取总用户数
        let total = users
            .count()
            .get_result::<i64>(conn)
            .map_err(|e| {
                eprintln!("获取用户总数失败: {:?}", e);
                LibraryError::DatabaseError(e)
            })?;

        // 获取分页用户
        let user_list = users
            .order(id.asc())
            .limit(limit)
            .offset(offset)
            .load::<User>(conn)
            .map_err(|e| {
                eprintln!("获取用户列表失败: {:?}", e);
                LibraryError::DatabaseError(e)
            })?;

        Ok((user_list, total))
    }
}