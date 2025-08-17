pub mod models;
pub mod query;

pub use models::*;
pub use query::StatisticsQuery;

// 为了保持向后兼容，重新导出原来的 StatisticsService
pub struct StatisticsService;

impl StatisticsService {
    /// 获取热门图书排行榜
    pub fn get_popular_books(
        conn: &mut diesel::mysql::MysqlConnection,
        params: &PopularBooksParams,
    ) -> Result<Vec<PopularBook>, crate::error::LibraryError> {
        StatisticsQuery::get_popular_books(conn, params)
    }

    /// 获取库存统计信息
    pub fn get_inventory_stats(
        conn: &mut diesel::mysql::MysqlConnection,
    ) -> Result<InventoryStats, crate::error::LibraryError> {
        StatisticsQuery::get_inventory_stats(conn)
    }
}