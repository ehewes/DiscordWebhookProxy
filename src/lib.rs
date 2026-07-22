mod tracing;
pub use tracing::setup as setup_tracing;
mod api;
pub use api::webhook::queue::{process::queue_process_database, WebhookQueue};
pub mod rocket_routes {
    pub use super::api::discord::{
        webhook_info, webhook_info_discord_compatibility_path, webhook_proxy,
        webhook_proxy_discord_compatibility_path,
    };
}
#[macro_export]
macro_rules! read_cfg_env_var {
    ($var_name:literal, $target_type:ty, $default:expr) => {{
        if let Ok(raw) = std::env::var($var_name) {
            match raw.parse::<$target_type>() {
                Ok(value) => value,
                Err(error) => {
                    tracing::error!(
                        "{} is not a valid {}, see: {error:#?}",
                        $var_name,
                        stringify!($target_type)
                    );
                    std::process::exit(1);
                }
            }
        } else {
            tracing::info!("{} is not set, using default", $var_name);
            $default
        }
    }};
}
