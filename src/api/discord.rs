use super::webhook::{Webhook, WebhookBody, WebhookQueue};
use super::{ApiError, ApiResult, webhook::forward_webhook};
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

    let (response_status, response_body, _) = forward_webhook(&webhook)?;

    match response_status.code {
        429 => {
            let queue_id = webhook_queue.send(webhook).await.map_err(|_| {
                ApiError::message(Status::InternalServerError, "Failed to queue request")
            })?;

            Ok((
                Status::Accepted,
                Some(Json(serde_json::json!({
                    "queueId": queue_id,
                }))),
            ))
        }
        _ => {
            if response_status == Status::NoContent {
                return Ok((response_status, None));
            }

            let response_body = serde_json::from_str::<serde_json::Value>(&response_body).ok();

            Ok((response_status, response_body.map(Json)))
        }
    }
}
