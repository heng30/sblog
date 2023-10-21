use crate::response::template;
use rocket::Request;

#[allow(unused)]
#[catch(404)]
pub fn not_found(_: &Request) -> String {
    template::nofound::TEMPLATE.to_string()
}
