use crate::api::webhook::Webhook;
use sled::Db;
use std::sync::{Arc, OnceLock};
use tokio::task;
use tracing::error;

const SLED_DB_DIRECTORY: &str = "/var/lib/discord-webhook-proxy/queue-sled";
static QUEUE_SLED_PATH: OnceLock<String> = OnceLock::new();
static WEBHOOK_QUEUE_DATABASE: OnceLock<Db> = OnceLock::new();

fn get_webhook_queue_database() -> Db {
    WEBHOOK_QUEUE_DATABASE
        .get_or_init(|| {
            sled::open(get_queue_sled_path()).expect("Failed to open sled queue database")
        })
        .clone()
}

fn get_queue_sled_path() -> String {
    QUEUE_SLED_PATH
        .get_or_init(|| format!("{SLED_DB_DIRECTORY}-{}", std::process::id()))
        .clone()
}

#[derive(Clone, Debug)]
pub struct WebhookQueueDatabase {
    database: Arc<Db>,
}

impl WebhookQueueDatabase {
    pub fn open() -> Self {
        Self {
            database: Arc::new(get_webhook_queue_database()),
        }
    }

    pub async fn insert(&self, webhook: Webhook) -> u64 {
        let database = self.database.clone();
        let id = database.generate_id().expect("Failed to generate sled ID");
        let value = serde_json::to_vec(&webhook).expect("Failed to serialize webhook");

        let _ = task::spawn_blocking(move || {
            if let Err(e) = database.insert(id.to_be_bytes(), value) {
                error!("Failed to insert into sled: {e:#?}");
            }
        })
        .await;

        id
    }

    pub async fn remove(&self, id: u64) {
        let database = self.database.clone();

        let _ = task::spawn_blocking(move || {
            let _ = database.remove(id.to_be_bytes());
        })
        .await;
    }

    pub fn iter(&self) -> sled::Iter {
        self.database.iter()
    }
}
