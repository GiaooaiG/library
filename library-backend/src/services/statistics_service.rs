use crate::error::LibraryError;
use diesel::mysql::MysqlConnection;
use diesel::prelude::*;
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

pub struct StatisticsService;

impl StatisticsService {
    /// 获取热门图书排行榜
    pub fn get_popular_books(
        conn: &mut MysqlConnection,
        params: &PopularBooksParams,
    ) -> Result<Vec<PopularBook>, LibraryError> {
        let limit = params.limit.unwrap_or(10).min(100);
        
        // 构建基础查询
        let mut query = r#"
            SELECT 
                b.id as book_id,
                b.title,
                b.author,
                b.category,
                b.publisher,
                b.total_copies,
                b.available_copies,
                COUNT(br.id) as borrow_count
            FROM books b
            INNER JOIN borrow_records br ON b.id = br.book_id
        "#.to_string();

        // 添加时间段筛选条件
        let mut conditions = Vec::new();
        if let Some(start_date) = &params.start_date {
            conditions.push(format!("br.borrow_date >= '{}'", start_date));
        }
        if let Some(end_date) = &params.end_date {
            conditions.push(format!("br.borrow_date <= '{}'", end_date));
        }

        // 添加WHERE条件
        if !conditions.is_empty() {
            query.push_str(" WHERE ");
            query.push_str(&conditions.join(" AND "));
        }

        // 添加分组和排序
        query.push_str(&format!(
            r#"
            GROUP BY b.id, b.title, b.author, b.category, b.publisher, b.total_copies, b.available_copies
            ORDER BY borrow_count DESC
            LIMIT {}
            "#,
            limit
        ));

        let results = diesel::sql_query(&query)
            .load::<PopularBookRow>(conn)
            .map_err(|e| {
                eprintln!("获取热门图书排行榜失败: {:?}", e);
                LibraryError::DatabaseError(e)
            })?;

        let popular_books = results
            .into_iter()
            .map(|row| PopularBook {
                book_id: row.book_id,
                title: row.title,
                author: row.author,
                category: row.category,
                publisher: row.publisher,
                borrow_count: row.borrow_count,
                total_copies: row.total_copies.unwrap_or(1),
                available_copies: row.available_copies.unwrap_or(1),
            })
            .collect();

        Ok(popular_books)
    }
}

#[derive(QueryableByName)]
struct PopularBookRow {
    #[sql_type = "diesel::sql_types::Integer"]
    book_id: i32,
    #[sql_type = "diesel::sql_types::Text"]
    title: String,
    #[sql_type = "diesel::sql_types::Text"]
    author: String,
    #[sql_type = "diesel::sql_types::Nullable<diesel::sql_types::Text>"]
    category: Option<String>,
    #[sql_type = "diesel::sql_types::Nullable<diesel::sql_types::Text>"]
    publisher: Option<String>,
    #[sql_type = "diesel::sql_types::BigInt"]
    borrow_count: i64,
    #[sql_type = "diesel::sql_types::Nullable<diesel::sql_types::Integer>"]
    total_copies: Option<i32>,
    #[sql_type = "diesel::sql_types::Nullable<diesel::sql_types::Integer>"]
    available_copies: Option<i32>,
}