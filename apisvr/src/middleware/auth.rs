use crate::config;
use rocket::fairing::{Fairing, Info, Kind};
use rocket::{
    http::{hyper::header, uri::Origin, Method, Status},
    Data, Request,
};

pub struct Auth;

#[get("/unauthorized")]
pub fn unauthorized() -> Status {
    Status::Unauthorized
}

#[rocket::async_trait]
impl Fairing for Auth {
    fn info(&self) -> Info {
        Info {
            name: "Check auth headers from request",
            kind: Kind::Request,
        }
    }

    async fn on_request(&self, request: &mut Request<'_>, _: &mut Data<'_>) {
        match request.method() {
            Method::Delete => {
                let prefix_paths = vec![
                    "/rssbox/android/feedback",
                    "/rssbox/rss/list/cn",
                    "/rssbox/rss/list/en",
                ];

                let token = config::auth_token().admin;
                if !handle_unauthorized(request, prefix_paths, &token) {
                    return;
                }
            }
            Method::Get => {
                let prefix_paths = vec!["/rssbox/android/recover"];
                let token = config::auth_token().rssbox_android;
                if !handle_unauthorized(request, prefix_paths, &token) {
                    return;
                }
            }
            Method::Post => {
                let prefix_paths = vec!["/latest/version"];
                let token = config::auth_token().admin;
                if !handle_unauthorized(request, prefix_paths, &token) {
                    return;
                }

                let prefix_paths = vec!["/rssbox/android/backup"];
                let token = config::auth_token().rssbox_android;
                if !handle_unauthorized(request, prefix_paths, &token) {
                    return;
                }
            }
            _ => (),
        }
    }
}

fn handle_unauthorized(request: &mut Request, prefix_paths: Vec<&str>, token: &str) -> bool {
    let mut is_continue = true;

    let (is_prefix, is_auth) = rssbox_android(request, prefix_paths, &token);
    if is_prefix && !is_auth {
        navigate_unauthorized(request);
        is_continue = false;
    }

    is_continue
}

fn navigate_unauthorized(request: &mut Request) {
    request.set_method(Method::Get);
    request.set_uri(Origin::parse("/unauthorized").unwrap());
}

fn rssbox_android(request: &Request, prefix_paths: Vec<&str>, token: &str) -> (bool, bool) {
    let path = request.uri().path().as_str();

    if prefix_paths
        .into_iter()
        .filter(|&item| path.starts_with(item))
        .collect::<Vec<_>>()
        .is_empty()
    {
        return (false, false);
    }

    if token.is_empty() {
        return (true, true);
    }

    if let Some(http_token) = request.headers().get(header::AUTHORIZATION.as_str()).next() {
        let http_token = http_token
            .split_whitespace()
            .into_iter()
            .collect::<Vec<_>>();
        if http_token.len() != 2 && http_token[0] != "Bearer" {
            return (true, false);
        }
        return (true, token == http_token[1]);
    }

    return (true, false);
}
