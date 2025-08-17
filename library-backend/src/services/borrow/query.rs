use crate::error::LibraryError;
use crate::models::{BorrowResponse, BorrowHistoryRow, CountRow, AvailableRow};
use diesel::prelude::*;
use diesel::mysql::MysqlConnection;
use chrono::Utc;

/// 获取用户的借阅历史
pub fn get_user_borrow_history(
    conn: &mut MysqlConnection,
    user_id_val: i32,
) -> Result<Vec<BorrowResponse>, LibraryError> {
    let results = diesel::sql_query(
        r#"
        SELECT br.id, br.book_id, b.title as book_title, br.borrow_date, br.due_date, br.status, br.renewal_count
        FROM borrow_records br
        INNER JOIN books b ON br.book_id = b.id
        WHERE br.user_id = ?
        ORDER BY br.borrow_date DESC
        "#
    )
    .bind::<diesel::sql_types::Integer, _>(user_id_val)
    .load::<BorrowHistoryRow>(conn)
    .map_err(|e| {
        eprintln!("获取借阅历史失败: {:?}", e);
        LibraryError::DatabaseError(e)
    })?;

    let responses = results
        .into_iter()
        .map(|row| BorrowResponse {
            id: row.id,
            book_id: row.book_id,
            book_title: row.book_title,
            borrow_date: row.borrow_date.unwrap_or_else(|| Utc::now().naive_utc()),
            due_date: row.due_date,
            status: row.status.unwrap_or_else(|| "borrowed".to_string()),
            renewal_count: row.renewal_count,
        })
        .collect();

    Ok(responses)
}

/// 获取图书当前库存
pub fn get_book_available_copies(
    conn: &mut MysqlConnection,
    book_id_val: i32,
) -> Result<i32, LibraryError> {
    let available = diesel::sql_query(
        "SELECT COALESCE(available_copies, 0) as available FROM books WHERE id = ?"
    )
    .bind::<diesel::sql_types::Integer, _>(book_id_val)
    .get_result::<AvailableRow>(conn)
    .map_err(|e| {
        eprintln!("获取图书信息失败: {:?}", e);
        LibraryError::DatabaseError(e)
    })?.available;

    Ok(available)
}

/// 获取所有用户的借阅记录（管理员功能）
pub fn get_all_borrow_records(
    conn: &mut MysqlConnection,
    limit: i64,
    offset: i64,
) -> Result<(Vec<BorrowResponse>, i64), LibraryError> {
    // 获取总记录数
    let total = diesel::sql_query(
        "SELECT COUNT(*) as count FROM borrow_records"
    )
    .get_result::<CountRow>(conn)
    .map_err(|e| {
        eprintln!("获取借阅记录总数失败: {:?}", e);
        LibraryError::DatabaseError(e)
    })?.count;

    // 获取分页记录
    let results = diesel::sql_query(
        r#"
        SELECT br.id, br.book_id, b.title as book_title, br.borrow_date, br.due_date, br.status, br.renewal_count
        FROM borrow_records br
        INNER JOIN books b ON br.book_id = b.id
        ORDER BY br.borrow_date DESC
        LIMIT ? OFFSET ?
        "#
    )
    .bind::<diesel::sql_types::BigInt, _>(limit)
    .bind::<diesel::sql_types::BigInt, _>(offset)
    .load::<BorrowHistoryRow>(conn)
    .map_err(|e| {
        eprintln!("获取所有借阅记录失败: {:?}", e);
        LibraryError::DatabaseError(e)
    })?;

    let responses = results
        .into_iter()
        .map(|row| BorrowResponse {
            id: row.id,
            book_id: row.book_id,
            book_title: row.book_title,
            borrow_date: row.borrow_date.unwrap_or_else(|| Utc::now().naive_utc()),
            due_date: row.due_date,
            status: row.status.unwrap_or_else(|| "borrowed".to_string()),
            renewal_count: row.renewal_count,
        })
        .collect();

    Ok((responses, total))
}