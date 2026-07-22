use super::{
    ApiError, ApiResult,
    webhook::{Webhook, WebhookBody, forward_webhook, queue::WebhookQueue},
};
use crate::api::webhook::{DISCORD_API_URL, status_from_code};
use rocket::serde::json::serde_json;
use rocket::{State, http::Status, serde::json::Json};
use rocket::{get, post};

#[post("/webhook/<webhook_id>/<webhook_token>", data = "<body>")]
pub async fn webhook_proxy(
    webhook_id: u64,
    webhook_token: &str,
    body: Json<WebhookBody>,
    webhook_queue: &State<WebhookQueue>,
) -> ApiResult<(Status, Json<serde_json::Value>)> {
    let webhook = Webhook {
        id: webhook_id,
        token: webhook_token.to_string(),
        body: body.0,
    };

    let (status, response_body, _retry_after_secs) = forward_webhook(&webhook).await?;

    if status == Status::NoContent {
        return Ok((Status::NoContent, Json(serde_json::json!({}))));
    }

    if status.code == 429 {
        let queue_id = webhook_queue.send(webhook).await.map_err(|_| {
            ApiError::message(Status::InternalServerError, "Failed to queue request")
        })?;

        return Ok((
            Status::Accepted,
            Json(serde_json::json!({
                "queueId": queue_id,
            })),
        ));
    }

    let parsed: serde_json::Value = if response_body.trim().is_empty() {
        serde_json::json!({})
    } else {
        serde_json::from_str(&response_body).unwrap_or_else(|_| serde_json::json!({}))
    };

    Ok((status, Json(parsed)))
}

#[get("/webhook/<webhook_id>/<webhook_token>")]
pub async fn webhook_info(
    webhook_id: u64,
    webhook_token: &str,
) -> Result<Json<serde_json::Value>, ApiError> {
    let url = format!(
        "{}/webhooks/{}/{}",
        DISCORD_API_URL, webhook_id, webhook_token
    );

    let response = minreq::get(&url)
        .send()
        .map_err(|_| ApiError::message(Status::BadGateway, "Failed to forward request"))?;

    let status_code = status_from_code(response.status_code)?;
    let body = response
        .as_str()
        .map_err(|_| {
            ApiError::message(Status::InternalServerError, "Failed to read response body")
        })?
        .to_string();

    if status_code != Status::Ok {
        return Err(ApiError::message(status_code, &body));
    }

    let parsed: serde_json::Value = serde_json::from_str(&body).map_err(|_| {
        ApiError::message(
            Status::InternalServerError,
            "Failed to parse Discord response",
        )
    })?;

    Ok(Json(parsed))
}
