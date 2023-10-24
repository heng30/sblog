use crate::response::data;
use crate::response::template;
use rocket::http::ContentType;

#[get("/search?<keyword>")]
pub async fn search(keyword: &str) -> data::Data {
    let html = template::render::search(keyword).await;
    data::Data::new(html.as_bytes().to_vec(), ContentType::HTML)
}
