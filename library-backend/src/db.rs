use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};
use dotenv::dotenv;
use std::env;

pub type DbPool = Pool<ConnectionManager<PgConnection>>;

pub fn establish_connection() -> DbPool {
    dotenv().ok();
    
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL 必须在 .env 文件中设置");
    
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    
    Pool::builder()
        .build(manager)
        .expect("创建数据库连接池失败")
}