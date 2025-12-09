use crate::error::LibraryError;
use crate::models::{BorrowRecord, BorrowRecordRow, CountRow, RenewalCountRow};
use diesel::prelude::*;
use diesel::pg::PgConnection;
use chrono::Duration;

/// 使用事务处理机制执行续借操作
pub fn renew_book(
    conn: &mut PgConnection,
    borrow_id_val: i32,
    user_id_val: i32,
) -> Result<BorrowRecord, LibraryError> {
    // 使用事务确保操作的原子性
    conn.transaction(|conn| {
        // 1. 使用 FOR UPDATE 锁定借阅记录，防止并发修改
        let borrow_record = diesel::sql_query(
            "SELECT id, user_id, book_id, borrow_date, due_date, return_date, status, renewal_count
             FROM borrow_records
             WHERE id = $1 AND user_id = $2 AND status = $3
             FOR UPDATE"
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

        // 2. 使用 FOR UPDATE 锁定图书记录，防止并发修改
        let _locked_book = diesel::sql_query(
            "SELECT id FROM books WHERE id = $1 FOR UPDATE"
        )
        .bind::<diesel::sql_types::Integer, _>(borrow_record.book_id)
        .execute(conn)
        .map_err(|e| {
            eprintln!("锁定图书记录失败: {:?}", e);
            LibraryError::DatabaseError(e)
        })?;

        // 检查图书是否被预约
        let has_reservation = check_reservation(conn, borrow_record.book_id)?;
        if has_reservation {
            return Err(LibraryError::BookReserved);
        }

        // 计算新的到期日期（延长7天）
        let new_due_date = borrow_record.due_date + Duration::days(7);

        // 3. 更新借阅记录
        diesel::sql_query(
            "UPDATE borrow_records
             SET due_date = $1, renewal_count = renewal_count + 1
             WHERE id = $2"
        )
        .bind::<diesel::sql_types::Timestamp, _>(new_due_date)
        .bind::<diesel::sql_types::Integer, _>(borrow_id_val)
        .execute(conn)
        .map_err(|e| {
            eprintln!("更新借阅记录失败: {:?}", e);
            LibraryError::DatabaseError(e)
        })?;

        // 4. 返回更新后的借阅记录
        let updated_record = diesel::sql_query(
            "SELECT id, user_id, book_id, borrow_date, due_date, return_date, status, renewal_count
             FROM borrow_records WHERE id = $1"
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
    })
}

/// 检查续借次数限制
pub fn check_renewal_limit(
    conn: &mut PgConnection,
    borrow_id_val: i32,
) -> Result<bool, LibraryError> {
    let record = diesel::sql_query(
        "SELECT renewal_count FROM borrow_records WHERE id = $1"
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
    conn: &mut PgConnection,
    book_id_val: i32,
) -> Result<bool, LibraryError> {
    let count = diesel::sql_query(
        "SELECT COUNT(*) as count FROM reservations
         WHERE book_id = $1 AND status = $2"
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
    conn: &mut PgConnection,
    borrow_id_val: i32,
) -> Result<i32, LibraryError> {
    let record = diesel::sql_query(
        "SELECT renewal_count FROM borrow_records WHERE id = $1"
    )
    .bind::<diesel::sql_types::Integer, _>(borrow_id_val)
    .get_result::<RenewalCountRow>(conn)
    .map_err(|e| {
        eprintln!("获取续借次数失败: {:?}", e);
        LibraryError::DatabaseError(e)
    })?;

    Ok(record.renewal_count.unwrap_or(0))
}