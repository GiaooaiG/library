use actix_web::{HttpResponse, ResponseError};
use diesel::result::{DatabaseErrorKind, Error as DieselError};
use thiserror::Error;
use validator::ValidationErrors;

#[derive(Error, Debug)]
pub enum LibraryError {
    #[error("数据库错误: {0}")]
    DatabaseError(#[from] DieselError),
    
    #[error("验证错误: {0}")]
    ValidationError(#[from] ValidationErrors),
    
    #[error("图书已存在: {0}")]
    BookAlreadyExists(String),
    
    #[error("图书未找到")]
    BookNotFound,
    
    #[error("用户已存在: {0}")]
    UserAlreadyExists(String),
    
    #[error("用户未找到")]
    UserNotFound,
    
    #[error("密码错误")]
    InvalidPassword,
    
    #[error("密码哈希错误")]
    PasswordHashError,
    
    #[error("内部服务器错误")]
    InternalServerError,

    #[error("续借次数已达上限")]
    RenewalLimitExceeded,

    #[error("未授权访问")]
    Unauthorized,

    #[error("图书当前不可用")]
    BookNotAvailable,
}

impl ResponseError for LibraryError {
    fn error_response(&self) -> HttpResponse {
        match self {
            LibraryError::DatabaseError(e) => {
                match e {
                    DieselError::DatabaseError(DatabaseErrorKind::UniqueViolation, _) => {
                        HttpResponse::Conflict().json(serde_json::json!({
                            "success": false,
                            "message": "ISBN已存在"
                        }))
                    }
                    _ => HttpResponse::InternalServerError().json(serde_json::json!({
                        "success": false,
                        "message": "数据库错误"
                    }))
                }
            }
            LibraryError::ValidationError(e) => {
                HttpResponse::BadRequest().json(serde_json::json!({
                    "success": false,
                    "message": "输入数据验证失败",
                    "errors": e.field_errors()
                }))
            }
            LibraryError::BookAlreadyExists(isbn) => {
                HttpResponse::Conflict().json(serde_json::json!({
                    "success": false,
                    "message": format!("ISBN {} 已存在", isbn)
                }))
            }
            LibraryError::BookNotFound => {
                HttpResponse::NotFound().json(serde_json::json!({
                    "success": false,
                    "message": "图书未找到"
                }))
            }
            LibraryError::InternalServerError => {
                HttpResponse::InternalServerError().json(serde_json::json!({
                    "success": false,
                    "message": "内部服务器错误"
                }))
            }
            LibraryError::UserAlreadyExists(username) => {
                HttpResponse::Conflict().json(serde_json::json!({
                    "success": false,
                    "message": format!("用户名 {} 已存在", username)
                }))
            }
            LibraryError::UserNotFound => {
                HttpResponse::NotFound().json(serde_json::json!({
                    "success": false,
                    "message": "用户未找到"
                }))
            }
            LibraryError::InvalidPassword => {
                HttpResponse::Unauthorized().json(serde_json::json!({
                    "success": false,
                    "message": "密码错误"
                }))
            }
            LibraryError::PasswordHashError => {
                HttpResponse::InternalServerError().json(serde_json::json!({
                    "success": false,
                    "message": "密码处理错误"
                }))
            }
            LibraryError::RenewalLimitExceeded => {
                HttpResponse::BadRequest().json(serde_json::json!({
                    "success": false,
                    "message": "续借次数已达上限"
                }))
            }
            LibraryError::Unauthorized => {
                HttpResponse::Unauthorized().json(serde_json::json!({
                    "success": false,
                    "message": "未授权访问"
                }))
            }
            LibraryError::BookNotAvailable => {
                HttpResponse::BadRequest().json(serde_json::json!({
                    "success": false,
                    "message": "图书当前不可用"
                }))
            }
        }
    }
}