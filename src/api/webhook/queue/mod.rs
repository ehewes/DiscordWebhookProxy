mod database;
use database::WebhookQueueDatabase;

use super::super::webhook::{Webhook, forward_webhook};
use std::sync::Arc;
use tokio::{
    sync::{
        Semaphore,
        mpsc::{Receiver, Sender, channel, error::SendError},
    },
    task::{self, JoinHandle},
    time::{Duration, sleep},
};
use tracing::{error, info};

const DEFAULT_FALLBACK_COOLDOWN_SECS: u64 = 10;
const DEFAULT_QUEUE_SIZE: usize = 50_000;
const DEFAULT_CONCURRENCY_LIMIT: usize = 10;

type QueueSender = Sender<Webhook>;
type QueueReceiver = Receiver<Webhook>;

impl Default for WebhookQueue {
    fn default() -> Self {
        let (sender, _queue_handle) = start_webhook_queue();
        let database = WebhookQueueDatabase::open();

        WebhookQueue {
            database,
            sender,
            _queue_handle,
        }
    }
}
pub struct WebhookQueue {
    database: WebhookQueueDatabase,
    sender: QueueSender,
    _queue_handle: JoinHandle<()>,
}

impl WebhookQueue {
    pub async fn send(&self, webhook: Webhook) -> Result<u64, SendError<Webhook>> {
        let id_future = self.database.insert(webhook.clone());

        self.sender.send(webhook).await?;

        Ok(id_future.await)
    }
}

macro_rules! read_cfg_env_var {
    ($var_name:literal, $target_type:ty, $default:expr) => {{
        if let Ok(raw) = std::env::var($var_name) {
            match raw.parse::<$target_type>() {
                Ok(value) => value,
                Err(error) => {
                    error!(
                        "{} is not a valid {}, see: {error:#?}",
                        $var_name,
                        stringify!($target_type)
                    );

                    std::process::exit(1);
                }
            }
        } else {
            info!("{} is not set, using default", $var_name);

            $default
        }
    }};
}

pub fn start_webhook_queue() -> (QueueSender, JoinHandle<()>) {
    let queue_size: usize = read_cfg_env_var!("QUEUE_SIZE", usize, DEFAULT_QUEUE_SIZE);
    let (queue_sender, queue_receiver) = channel::<Webhook>(queue_size);

    let webhook_queue_database = WebhookQueueDatabase::open();

    let handle = tokio::spawn(async move {
        webhook_queue_handler(queue_receiver, Arc::new(webhook_queue_database)).await;
    });

    (queue_sender, handle)
}

fn get_retry_seconds(response_headers: &[(String, String)]) -> u64 {
    let fallback_cooldown_secs = read_cfg_env_var!(
        "FALLBACK_COOLDOWN_SECS",
        u64,
        DEFAULT_FALLBACK_COOLDOWN_SECS
    );

    response_headers
        .iter()
        .find(|(name, _)| name.eq_ignore_ascii_case("Retry-After"))
        .and_then(|(_, value)| value.parse::<u64>().ok())
        .unwrap_or(fallback_cooldown_secs)
}

async fn webhook_queue_handler(
    mut queue_receiver: QueueReceiver,
    database: Arc<WebhookQueueDatabase>,
) {
    let concurrency_limit =
        read_cfg_env_var!("CONCURRENCY_LIMIT", usize, DEFAULT_CONCURRENCY_LIMIT);

    let concurrency_limiter = Arc::new(Semaphore::new(concurrency_limit));

    while let Some(webhook) = queue_receiver.recv().await {
        let permit = match concurrency_limiter.clone().acquire_owned().await {
            Ok(permit) => permit,
            Err(error) => {
                error!("Failed to acquire concurrency permit: {:?}", error);

                std::process::exit(1);
            }
        };

        let database = database.clone();

        task::spawn(async move {
            loop {
                let record_id_future = database.insert(webhook.clone());
                let response = forward_webhook(&webhook);

                match response {
                    Ok((status, _, response)) => match status.code {
                        429 => {
                            let retry_after = get_retry_seconds(&response.headers);

                            sleep(Duration::from_secs(retry_after)).await;

                            info!(
                                "Queued: retrying in {} seconds, Internal ID: {}, Webhook ID: {}",
                                retry_after,
                                record_id_future.await,
                                webhook.id
                            );
                        }
                        _ => {
                            info!(
                                "Successfully proxied request: \nInternal ID: {}\nWebhook ID: {}\nWebhook Body: {:#?}",
                                webhook.id,
                                record_id_future.await,
                                webhook.body
                            );

                            break;
                        }
                    },

                    Err(error) => {
                        error!("Failed to proxy request, skipping. See: {error:#?}");

                        continue;
                    }
                }
            }

            drop(permit);
        });
    }
}
