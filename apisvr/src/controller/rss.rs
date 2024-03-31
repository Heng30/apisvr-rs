use super::*;
use crate::{
    db::{RSSBOX_ANDROID_RSS_CN_TABLE, RSSBOX_ANDROID_RSS_EN_TABLE},
    response::data,
};

macro_rules! table_name {
    ($language: expr) => {
        if $language == "cn" {
            RSSBOX_ANDROID_RSS_CN_TABLE
        } else {
            RSSBOX_ANDROID_RSS_EN_TABLE
        }
    };
}

#[get("/<language>")]
pub async fn all(language: &str) -> data::Data {
    com_all(table_name!(language)).await
}

#[post("/<language>", format = "application/json", data = "<input>")]
pub async fn insert(language: &str, input: &str) -> data::Data {
    com_insert(table_name!(language), input).await
}

#[delete("/<language>/<uuid>")]
pub async fn delete(language: &str, uuid: &str) -> data::Data {
    com_delete(table_name!(language), uuid).await
}
