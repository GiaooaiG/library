pub mod create;
pub mod query;
pub mod return_book;
pub mod renew;

pub use create::{create_borrow_record, has_active_borrow};
pub use query::{get_user_borrow_history, get_book_available_copies, get_all_borrow_records};
pub use return_book::{return_book, update_book_stock, calculate_overdue, get_overdue_days};
pub use renew::{renew_book, check_renewal_limit, check_reservation, get_renewal_count};

use crate::error::LibraryError;
use crate::models::{BorrowRecord, BorrowResponse};
use diesel::pg::PgConnection;

pub struct BorrowService;

impl BorrowService {
    /// 创建借阅记录
    pub fn create_borrow_record(
        conn: &mut PgConnection,
        user_id: i32,
        book_id: i32,
    ) -> Result<BorrowRecord, LibraryError> {
        create::create_borrow_record(conn, user_id, book_id)
    }

    /// 检查用户是否已经借阅了某本书
    pub fn has_active_borrow(
        conn: &mut PgConnection,
        user_id: i32,
        book_id: i32,
    ) -> Result<bool, LibraryError> {
        create::has_active_borrow(conn, user_id, book_id)
    }

    /// 获取用户的借阅历史
    pub fn get_user_borrow_history(
        conn: &mut PgConnection,
        user_id: i32,
    ) -> Result<Vec<BorrowResponse>, LibraryError> {
        query::get_user_borrow_history(conn, user_id)
    }

    /// 更新图书库存
    pub fn update_book_stock(
        conn: &mut PgConnection,
        book_id: i32,
        decrement: bool,
    ) -> Result<(), LibraryError> {
        return_book::update_book_stock(conn, book_id, decrement)
    }

    /// 获取图书当前库存
    pub fn get_book_available_copies(
        conn: &mut PgConnection,
        book_id: i32,
    ) -> Result<i32, LibraryError> {
        query::get_book_available_copies(conn, book_id)
    }

    /// 还书处理
    pub fn return_book(
        conn: &mut PgConnection,
        borrow_id: i32,
        user_id: i32,
    ) -> Result<BorrowRecord, LibraryError> {
        return_book::return_book(conn, borrow_id, user_id)
    }

    /// 计算是否逾期
    pub fn calculate_overdue(
        due_date: chrono::NaiveDateTime,
    ) -> bool {
        return_book::calculate_overdue(due_date)
    }

    /// 获取逾期天数
    pub fn get_overdue_days(
        due_date: chrono::NaiveDateTime,
    ) -> i64 {
        return_book::get_overdue_days(due_date)
    }

    /// 续借图书
    pub fn renew_book(
        conn: &mut PgConnection,
        borrow_id: i32,
        user_id: i32,
    ) -> Result<BorrowRecord, LibraryError> {
        renew::renew_book(conn, borrow_id, user_id)
    }

    /// 检查续借次数限制
    pub fn check_renewal_limit(
        conn: &mut PgConnection,
        borrow_id: i32,
    ) -> Result<bool, LibraryError> {
        renew::check_renewal_limit(conn, borrow_id)
    }

    /// 检查图书是否被预约
    pub fn check_reservation(
        conn: &mut PgConnection,
        book_id: i32,
    ) -> Result<bool, LibraryError> {
        renew::check_reservation(conn, book_id)
    }

    /// 获取当前续借次数
    pub fn get_renewal_count(
        conn: &mut PgConnection,
        borrow_id: i32,
    ) -> Result<i32, LibraryError> {
        renew::get_renewal_count(conn, borrow_id)
    }

    /// 获取所有用户的借阅记录（管理员功能）
    pub fn get_all_borrow_records(
        conn: &mut PgConnection,
        limit: i64,
        offset: i64,
    ) -> Result<(Vec<BorrowResponse>, i64), LibraryError> {
        query::get_all_borrow_records(conn, limit, offset)
    }
}