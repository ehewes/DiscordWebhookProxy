mod tracing;
pub use tracing::setup as setup_tracing;

mod api;

pub use api::webhook::queue::{WebhookQueue, process::queue_process_database};

pub mod rocket_routes {
    pub use super::api::discord::webhook_proxy;
}

pub(crate) mod util_macros {
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

    pub(crate) use read_cfg_env_var;
}
