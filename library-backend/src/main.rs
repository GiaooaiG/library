mod models;
mod error;
mod db;
mod services;
mod handlers;
mod schema;
mod middleware;

use actix_web::{web, App, HttpServer};
use db::establish_connection;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    
    let pool = establish_connection();
    
    println!("🚀 图书馆管理系统后端启动成功！");
    println!("📚 服务器运行在: http://127.0.0.1:8080");
    println!("🔍 API文档: http://127.0.0.1:8080/api/v1/books");
    
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(
                web::scope("/api/v1")
                    .service(
                        web::scope("/books")
                            .route("", web::post().to(handlers::create_book))
                            .route("", web::get().to(handlers::get_books))
                            .route("/{id}", web::get().to(handlers::get_book))
                    )
                    .service(
                        web::scope("/auth")
                            .route("/register", web::post().to(handlers::register))
                            .route("/login", web::post().to(handlers::login))
                    )
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
