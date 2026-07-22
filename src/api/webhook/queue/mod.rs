pub mod database;
pub mod process;
use crate::api::webhook::{
    queue::{database::WebhookQueueDatabase, process::queue_process_webhook},
    Webhook,
};
use crate::read_cfg_env_var;
use std::sync::Arc;
use tokio::sync::{
    mpsc::{channel, error::SendError, Receiver, Sender},
    Semaphore,
};
use tokio::task::{self, JoinHandle};
use tracing::{error, info};
const DEFAULT_QUEUE_SIZE: usize = 50_000;
const DEFAULT_CONCURRENCY_LIMIT: usize = 20;
type QueueSender = Sender<(Webhook, u64)>;
type QueueReceiver = Receiver<(Webhook, u64)>;

pub struct WebhookQueue {
    database: Arc<WebhookQueueDatabase>,
    sender: QueueSender,
    _queue_handle: JoinHandle<()>,
}

impl Default for WebhookQueue {
    fn default() -> Self {
        WebhookQueue::new()
    }
}

impl WebhookQueue {
    fn new() -> Self {
        let (sender, _queue_handle) = start_webhook_queue();
        let database = WebhookQueueDatabase::open();

        WebhookQueue {
            database: Arc::new(database),
            sender,
            _queue_handle,
        }
    }

    pub fn get_database(&self) -> &WebhookQueueDatabase {
        &self.database
    }

    pub async fn send(&self, webhook: Webhook) -> Result<u64, SendError<(Webhook, u64)>> {
        let queue_id = self.database.insert(webhook.clone()).await;

        self.sender.send((webhook, queue_id)).await?;

        Ok(queue_id)
    }
}
pub fn start_webhook_queue() -> (QueueSender, JoinHandle<()>) {
    let queue_size: usize = read_cfg_env_var!("QUEUE_SIZE", usize, DEFAULT_QUEUE_SIZE);
    let (queue_sender, queue_receiver) = channel::<(Webhook, u64)>(queue_size);

    let database = WebhookQueueDatabase::open();

    let handle = tokio::spawn(async move {
        webhook_queue_handler(queue_receiver, Arc::new(database)).await;
    });

    (queue_sender, handle)
}
async fn webhook_queue_handler(
    mut queue_receiver: QueueReceiver,
    database: Arc<WebhookQueueDatabase>,
) {
    let concurrency_limit =
        read_cfg_env_var!("CONCURRENCY_LIMIT", usize, DEFAULT_CONCURRENCY_LIMIT);
    let concurrency_limiter = Arc::new(Semaphore::new(concurrency_limit));

    while let Some((webhook, queue_id)) = queue_receiver.recv().await {
        let permit = concurrency_limiter
            .clone()
            .acquire_owned()
            .await
            .unwrap_or_else(|e| {
                error!("Failed to acquire permit: {:?}", e);
                std::process::exit(1);
            });

        let database = database.clone();

        task::spawn(async move {
            match queue_process_webhook(&database, &webhook, queue_id).await {
                Ok(_) => info!(
                    "Queue processed webhook {} (queue ID {})",
                    webhook.id, queue_id
                ),
                Err(e) => error!(
                    "Queue failed for webhook {} (queue ID {}): {:?}",
                    webhook.id, queue_id, e
                ),
            }

            drop(permit);
        });
    }
}
