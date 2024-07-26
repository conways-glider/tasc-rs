use std::time::{SystemTime, UNIX_EPOCH};

use anyhow::{anyhow, Context};
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Argon2, PasswordHash, PasswordVerifier,
};
use jsonwebtoken::{errors::ErrorKind, DecodingKey, EncodingKey, Header, TokenData};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct User {
    pub id: uuid::Uuid,
    pub username: String,
    pub name: String,

    #[serde(skip_serializing)]
    pub passhash: Option<String>,
    pub added: time::OffsetDateTime,
}

impl User {
    pub fn validate_password(&self, password: String) -> Result<(), anyhow::Error> {
        match &self.passhash {
            Some(passhash) => {
                let parsed_hash = PasswordHash::new(&passhash)?;
                // let status = Argon2::default().verify_password(password, &parsed_hash).is_ok();
                let status = Argon2::default()
                    .verify_password(password.as_bytes(), &parsed_hash)
                    .is_ok();
                match status {
                    true => Ok(()),
                    false => Err(anyhow!("Invalid password for user {}", self.username)),
                }
            }
            None => Err(anyhow!(
                "User {} does not have a set password",
                self.username
            )),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct NewUser {
    pub id: Option<uuid::Uuid>,
    pub username: String,
    pub name: String,
    pub password: Option<String>,
}

impl NewUser {
    pub fn get_uuid(&self) -> uuid::Uuid {
        self.id.unwrap_or(uuid::Uuid::now_v7())
    }

    pub fn get_pass_hash(&self) -> Result<Option<String>, anyhow::Error> {
        match &self.password {
            Some(password) => {
                let salt = SaltString::generate(&mut OsRng);
                let argon2 = Argon2::default();
                let password_hash = argon2
                    .hash_password(password.as_bytes(), &salt)?
                    .to_string();
                Ok(Some(password_hash))
            }
            None => Ok(None),
        }
    }
}

const ISSUER: &str = "tasc-rs";

// This is how long the JWT's are valid
// Currently, this is 4 hours
const TOKEN_VALID_SECONDS: u64 = 60 * 60 * 4;

#[derive(Deserialize, Serialize)]
pub struct AuthUser {
    // Claims
    // https://github.com/Keats/jsonwebtoken?tab=readme-ov-file#claims
    pub exp: u64,
    pub iss: String,
    pub sub: String,

    // Custom Fields
    pub id: uuid::Uuid,
    pub username: String,
    pub name: String,
}

impl AuthUser {
    pub fn new_auth_user(user: User) -> Result<AuthUser, anyhow::Error> {
        let default_issuer = String::from(ISSUER);

        let valid_duration = std::time::Duration::from_secs(TOKEN_VALID_SECONDS);
        let expiration_time = SystemTime::now()
            .checked_add(valid_duration)
            .context("Could not generate expiration time")?;
        let unix_time_expiration = expiration_time
            .duration_since(UNIX_EPOCH)
            .context("Could not get expiration unix time")?
            .as_secs();

        let claims = AuthUser {
            exp: unix_time_expiration,
            iss: default_issuer,
            sub: user.id.to_string(),

            id: user.id,
            username: user.username,
            name: user.name,
        };
        Ok(claims)
    }

    pub fn encode(&self, encoding_key: &EncodingKey) -> Result<String, anyhow::Error> {
        let token = jsonwebtoken::encode(&Header::default(), &self, encoding_key)?;
        Ok(token)
    }

    pub fn decode(token: &str, decoding_key: &DecodingKey) -> Result<AuthUser, anyhow::Error> {
        let mut validation = jsonwebtoken::Validation::new(jsonwebtoken::Algorithm::default());
        validation.set_issuer(&[ISSUER]);
        validation.set_required_spec_claims(&["exp", "iss"]);

        let token = match jsonwebtoken::decode::<AuthUser>(&token, decoding_key, &validation) {
            Ok(c) => c,
            Err(err) => match *err.kind() {
                ErrorKind::InvalidToken => panic!("Token is invalid"), // Example on how to handle a specific error
                ErrorKind::InvalidIssuer => panic!("Issuer is invalid"), // Example on how to handle a specific error
                _ => panic!("Some other errors"),
            },
        };
        Ok(token.claims)
    }
}
