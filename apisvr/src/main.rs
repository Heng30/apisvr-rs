#[macro_use]
extern crate rocket;

#[macro_use]
extern crate lazy_static;

use chrono::Local;
use env_logger::fmt::Color as LColor;
use log::debug;
use rocket::config::Config as RConfig;
use rocket::Rocket;
use std::io::Write;
use std::net::IpAddr;
use std::str::FromStr;

mod config;
mod controller;
mod db;
mod middleware;
mod response;

use config::conf;
use middleware::cors;

#[launch]
async fn rocket() -> _ {
    init_logger();

    debug!("start...");

    conf::init();
    response::init();
    db::init(config::db_path().to_str().expect("db_path is invalid")).await;

    server_start()
}

fn server_start() -> Rocket<rocket::Build> {
    let mut config = RConfig::release_default();
    config.port = conf::server().listen_port;
    config.address = IpAddr::from_str(conf::server().listen_address.as_str()).unwrap();

    rocket::custom(config)
        .attach(cors::Cors)
        .mount(
            "/",
            routes![
                controller::ping::ping,
                controller::cryptocurrency::latest,
                controller::cryptocurrency::greed_fear,
                controller::market::latest,
            ],
        )
        .mount(
            "/rssbox/android",
            routes![
                controller::feedback::rssbox_android::all,
                controller::feedback::rssbox_android::insert,
                controller::feedback::rssbox_android::delete,
            ],
        )
}

fn init_logger() {
    env_logger::builder()
        .format(|buf, record| {
            let ts = Local::now().format("%Y-%m-%d %H:%M:%S");
            let mut level_style = buf.style();
            match record.level() {
                log::Level::Warn => level_style.set_color(LColor::Red).set_bold(true),
                log::Level::Error => level_style.set_color(LColor::Yellow).set_bold(true),
                log::Level::Info => level_style.set_color(LColor::Green).set_bold(true),
                _ => level_style.set_color(LColor::Blue).set_bold(true),
            };

            writeln!(
                buf,
                "[{} {} {} {}] {}",
                ts,
                level_style.value(record.level()),
                record
                    .file()
                    .unwrap_or("None")
                    .split('/')
                    .last()
                    .unwrap_or("None"),
                record.line().unwrap_or(0),
                record.args()
            )
        })
        .init();
}
