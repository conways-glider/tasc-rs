use anyhow::anyhow;
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Argon2, PasswordHash, PasswordVerifier,
};
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
                let status = Argon2::default().verify_password(password.as_bytes(), &parsed_hash).is_ok();
                match status {
                    true => Ok(()),
                    false => Err(anyhow!("Invalid password for user {}", self.username)),
                }
            }
            None => Err(anyhow!("User {} does not have a set password", self.username)),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct NewUser {
    pub id: Option<uuid::Uuid>,
    pub username: String,
    pub email: Option<String>,
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
