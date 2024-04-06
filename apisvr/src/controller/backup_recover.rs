use super::*;
use crate::response::data;

pub mod rssbox_android {
    use super::*;
    use crate::db::RSSBOX_ANDROID_BACKUP_TABLE;
    use rocket::data::{Data, Limits, ToByteUnit};

    #[post("/backup?<api_token>", format = "application/json", data = "<input>")]
    pub async fn backup(api_token: &str, input: Data<'_>, limits: &Limits) -> data::Data {
        let limit = limits.get("input").unwrap_or(1.mebibytes());
        match input.open(limit).into_string().await {
            Err(e) => data::Data::new_with_status(
                e.to_string().as_bytes().to_vec(),
                ContentType::Plain,
                Status::InternalServerError,
            ),
            Ok(v) => com_update(RSSBOX_ANDROID_BACKUP_TABLE, api_token, &v.value).await,
        }
    }

    #[get("/recover?<api_token>")]
    pub async fn recover(api_token: &str) -> data::Data {
        com_select(RSSBOX_ANDROID_BACKUP_TABLE, api_token).await
    }
}
