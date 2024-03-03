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

impl Data {
    pub fn new(data: Vec<u8>, t: ContentType) -> Self {
        Self {
            data,
            r#type: t,
            status: Status::Ok,
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
