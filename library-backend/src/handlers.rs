use actix_web::{web, HttpResponse};
use diesel::prelude::*;
use validator::Validate;
use crate::models::{NewBook, BookResponse, ApiResponse, PaginationParams, PaginatedResponse};
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
    query: web::Query<PaginationParams>,
) -> Result<HttpResponse, LibraryError> {
    let mut conn = pool.get().map_err(|_| LibraryError::InternalServerError)?;

    let params = query.into_inner();
    let per_page = params.per_page.unwrap_or(20).min(100);
    let page = params.page.unwrap_or(1).max(1);
    
    let (books, total) = web::block(move || {
        BookService::get_all_books(&mut conn, &params)
    })
    .await
    .map_err(|_| LibraryError::InternalServerError)??;

    let book_responses: Vec<BookResponse> = books
        .into_iter()
        .map(BookResponse::from)
        .collect();

    let total_pages = ((total as f64) / (per_page as f64)).ceil() as i32;

    let response = ApiResponse::success(PaginatedResponse {
        data: book_responses,
        total,
        page,
        per_page,
        total_pages,
    });
    
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