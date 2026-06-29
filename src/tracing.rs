use tracing::subscriber;
use tracing_subscriber::{EnvFilter, FmtSubscriber};

pub fn setup() {
    let default_level = if cfg!(debug_assertions) {
        "debug"
    } else {
        "info"
    };

    let subscriber = FmtSubscriber::builder()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(default_level)),
        )
        .finish();

    subscriber::set_global_default(subscriber)
        .expect("failed to set the global tracing subscriber");
}
