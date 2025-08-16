use crate::models::{Book, NewBook, User, NewUser};
use crate::error::LibraryError;
use crate::schema::{books, users};
use diesel::prelude::*;
use argon2::{
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use rand_core::OsRng;

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

    pub fn get_all_books(
        conn: &mut MysqlConnection,
        params: &crate::models::PaginationParams,
    ) -> Result<(Vec<Book>, i64), LibraryError> {
        use crate::schema::books::dsl::*;
        
        let per_page = params.per_page.unwrap_or(20).min(100);
        let page = params.page.unwrap_or(1).max(1);
        let offset = (page - 1) * per_page;
        
        // 构建查询条件
        let mut query = books.into_boxed();
        
        // 搜索功能
        if let Some(search_term) = &params.search {
            let search_pattern = format!("%{}%", search_term);
            query = query.filter(
                title.like(search_pattern.clone())
                    .or(author.like(search_pattern.clone()))
                    .or(isbn.like(search_pattern))
            );
        }
        
        // 分类筛选
        if let Some(category_filter) = &params.category {
            let category_pattern = format!("%{}%", category_filter);
            query = query.filter(category.like(category_pattern));
        }
        
        // 获取总数 - 重新构建查询
        let mut total_query = books.into_boxed();
        
        if let Some(search_term) = &params.search {
            let search_pattern = format!("%{}%", search_term);
            total_query = total_query.filter(
                title.like(search_pattern.clone())
                    .or(author.like(search_pattern.clone()))
                    .or(isbn.like(search_pattern))
            );
        }
        
        if let Some(category_filter) = &params.category {
            let category_pattern = format!("%{}%", category_filter);
            total_query = total_query.filter(category.like(category_pattern));
        }
        
        let total = total_query.count().get_result::<i64>(conn)?;
        
        // 获取分页数据
        let results = query
            .order(id.desc())
            .limit(per_page as i64)
            .offset(offset as i64)
            .load::<Book>(conn)?;
        
        Ok((results, total))
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

pub struct UserService;

impl UserService {
    pub fn create_user(conn: &mut MysqlConnection, new_user: NewUser) -> Result<User, LibraryError> {
        // 检查用户名是否已存在
        let existing_user = users::table
            .filter(users::username.eq(&new_user.username))
            .first::<User>(conn)
            .optional()?;

        if existing_user.is_some() {
            return Err(LibraryError::UserAlreadyExists(new_user.username));
        }

        // 创建用户
        diesel::insert_into(users::table)
            .values(&new_user)
            .execute(conn)?;

        // 获取刚创建的用户
        let user = users::table
            .order(users::id.desc())
            .first(conn)?;

        Ok(user)
    }

    pub fn get_user_by_username(conn: &mut MysqlConnection, username: &str) -> Result<User, LibraryError> {
        let user = users::table
            .filter(users::username.eq(username))
            .first::<User>(conn)
            .optional()?
            .ok_or(LibraryError::UserNotFound)?;

        Ok(user)
    }

    pub fn hash_password(password: &str) -> Result<String, LibraryError> {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        
        let password_hash = argon2
            .hash_password(password.as_bytes(), &salt)
            .map_err(|_| LibraryError::PasswordHashError)?
            .to_string();
        
        Ok(password_hash)
    }

    pub fn verify_password(password: &str, hash: &str) -> Result<bool, LibraryError> {
        let parsed_hash = PasswordHash::new(hash).map_err(|_| LibraryError::PasswordHashError)?;
        let argon2 = Argon2::default();
        
        match argon2.verify_password(password.as_bytes(), &parsed_hash) {
            Ok(()) => Ok(true),
            Err(_) => Ok(false),
        }
    }
}