use super::super::{
    ApiError, ApiResult,
    webhook::{Webhook, get_fallback_cooldown_secs},
};
use rocket::http::Status;
use tracing::info;

const DISCORD_API_URL: &str = "https://discord.com/api";

pub async fn forward_webhook(webhook: &Webhook) -> Result<(Status, String, u64), ApiError> {
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
        .map_err(|_| ApiError::message(Status::BadGateway, "Failed to forward request"))?;

    let response_status_code = status_from_code(response.status_code)?;
    let response_body = response
        .as_str()
        .map_err(|_| ApiError::message(Status::InternalServerError, "Failed to encode the body"))?
        .to_string();

    let retry_after_secs = get_retry_after_secs_from_header(&response.headers);

    info!(
        "Proxied request:\n- Webhook ID: {}\n- Status Code: {response_status_code}\n- Webhook Body: {:#?}",
        webhook.id, webhook.body
    );

    Ok((response_status_code, response_body, retry_after_secs))
}

fn get_retry_after_secs_from_header(response_headers: &[(String, String)]) -> u64 {
    let fallback_cooldown_secs = get_fallback_cooldown_secs();

    response_headers
        .iter()
        .find(|(name, _)| name.eq_ignore_ascii_case("Retry-After"))
        .and_then(|(_, value)| value.parse::<u64>().ok())
        .unwrap_or(fallback_cooldown_secs)
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
