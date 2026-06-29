use discord_webhook_proxy::{
    api::{discord::webhook_proxy, webhook_queue::start_webhook_queue},
    setup_tracing,
};

#[rocket::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    setup_tracing();

    let queue_sender = start_webhook_queue();

    rocket::build()
        .manage(queue_sender)
        .mount("/", rocket::routes![webhook_proxy])
        .launch()
        .await?;

    Ok(())
}
