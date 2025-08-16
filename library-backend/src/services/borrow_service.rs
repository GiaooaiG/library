use crate::error::LibraryError;
use crate::models::{BorrowRecord, BorrowResponse};
use diesel::prelude::*;
use diesel::mysql::MysqlConnection;
use chrono::{Duration, Utc};

pub struct BorrowService;

impl BorrowService {
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
        .get_result::<CountRow>(conn)
        .map_err(|e| {
            eprintln!("检查借阅状态失败: {:?}", e);
            LibraryError::DatabaseError(e)
        })?.count;

        Ok(count > 0)
    }

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
                renewal_count: None,
            })
            .collect();

        Ok(responses)
    }

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

    /// 还书处理
    pub fn return_book(
        conn: &mut MysqlConnection,
        borrow_id_val: i32,
        user_id_val: i32,
    ) -> Result<BorrowRecord, LibraryError> {
        // 首先检查借阅记录是否存在且属于该用户
        let borrow_record = diesel::sql_query(
            "SELECT id, user_id, book_id, borrow_date, due_date, return_date, status
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
        Self::update_book_stock(conn, borrow_record.book_id, false)?;

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
        let has_reservation = Self::check_reservation(conn, borrow_record.book_id)?;
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
}

#[derive(QueryableByName)]
struct BorrowRecordRow {
    #[sql_type = "diesel::sql_types::Integer"]
    id: i32,
    #[sql_type = "diesel::sql_types::Integer"]
    user_id: i32,
    #[sql_type = "diesel::sql_types::Integer"]
    book_id: i32,
    #[sql_type = "diesel::sql_types::Nullable<diesel::sql_types::Timestamp>"]
    borrow_date: Option<chrono::NaiveDateTime>,
    #[sql_type = "diesel::sql_types::Timestamp"]
    due_date: chrono::NaiveDateTime,
    #[sql_type = "diesel::sql_types::Nullable<diesel::sql_types::Timestamp>"]
    return_date: Option<chrono::NaiveDateTime>,
    #[sql_type = "diesel::sql_types::Nullable<diesel::sql_types::Text>"]
    status: Option<String>,
    #[sql_type = "diesel::sql_types::Nullable<diesel::sql_types::Integer>"]
    renewal_count: Option<i32>,
}

#[derive(QueryableByName)]
struct CountRow {
    #[sql_type = "diesel::sql_types::BigInt"]
    count: i64,
}

#[derive(QueryableByName)]
struct BorrowHistoryRow {
    #[sql_type = "diesel::sql_types::Integer"]
    id: i32,
    #[sql_type = "diesel::sql_types::Integer"]
    book_id: i32,
    #[sql_type = "diesel::sql_types::Text"]
    book_title: String,
    #[sql_type = "diesel::sql_types::Nullable<diesel::sql_types::Timestamp>"]
    borrow_date: Option<chrono::NaiveDateTime>,
    #[sql_type = "diesel::sql_types::Timestamp"]
    due_date: chrono::NaiveDateTime,
    #[sql_type = "diesel::sql_types::Nullable<diesel::sql_types::Text>"]
    status: Option<String>,
    #[sql_type = "diesel::sql_types::Nullable<diesel::sql_types::Integer>"]
    renewal_count: Option<i32>,
}

#[derive(QueryableByName)]
struct AvailableRow {
    #[sql_type = "diesel::sql_types::Integer"]
    available: i32,
}

#[derive(QueryableByName)]
struct RenewalCountRow {
    #[sql_type = "diesel::sql_types::Nullable<diesel::sql_types::Integer>"]
    renewal_count: Option<i32>,
}