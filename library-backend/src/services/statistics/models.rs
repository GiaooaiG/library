use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PopularBook {
    pub book_id: i32,
    pub title: String,
    pub author: String,
    pub category: Option<String>,
    pub publisher: Option<String>,
    pub borrow_count: i64,
    pub total_copies: i32,
    pub available_copies: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PopularBooksParams {
    pub start_date: Option<String>,
    pub end_date: Option<String>,
    pub limit: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InventoryStats {
    pub total_books: i64,
    pub total_copies: i64,
    pub available_copies: i64,
    pub borrowed_copies: i64,
    pub category_stats: Vec<CategoryStats>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CategoryStats {
    pub category: Option<String>,
    pub book_count: i64,
    pub total_copies: i64,
    pub available_copies: i64,
}

#[derive(diesel::QueryableByName)]
pub struct PopularBookRow {
    #[diesel(sql_type = diesel::sql_types::Integer)]
    pub book_id: i32,
    #[diesel(sql_type = diesel::sql_types::Text)]
    pub title: String,
    #[diesel(sql_type = diesel::sql_types::Text)]
    pub author: String,
    #[diesel(sql_type = diesel::sql_types::Nullable<diesel::sql_types::Text>)]
    pub category: Option<String>,
    #[diesel(sql_type = diesel::sql_types::Nullable<diesel::sql_types::Text>)]
    pub publisher: Option<String>,
    #[diesel(sql_type = diesel::sql_types::BigInt)]
    pub borrow_count: i64,
    #[diesel(sql_type = diesel::sql_types::Nullable<diesel::sql_types::Integer>)]
    pub total_copies: Option<i32>,
    #[diesel(sql_type = diesel::sql_types::Nullable<diesel::sql_types::Integer>)]
    pub available_copies: Option<i32>,
}

#[derive(diesel::QueryableByName)]
pub struct TotalStatsRow {
    #[sql_type = "diesel::sql_types::BigInt"]
    pub total_books: i64,
    #[sql_type = "diesel::sql_types::BigInt"]
    pub total_copies: i64,
    #[sql_type = "diesel::sql_types::BigInt"]
    pub available_copies: i64,
}

#[derive(diesel::QueryableByName)]
pub struct CategoryStatsRow {
    #[sql_type = "diesel::sql_types::Nullable<diesel::sql_types::Text>"]
    pub category: Option<String>,
    #[sql_type = "diesel::sql_types::BigInt"]
    pub book_count: i64,
    #[sql_type = "diesel::sql_types::BigInt"]
    pub total_copies: i64,
    #[sql_type = "diesel::sql_types::BigInt"]
    pub available_copies: i64,
}