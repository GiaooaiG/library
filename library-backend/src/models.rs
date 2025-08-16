use chrono::{NaiveDateTime};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationError};

// 引入schema中定义的枚举类型
use crate::schema::sql_types::UsersRoleEnum;

#[derive(Debug, Serialize, Deserialize, Clone, diesel::deserialize::FromSqlRow, diesel::sql_types::SqlType)]
#[diesel(sql_type = UsersRoleEnum)]
pub enum UsersRole {
    Admin,
    User,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum BorrowStatus {
    Borrowed,
    Returned,
    Overdue,
}

impl BorrowStatus {
    pub fn to_str(&self) -> &'static str {
        match self {
            BorrowStatus::Borrowed => "borrowed",
            BorrowStatus::Returned => "returned",
            BorrowStatus::Overdue => "overdue",
        }
    }
    
    pub fn from_str(s: &str) -> Self {
        match s {
            "returned" => BorrowStatus::Returned,
            "overdue" => BorrowStatus::Overdue,
            _ => BorrowStatus::Borrowed,
        }
    }
}

impl<DB> diesel::deserialize::FromSql<UsersRoleEnum, DB> for UsersRole
where
    DB: diesel::backend::Backend,
    *const str: diesel::deserialize::FromSql<diesel::sql_types::Text, DB>,
{
    fn from_sql(bytes: DB::RawValue<'_>) -> diesel::deserialize::Result<Self> {
        let s = <*const str as diesel::deserialize::FromSql<diesel::sql_types::Text, DB>>::from_sql(bytes)?;
        let s = unsafe { &*s };
        match s {
            "admin" => Ok(UsersRole::Admin),
            "user" => Ok(UsersRole::User),
            _ => Ok(UsersRole::User),
        }
    }
}

impl<DB> diesel::serialize::ToSql<UsersRoleEnum, DB> for UsersRole
where
    DB: diesel::backend::Backend,
    str: diesel::serialize::ToSql<diesel::sql_types::Text, DB>,
{
    fn to_sql<'b>(&'b self, out: &mut diesel::serialize::Output<'b, '_, DB>) -> diesel::serialize::Result {
        let s = match self {
            UsersRole::Admin => "admin",
            UsersRole::User => "user",
        };
        s.to_sql(out)
    }
}

#[derive(Queryable, Serialize, Deserialize, Debug, Clone)]
pub struct Book {
    pub id: i32,
    pub isbn: String,
    pub title: String,
    pub author: String,
    pub category: Option<String>,
    pub publisher: Option<String>,
    pub total_copies: Option<i32>,
    pub available_copies: Option<i32>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Insertable, Serialize, Deserialize, Validate, Debug)]
#[diesel(table_name = crate::schema::books)]
pub struct NewBook {
    #[validate(length(min = 13, max = 13, message = "ISBN必须是13位数字"))]
    #[validate(custom = "validate_isbn")]
    pub isbn: String,
    
    #[validate(length(min = 1, message = "书名不能为空"))]
    pub title: String,
    
    #[validate(length(min = 1, message = "作者不能为空"))]
    pub author: String,
    
    pub category: Option<String>,
    pub publisher: Option<String>,
    
    #[validate(range(min = 1, message = "图书数量必须大于0"))]
    pub total_copies: Option<i32>,
    
    #[validate(range(min = 0, message = "可用数量不能为负数"))]
    pub available_copies: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BookResponse {
    pub id: i32,
    pub isbn: String,
    pub title: String,
    pub author: String,
    pub category: Option<String>,
    pub publisher: Option<String>,
    pub total_copies: i32,
    pub available_copies: i32,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

impl From<Book> for BookResponse {
    fn from(book: Book) -> Self {
        BookResponse {
            id: book.id,
            isbn: book.isbn,
            title: book.title,
            author: book.author,
            category: book.category,
            publisher: book.publisher,
            total_copies: book.total_copies.unwrap_or(1),
            available_copies: book.available_copies.unwrap_or(1),
            created_at: book.created_at,
            updated_at: book.updated_at,
        }
    }
}

fn validate_isbn(isbn: &str) -> Result<(), ValidationError> {
    if !isbn.chars().all(|c| c.is_ascii_digit()) {
        return Err(ValidationError::new("isbn_format"));
    }
    Ok(())
}

#[derive(Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub message: String,
    pub data: Option<T>,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        ApiResponse {
            success: true,
            message: "操作成功".to_string(),
            data: Some(data),
        }
    }

    pub fn error(message: String) -> ApiResponse<()> {
        ApiResponse {
            success: false,
            message,
            data: None,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PaginationParams {
    pub page: Option<i32>,
    pub per_page: Option<i32>,
    pub search: Option<String>,
    pub category: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct PaginatedResponse<T> {
    pub data: Vec<T>,
    pub total: i64,
    pub page: i32,
    pub per_page: i32,
    pub total_pages: i32,
}

#[derive(Queryable, Serialize, Deserialize, Debug, Clone)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password_hash: String,
    pub role: Option<UsersRole>,
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
            role: user.role.map(|r| match r {
                UsersRole::Admin => "admin".to_string(),
                UsersRole::User => "user".to_string(),
            }),
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

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub username: String,
    pub role: Option<String>,
    pub exp: usize,
}

#[derive(Queryable, Serialize, Deserialize, Debug, Clone)]
pub struct BorrowRecord {
    pub id: i32,
    pub user_id: i32,
    pub book_id: i32,
    pub borrow_date: Option<chrono::NaiveDateTime>,
    pub due_date: chrono::NaiveDateTime,
    pub return_date: Option<chrono::NaiveDateTime>,
    pub status: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NewBorrowRecord {
    pub user_id: i32,
    pub book_id: i32,
    pub due_date: chrono::NaiveDateTime,
    pub status: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BorrowRequest {
    pub book_id: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BorrowResponse {
    pub id: i32,
    pub book_id: i32,
    pub book_title: String,
    pub borrow_date: chrono::NaiveDateTime,
    pub due_date: chrono::NaiveDateTime,
    pub status: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BorrowHistoryResponse {
    pub records: Vec<BorrowResponse>,
    pub total: i64,
}