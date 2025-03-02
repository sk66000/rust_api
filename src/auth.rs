use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey};
use serde::{Deserialize, Serialize};
use actix_web::error::ErrorUnauthorized;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
}

pub fn create_jwt(user_id: &str) -> String {

    let expiration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() + 3600; // Token expires in 1 hour

    let claims = Claims {
        sub: user_id.to_owned(),
        exp: expiration as usize,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret("your_secret_key".as_ref()),
    )
    .unwrap()
}

pub fn validate_jwt(token: &str) -> Result <String, actix_web::Error> {

    let validation = Validation::default();
    let token_data = decode:: <Claims>(
        token,
        &DecodingKey::from_secret("your_secret_key".as_ref()),
        &validation,
    )
    .map_err(|_| ErrorUnauthorized("Invalid token"))?;

    Ok(token_data.claims.sub)
}
