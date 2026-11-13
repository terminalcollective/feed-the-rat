use serde::Serialize;

#[derive(Serialize)]
pub struct WebhookPayload {
    /// The message contents (up to 2000 characters)
    pub content: Option<String>,
    /// Override the default username of the webhook
    pub username: Option<String>,
    /// Override the default avatar of the webhook
    pub avatar_url: Option<String>,
    /// True if this is a TTS message
    pub tts: Option<bool>,
    /// Embedded `rich` content
    pub embeds: Option<Vec<DiscordEmbed>>,
}

#[derive(Serialize)]
pub struct DiscordEmbed {
    pub title: Option<String>,
    #[serde(rename = "type")]
    pub embed_type: Option<String>,
    pub description: Option<String>,
    pub url: Option<String>,
    pub timestamp: Option<String>,
    pub color: Option<u32>,
    pub footer: Option<EmbedFooter>,
    pub image: Option<EmbedImage>,
    pub thumbnail: Option<EmbedImage>,
    pub author: Option<EmbedAuthor>,
    pub fields: Option<Vec<EmbedField>>,
}

#[derive(Serialize)]
pub struct EmbedAuthor {
    pub name: Option<String>,
    pub url: Option<String>,
    pub icon_url: Option<String>,
    pub proxy_icon_url: Option<String>,
}

#[derive(Serialize)]
pub struct EmbedImage {
    pub url: String,
}

#[derive(Serialize)]
pub struct EmbedFooter {
    pub text: String,
    pub icon_url: Option<String>,
    pub proxy_icon_url: Option<String>,
}

#[derive(Serialize)]
pub struct EmbedField {
    pub name: String,
    pub value: String,
    pub inline: bool,
}
