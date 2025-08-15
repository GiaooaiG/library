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
    
    #[error("内部服务器错误")]
    InternalServerError,
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
        }
    }
}