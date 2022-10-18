use serde::Deserialize;

#[derive(Deserialize)]
pub struct PxollyConfig {
    pub(crate) token: String,
    pub(crate) secret_key: String,
}

impl PxollyConfig {
    pub fn token(&self) -> &str {
        &self.token
    }

    pub fn secret_key(&self) -> &str {
        &self.secret_key
    }
}
