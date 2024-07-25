use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Argon2,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct User {
    pub id: uuid::Uuid,
    pub name: String,

    #[serde(skip_serializing)]
    pub passhash: Option<String>,
    pub added: time::OffsetDateTime,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct NewUser {
    pub id: Option<uuid::Uuid>,
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
