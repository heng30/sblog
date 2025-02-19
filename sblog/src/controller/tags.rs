use crate::response::data;
use crate::response::template;
use rocket::http::ContentType;

#[get("/tags")]
pub async fn tags() -> data::Data {
    let tags = template::render::tags().await;
    data::Data::new(tags.as_bytes().to_vec(), ContentType::HTML)
}
