use super::{structs::DiscordWebhookBody, ApiError, ApiResult};
use rocket::{
    http::Status,
    post,
    serde::json::{serde_json, Json},
};

pub const DISCORD_WEBHOOK_ENDPOINT: &str = "https://discord.com/api/webhooks";

#[post("/<webhook_id>/<webhook_token>", data = "<body>")]
pub async fn webhook_proxy(
    webhook_id: &str,
    webhook_token: &str,
    body: Json<DiscordWebhookBody>,
) -> ApiResult<(Status, Json<String>)> {
    let url = format!("{DISCORD_WEBHOOK_ENDPOINT}/{webhook_id}/{webhook_token}");
    let json_body = serde_json::to_string(&body.0)
        .map_err(|_| ApiError::new(Status::InternalServerError, "Failed to encode the body"))?;

    let response = minreq::post(url)
        .with_header("Content-Type", "application/json")
        .with_body(json_body)
        .send()
        .map_err(|_| {
            ApiError::new(
                Status::InternalServerError,
                "Failed to send the request to Discord",
            )
        })?;

    let response_status = status_from_code(response.status_code)?;
    let response_body = response
        .as_str()
        .map_err(|_| ApiError::new(Status::InternalServerError, "Failed to parse the response"))?;

    Ok((response_status, Json(response_body.to_string())))
}

fn status_from_code(code: i32) -> ApiResult<Status> {
    let code: u16 = code.try_into().map_err(|_| {
        ApiError::new(
            Status::InternalServerError,
            "Failed to convert the status code",
        )
    })?;

    let status = Status::from_code(code).ok_or_else(|| {
        ApiError::new(
            Status::InternalServerError,
            "Failed to convert the status code",
        )
    })?;

    Ok(status)
}
