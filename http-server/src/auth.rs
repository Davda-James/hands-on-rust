use crate::models::*;
use argon2::{
    Argon2, PasswordHash, PasswordVerifier,
    password_hash::{PasswordHasher, SaltString, rand_core},
};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use rand_core::OsRng;
use time;
use uuid::Uuid;
pub fn hash_password(passwd: &str) -> String {
    let salt = SaltString::generate(&mut OsRng);
    Argon2::default()
        .hash_password(passwd.as_bytes(), &salt)
        .unwrap()
        .to_string()
}

pub fn verify_password(passwd: &str, hash: &str) -> bool {
    let parsed = PasswordHash::new(hash).unwrap();
    Argon2::default()
        .verify_password(passwd.as_bytes(), &parsed)
        .is_ok()
}

pub fn generate_token(user_id: Uuid) -> String {
    let claims = Claims {
        user_id,
        exp: (time::OffsetDateTime::now_utc().unix_timestamp() + 7200) as usize,
    };
    let secret = std::env::var("JWT_SECRET").unwrap();
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_ref()),
    )
    .unwrap()
}

pub fn verify_token(token: &str) -> Claims {
    decode(
        token,
        &DecodingKey::from_secret(std::env::var("JWT_SECRET").unwrap().as_ref()),
        &Validation::default(),
    )
    .unwrap()
    .claims
}
