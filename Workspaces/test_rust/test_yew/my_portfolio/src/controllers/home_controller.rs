use rocket::get;
use rocket::response::content::Html;

#[get("/")]
pub fn index() -> Html<&'static str> {
    Html(include_str!("../views/home.html"))
}

