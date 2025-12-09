pub mod create;
pub mod query;
pub mod auth;

pub use create::UserCreate;
pub use query::UserQuery;
pub use auth::UserAuth;

// 为了保持向后兼容，重新导出原来的 UserService
use crate::models::{User, NewUser};
use crate::error::LibraryError;
use diesel::pg::PgConnection;

pub struct UserService;

impl UserService {
    pub fn create_user(conn: &mut PgConnection, new_user: NewUser) -> Result<User, LibraryError> {
        UserCreate::create_user(conn, new_user)
    }

    pub fn get_user_by_username(conn: &mut PgConnection, username: &str) -> Result<User, LibraryError> {
        UserQuery::get_user_by_username(conn, username)
    }

    pub fn hash_password(password: &str) -> Result<String, LibraryError> {
        UserAuth::hash_password(password)
    }

    pub fn verify_password(password: &str, hash: &str) -> Result<bool, LibraryError> {
        UserAuth::verify_password(password, hash)
    }

    pub fn get_all_users(
        conn: &mut PgConnection,
        limit: i64,
        offset: i64,
    ) -> Result<(Vec<User>, i64), LibraryError> {
        UserQuery::get_all_users(conn, limit, offset)
    }
}