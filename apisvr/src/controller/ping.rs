use crate::response::data;
use rocket::http::ContentType;

#[get("/ping")]
pub fn ping() -> data::Data {
    data::Data::new("pong".as_bytes().to_vec(), ContentType::Plain)
}
