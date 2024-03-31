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
                let (is_prefix, is_auth) = rssbox_android(request);
                if is_prefix && !is_auth {
                    request.set_method(Method::Get);
                    request.set_uri(Origin::parse("/unauthorized").unwrap());
                }
            }
            _ => (),
        }
    }
}

fn rssbox_android(request: &Request<'_>) -> (bool, bool) {
    let prefix_paths = vec![
        "/rssbox/android/feedback",
        "/rssbox/rss/list/cn",
        "/rssbox/rss/list/en",
    ];

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
