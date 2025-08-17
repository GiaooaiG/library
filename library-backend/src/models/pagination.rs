use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct PaginationParams {
    pub page: Option<i32>,
    pub per_page: Option<i32>,
    pub search: Option<String>,
    pub category: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct PaginatedResponse<T> {
    pub data: Vec<T>,
    pub total: i64,
    pub page: i32,
    pub per_page: i32,
    pub total_pages: i32,
}