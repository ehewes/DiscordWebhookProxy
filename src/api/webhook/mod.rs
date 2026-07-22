mod forward_webhook;
pub use forward_webhook::forward_webhook;

pub mod queue;

mod structs;
pub use structs::*;

use crate::api::{ApiError, ApiResult};
use crate::util_macros::read_cfg_env_var;
use rocket::http::Status;

pub const DISCORD_API_URL: &str = "https://discord.com/api";

pub const DEFAULT_FALLBACK_COOLDOWN_SECS: u64 = 10;

pub fn get_fallback_cooldown_secs() -> u64 {
    read_cfg_env_var!(
        "FALLBACK_COOLDOWN_SECS",
        u64,
        DEFAULT_FALLBACK_COOLDOWN_SECS
    )
}

pub fn status_from_code(code: u16) -> ApiResult<Status> {
    let status = Status::from_code(code).ok_or_else(|| {
        ApiError::message(
            Status::InternalServerError,
            "Failed to convert the status code",
        )
    })?;

    Ok(status)
}
