use crate::error::LibraryError;
use crate::models::{BorrowRecord, BorrowRecordRow};
use diesel::prelude::*;
use diesel::pg::PgConnection;
use diesel::{Connection, RunQueryDsl};
use chrono::Utc;

/// 使用事务处理机制执行归还图书操作
pub fn return_book(
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

        // 2. 使用 FOR UPDATE 锁定对应的图书记录，防止并发修改库存
        let _locked_book = diesel::sql_query(
            "SELECT id FROM books WHERE id = $1 FOR UPDATE"
        )
        .bind::<diesel::sql_types::Integer, _>(borrow_record.book_id)
        .execute(conn)
        .map_err(|e| {
            eprintln!("锁定图书记录失败: {:?}", e);
            LibraryError::DatabaseError(e)
        })?;

        let now = Utc::now().naive_utc();

        // 3. 更新借阅记录状态为已归还
        let rows_affected = diesel::sql_query(
            "UPDATE borrow_records
             SET status = $1, return_date = $2
             WHERE id = $3 AND status = $4"
        )
        .bind::<diesel::sql_types::Text, _>("returned")
        .bind::<diesel::sql_types::Timestamp, _>(now)
        .bind::<diesel::sql_types::Integer, _>(borrow_id_val)
        .bind::<diesel::sql_types::Text, _>("borrowed")
        .execute(conn)
        .map_err(|e| {
            eprintln!("更新借阅记录失败: {:?}", e);
            LibraryError::DatabaseError(e)
        })?;

        // 确保更新成功
        if rows_affected == 0 {
            return Err(LibraryError::DatabaseError(
                diesel::result::Error::NotFound
            ));
        }

        // 4. 更新图书库存（增加1）
        let stock_rows_affected = diesel::sql_query(
            "UPDATE books 
             SET available_copies = COALESCE(available_copies, 0) + 1 
             WHERE id = $1"
        )
        .bind::<diesel::sql_types::Integer, _>(borrow_record.book_id)
        .execute(conn)
        .map_err(|e| {
            eprintln!("更新库存失败: {:?}", e);
            LibraryError::DatabaseError(e)
        })?;

        // 确保库存更新成功
        if stock_rows_affected == 0 {
            return Err(LibraryError::DatabaseError(
                diesel::result::Error::NotFound
            ));
        }

        // 5. 返回更新后的借阅记录
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
    })
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

/// 更新图书库存
pub fn update_book_stock(
    conn: &mut PgConnection,
    book_id: i32,
    decrement: bool,
) -> Result<(), LibraryError> {
    let change = if decrement { -1 } else { 1 };
    
    diesel::sql_query(
        "UPDATE books
         SET available_copies = COALESCE(available_copies, 0) + $1
         WHERE id = $2"
    )
    .bind::<diesel::sql_types::Integer, _>(change)
    .bind::<diesel::sql_types::Integer, _>(book_id)
    .execute(conn)
    .map_err(|e| {
        eprintln!("更新库存失败: {:?}", e);
        LibraryError::DatabaseError(e)
    })?;

    Ok(())
}