use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Serialize, Deserialize, Debug, Clone)]
pub struct BorrowRecord {
    pub id: i32,
    pub user_id: i32,
    pub book_id: i32,
    pub borrow_date: Option<NaiveDateTime>,
    pub due_date: NaiveDateTime,
    pub return_date: Option<NaiveDateTime>,
    pub status: Option<String>,
    pub renewal_count: Option<i32>,
}

#[derive(Insertable, Serialize, Deserialize, Debug)]
#[diesel(table_name = crate::schema::borrow_records)]
pub struct NewBorrowRecord {
    pub user_id: i32,
    pub book_id: i32,
    pub due_date: NaiveDateTime,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BorrowRequest {
    pub book_id: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BorrowResponse {
    pub id: i32,
    pub book_id: i32,
    pub book_title: String,
    pub borrow_date: NaiveDateTime,
    pub due_date: NaiveDateTime,
    pub status: String,
    pub renewal_count: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BorrowHistoryResponse {
    pub records: Vec<BorrowResponse>,
    pub total: i64,
}

// 用于数据库查询的辅助结构体
#[derive(QueryableByName)]
pub struct BorrowRecordRow {
    #[diesel(sql_type = diesel::sql_types::Integer)]
    pub id: i32,
    #[diesel(sql_type = diesel::sql_types::Integer)]
    pub user_id: i32,
    #[diesel(sql_type = diesel::sql_types::Integer)]
    pub book_id: i32,
    #[diesel(sql_type = diesel::sql_types::Nullable<diesel::sql_types::Timestamp>)]
    pub borrow_date: Option<NaiveDateTime>,
    #[diesel(sql_type = diesel::sql_types::Timestamp)]
    pub due_date: NaiveDateTime,
    #[diesel(sql_type = diesel::sql_types::Nullable<diesel::sql_types::Timestamp>)]
    pub return_date: Option<NaiveDateTime>,
    #[diesel(sql_type = diesel::sql_types::Nullable<diesel::sql_types::Text>)]
    pub status: Option<String>,
    #[diesel(sql_type = diesel::sql_types::Nullable<diesel::sql_types::Integer>)]
    pub renewal_count: Option<i32>,
}

#[derive(QueryableByName)]
pub struct BorrowHistoryRow {
    #[diesel(sql_type = diesel::sql_types::Integer)]
    pub id: i32,
    #[diesel(sql_type = diesel::sql_types::Integer)]
    pub book_id: i32,
    #[diesel(sql_type = diesel::sql_types::Text)]
    pub book_title: String,
    #[diesel(sql_type = diesel::sql_types::Nullable<diesel::sql_types::Timestamp>)]
    pub borrow_date: Option<NaiveDateTime>,
    #[diesel(sql_type = diesel::sql_types::Timestamp)]
    pub due_date: NaiveDateTime,
    #[diesel(sql_type = diesel::sql_types::Nullable<diesel::sql_types::Text>)]
    pub status: Option<String>,
    #[diesel(sql_type = diesel::sql_types::Nullable<diesel::sql_types::Integer>)]
    pub renewal_count: Option<i32>,
}