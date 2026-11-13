use serde::Serialize;

#[derive(Serialize)]
pub struct WebhookPayload {
    pub content: String,
}
