/// Responses:
/// Text(String) -> return text (for example: Text("API.messages.send({ ... });"))
/// ConfirmationCode(String) -> return code for confirm
/// ErrorCode(u8) -> return error code
/// Success -> return 1
/// Locked -> ip isn't pxolly
#[derive(Debug)]
pub enum PxollyResponse {
    Text(String),
    ConfirmationCode(String),
    ErrorCode(i8),
    Success,
    Locked,
}

impl ToString for PxollyResponse {
    fn to_string(&self) -> String {
        match self {
            Self::Text(text) => text.into(),
            Self::ConfirmationCode(code) => code.into(),
            Self::ErrorCode(code) => code.to_string(),
            Self::Success => "1".into(),
            Self::Locked => "locked".into(),
        }
    }
}
