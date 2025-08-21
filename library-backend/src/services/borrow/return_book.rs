use crate::error::LibraryError;
use crate::models::{BorrowRecord, BorrowRecordRow};
use diesel::prelude::*;
use diesel::mysql::MysqlConnection;
use chrono::Utc;

/// 更新图书库存
pub fn update_book_stock(
    conn: &mut MysqlConnection,
    book_id_val: i32,
    decrement: bool,
) -> Result<(), LibraryError> {
    let change = if decrement { -1 } else { 1 };
    
    diesel::sql_query(
        "UPDATE books SET available_copies = COALESCE(available_copies, 0) + ? WHERE id = ?"
    )
    .bind::<diesel::sql_types::Integer, _>(change)
    .bind::<diesel::sql_types::Integer, _>(book_id_val)
    .execute(conn)
    .map_err(|e| {
        eprintln!("更新库存失败: {:?}", e);
        LibraryError::DatabaseError(e)
    })?;

    Ok(())
}

/// 还书处理
pub fn return_book(
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

    let now = Utc::now().naive_utc();

    // 更新借阅记录状态为已归还
    diesel::sql_query(
        "UPDATE borrow_records
         SET status = ?, return_date = ?
         WHERE id = ?"
    )
    .bind::<diesel::sql_types::Text, _>("returned")
    .bind::<diesel::sql_types::Timestamp, _>(now)
    .bind::<diesel::sql_types::Integer, _>(borrow_id_val)
    .execute(conn)
    .map_err(|e| {
        eprintln!("更新借阅记录失败: {:?}", e);
        LibraryError::DatabaseError(e)
    })?;

    // 更新图书库存（增加1）
    update_book_stock(conn, borrow_record.book_id, false)?;

    // 返回更新后的借阅记录
    let updated_record = BorrowRecord {
        id: borrow_record.id,
        user_id: borrow_record.user_id,
        book_id: borrow_record.book_id,
        borrow_date: borrow_record.borrow_date,
        due_date: borrow_record.due_date,
        return_date: Some(now),
        status: Some("returned".to_string()),
        renewal_count: borrow_record.renewal_count,
    };

    Ok(updated_record)
}

/// 计算是否逾期
pub fn calculate_overdue(
    due_date: chrono::NaiveDateTime,
) -> bool {
    let now = Utc::now().naive_utc();
    now > due_date
}

/// 获取逾期天数
pub fn get_overdue_days(
    due_date: chrono::NaiveDateTime,
) -> i64 {
    let now = Utc::now().naive_utc();
    if now > due_date {
        (now - due_date).num_days()
    } else {
        0
    }
}