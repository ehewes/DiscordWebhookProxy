pub mod discord;
pub mod webhook;

use rocket::{
    http::Status,
    response,
    serde::{json::serde_json::json, Serialize},
    Request, Response,
};

pub type ApiResult<R> = Result<R, ApiError>;

#[derive(Serialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct ApiError {
    status: Status,
    status_code: u16,
    message: String,
}

impl ApiError {
    pub fn new<S>(status: Status, message: S) -> Self
    where
        S: Into<String>,
    {
        Self {
            status,
            status_code: status.code,
            message: message.into(),
        }
    }

    pub fn message<S>(status: Status, message: S) -> Self
    where
        S: Into<String>,
    {
        Self::new(status, message)
    }

    pub fn is_retryable(&self) -> bool {
        matches!(self.status_code, 429 | 500 | 502 | 503 | 504)
    }
}

impl<'a> response::Responder<'a, 'a> for ApiError {
    fn respond_to(self, request: &'a Request<'_>) -> response::Result<'a> {
        let error = json!({ "error": self });

        Response::build_from(error.respond_to(request)?)
            .status(self.status)
            .ok()
    }
}
