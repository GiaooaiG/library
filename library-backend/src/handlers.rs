use actix_web::{web, HttpResponse};
use validator::Validate;
use crate::models::{NewBook, BookResponse, ApiResponse, PaginationParams, PaginatedResponse, NewUser, LoginUser, UserResponse, AuthResponse, RegisterUser, BorrowRequest, BorrowResponse};
use crate::services::statistics::{PopularBooksParams, InventoryStats};
use crate::error::LibraryError;
use crate::services::{BookService, UserService, BorrowService, statistics::StatisticsService};
use crate::db::DbPool;
use crate::middleware::{Claims, check_admin_role, check_user_ownership, is_admin};
use jsonwebtoken::{encode, EncodingKey, Header};
use std::env;

pub async fn create_book(
    pool: web::Data<DbPool>,
    book_data: web::Json<NewBook>,
    claims: Claims,
) -> Result<HttpResponse, LibraryError> {
    // 检查管理员权限
    check_admin_role(&claims).map_err(|_| LibraryError::Unauthorized)?;
    
    // 验证输入数据
    book_data.validate().map_err(LibraryError::ValidationError)?;

    let mut conn = pool.get().map_err(|_| LibraryError::InternalServerError)?;

    let book = web::block(move || {
        BookService::create_book(&mut conn, book_data.into_inner())
    })
    .await
    .map_err(|_| LibraryError::InternalServerError)??;

    let response = ApiResponse::success(BookResponse::from(book));
    
    Ok(HttpResponse::Created().json(response))
}

pub async fn get_books(
    pool: web::Data<DbPool>,
    query: web::Query<PaginationParams>,
) -> Result<HttpResponse, LibraryError> {
    let mut conn = pool.get().map_err(|_| LibraryError::InternalServerError)?;

    let params = query.into_inner();
    let per_page = params.per_page.unwrap_or(20).min(100);
    let page = params.page.unwrap_or(1).max(1);
    
    let (books, total) = web::block(move || {
        BookService::get_all_books(&mut conn, &params)
    })
    .await
    .map_err(|_| LibraryError::InternalServerError)??;

    let book_responses: Vec<BookResponse> = books
        .into_iter()
        .map(BookResponse::from)
        .collect();

    let total_pages = ((total as f64) / (per_page as f64)).ceil() as i32;

    let response = ApiResponse::success(PaginatedResponse {
        data: book_responses,
        total,
        page,
        per_page,
        total_pages,
    });
    
    Ok(HttpResponse::Ok().json(response))
}

pub async fn get_book(
    pool: web::Data<DbPool>,
    book_id: web::Path<i32>,
) -> Result<HttpResponse, LibraryError> {
    let mut conn = pool.get().map_err(|_| LibraryError::InternalServerError)?;

    let book = web::block(move || {
        BookService::get_book_by_id(&mut conn, book_id.into_inner())
    })
    .await
    .map_err(|_| LibraryError::InternalServerError)??;

    let response = ApiResponse::success(BookResponse::from(book));
    
    Ok(HttpResponse::Ok().json(response))
}

pub async fn register(
    pool: web::Data<DbPool>,
    user_data: web::Json<RegisterUser>,
) -> Result<HttpResponse, LibraryError> {
    // 验证输入数据
    user_data.validate().map_err(LibraryError::ValidationError)?;

    let mut conn = pool.get().map_err(|_| LibraryError::InternalServerError)?;

    let user_data = user_data.into_inner();
    
    // 哈希密码
    let password_hash = web::block(move || {
        UserService::hash_password(&user_data.password)
    })
    .await
    .map_err(|_| LibraryError::InternalServerError)??;

    // 创建新用户
    let new_user = NewUser {
        username: user_data.username,
        password_hash,
        phone: user_data.phone,
    };

    let user = web::block(move || {
        UserService::create_user(&mut conn, new_user)
    })
    .await
    .map_err(|_| LibraryError::InternalServerError)??;

    let user_id = user.id;
    let username = user.username.clone();
    let role = user.role.clone();
    
    // 生成JWT令牌
    let secret = env::var("JWT_SECRET").unwrap_or_else(|_| "secret_key".to_string());
    let expiration = chrono::Utc::now()
        .checked_add_signed(chrono::Duration::days(7))
        .expect("valid timestamp")
        .timestamp() as usize;

    let claims = Claims {
        sub: user_id.to_string(),
        username: username.clone(),
        role: role.clone(),
        exp: expiration,
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_ref()),
    ).map_err(|_| LibraryError::InternalServerError)?;

    let response = ApiResponse::success(AuthResponse {
        user: UserResponse {
            id: user_id,
            username,
            role,
            phone: user.phone,
            created_at: user.created_at,
        },
        token,
    });

    Ok(HttpResponse::Created().json(response))
}

pub async fn login(
    pool: web::Data<DbPool>,
    login_data: web::Json<LoginUser>,
) -> Result<HttpResponse, LibraryError> {
    // 验证输入数据
    login_data.validate().map_err(LibraryError::ValidationError)?;

    let mut conn = pool.get().map_err(|_| LibraryError::InternalServerError)?;

    let login_data = login_data.into_inner();
    
    // 获取用户
    let user = web::block(move || {
        UserService::get_user_by_username(&mut conn, &login_data.username)
    })
    .await
    .map_err(|_| LibraryError::InternalServerError)??;

    // 验证密码
    let is_valid = web::block(move || {
        UserService::verify_password(&login_data.password, &user.password_hash)
    })
    .await
    .map_err(|_| LibraryError::InternalServerError)??;

    if !is_valid {
        return Err(LibraryError::InvalidPassword);
    }

    let user_id = user.id;
    let username = user.username.clone();
    let role = user.role.clone();
    
    // 生成JWT令牌
    let secret = env::var("JWT_SECRET").unwrap_or_else(|_| "secret_key".to_string());
    let expiration = chrono::Utc::now()
        .checked_add_signed(chrono::Duration::days(7))
        .expect("valid timestamp")
        .timestamp() as usize;

    let claims = Claims {
        sub: user_id.to_string(),
        username: username.clone(),
        role: role.clone(),
        exp: expiration,
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_ref()),
    ).map_err(|_| LibraryError::InternalServerError)?;

    let response = ApiResponse::success(AuthResponse {
        user: UserResponse {
            id: user_id,
            username,
            role,
            phone: user.phone,
            created_at: user.created_at,
        },
        token,
    });

    Ok(HttpResponse::Ok().json(response))
}

pub async fn borrow_book(
    pool: web::Data<DbPool>,
    borrow_data: web::Json<BorrowRequest>,
    claims: crate::middleware::Claims,
) -> Result<HttpResponse, LibraryError> {
    let user_id = claims.sub.parse::<i32>().map_err(|_| LibraryError::InternalServerError)?;
    let book_id = borrow_data.book_id;

    let mut conn = pool.get().map_err(|_| LibraryError::InternalServerError)?;

    // 检查用户是否已经借阅了这本书
    let has_active = web::block(move || {
        BorrowService::has_active_borrow(&mut conn, user_id, book_id)
    })
    .await
    .map_err(|_| LibraryError::InternalServerError)??;

    if has_active {
        return Ok(HttpResponse::BadRequest().json(ApiResponse::<()>::error(
            "您已经借阅了这本书".to_string(),
        )));
    }

    // 检查库存
    let mut conn = pool.get().map_err(|_| LibraryError::InternalServerError)?;
    let available_copies = web::block(move || {
        BorrowService::get_book_available_copies(&mut conn, book_id)
    })
    .await
    .map_err(|_| LibraryError::InternalServerError)??;

    if available_copies <= 0 {
        return Ok(HttpResponse::BadRequest().json(ApiResponse::<()>::error(
            "图书库存不足".to_string(),
        )));
    }

    // 设置7天借阅期限
    let _due_date = chrono::Utc::now()
        .checked_add_signed(chrono::Duration::days(7))
        .expect("valid timestamp")
        .naive_utc();

    // 创建借阅记录
    let mut conn = pool.get().map_err(|_| LibraryError::InternalServerError)?;
    let record = web::block(move || {
        BorrowService::create_borrow_record(&mut conn, user_id, book_id)
    })
    .await
    .map_err(|_| LibraryError::InternalServerError)??;

    // 更新库存
    let mut conn = pool.get().map_err(|_| LibraryError::InternalServerError)?;
    web::block(move || {
        BorrowService::update_book_stock(&mut conn, book_id, true)
    })
    .await
    .map_err(|_| LibraryError::InternalServerError)??;

    // 获取图书信息用于响应
    let mut conn = pool.get().map_err(|_| LibraryError::InternalServerError)?;
    let book = web::block(move || {
        BookService::get_book_by_id(&mut conn, book_id)
    })
    .await
    .map_err(|_| LibraryError::InternalServerError)??;

    let response = BorrowResponse {
        id: record.id,
        book_id,
        book_title: book.title,
        borrow_date: record.borrow_date.unwrap_or_else(|| chrono::Utc::now().naive_utc()),
        due_date: record.due_date,
        status: record.status.unwrap_or_else(|| "borrowed".to_string()),
        renewal_count: record.renewal_count,
    };

    Ok(HttpResponse::Created().json(ApiResponse::success(response)))
}

pub async fn get_borrow_history(
    pool: web::Data<DbPool>,
    claims: crate::middleware::Claims,
) -> Result<HttpResponse, LibraryError> {
    let user_id = claims.sub.parse::<i32>().map_err(|_| LibraryError::InternalServerError)?;

    let mut conn = pool.get().map_err(|_| LibraryError::InternalServerError)?;

    let records = web::block(move || {
        BorrowService::get_user_borrow_history(&mut conn, user_id)
    })
    .await
    .map_err(|_| LibraryError::InternalServerError)??;

    Ok(HttpResponse::Ok().json(ApiResponse::success(records)))
}

pub async fn return_book(
    pool: web::Data<DbPool>,
    borrow_id: web::Path<i32>,
    claims: crate::middleware::Claims,
) -> Result<HttpResponse, LibraryError> {
    let user_id = claims.sub.parse::<i32>().map_err(|_| LibraryError::InternalServerError)?;
    let borrow_id_val = borrow_id.into_inner();

    let mut conn = pool.get().map_err(|_| LibraryError::InternalServerError)?;

    // 执行还书操作
    let record = web::block(move || {
        BorrowService::return_book(&mut conn, borrow_id_val, user_id)
    })
    .await
    .map_err(|_| LibraryError::InternalServerError)??;

    // 获取图书信息用于响应
    let mut conn = pool.get().map_err(|_| LibraryError::InternalServerError)?;
    let book = web::block(move || {
        BookService::get_book_by_id(&mut conn, record.book_id)
    })
    .await
    .map_err(|_| LibraryError::InternalServerError)??;

    // 计算是否逾期
    let is_overdue = BorrowService::calculate_overdue(record.due_date);
    let overdue_days = BorrowService::get_overdue_days(record.due_date);

    let response = BorrowResponse {
        id: record.id,
        book_id: record.book_id,
        book_title: book.title,
        borrow_date: record.borrow_date.unwrap_or_else(|| chrono::Utc::now().naive_utc()),
        due_date: record.due_date,
        status: record.status.unwrap_or_else(|| "returned".to_string()),
        renewal_count: record.renewal_count,
    };

    let mut api_response = ApiResponse::success(response);
    
    // 如果逾期，添加提示信息
    if is_overdue {
        api_response.message = format!("图书已归还，逾期{}天", overdue_days);
    } else {
        api_response.message = "图书已成功归还".to_string();
    }

    Ok(HttpResponse::Ok().json(api_response))
}

pub async fn renew_book(
    pool: web::Data<DbPool>,
    borrow_id: web::Path<i32>,
    claims: crate::middleware::Claims,
) -> Result<HttpResponse, LibraryError> {
    let user_id = claims.sub.parse::<i32>().map_err(|_| LibraryError::InternalServerError)?;
    let borrow_id_val = borrow_id.into_inner();

    let mut conn = pool.get().map_err(|_| LibraryError::InternalServerError)?;

    // 执行续借操作
    let record = web::block(move || {
        BorrowService::renew_book(&mut conn, borrow_id_val, user_id)
    })
    .await
    .map_err(|_| LibraryError::InternalServerError)??;

    // 获取图书信息用于响应
    let mut conn = pool.get().map_err(|_| LibraryError::InternalServerError)?;
    let book = web::block(move || {
        BookService::get_book_by_id(&mut conn, record.book_id)
    })
    .await
    .map_err(|_| LibraryError::InternalServerError)??;

    let response = BorrowResponse {
        id: record.id,
        book_id: record.book_id,
        book_title: book.title,
        borrow_date: record.borrow_date.unwrap_or_else(|| chrono::Utc::now().naive_utc()),
        due_date: record.due_date,
        status: record.status.unwrap_or_else(|| "borrowed".to_string()),
        renewal_count: record.renewal_count,
    };

    let mut api_response = ApiResponse::success(response);
    
    // 添加续借成功提示
    let renewal_count = record.renewal_count.unwrap_or(0);
    if renewal_count > 0 {
        api_response.message = format!("图书续借成功，新的到期日期为 {}", record.due_date.format("%Y-%m-%d"));
    }

    Ok(HttpResponse::Ok().json(api_response))
}

pub async fn get_popular_books(
    pool: web::Data<DbPool>,
    query: web::Query<PopularBooksParams>,
) -> Result<HttpResponse, LibraryError> {
    let mut conn = pool.get().map_err(|_| LibraryError::InternalServerError)?;

    let params = query.into_inner();
    
    let popular_books = web::block(move || {
        StatisticsService::get_popular_books(&mut conn, &params)
    })
    .await
    .map_err(|_| LibraryError::InternalServerError)??;

    let response = ApiResponse::success(popular_books);
    
    Ok(HttpResponse::Ok().json(response))
}

pub async fn get_inventory_stats(
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, LibraryError> {
    let mut conn = pool.get().map_err(|_| LibraryError::InternalServerError)?;

    let stats = web::block(move || {
        StatisticsService::get_inventory_stats(&mut conn)
    })
    .await
    .map_err(|_| LibraryError::InternalServerError)??;

    let response = ApiResponse::success(stats);
    
    Ok(HttpResponse::Ok().json(response))
}

// 管理员功能：获取所有用户的借阅记录
pub async fn get_all_borrow_records(
    pool: web::Data<DbPool>,
    query: web::Query<PaginationParams>,
    claims: Claims,
) -> Result<HttpResponse, LibraryError> {
    // 检查管理员权限
    check_admin_role(&claims).map_err(|_| LibraryError::Unauthorized)?;

    let mut conn = pool.get().map_err(|_| LibraryError::InternalServerError)?;

    let params = query.into_inner();
    let per_page = params.per_page.unwrap_or(20).min(100);
    let page = params.page.unwrap_or(1).max(1);
    let offset = (page - 1) * per_page;

    let (records, total) = web::block(move || {
        BorrowService::get_all_borrow_records(&mut conn, per_page as i64, offset as i64)
    })
    .await
    .map_err(|_| LibraryError::InternalServerError)??;

    let total_pages = ((total as f64) / (per_page as f64)).ceil() as i32;

    let response = ApiResponse::success(PaginatedResponse {
        data: records,
        total,
        page,
        per_page,
        total_pages,
    });

    Ok(HttpResponse::Ok().json(response))
}

// 管理员功能：获取所有用户列表
pub async fn get_all_users(
    pool: web::Data<DbPool>,
    query: web::Query<PaginationParams>,
    claims: Claims,
) -> Result<HttpResponse, LibraryError> {
    // 检查管理员权限
    check_admin_role(&claims).map_err(|_| LibraryError::Unauthorized)?;

    let mut conn = pool.get().map_err(|_| LibraryError::InternalServerError)?;

    let params = query.into_inner();
    let per_page = params.per_page.unwrap_or(20).min(100);
    let page = params.page.unwrap_or(1).max(1);
    let offset = (page - 1) * per_page;

    let (users, total) = web::block(move || {
        UserService::get_all_users(&mut conn, per_page as i64, offset as i64)
    })
    .await
    .map_err(|_| LibraryError::InternalServerError)??;

    let user_responses: Vec<UserResponse> = users
        .into_iter()
        .map(UserResponse::from)
        .collect();

    let total_pages = ((total as f64) / (per_page as f64)).ceil() as i32;

    let response = ApiResponse::success(PaginatedResponse {
        data: user_responses,
        total,
        page,
        per_page,
        total_pages,
    });

    Ok(HttpResponse::Ok().json(response))
}

// 管理员功能：获取特定用户的借阅记录
pub async fn get_user_borrow_records(
    pool: web::Data<DbPool>,
    user_id: web::Path<i32>,
    claims: Claims,
) -> Result<HttpResponse, LibraryError> {
    let target_user_id = user_id.into_inner();
    
    // 检查权限：管理员可以查看任何用户，普通用户只能查看自己
    check_user_ownership(&claims, target_user_id).map_err(|_| LibraryError::Unauthorized)?;

    let mut conn = pool.get().map_err(|_| LibraryError::InternalServerError)?;

    let records = web::block(move || {
        BorrowService::get_user_borrow_history(&mut conn, target_user_id)
    })
    .await
    .map_err(|_| LibraryError::InternalServerError)??;

    Ok(HttpResponse::Ok().json(ApiResponse::success(records)))
}