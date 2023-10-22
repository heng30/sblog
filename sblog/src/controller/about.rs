use crate::response::data;
use crate::response::template;
use rocket::http::ContentType;

#[get("/about")]
pub async fn about() -> data::Data {
    let about = template::render::about().await;
    data::Data::new(about.as_bytes().to_vec(), ContentType::HTML)
}
