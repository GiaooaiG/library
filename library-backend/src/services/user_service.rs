use crate::models::{User, NewUser};
use crate::error::LibraryError;
use crate::schema::users;
use diesel::prelude::*;
use argon2::{
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use rand_core::OsRng;

pub struct UserService;

impl UserService {
    pub fn create_user(conn: &mut MysqlConnection, new_user: NewUser) -> Result<User, LibraryError> {
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

    pub fn get_user_by_username(conn: &mut MysqlConnection, username: &str) -> Result<User, LibraryError> {
        let user = users::table
            .filter(users::username.eq(username))
            .first::<User>(conn)
            .optional()?
            .ok_or(LibraryError::UserNotFound)?;

        Ok(user)
    }

    pub fn hash_password(password: &str) -> Result<String, LibraryError> {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        
        let password_hash = argon2
            .hash_password(password.as_bytes(), &salt)
            .map_err(|_| LibraryError::PasswordHashError)?
            .to_string();
        
        Ok(password_hash)
    }

    pub fn verify_password(password: &str, hash: &str) -> Result<bool, LibraryError> {
        let parsed_hash = PasswordHash::new(hash).map_err(|_| LibraryError::PasswordHashError)?;
        let argon2 = Argon2::default();
        
        match argon2.verify_password(password.as_bytes(), &parsed_hash) {
            Ok(()) => Ok(true),
            Err(_) => Ok(false),
        }
    }

    /// 获取所有用户（管理员功能）
    pub fn get_all_users(
        conn: &mut MysqlConnection,
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