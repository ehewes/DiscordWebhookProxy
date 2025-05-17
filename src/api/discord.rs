use rocket::post;
use super::webhook_queue::{QueueSender, Webhook};
use super::{ApiResult, ApiError};
use rocket::serde::json::serde_json;
use rocket::{http::Status, serde::json::Json, State};

#[post("/<webhook_id>/<webhook_token>", data = "<body>")]
pub async fn webhook_proxy(
    webhook_id: u64,
    webhook_token: &str,
    body: Json<serde_json::Value>,
    queue_sender: &State<QueueSender>,
) -> ApiResult<(Status, Json<serde_json::Value>)> {
    let (response_status, response_body, _) =
        forward_webhook_request(webhook_id, webhook_token, &body.to_string())?;

    match response_status.code {
        429 => {
            queue_sender
                .send(Webhook {
                    id: webhook_id,
                    token: webhook_token.to_string(),
                    body: body.to_string(),
                })
                .await
                .map_err(|_| {
                    ApiError::message(Status::InternalServerError, "Failed to queue request")
                })?;

            Ok((Status::Accepted, Json(serde_json::json!({"queued": true}))))
        },
        _ => {
            let response_body: serde_json::Value =
                serde_json::from_str::<serde_json::Value>(&response_body)
                    .map_or(serde_json::json!({}), |body| body);

            Ok((response_status, Json(response_body)))
        },
    }
}

pub fn forward_webhook_request(
    webhook_id: u64,
    webhook_token: &str,
    body: &str,
) -> Result<(Status, String, minreq::Response), ApiError> {
    let url = format!("https://discord.com/api/webhooks/{webhook_id}/{webhook_token}");

    let response = minreq::post(&url)
        .with_header("Content-Type", "application/json")
        .with_body(body)
        .send()
        .map_err(|_| ApiError::message(Status::InternalServerError, "Failed to forward request"))?;

    let response_status_code = status_from_code(response.status_code)?;
    let response_body = response
        .as_str()
        .map_err(|_| ApiError::message(Status::InternalServerError, "Failed to encode the body"))?
        .to_string();

    Ok((response_status_code, response_body, response))
}

fn status_from_code(code: i32) -> ApiResult<Status> {
    let code: u16 = code.try_into().map_err(|_| {
        ApiError::message(
            Status::InternalServerError,
            "Failed to convert the status code",
        )
    })?;

    let status = Status::from_code(code).ok_or_else(|| {
        ApiError::message(
            Status::InternalServerError,
            "Failed to convert the status code",
        )
    })?;

    Ok(status)
}