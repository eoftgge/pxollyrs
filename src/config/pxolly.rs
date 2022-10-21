use serde::Deserialize;

#[derive(Deserialize, Default)]
pub struct PxollyConfig {
    pub(crate) token: String,
}

impl PxollyConfig {
    pub fn token(&self) -> &str {
        &self.token
    }
}
