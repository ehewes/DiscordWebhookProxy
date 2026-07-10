mod forward_webhook;
pub use forward_webhook::forward_webhook;

pub mod queue;

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
