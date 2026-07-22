use discord_webhook_proxy::{
    queue_process_database,
    rocket_routes::{
        webhook_info, webhook_info_discord_compatibility_path, webhook_proxy,
        webhook_proxy_discord_compatibility_path,
    },
    setup_tracing, WebhookQueue,
};
use rocket::fairing::AdHoc;

#[rocket::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    setup_tracing();

    rocket::build()
        .manage(WebhookQueue::default())
        .mount(
            "/",
            rocket::routes![
                webhook_info,
                webhook_info_discord_compatibility_path,
                webhook_proxy,
                webhook_proxy_discord_compatibility_path,
            ],
        )
        .attach(AdHoc::on_liftoff(
            "Processing leftover webhook queue",
            |rocket| {
                Box::pin(async move {
                    let queue = rocket
                        .state::<WebhookQueue>()
                        .expect("Failed to get rocket state WebhookQueue");
                    queue_process_database(queue.get_database()).await;
                })
            },
        ))
        .launch()
        .await?;

    Ok(())
}
