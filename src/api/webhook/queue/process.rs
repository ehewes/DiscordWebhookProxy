use super::super::{Webhook, forward_webhook, queue::WebhookQueueDatabase};
use crate::api::ApiError;
use rocket::http::Status;
use tracing::{error, info};

pub async fn queue_process_webhook(
    queue_database: &WebhookQueueDatabase,
    webhook: &Webhook,
    queue_id: u64,
) -> Result<(Status, String, u64), ApiError> {
    let (status, response_body, retry_after_secs) = forward_webhook(webhook).await?;

    {
        match status.code {
            204 => {
                info!(
                    "Queue Processed & Proxied request:\n- Webhook ID: {}\n- Status Code: {}\n- Webhook Body: {:#?}",
                    webhook.id, status.code, webhook.body
                );

                Ok((status, response_body, retry_after_secs))
            }
            429 => Box::pin(queue_process_webhook(queue_database, webhook, queue_id)).await,
            _ => Err(ApiError::message(
                status,
                format!(
                    "Failed to proxy request: Unexpected response code, keeping it in database: Internal ID: {}\nWebhook ID: {}\nWebhook Body: {:#?}",
                    webhook.id, status.code, webhook.body
                ),
            )),
        }
    }
}

pub async fn queue_process_database(queue_database: &WebhookQueueDatabase) {
    for item in queue_database.iter() {
        match item {
            Ok((key, value)) => {
                let queue_id = match <[u8; 8]>::try_from(key.as_ref()) {
                    Ok(bytes) => u64::from_be_bytes(bytes),
                    Err(_) => {
                        error!("Invalid webhook record key length");

                        continue;
                    }
                };

                let queue_database_clone = queue_database.clone();

                match serde_json::from_slice::<Webhook>(&value) {
                    Ok(webhook) => {
                        tokio::spawn(async move {
                            let _ =
                                queue_process_webhook(&queue_database_clone, &webhook, queue_id)
                                    .await;

                            match queue_process_webhook(&queue_database_clone, &webhook, queue_id)
                                .await
                            {
                                Ok(_) => (),
                                Err(error) => {
                                    error!(
                                        "Failed to process a webhook during queue leftover cleanup, see: {error:#?}"
                                    );
                                }
                            }
                        });
                    }
                    Err(error) => {
                        error!("Failed to deserialize webhook record, see: {error:#?}");
                    }
                }
            }
            Err(error) => {
                error!("Failed to read webhook record from Sled queue database, see: {error:#?}");
            }
        }
    }
}
