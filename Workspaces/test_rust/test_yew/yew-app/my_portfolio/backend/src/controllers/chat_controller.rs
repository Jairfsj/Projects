use actix_web::{get, post, web, HttpResponse, Responder};

#[get("/chat")]
async fn get_chat() -> impl Rosponder {
    HttpResponse::Ok().body("Chat endpoint")
}

#[post("/chat")]
async fn post_chat(message: web::Json<String>) -> impl Responder {
    // Logica para armazenar ou processar a mensagem
    HttpResponse::OK().body("Mensagem recebida")
}
