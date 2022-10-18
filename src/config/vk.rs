use serde::Deserialize;

#[derive(Deserialize)]
pub struct VKConfig {
    pub(crate) token: String,
    pub(crate) version: String,
}

impl VKConfig {
    pub fn token(&self) -> &str {
        &self.token
    }

    pub fn version(&self) -> &str {
        &self.version
    }
}
