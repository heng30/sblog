use crate::response::data;
use crate::response::template;
use rocket::http::ContentType;

#[get("/?<page>")]
pub async fn homepage(page: Option<usize>) -> data::Data {
    let homepage = template::render::homepage(page.unwrap_or(0)).await;
    data::Data::new(homepage.as_bytes().to_vec(), ContentType::HTML)
}
