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
    println!("🔍 书籍列表API: http://127.0.0.1:8080/api/v1/books");
    
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
                    .service(
                        web::scope("/borrow")
                            .route("", web::post().to(handlers::borrow_book))
                            .route("/history", web::get().to(handlers::get_borrow_history))
                    )
                    .service(
                        web::scope("/return")
                            .route("/{id}", web::post().to(handlers::return_book))
                    )
                    .service(
                        web::scope("/renew")
                            .route("/{id}", web::post().to(handlers::renew_book))
                    )
                    .service(
                        web::scope("/statistics")
                            .route("/popular-books", web::get().to(handlers::get_popular_books))
                            .route("/inventory", web::get().to(handlers::get_inventory_stats))
                    )
                    .service(
                        web::scope("/admin")
                            .route("/users", web::get().to(handlers::get_all_users))
                            .route("/borrow-records", web::get().to(handlers::get_all_borrow_records))
                            .route("/users/{id}/borrow-records", web::get().to(handlers::get_user_borrow_records))
                    )
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
