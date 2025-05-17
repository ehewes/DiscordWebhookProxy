pub(super) mod structs;
pub mod discord;
pub mod webhook_queue;

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
    code: u16,
    message: String,
}

impl ApiError {
    pub fn new<S>(status: Status, message: S) -> Self
    where
        S: Into<String>,
    {
        Self {
            code: status.code,
            message: message.into(),
        }
    }

    pub fn message<S>(status: Status, message: S) -> Self
    where
        S: Into<String>,
    {
        Self::new(status, message)
    }
}

impl<'a> response::Responder<'a, 'a> for ApiError {
    fn respond_to(self, request: &'a Request<'_>) -> response::Result<'a> {
        let status = Status::from_code(self.code).unwrap_or_default();
        let error = json!({ "error": self });

        Response::build_from(error.respond_to(request)?)
            .status(status)
            .ok()
    }
}