use rocket::serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct DiscordWebhookBody {
    content: Option<String>,
    username: Option<String>,
    avatar_url: Option<String>,
    tts: Option<bool>,
    embeds: Option<Vec<Embed>>,
    allowed_mentions: Option<AllowedMentions>,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Embed {
    title: Option<String>,
    description: Option<String>,
    url: Option<String>,
    color: Option<u32>,
    footer: Option<Footer>,
    image: Option<Image>,
    thumbnail: Option<Image>,
    author: Option<Author>,
    fields: Option<Vec<Field>>,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Footer {
    text: String,
    icon_url: Option<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Image {
    url: String,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Author {
    name: String,
    url: Option<String>,
    icon_url: Option<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Field {
    name: String,
    value: String,
    inline: Option<bool>,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct AllowedMentions {
    parse: Option<Vec<String>>,
    roles: Option<Vec<String>>,
    users: Option<Vec<String>>,
}
