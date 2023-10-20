use crate::response::data;
use crate::response::template;
use rocket::http::ContentType;
use rocket::http::Status;

#[get("/post/<id>")]
pub async fn post(id: &str) -> data::Data {
    match template::render::post(id).await {
        Some(post) => data::Data::new(post.as_bytes().to_vec(), ContentType::HTML),
        _ => {
            let mut d = data::Data::new(vec![], ContentType::HTML);
            d.status = Status::NotFound;
            d
        }
    }
}
