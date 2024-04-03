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
                if !handle_unauthorized_rssbox(request, prefix_paths) {
                    return;
                }
            }
            Method::Get => {
                let prefix_paths = vec!["/rssbox/android/recover"];
                if !handle_unauthorized_rssbox(request, prefix_paths) {
                    return;
                }
            }
            Method::Post => {
                let prefix_paths = vec!["/rssbox/android/backup"];
                if !handle_unauthorized_rssbox(request, prefix_paths) {
                    return;
                }
            }
            _ => (),
        }
    }
}

fn handle_unauthorized_rssbox(request: &mut Request, prefix_paths: Vec<&str>) -> bool {
    let mut is_continue = true;

    let (is_prefix, is_auth) = rssbox_android(request, prefix_paths);
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

fn rssbox_android(request: &Request, prefix_paths: Vec<&str>) -> (bool, bool) {
    let path = request.uri().path().as_str();
    log::debug!("{path:?}");

    if prefix_paths
        .into_iter()
        .filter(|&item| path.starts_with(item))
        .collect::<Vec<_>>()
        .is_empty()
    {
        return (false, false);
    }

    let token = config::auth_token().rssbox_android;
    if token.is_empty() {
        return (true, true);
    }

    log::warn!("{}", header::AUTHORIZATION.as_str());
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
