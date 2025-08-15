use actix_web::{web, HttpResponse};
use diesel::prelude::*;
use validator::Validate;
use crate::models::{NewBook, BookResponse, ApiResponse};
use crate::error::LibraryError;
use crate::services::BookService;
use crate::db::DbPool;

pub async fn create_book(
    pool: web::Data<DbPool>,
    book_data: web::Json<NewBook>,
) -> Result<HttpResponse, LibraryError> {
    // 验证输入数据
    book_data.validate().map_err(LibraryError::ValidationError)?;

    let mut conn = pool.get().map_err(|_| LibraryError::InternalServerError)?;

    let book = web::block(move || {
        BookService::create_book(&mut conn, book_data.into_inner())
    })
    .await
    .map_err(|_| LibraryError::InternalServerError)??;

    let response = ApiResponse::success(BookResponse::from(book));
    
    Ok(HttpResponse::Created().json(response))
}

pub async fn get_books(
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, LibraryError> {
    let mut conn = pool.get().map_err(|_| LibraryError::InternalServerError)?;

    let books = web::block(move || {
        BookService::get_all_books(&mut conn)
    })
    .await
    .map_err(|_| LibraryError::InternalServerError)??;

    let book_responses: Vec<BookResponse> = books
        .into_iter()
        .map(BookResponse::from)
        .collect();

    let response = ApiResponse::success(book_responses);
    
    Ok(HttpResponse::Ok().json(response))
}

pub async fn get_book(
    pool: web::Data<DbPool>,
    book_id: web::Path<i32>,
) -> Result<HttpResponse, LibraryError> {
    let mut conn = pool.get().map_err(|_| LibraryError::InternalServerError)?;

    let book = web::block(move || {
        BookService::get_book_by_id(&mut conn, book_id.into_inner())
    })
    .await
    .map_err(|_| LibraryError::InternalServerError)??;

    let response = ApiResponse::success(BookResponse::from(book));
    
    Ok(HttpResponse::Ok().json(response))
}