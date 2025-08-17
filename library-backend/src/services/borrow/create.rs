use crate::error::LibraryError;
use crate::models::{BorrowRecord, BorrowRecordRow};
use diesel::prelude::*;
use diesel::mysql::MysqlConnection;
use chrono::{Duration, Utc};

/// 创建借阅记录
pub fn create_borrow_record(
    conn: &mut MysqlConnection,
    user_id_val: i32,
    book_id_val: i32,
) -> Result<BorrowRecord, LibraryError> {
    let due_date = (Utc::now() + Duration::days(7)).naive_utc();
    
    diesel::sql_query(
        "INSERT INTO borrow_records (user_id, book_id, due_date, status) VALUES (?, ?, ?, ?)"
    )
    .bind::<diesel::sql_types::Integer, _>(user_id_val)
    .bind::<diesel::sql_types::Integer, _>(book_id_val)
    .bind::<diesel::sql_types::Timestamp, _>(due_date)
    .bind::<diesel::sql_types::Text, _>("borrowed")
    .execute(conn)
    .map_err(|e| {
        eprintln!("创建借阅记录失败: {:?}", e);
        LibraryError::DatabaseError(e)
    })?;

    // 获取刚插入的记录
    let record = diesel::sql_query(
        "SELECT id, user_id, book_id, borrow_date, due_date, return_date, status, renewal_count FROM borrow_records WHERE id = LAST_INSERT_ID()"
    )
    .get_result::<BorrowRecordRow>(conn)
    .map_err(|e| {
        eprintln!("获取借阅记录失败: {:?}", e);
        LibraryError::DatabaseError(e)
    })?;

    Ok(BorrowRecord {
        id: record.id,
        user_id: record.user_id,
        book_id: record.book_id,
        borrow_date: record.borrow_date,
        due_date: record.due_date,
        return_date: record.return_date,
        status: record.status,
        renewal_count: None,
    })
}

/// 检查用户是否已经借阅了某本书
pub fn has_active_borrow(
    conn: &mut MysqlConnection,
    user_id_val: i32,
    book_id_val: i32,
) -> Result<bool, LibraryError> {
    let count = diesel::sql_query(
        "SELECT COUNT(*) as count FROM borrow_records WHERE user_id = ? AND book_id = ? AND status = ?"
    )
    .bind::<diesel::sql_types::Integer, _>(user_id_val)
    .bind::<diesel::sql_types::Integer, _>(book_id_val)
    .bind::<diesel::sql_types::Text, _>("borrowed")
    .get_result::<crate::models::CountRow>(conn)
    .map_err(|e| {
        eprintln!("检查借阅状态失败: {:?}", e);
        LibraryError::DatabaseError(e)
    })?.count;

    Ok(count > 0)
}