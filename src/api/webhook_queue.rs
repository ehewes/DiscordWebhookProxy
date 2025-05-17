use super::discord::forward_webhook_request;
use rocket::serde::{Deserialize, Serialize};
use sled::Db;
use std::sync::Arc;
use tokio::{
    sync::{
        mpsc::{channel, Receiver, Sender},
        Semaphore,
    },
    task,
    time::{sleep, Duration},
};

const FALLBACK_COOLDOWN_SECS: u64 = 10;
const QUEUE_SIZE: usize = 50_000;
const CONCURRENCY_LIMIT: usize = 10;

pub type QueueSender = Sender<Webhook>;
pub type QueueReceiver = Receiver<Webhook>;

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Webhook {
    pub id: u64,
    pub token: String,
    pub body: String,
}

pub fn start_webhook_queue() -> QueueSender {
    let (queue_sender, queue_receiver) = channel::<Webhook>(QUEUE_SIZE);
    let db = sled::open("webhook-proxy-queue-db").expect("Failed to open sled database");

    tokio::spawn(async move {
        webhook_queue_handler(queue_receiver, Arc::new(db)).await;
    });

    queue_sender
}

// TODO: idk take a look at the expects maybe i doubt sum will go wrong tho
async fn webhook_queue_handler(mut queue_receiver: QueueReceiver, db: Arc<Db>) {
    let concurrency_limiter = Arc::new(Semaphore::new(CONCURRENCY_LIMIT));

    while let Some(webhook) = queue_receiver.recv().await {
        let permit = concurrency_limiter.clone().acquire_owned().await.unwrap();
        let db = db.clone();

        task::spawn(async move {
            let id = db.generate_id().expect("Failed to generate id");

            db.insert(
                id.to_be_bytes(),
                serde_json::to_vec(&webhook).expect("Failed to deserialize webhook"),
            )
                .expect("Failed to add webhook to queue");

            loop {
                let response = forward_webhook_request(webhook.id, &webhook.token, &webhook.body);

                match response {
                    Ok((status, _, response)) => match status.code {
                        429 => {
                            let retry_after = response
                                .headers
                                .get("Retry-After")
                                .and_then(|retry_after| retry_after.parse::<u64>().ok())
                                .unwrap_or(FALLBACK_COOLDOWN_SECS);

                            sleep(Duration::from_secs(retry_after)).await;

                            println!("queueed");
                        },
                        _ => {
                            db.remove(id.to_be_bytes())
                                .expect("Failed to remove webhook from queue");

                            println!("queue done");

                            break;
                        },
                    },
                    Err(_) => continue,
                }
            }

            drop(permit);
        });
    }
}