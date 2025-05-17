mod api;
use api::discord::webhook_proxy;
use api::webhook_queue::start_webhook_queue;

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let queue_sender = start_webhook_queue();
    rocket::build()
        .manage(queue_sender)
        .mount("/", rocket::routes![webhook_proxy])
        .launch()
        .await?;
    Ok(())
}