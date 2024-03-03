use crate::response::cryptocurrency;
use crate::response::data;
use rocket::http::ContentType;
use rocket::http::Status;

#[get("/cryptocurrency/latest")]
pub async fn latest() -> data::Data {
    if let Some(v) = cryptocurrency::latest_cache().await {
        data::Data::new(v.as_bytes().to_vec(), ContentType::JSON)
    } else {
        match cryptocurrency::fetch_latest().await {
            Ok(v) => data::Data::new(v.as_bytes().to_vec(), ContentType::JSON),
            Err(e) => {
                let mut d = data::Data::new(e.to_string().as_bytes().to_vec(), ContentType::Plain);
                d.status = Status::NotFound;
                d
            }
        }
    }
}
