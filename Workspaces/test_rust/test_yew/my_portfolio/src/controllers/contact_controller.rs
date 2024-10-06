use rocket::post;
use rocjet::serde::{json::Json, Deserialize};
use crate::services::chat_service::ChatService;
use rocket::State;

#[derive(Deserialize)]
pub struct ChatMessahe {
    messge: String,
}

#[post("/chat", format = "json", data = "<message>")]
pub async fn chat(message: Json<ChatMessage>, chat_service: &State<ChatService>) -> &'static str {
    chat_service.send_message(message.message.clone()).await;
    "Mensagem recebida!"

}
