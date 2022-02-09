use reqwest::{Error, Response, StatusCode};
use std::fmt;

pub type Result<T> = std::result::Result<T, LiveClientError>;

#[derive(Debug)]
pub struct LiveClientError {
    error: Error,
    retries: u8,
    response: Option<Response>,
    status_code: Option<StatusCode>,
}

impl LiveClientError {
    pub fn new(
        error: Error,
        retries: u8,
        response: Option<Response>,
        status_code: Option<StatusCode>,
    ) -> Self {
        Self {
            error,
            retries,
            response,
            status_code,
        }
    }
}

impl fmt::Display for LiveClientError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:#?}", self)
    }
}

impl std::error::Error for LiveClientError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.error)
    }
}
