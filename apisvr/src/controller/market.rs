use crate::response::{data, market};
use rocket::http::{ContentType, Status};

#[get("/market/latest")]
pub async fn latest() -> data::Data {
    if let Some(v) = market::latest_cache().await {
        data::Data::new(v.as_bytes().to_vec(), ContentType::JSON)
    } else {
        match market::fetch().await {
            Ok(v) => data::Data::new(v.as_bytes().to_vec(), ContentType::JSON),
            Err(e) => {
                let mut d = data::Data::new(e.to_string().as_bytes().to_vec(), ContentType::Plain);
                d.status = Status::InternalServerError;
                d
            }
        }
    }
}
