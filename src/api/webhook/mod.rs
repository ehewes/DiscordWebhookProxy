mod forward_webhook;
pub use forward_webhook::forward_webhook;

mod queue;
pub use queue::WebhookQueue;

mod structs;
pub use structs::*;

use crate::util_macros::read_cfg_env_var;

pub const DEFAULT_FALLBACK_COOLDOWN_SECS: u64 = 10;

pub fn get_fallback_cooldown_secs() -> u64 {
    read_cfg_env_var!(
        "FALLBACK_COOLDOWN_SECS",
        u64,
        DEFAULT_FALLBACK_COOLDOWN_SECS
    )
}

pub fn get_retry_seconds_from_headers(response_headers: &[(String, String)]) -> u64 {
    let fallback_cooldown_secs = get_fallback_cooldown_secs();

    response_headers
        .iter()
        .find(|(name, _)| name.eq_ignore_ascii_case("Retry-After"))
        .and_then(|(_, value)| value.parse::<u64>().ok())
        .unwrap_or(fallback_cooldown_secs)
}
