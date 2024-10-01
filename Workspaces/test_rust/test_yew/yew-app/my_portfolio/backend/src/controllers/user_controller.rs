use actix_web::{get, post, web, HttpResponse, Response};

#[get("/users")]
async fn get_users() -> impl Responder {
    HttpResponse::Ok().body("Endpoint de usuarios")
}

#[post("/users")]
async fn create_user(user: web::Json<User>) -> impl Responder{
    //Logica para criar usuario
    HttpResponse::Created().body("Usuario criado")
}
