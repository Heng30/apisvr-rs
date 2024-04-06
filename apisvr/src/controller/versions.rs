use super::*;
use crate::response::data;

use crate::db::VERSIONS_TABLE;

#[post("/latest/version?<q>", format = "application/json", data = "<input>")]
pub async fn update(q: &str, input: &str) -> data::Data {
    com_update(VERSIONS_TABLE, q, input).await
}

#[get("/latest/version?<q>")]
pub async fn get(q: &str) -> data::Data {
    com_select(VERSIONS_TABLE, q).await
}
