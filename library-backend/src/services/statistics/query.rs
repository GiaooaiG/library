use crate::error::LibraryError;
use diesel::mysql::MysqlConnection;
use diesel::prelude::*;
use super::models::*;

pub struct StatisticsQuery;

impl StatisticsQuery {
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

    /// 获取库存统计信息
    pub fn get_inventory_stats(conn: &mut MysqlConnection) -> Result<InventoryStats, LibraryError> {
        // 获取总体统计
        let total_query = r#"
            SELECT
                COUNT(*) as total_books,
                COALESCE(SUM(total_copies), 0) as total_copies,
                COALESCE(SUM(available_copies), 0) as available_copies
            FROM books
        "#;
        
        let total_result = diesel::sql_query(total_query)
            .load::<TotalStatsRow>(conn)
            .map_err(|e| {
                eprintln!("获取总体库存统计失败: {:?}", e);
                LibraryError::DatabaseError(e)
            })?;
        
        let total_stats = total_result.into_iter().next().unwrap_or(TotalStatsRow {
            total_books: 0,
            total_copies: 0,
            available_copies: 0,
        });
        
        // 获取按分类统计
        let category_query = r#"
            SELECT
                category,
                COUNT(*) as book_count,
                COALESCE(SUM(total_copies), 0) as total_copies,
                COALESCE(SUM(available_copies), 0) as available_copies
            FROM books
            GROUP BY category
            ORDER BY book_count DESC
        "#;
        
        let category_results = diesel::sql_query(category_query)
            .load::<CategoryStatsRow>(conn)
            .map_err(|e| {
                eprintln!("获取分类库存统计失败: {:?}", e);
                LibraryError::DatabaseError(e)
            })?;
        
        let category_stats = category_results
            .into_iter()
            .map(|row| CategoryStats {
                category: row.category,
                book_count: row.book_count,
                total_copies: row.total_copies,
                available_copies: row.available_copies,
            })
            .collect();
        
        let borrowed_copies = total_stats.total_copies - total_stats.available_copies;
        
        Ok(InventoryStats {
            total_books: total_stats.total_books,
            total_copies: total_stats.total_copies,
            available_copies: total_stats.available_copies,
            borrowed_copies,
            category_stats,
        })
    }
}