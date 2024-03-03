pub mod cryptocurrency;
pub mod market;
pub mod data;

use rocket::tokio;
use std::sync::Mutex;

lazy_static! {
    pub static ref RUNTIME: Mutex<tokio::runtime::Runtime> =
        Mutex::new(tokio::runtime::Runtime::new().unwrap());
}

pub fn init() {
    cryptocurrency::init();
    market::init();
}
