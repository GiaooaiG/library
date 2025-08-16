use actix_web::{dev::Payload, FromRequest, HttpRequest, error::ErrorUnauthorized};
use jsonwebtoken::{decode, DecodingKey, Validation, Algorithm};
use futures::future::{ready, Ready};
use crate::models::Claims;

pub struct AuthenticatedUser {
    pub id: i32,
    pub username: String,
    pub role: Option<String>,
}

impl FromRequest for AuthenticatedUser {
    type Error = actix_web::Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let auth_header = req.headers().get("Authorization");
        
        let token = match auth_header {
            Some(header) => {
                let header_str = header.to_str().unwrap_or("");
                if header_str.starts_with("Bearer ") {
                    header_str[7..].to_string()
                } else {
                    return ready(Err(ErrorUnauthorized("无效的认证头格式")));
                }
            }
            None => return ready(Err(ErrorUnauthorized("未提供认证令牌"))),
        };

        let secret = std::env::var("JWT_SECRET").unwrap_or_else(|_| "secret_key".to_string());
        
        match decode::<Claims>(
            &token,
            &DecodingKey::from_secret(secret.as_ref()),
            &Validation::new(Algorithm::HS256),
        ) {
            Ok(token_data) => {
                let claims = token_data.claims;
                let user_id = claims.sub.parse::<i32>().unwrap_or(0);
                
                ready(Ok(AuthenticatedUser {
                    id: user_id,
                    username: claims.username,
                    role: claims.role,
                }))
            }
            Err(_) => ready(Err(ErrorUnauthorized("无效的认证令牌"))),
        }
    }
}

pub struct AdminUser(AuthenticatedUser);

impl FromRequest for AdminUser {
    type Error = actix_web::Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, payload: &mut Payload) -> Self::Future {
        let auth_future = AuthenticatedUser::from_request(req, payload);
        
        match auth_future.into_inner() {
            Ok(user) => {
                if user.role.as_deref() == Some("admin") {
                    ready(Ok(AdminUser(user)))
                } else {
                    ready(Err(ErrorUnauthorized("需要管理员权限")))
                }
            }
            Err(e) => ready(Err(e)),
        }
    }
}