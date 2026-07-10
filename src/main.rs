use discord_webhook_proxy::{
    WebhookQueue, queue_process_database, rocket_routes::webhook_proxy, setup_tracing,
};
use rocket::fairing::AdHoc;

#[rocket::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    setup_tracing();

    rocket::build()
        .manage(WebhookQueue::default())
        .mount("/", rocket::routes![webhook_proxy])
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

    rocket::build();

    Ok(())
}
