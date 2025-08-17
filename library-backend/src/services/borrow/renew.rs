use crate::error::LibraryError;
use crate::models::{BorrowRecord, BorrowRecordRow, CountRow, RenewalCountRow};
use diesel::prelude::*;
use diesel::mysql::MysqlConnection;
use chrono::Duration;

/// 续借图书
pub fn renew_book(
    conn: &mut MysqlConnection,
    borrow_id_val: i32,
    user_id_val: i32,
) -> Result<BorrowRecord, LibraryError> {
    // 首先检查借阅记录是否存在且属于该用户
    let borrow_record = diesel::sql_query(
        "SELECT id, user_id, book_id, borrow_date, due_date, return_date, status, renewal_count
         FROM borrow_records
         WHERE id = ? AND user_id = ? AND status = ?"
    )
    .bind::<diesel::sql_types::Integer, _>(borrow_id_val)
    .bind::<diesel::sql_types::Integer, _>(user_id_val)
    .bind::<diesel::sql_types::Text, _>("borrowed")
    .get_result::<BorrowRecordRow>(conn)
    .map_err(|e| {
        eprintln!("获取借阅记录失败: {:?}", e);
        LibraryError::DatabaseError(e)
    })?;

    // 检查续借次数限制
    let renewal_count = borrow_record.renewal_count.unwrap_or(0);
    if renewal_count >= 1 {
        return Err(LibraryError::RenewalLimitExceeded);
    }

    // 检查图书是否被预约
    let has_reservation = check_reservation(conn, borrow_record.book_id)?;
    if has_reservation {
        return Err(LibraryError::BookReserved);
    }

    // 计算新的到期日期（延长7天）
    let new_due_date = borrow_record.due_date + Duration::days(7);

    // 更新借阅记录
    diesel::sql_query(
        "UPDATE borrow_records
         SET due_date = ?, renewal_count = renewal_count + 1
         WHERE id = ?"
    )
    .bind::<diesel::sql_types::Timestamp, _>(new_due_date)
    .bind::<diesel::sql_types::Integer, _>(borrow_id_val)
    .execute(conn)
    .map_err(|e| {
        eprintln!("更新借阅记录失败: {:?}", e);
        LibraryError::DatabaseError(e)
    })?;

    // 返回更新后的借阅记录
    let updated_record = diesel::sql_query(
        "SELECT id, user_id, book_id, borrow_date, due_date, return_date, status, renewal_count
         FROM borrow_records WHERE id = ?"
    )
    .bind::<diesel::sql_types::Integer, _>(borrow_id_val)
    .get_result::<BorrowRecordRow>(conn)
    .map_err(|e| {
        eprintln!("获取更新后的借阅记录失败: {:?}", e);
        LibraryError::DatabaseError(e)
    })?;

    Ok(BorrowRecord {
        id: updated_record.id,
        user_id: updated_record.user_id,
        book_id: updated_record.book_id,
        borrow_date: updated_record.borrow_date,
        due_date: updated_record.due_date,
        return_date: updated_record.return_date,
        status: updated_record.status,
        renewal_count: updated_record.renewal_count,
    })
}

/// 检查续借次数限制
pub fn check_renewal_limit(
    conn: &mut MysqlConnection,
    borrow_id_val: i32,
) -> Result<bool, LibraryError> {
    let record = diesel::sql_query(
        "SELECT renewal_count FROM borrow_records WHERE id = ?"
    )
    .bind::<diesel::sql_types::Integer, _>(borrow_id_val)
    .get_result::<RenewalCountRow>(conn)
    .map_err(|e| {
        eprintln!("获取续借次数失败: {:?}", e);
        LibraryError::DatabaseError(e)
    })?;

    Ok(record.renewal_count.unwrap_or(0) < 1)
}

/// 检查图书是否被预约
pub fn check_reservation(
    conn: &mut MysqlConnection,
    book_id_val: i32,
) -> Result<bool, LibraryError> {
    let count = diesel::sql_query(
        "SELECT COUNT(*) as count FROM reservations
         WHERE book_id = ? AND status = ?"
    )
    .bind::<diesel::sql_types::Integer, _>(book_id_val)
    .bind::<diesel::sql_types::Text, _>("active")
    .get_result::<CountRow>(conn)
    .map_err(|e| {
        eprintln!("检查预约状态失败: {:?}", e);
        LibraryError::DatabaseError(e)
    })?.count;

    Ok(count > 0)
}

/// 获取当前续借次数
pub fn get_renewal_count(
    conn: &mut MysqlConnection,
    borrow_id_val: i32,
) -> Result<i32, LibraryError> {
    let record = diesel::sql_query(
        "SELECT renewal_count FROM borrow_records WHERE id = ?"
    )
    .bind::<diesel::sql_types::Integer, _>(borrow_id_val)
    .get_result::<RenewalCountRow>(conn)
    .map_err(|e| {
        eprintln!("获取续借次数失败: {:?}", e);
        LibraryError::DatabaseError(e)
    })?;

    Ok(record.renewal_count.unwrap_or(0))
}