use crate::{db::entry, response::data};
use anyhow::Result;
use rocket::http::{ContentType, Status};
use uuid::Uuid;

pub mod rssbox_android {
    use super::*;
    use crate::db::RSSBOX_ANDROID_FEEDBACK_TABLE;

    async fn _all() -> Result<String> {
        let entrys = entry::select_all(RSSBOX_ANDROID_FEEDBACK_TABLE).await?;
        Ok(serde_json::to_string(&entrys)?)
    }

    #[get("/feedbacks")]
    pub async fn all() -> data::Data {
        match _all().await {
            Ok(entrys) => data::Data::new(entrys.as_bytes().to_vec(), ContentType::JSON),
            Err(e) => data::Data::new_with_status(
                e.to_string().as_bytes().to_vec(),
                ContentType::Plain,
                Status::InternalServerError,
            ),
        }
    }

    #[post("/feedback", data = "<input>")]
    pub async fn insert(input: &str) -> data::Data {
        match entry::insert(
            RSSBOX_ANDROID_FEEDBACK_TABLE,
            &Uuid::new_v4().to_string(),
            input,
        )
        .await
        {
            Err(e) => data::Data::new_with_status(
                e.to_string().as_bytes().to_vec(),
                ContentType::Plain,
                Status::InternalServerError,
            ),
            _ => data::Data::default(),
        }
    }

    #[get("/feedback/delete?<uuid>")]
    pub async fn delete(uuid: &str) -> data::Data {
        match entry::delete(RSSBOX_ANDROID_FEEDBACK_TABLE, uuid).await {
            Err(e) => data::Data::new_with_status(
                e.to_string().as_bytes().to_vec(),
                ContentType::Plain,
                Status::InternalServerError,
            ),
            _ => data::Data::default(),
        }
    }
}
