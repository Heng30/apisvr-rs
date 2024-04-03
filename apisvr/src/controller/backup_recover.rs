use super::*;
use crate::response::data;

pub mod rssbox_android {
    use super::*;
    use crate::db::RSSBOX_ANDROID_BACKUP_TABLE;

    #[post("/backup?<api_token>", format = "application/json", data = "<input>")]
    pub async fn backup(api_token: &str, input: &str) -> data::Data {
        com_update(RSSBOX_ANDROID_BACKUP_TABLE, api_token, input).await
    }

    #[get("/recover?<api_token>")]
    pub async fn recover(api_token: &str) -> data::Data {
        com_select(RSSBOX_ANDROID_BACKUP_TABLE, api_token).await
    }
}
