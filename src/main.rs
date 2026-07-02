use discord_webhook_proxy::{
    api::{discord::webhook_proxy, webhook::WebhookQueue},
    setup_tracing,
};

#[rocket::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    setup_tracing();

    rocket::build()
        .manage(WebhookQueue::default())
        .mount("/", rocket::routes![webhook_proxy])
        .launch()
        .await?;

    Ok(())
}
