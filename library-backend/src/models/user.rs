use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use validator::Validate;


#[derive(Queryable, Serialize, Deserialize, Debug, Clone)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password_hash: String,
    pub role: Option<String>,
    pub phone: Option<String>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Insertable, Serialize, Deserialize, Validate, Debug)]
#[diesel(table_name = crate::schema::users)]
pub struct NewUser {
    #[validate(length(min = 3, max = 50, message = "用户名长度必须在3-50位之间"))]
    pub username: String,
    
    pub password_hash: String,
    
    pub phone: Option<String>,
}

#[derive(Serialize, Deserialize, Validate, Debug)]
pub struct RegisterUser {
    #[validate(length(min = 3, max = 50, message = "用户名长度必须在3-50位之间"))]
    pub username: String,
    
    #[validate(length(min = 6, message = "密码长度至少6位"))]
    pub password: String,
    
    #[validate(length(min = 11, max = 11, message = "手机号必须是11位"))]
    pub phone: Option<String>,
}

#[derive(Serialize, Deserialize, Validate, Debug)]
pub struct LoginUser {
    #[validate(length(min = 1, message = "用户名不能为空"))]
    pub username: String,
    
    #[validate(length(min = 1, message = "密码不能为空"))]
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct UserResponse {
    pub id: i32,
    pub username: String,
    pub role: Option<String>,
    pub phone: Option<String>,
    pub created_at: Option<NaiveDateTime>,
}

impl From<User> for UserResponse {
    fn from(user: User) -> Self {
        UserResponse {
            id: user.id,
            username: user.username,
            role: user.role,
            phone: user.phone,
            created_at: user.created_at,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct AuthResponse {
    pub user: UserResponse,
    pub token: String,
}

#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub username: String,
    pub role: Option<String>,
    pub exp: usize,
}