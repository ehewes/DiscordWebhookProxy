use discord_webhook_proxy::api::routes::webhook_proxy;

#[rocket::launch]
fn rocket() -> _ {
    rocket::build().mount("/", rocket::routes![webhook_proxy])
}
