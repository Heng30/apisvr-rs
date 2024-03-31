use rocket::{
    http::{ContentType, Status},
    response::{Responder, Response, Result},
    Request,
};
use std::io::Cursor;

pub struct Data {
    data: Vec<u8>,
    r#type: ContentType,
    pub status: Status,
}

impl Default for Data {
    fn default() -> Self {
        Data {
            data: vec![],
            r#type: ContentType::Plain,
            status: Status::Ok,
        }
    }
}

impl Data {
    pub fn new(data: Vec<u8>, t: ContentType) -> Self {
        Self {
            data,
            r#type: t,
            status: Status::Ok,
        }
    }

    pub fn new_with_status(data: Vec<u8>, t: ContentType, status: Status) -> Self {
        Self {
            data,
            r#type: t,
            status,
        }
    }
}

impl<'a> Responder<'a, 'static> for Data {
    fn respond_to(self, _: &'a Request<'_>) -> Result<'static> {
        Response::build()
            .header(self.r#type)
            .status(self.status)
            .sized_body(self.data.len(), Cursor::new(self.data))
            .ok()
    }
}
