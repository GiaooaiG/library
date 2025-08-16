use actix_web::{web, HttpResponse};
use validator::Validate;
use crate::models::{NewBook, BookResponse, ApiResponse, PaginationParams, PaginatedResponse, NewUser, LoginUser, UserResponse, AuthResponse, Claims, RegisterUser};
use crate::error::LibraryError;
use crate::services::{BookService, UserService};
use crate::db::DbPool;
use jsonwebtoken::{encode, EncodingKey, Header};
use std::env;

pub async fn create_book(
    pool: web::Data<DbPool>,
    book_data: web::Json<NewBook>,
) -> Result<HttpResponse, LibraryError> {
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
    let role = user.role.as_ref().map(|r| match r {
        crate::models::UsersRole::Admin => "admin".to_string(),
        crate::models::UsersRole::User => "user".to_string(),
    });
    
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
    let role = user.role.as_ref().map(|r| match r {
        crate::models::UsersRole::Admin => "admin".to_string(),
        crate::models::UsersRole::User => "user".to_string(),
    });
    
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