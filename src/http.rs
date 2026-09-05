use std::fmt::Display;

/// HTTP status codes.
#[derive(Debug, Clone, Copy)]
#[repr(u16)]
pub enum Status {
    Ok = 200,
    BadRequest = 400,
    NotFound = 404,
    InternalServerError = 500,
    ServiceUnavailable = 503,
}

impl Status {
    /// Return the numeric value of this status code.
    pub fn code(&self) -> u16 {
        *self as u16
    }
}

impl Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}
