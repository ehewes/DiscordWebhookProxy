use discord_webhook_proxy::{
    WebhookQueue, queue_process_database, rocket_routes::webhook_proxy, setup_tracing,
};
use rocket::{catch, catchers, fairing::AdHoc};

#[catch(404)]
fn not_found() -> &'static str {
    "Not found"
}

#[rocket::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    setup_tracing();

    rocket::build()
        .manage(WebhookQueue::default())
        .register("/", catchers![not_found])
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
