use diesel::prelude::*;
use crate::models::{Book, NewBook};
use crate::error::LibraryError;
use crate::schema::books;
use chrono::Utc;

pub fn update_book(
    conn: &mut PgConnection,
    book_id: i32,
    book_data: NewBook,
) -> Result<Book, LibraryError> {
    use crate::schema::books::dsl::*;

    let now = Utc::now().naive_utc();

    diesel::update(books.find(book_id))
        .set((
            isbn.eq(&book_data.isbn),
            title.eq(&book_data.title),
            author.eq(&book_data.author),
            category.eq(&book_data.category),
            publisher.eq(&book_data.publisher),
            total_copies.eq(&book_data.total_copies),
            available_copies.eq(&book_data.available_copies),
            updated_at.eq(now),
        ))
        .get_result::<Book>(conn)
        .map_err(|e| {
            match e {
                diesel::result::Error::NotFound => LibraryError::BookNotFound,
                _ => LibraryError::DatabaseError(e),
            }
        })
}