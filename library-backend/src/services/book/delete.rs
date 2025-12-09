use diesel::prelude::*;
use crate::models::Book;
use crate::error::LibraryError;
use crate::schema::books;

pub fn delete_book(conn: &mut PgConnection, book_id: i32) -> Result<(), LibraryError> {
    use crate::schema::books::dsl::*;

    // 首先检查图书是否存在
    let book_exists = books
        .find(book_id)
        .first::<Book>(conn)
        .optional()?
        .is_some();

    if !book_exists {
        return Err(LibraryError::BookNotFound);
    }

    // 执行删除操作
    diesel::delete(books.find(book_id))
        .execute(conn)
        .map_err(|e| {
            match e {
                diesel::result::Error::NotFound => LibraryError::BookNotFound,
                _ => LibraryError::DatabaseError(e),
            }
        })?;

    Ok(())
}