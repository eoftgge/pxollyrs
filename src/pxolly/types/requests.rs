use std::fmt::Debug;
use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct PxollyAPIRequestParams<'a, T: Serialize + Debug> {
    pub access_token: &'a str,
    pub format: &'a str,
    #[serde(flatten)]
    pub extras: T,
}
