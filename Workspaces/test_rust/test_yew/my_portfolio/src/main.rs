#[macro_use] extern crate rocket;

mod controllers;
mod services;
mod models;

use rocket::routes;


#[launch]

fn rocket() -> _{
    rocket::build()
        .mount("/", routes![
            controllers::home_controller::index,
            controllers::skills_controller::skills,
            controllers::contact_controller::contact
        ])
        .manage(services::chat_service::ChatService::new())

}
