use actix_web::{dev::Payload, FromRequest, HttpRequest, error::ErrorUnauthorized, Error};
use futures::future::{ready, Ready};
use jsonwebtoken::{decode, DecodingKey, Validation, Algorithm};
pub use crate::models::Claims;
use std::env;

impl FromRequest for Claims {
    type Error = actix_web::Error;
    type Future = Ready<Result<Claims, actix_web::Error>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let auth_header = req.headers().get("Authorization");
        
        let token = match auth_header {
            Some(header_value) => {
                let header_str = match header_value.to_str() {
                    Ok(s) => s,
                    Err(_) => return ready(Err(ErrorUnauthorized("无效的认证头"))),
                };
                
                if header_str.starts_with("Bearer ") {
                    &header_str[7..]
                } else {
                    return ready(Err(ErrorUnauthorized("无效的认证格式")));
                }
            }
            None => return ready(Err(ErrorUnauthorized("缺少认证头"))),
        };

        let secret = env::var("JWT_SECRET").unwrap_or_else(|_| "secret_key".to_string());
        
        match decode::<Claims>(
            token,
            &DecodingKey::from_secret(secret.as_ref()),
            &Validation::new(Algorithm::HS256),
        ) {
            Ok(token_data) => ready(Ok(token_data.claims)),
            Err(_) => ready(Err(ErrorUnauthorized("无效的令牌"))),
        }
    }
}

// 权限检查中间件
pub struct RequireRole {
    pub role: String,
}

impl RequireRole {
    pub fn admin() -> Self {
        RequireRole {
            role: "admin".to_string(),
        }
    }

    pub fn user() -> Self {
        RequireRole {
            role: "user".to_string(),
        }
    }
}

impl FromRequest for RequireRole {
    type Error = Error;
    type Future = Ready<Result<Self, Error>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let auth_header = req.headers().get("Authorization");
        
        let token = match auth_header {
            Some(header_value) => {
                let header_str = match header_value.to_str() {
                    Ok(s) => s,
                    Err(_) => return ready(Err(ErrorUnauthorized("无效的认证头"))),
                };
                
                if header_str.starts_with("Bearer ") {
                    &header_str[7..]
                } else {
                    return ready(Err(ErrorUnauthorized("无效的认证格式")));
                }
            }
            None => return ready(Err(ErrorUnauthorized("缺少认证头"))),
        };

        let secret = env::var("JWT_SECRET").unwrap_or_else(|_| "secret_key".to_string());
        
        match decode::<Claims>(
            token,
            &DecodingKey::from_secret(secret.as_ref()),
            &Validation::new(Algorithm::HS256),
        ) {
            Ok(token_data) => {
                let claims = token_data.claims;
                let user_role = claims.role.unwrap_or_else(|| "user".to_string());
                
                // 检查角色权限
                if user_role == "admin" {
                    ready(Ok(RequireRole { role: user_role }))
                } else if user_role == "user" {
                    ready(Ok(RequireRole { role: user_role }))
                } else {
                    ready(Err(ErrorUnauthorized("无效的用户角色")))
                }
            }
            Err(_) => ready(Err(ErrorUnauthorized("无效的令牌"))),
        }
    }
}

// 权限检查辅助函数
pub fn check_admin_role(claims: &Claims) -> Result<(), Error> {
    let role = claims.role.as_ref().map(|s| s.as_str()).unwrap_or("user");
    if role != "admin" {
        return Err(ErrorUnauthorized("需要管理员权限"));
    }
    Ok(())
}

pub fn check_user_ownership(claims: &Claims, target_user_id: i32) -> Result<(), Error> {
    let current_user_id = claims.sub.parse::<i32>()
        .map_err(|_| ErrorUnauthorized("无效的用户ID"))?;
    
    let role = claims.role.as_ref().map(|s| s.as_str()).unwrap_or("user");
    
    // 管理员可以访问所有用户数据
    if role == "admin" {
        return Ok(());
    }
    
    // 普通用户只能访问自己的数据
    if current_user_id != target_user_id {
        return Err(ErrorUnauthorized("只能访问自己的数据"));
    }
    
    Ok(())
}

// 检查是否为管理员
pub fn is_admin(claims: &Claims) -> bool {
    claims.role.as_ref().map(|s| s.as_str()).unwrap_or("user") == "admin"
}