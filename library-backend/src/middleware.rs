use actix_web::{dev::Payload, FromRequest, HttpRequest, error::ErrorUnauthorized};
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