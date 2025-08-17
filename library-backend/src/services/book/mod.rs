pub mod create;
pub mod query;

pub use create::create_book;
pub use query::{get_all_books, get_book_by_id, get_book_by_isbn};

// 为了保持向后兼容，创建 BookService 结构体
use crate::models::{NewBook, Book};
use crate::error::LibraryError;
use diesel::mysql::MysqlConnection;

pub struct BookService;

impl BookService {
    pub fn create_book(conn: &mut MysqlConnection, new_book: NewBook) -> Result<Book, LibraryError> {
        create::create_book(conn, new_book)
    }

    pub fn get_all_books(
        conn: &mut MysqlConnection,
        params: &crate::models::PaginationParams,
    ) -> Result<(Vec<Book>, i64), LibraryError> {
        query::get_all_books(conn, params)
    }

    pub fn get_book_by_id(conn: &mut MysqlConnection, book_id: i32) -> Result<Book, LibraryError> {
        query::get_book_by_id(conn, book_id)
    }

    pub fn get_book_by_isbn(conn: &mut MysqlConnection, isbn: &str) -> Result<Option<Book>, LibraryError> {
        query::get_book_by_isbn(conn, isbn).map(Some)
    }
}