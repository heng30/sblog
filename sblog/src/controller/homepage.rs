use crate::response::data;
use crate::response::template;
use rocket::http::ContentType;

#[get("/")]
pub fn homepage() -> data::Data {
    let homepage = template::render::homepage();
    data::Data::new(homepage.as_bytes().to_vec(), ContentType::HTML)
}
