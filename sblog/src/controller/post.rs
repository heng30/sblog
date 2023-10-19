use crate::response::data;
use rocket::http::ContentType;

#[get("/post/<id>")]
pub fn post(id: &str) -> data::Data {
    // TODO: Generate html
    data::Data::new(id.as_bytes().to_vec(), ContentType::HTML)
}
