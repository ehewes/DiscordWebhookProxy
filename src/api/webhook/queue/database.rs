use super::super::Webhook;
use sled::Db;
use std::{process, sync::OnceLock};
use tokio::task;
use tracing::{error, info};

static QUEUE_SLED_PATH: OnceLock<String> = OnceLock::new();

fn get_queue_sled_path() -> String {
    QUEUE_SLED_PATH.get().cloned().unwrap_or_else(|| {
        let pid = process::id();
        let path = format!("/tmp/discord-webhook-proxy/queue-sled-{}", pid);

        if QUEUE_SLED_PATH.set(path).is_err() {
            error!("Failed to set QUEUE_SLED_PATH: already set");

            std::process::exit(1);
        }

        if let Some(queue_sled_path) = QUEUE_SLED_PATH.get() {
            return queue_sled_path.clone();
        }

        error!("Failed to read QUEUE_SLED_PATH: not set");

        process::exit(1)
    })
}

pub struct WebhookQueueDatabase {
    database: Db,
}

impl WebhookQueueDatabase {
    pub fn open() -> Self {
        let database = match sled::open(get_queue_sled_path()) {
            Ok(database) => database,
            Err(error) => {
                error!("Failed to open sled database for the queue, see: {error:#?}");

                process::exit(1)
            }
        };

        Self { database }
    }

    pub async fn insert(&self, webhook: Webhook) -> u64 {
        let database = self.database.clone();

        let result = task::spawn_blocking(move || {
            let id = match database.generate_id() {
                Ok(id) => id,
                Err(error) => {
                    error!(
                        "Failed to insert webhook record into Sled queue database: Failed to generate record ID, see: {error:#?}"
                    );

                    process::exit(1)
                }
            };

            let value = match serde_json::to_vec(&webhook) {
                Ok(value) => value,
                Err(error) => {
                    error!(
                        "Failed to insert webhook record into Sled queue database: Failed to serialize webhook, see: {error:#?}"
                    );

                    process::exit(1)
                }
            };

            match database.insert(id.to_be_bytes(), value) {
                Err(error) =>  {
                        error!(
                            "Failed to insert webhook record into Sled queue database: Failed to serialize webhook, see: {error:#?}"
                        );

                        process::exit(1);
                }, 
                Ok(_) => id,
            }

        })
        .await;
        match result {
            Ok(id) => id,
            Err(error) => {
                error!("Failed to join blocking insert task, see: {error:#?}");

                process::exit(1)
            }
        }
    }

    pub async fn remove(&self, id: u64) {
        let database = self.database.clone();

        let result = task::spawn_blocking(move || database.remove(id.to_be_bytes())).await;

        match result {
            Ok(Ok(_)) => (),
            Ok(Err(error)) => {
                error!("Failed to remove webhook record from Sled queue database, see: {error:#?}");
            }

            Err(error) => {
                error!("Failed to join blocking remove task, see: {error:#?}");

                process::exit(1)
            }
        }
    }

    pub async fn get(&self, id: u64) -> Option<Webhook> {
        let database = self.database.clone();

        let result = task::spawn_blocking(move || database.get(id.to_be_bytes())).await;

        match result {
            Ok(Ok(webhook_bytes)) => match serde_json::from_slice(&webhook_bytes?) {
                Ok(webhook) => Some(webhook),
                Err(error) => {
                    info!("Failed to deserialize webhook record, see: {error:#?}");

                    None
                }
            },
            Ok(Err(error)) => {
                error!("Failed to get webhook record from Sled queue database, see: {error:#?}");

                None
            }

            Err(error) => {
                error!("Failed to join blocking get task, see: {error:#?}");

                None
            }
        }
    }
}
