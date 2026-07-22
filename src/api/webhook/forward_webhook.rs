use super::{
    super::ApiError, get_fallback_cooldown_secs, status_from_code, Webhook, DISCORD_API_URL,
};
use rocket::http::Status;

pub async fn forward_webhook(webhook: &Webhook) -> Result<(Status, String, u64), ApiError> {
    let client = reqwest::Client::new();

    let url = format!(
        "{}/webhooks/{}/{}",
        DISCORD_API_URL, webhook.id, webhook.token
    );

    let response = client
        .post(&url)
        .header("Content-Type", "application/json")
        .json(&webhook.body)
        .send()
        .await
        .map_err(|_| ApiError::message(Status::BadGateway, "Failed to forward request"))?;

    let headers = response.headers().clone();
    let response_status_code = status_from_code(response.status().as_u16())?;

    let response_body = response.text().await.map_err(|_| {
        ApiError::message(Status::InternalServerError, "Failed to read response body")
    })?;

    let retry_after_secs = get_retry_after_secs_from_header(&headers);

    Ok((response_status_code, response_body, retry_after_secs))
}

fn get_retry_after_secs_from_header(headers: &reqwest::header::HeaderMap) -> u64 {
    let fallback_cooldown_secs = get_fallback_cooldown_secs();

    headers
        .get(reqwest::header::RETRY_AFTER)
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.parse::<u64>().ok())
        .unwrap_or(fallback_cooldown_secs)
}
