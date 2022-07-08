use std::fmt;

use chrono::Utc;
use jsonwebtoken::{Algorithm, decode, DecodingKey, encode, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::utils::g::JWT_SECRET;

const BEARER: &str = "Bearer ";

#[derive(Error, Debug)]
pub enum JwtError {
    #[error("wrong credentials")]
    WrongCredentialsError,
    #[error("jwt token not valid")]
    JWTTokenError,
    #[error("jwt token creation error")]
    JWTTokenCreationError,
    #[error("no auth header")]
    NoAuthHeaderError,
    #[error("invalid auth header")]
    InvalidAuthHeaderError,
    #[error("no permission")]
    NoPermissionError,
}

#[derive(Clone, PartialEq)]
pub enum Role {
    Admin,
    User,
}

impl Role {
    pub fn from_str(role: &str) -> Role {
        match role {
            "Admin" => Role::Admin,
            _ => Role::User,
        }
    }
}

impl fmt::Display for Role {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Role::User => write!(f, "User"),
            Role::Admin => write!(f, "Admin"),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Claims {
    sub: String,
    role: String,
    exp: usize,
}

pub async fn create_jwt(uid: &str, role: &Role, expire: i64) -> Result<String, JwtError> {
    let expiration = Utc::now()
        .checked_add_signed(chrono::Duration::seconds(expire))
        .expect("invalid timestamp")
        .timestamp();

    let claims = Claims {
        sub: uid.to_owned(),
        role: role.to_string(),
        exp: expiration as usize,
    };
    let header = Header::new(Algorithm::HS512);
    encode(&header, &claims, &EncodingKey::from_secret(JWT_SECRET.as_ref().read().await.clone().as_ref()))
        .map(|jwt_str| BEARER.to_string() + &jwt_str)
        .map_err(|_| JwtError::JWTTokenCreationError)
}

pub async fn verify_jwt(jwt_str: &String) -> Result<Claims, JwtError> {
    if !jwt_str.starts_with(BEARER) {
        return Err(JwtError::InvalidAuthHeaderError);
    }
    let jwt_str = jwt_str.trim_start_matches(BEARER).to_owned();

    let decoded = decode::<Claims>(jwt_str.as_ref(),
                                   &DecodingKey::from_secret(JWT_SECRET.as_ref().read().await.clone().as_ref()),
                                   &Validation::new(Algorithm::HS512)).map_err(|_| JwtError::JWTTokenError)?;
    Ok(decoded.claims.clone())
}

pub async fn authorize(role: Role, jwt_str: &String) -> Result<Claims, JwtError> {
    let claim = match verify_jwt(jwt_str).await {
        Ok(v) => v,
        Err(e) => return Err(e),
    };

    if role == Role::Admin && Role::from_str(&claim.role) != Role::Admin {
        return Err(JwtError::NoPermissionError);
    }
    Ok(claim)
}
