use crate::models::{Book, NewBook};
use crate::error::LibraryError;
use crate::schema::books;
use diesel::prelude::*;

pub struct BookService;

impl BookService {
    pub fn create_book(conn: &mut MysqlConnection, new_book: NewBook) -> Result<Book, LibraryError> {
        // 检查ISBN是否已存在
        let existing_book = books::table
            .filter(books::isbn.eq(&new_book.isbn))
            .first::<Book>(conn)
            .optional()?;

        if existing_book.is_some() {
            return Err(LibraryError::BookAlreadyExists(new_book.isbn));
        }

        // 设置默认值
        let new_book = NewBook {
            total_copies: new_book.total_copies.or(Some(1)),
            available_copies: new_book.available_copies.or(new_book.total_copies.or(Some(1))),
            ..new_book
        };

        // 插入新图书
        diesel::insert_into(books::table)
            .values(&new_book)
            .execute(conn)?;

        // 获取刚插入的图书
        let book = books::table
            .order(books::id.desc())
            .first(conn)?;

        Ok(book)
    }

    pub fn get_all_books(conn: &mut MysqlConnection) -> Result<Vec<Book>, LibraryError> {
        let books = books::table
            .order(books::id.desc())
            .load::<Book>(conn)?;
        
        Ok(books)
    }

    pub fn get_book_by_id(conn: &mut MysqlConnection, book_id: i32) -> Result<Book, LibraryError> {
        let book = books::table
            .find(book_id)
            .first::<Book>(conn)
            .optional()?
            .ok_or(LibraryError::BookNotFound)?;
        
        Ok(book)
    }

    pub fn get_book_by_isbn(conn: &mut MysqlConnection, isbn: &str) -> Result<Book, LibraryError> {
        let book = books::table
            .filter(books::isbn.eq(isbn))
            .first::<Book>(conn)
            .optional()?
            .ok_or(LibraryError::BookNotFound)?;
        
        Ok(book)
    }
}