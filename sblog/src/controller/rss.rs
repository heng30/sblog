use crate::response::data;
use crate::response::template;
use rocket::http::ContentType;

#[get("/rss")]
pub async fn rss() -> data::Data {
    let rss = template::render::rss().await;
    data::Data::new(rss.as_bytes().to_vec(), ContentType::XML)
}
