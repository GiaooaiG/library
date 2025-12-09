pub mod create;
pub mod query;
pub mod update;
pub mod delete;

pub use create::create_book;
pub use query::{get_all_books, get_book_by_id, get_book_by_isbn};
pub use update::update_book;
pub use delete::delete_book;

// 为了保持向后兼容，创建 BookService 结构体
use crate::models::{NewBook, Book};
use crate::error::LibraryError;
use diesel::pg::PgConnection;

pub struct BookService;

impl BookService {
    pub fn create_book(conn: &mut PgConnection, new_book: NewBook) -> Result<Book, LibraryError> {
        create::create_book(conn, new_book)
    }

    pub fn get_all_books(
        conn: &mut PgConnection,
        params: &crate::models::PaginationParams,
    ) -> Result<(Vec<Book>, i64), LibraryError> {
        query::get_all_books(conn, params)
    }

    pub fn get_book_by_id(conn: &mut PgConnection, book_id: i32) -> Result<Book, LibraryError> {
        query::get_book_by_id(conn, book_id)
    }

    pub fn get_book_by_isbn(conn: &mut PgConnection, isbn: &str) -> Result<Option<Book>, LibraryError> {
        query::get_book_by_isbn(conn, isbn).map(Some)
    }

    pub fn update_book(conn: &mut PgConnection, book_id: i32, book_data: NewBook) -> Result<Book, LibraryError> {
        update::update_book(conn, book_id, book_data)
    }

    pub fn delete_book(conn: &mut PgConnection, book_id: i32) -> Result<(), LibraryError> {
        delete::delete_book(conn, book_id)
    }
}