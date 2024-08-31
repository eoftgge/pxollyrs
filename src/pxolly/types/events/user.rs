use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct PxollyUser {
    pub id: i64,
    pub role: u16, // ???
    pub name: String,
}