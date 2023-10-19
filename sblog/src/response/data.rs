use rocket::http::ContentType;
use rocket::response::{Responder, Response, Result};
use rocket::Request;
use std::io::Cursor;

pub struct Data {
    data: Vec<u8>,
    r#type: ContentType,
}

impl Data {
    pub fn new(data: Vec<u8>, t: ContentType) -> Self {
        Self { data, r#type: t }
    }
}

impl<'a> Responder<'a, 'static> for Data {
    fn respond_to(self, _: &'a Request<'_>) -> Result<'static> {
        Response::build()
            .header(self.r#type)
            .sized_body(self.data.len(), Cursor::new(self.data))
            .ok()
    }
}
