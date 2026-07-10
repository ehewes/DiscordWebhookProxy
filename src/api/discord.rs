use super::{
    ApiError, ApiResult,
    webhook::{Webhook, WebhookBody, WebhookQueue, forward_webhook},
};
use rocket::post;
use rocket::serde::json::serde_json;
use rocket::{State, http::Status, serde::json::Json};

#[post("/webhook/<webhook_id>/<webhook_token>", data = "<body>")]
pub async fn webhook_proxy(
    webhook_id: u64,
    webhook_token: &str,
    body: Json<WebhookBody>,
    webhook_queue: &State<WebhookQueue>,
) -> ApiResult<(Status, Option<Json<serde_json::Value>>)> {
    let webhook = Webhook {
        id: webhook_id,
        token: webhook_token.to_string(),
        body: body.0,
    };

    let (status, response_body, _retry_after_secs) = forward_webhook(&webhook).await?;

    if status.code == 429 || status.code >= 500 {
        let queue_id = webhook_queue.send(webhook).await.map_err(|_| {
            ApiError::message(Status::InternalServerError, "Failed to queue request")
        })?;

        return Ok((
            Status::Accepted,
            Some(Json(serde_json::json!({
                "queueId": queue_id,
            }))),
        ));
    }

    if status == Status::NoContent {
        return Ok((Status::NoContent, None));
    }

    let parsed = if response_body.trim().is_empty() {
        None
    } else {
        serde_json::from_str::<serde_json::Value>(&response_body).ok()
    };

    Ok((status, parsed.map(Json)))
}
