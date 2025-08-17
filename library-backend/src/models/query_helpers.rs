use diesel::QueryableByName;

#[derive(QueryableByName)]
pub struct CountRow {
    #[diesel(sql_type = diesel::sql_types::BigInt)]
    pub count: i64,
}

#[derive(QueryableByName)]
pub struct AvailableRow {
    #[diesel(sql_type = diesel::sql_types::Integer)]
    pub available: i32,
}

#[derive(QueryableByName)]
pub struct RenewalCountRow {
    #[diesel(sql_type = diesel::sql_types::Nullable<diesel::sql_types::Integer>)]
    pub renewal_count: Option<i32>,
}