use chrono::{NaiveDateTime};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationError};

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