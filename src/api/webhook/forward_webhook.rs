use super::super::{ApiError, ApiResult, webhook::Webhook};
use rocket::http::Status;
use tracing::info;

const DISCORD_API_URL: &str = "https://discord.com/api";

pub fn forward_webhook(webhook: &Webhook) -> Result<(Status, String, minreq::Response), ApiError> {
    let url = format!(
        "{}/webhooks/{}/{}",
        DISCORD_API_URL, webhook.id, webhook.token
    );

    let response = minreq::post(&url)
        .with_header("Content-Type", "application/json")
        .with_body(serde_json::to_vec(&webhook.body).map_err(|_| {
            ApiError::message(
                Status::InternalServerError,
                "Failed to serialize webhook body",
            )
        })?)
        .send()
        .map_err(|_| ApiError::message(Status::InternalServerError, "Failed to forward request"))?;

    let response_status_code = status_from_code(response.status_code)?;
    let response_body = response
        .as_str()
        .map_err(|_| ApiError::message(Status::InternalServerError, "Failed to encode the body"))?
        .to_string();

    info!(
        "Proxied request:\n- Webhook ID: {}\n- Status Code: {response_status_code}\n- Webhook Body: {:#?}",
        webhook.id, webhook.body
    );

    Ok((response_status_code, response_body, response))
}

fn status_from_code(code: u16) -> ApiResult<Status> {
    let status = Status::from_code(code).ok_or_else(|| {
        ApiError::message(
            Status::InternalServerError,
            "Failed to convert the status code",
        )
    })?;

    Ok(status)
}
