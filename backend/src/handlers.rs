use axum::Json;
use serde::Serialize;

#[derive(Serialize)]
pub struct Message {
    message: String,
    author: String,
}

pub async fn root() -> Json<Message> {
    let message_data = Message {
        message: "¡Hola desde nuestro BaaS con Rust!".to_string(),
        author: "Gemini".to_string(),
    };
    Json(message_data)
}