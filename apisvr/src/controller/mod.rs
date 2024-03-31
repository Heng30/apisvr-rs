pub mod cryptocurrency;
pub mod feedback;
pub mod market;
pub mod ping;
pub mod rss;

use crate::{db::entry, response::data};
use anyhow::Result;
use rocket::http::{ContentType, Status};
use uuid::Uuid;

async fn _all(table: &str) -> Result<String> {
    let entrys = entry::select_all(table).await?;
    Ok(serde_json::to_string(&entrys)?)
}

async fn com_all(table: &str) -> data::Data {
    match _all(table).await {
        Ok(entrys) => data::Data::new(entrys.as_bytes().to_vec(), ContentType::JSON),
        Err(e) => data::Data::new_with_status(
            e.to_string().as_bytes().to_vec(),
            ContentType::Plain,
            Status::InternalServerError,
        ),
    }
}

async fn com_insert(table: &str, input: &str) -> data::Data {
    match entry::insert(table, &Uuid::new_v4().to_string(), input).await {
        Err(e) => data::Data::new_with_status(
            e.to_string().as_bytes().to_vec(),
            ContentType::Plain,
            Status::InternalServerError,
        ),
        _ => data::Data::default(),
    }
}

async fn com_delete(table: &str, uuid: &str) -> data::Data {
    match entry::delete(table, uuid).await {
        Err(e) => data::Data::new_with_status(
            e.to_string().as_bytes().to_vec(),
            ContentType::Plain,
            Status::InternalServerError,
        ),
        _ => data::Data::default(),
    }
}
