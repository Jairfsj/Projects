use actix::prelude::*;
use actix_web::{web, App, HttpServer, HttpRequest, HttpResponse, Result};
use actix_web_actors::ws;

struct ChatSession;

impl Actor for ChatSession {
    type Context = ws::WebsocketContext<Self>;
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for ChatSession {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        if let Ok(ws::Message::Text(text)) = msg {
            ctx.text(text);  // Echo de volta a mensagem recebida
        }
    }
}

async fn chat_route(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse> {
    ws::start(ChatSession {}, &req, stream)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/chat", web::get().to(chat_route))  // Rota para chat em tempo real
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
