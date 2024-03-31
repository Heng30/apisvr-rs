use super::*;
use crate::response::data;

pub mod rssbox_android {
    use super::*;
    use crate::db::RSSBOX_ANDROID_FEEDBACK_TABLE;

    #[get("/feedbacks")]
    pub async fn all() -> data::Data {
        com_all(RSSBOX_ANDROID_FEEDBACK_TABLE).await
    }

    #[post("/feedback", format = "application/json", data = "<input>")]
    pub async fn insert(input: &str) -> data::Data {
        com_insert(RSSBOX_ANDROID_FEEDBACK_TABLE, input).await
    }

    #[delete("/feedback/<uuid>")]
    pub async fn delete(uuid: &str) -> data::Data {
        com_delete(RSSBOX_ANDROID_FEEDBACK_TABLE, uuid).await
    }
}
