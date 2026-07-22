use crate::api::webhook::queue::database::WebhookQueueDatabase;
use crate::api::webhook::{forward_webhook, Webhook};
use crate::api::ApiError;
use rocket::http::Status;
use std::time::Duration;
use tracing::{error, info, warn};

pub async fn queue_process_webhook(
    queue_database: &WebhookQueueDatabase,
    webhook: &Webhook,
    queue_id: u64,
) -> Result<(Status, String, u64), ApiError> {
    loop {
        let (status, response_body, retry_after_secs) = forward_webhook(webhook).await?;

        match status.code {
            204 => {
                info!(
                    "Queue processed webhook {} (queue ID {}) successfully",
                    webhook.id, queue_id
                );

                queue_database.remove(queue_id).await;

                return Ok((status, response_body, retry_after_secs));
            }
            429 => {
                warn!(
                    "Rate limited (429) on webhook {}, retrying in {}s",
                    webhook.id, retry_after_secs
                );

                tokio::time::sleep(Duration::from_secs(retry_after_secs)).await;

                continue;
            }
            _ => {
                error!(
                    "Failed to proxy webhook {} (queue ID {}): unexpected status {}",
                    webhook.id, queue_id, status.code
                );

                return Err(ApiError::message(
                    status,
                    format!("Unexpected response code {} from Discord", status.code),
                ));
            }
        }
    }
}

pub async fn queue_process_database(queue_database: &WebhookQueueDatabase) {
    let items: Vec<_> = queue_database
        .iter()
        .filter_map(|item| item.ok())
        .filter_map(|(key, value)| {
            let queue_id = <[u8; 8]>::try_from(key.as_ref()).ok()?;

            Some((
                u64::from_be_bytes(queue_id),
                serde_json::from_slice::<Webhook>(&value).ok()?,
            ))
        })
        .collect();

    if items.is_empty() {
        return;
    }

    info!("Processing {} leftover queued webhooks", items.len());

    for (queue_id, webhook) in items {
        let db = queue_database.clone();

        tokio::spawn(async move {
            match queue_process_webhook(&db, &webhook, queue_id).await {
                Ok(_) => info!(
                    "Leftover webhook {} (queue ID {}) processed",
                    webhook.id, queue_id
                ),
                Err(e) => error!(
                    "Failed to process leftover (queue ID {}): {:?}",
                    queue_id, e
                ),
            }
        });
    }
}
