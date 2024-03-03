use super::RUNTIME;
use crate::conf;
use anyhow::Result;
use reqwest::{
    header::{HeaderMap, ACCEPT},
    Client, Proxy,
};
use rocket::tokio::{self, sync::Mutex, time::Duration};

lazy_static! {
    static ref LATEST: Mutex<Option<String>> = Mutex::new(None);
}

pub async fn latest_cache() -> Option<String> {
    LATEST.lock().await.clone()
}

pub fn init() {
    latest_timer();
}

fn latest_timer() {
    RUNTIME.lock().unwrap().spawn(async move {
        log::debug!("latest timer start...");

        let mut count = 0_u64;
        let interval = u64::max(10, conf::timer().coinmarketcap_latest);
        loop {
            if count % interval == 0 {
                match fetch_latest().await {
                    Ok(v) => *LATEST.lock().await = Some(v),
                    Err(e) => log::warn!("fetch_latest error: {e:?}"),
                }
            }

            tokio::time::sleep(Duration::from_secs(1)).await;
            count += 1;
        }
    });
}

// curl -H "X-CMC_PRO_API_KEY: $API_KEY" -H "Accept: application/json" -d "start=1&limit=100&convert=USD&aux=cmc_rank" -G https://pro-api.coinmarketcap.com/v1/cryptocurrency/listings/latest
pub async fn fetch_latest() -> Result<String> {
    let (socket5, api_key) = (conf::socket5(), conf::api_key().coinmarketcap);

    let mut headers = HeaderMap::new();
    headers.insert(ACCEPT, "application/json".parse().unwrap());
    headers.insert("X-CMC_PRO_API_KEY", api_key.parse().unwrap());

    const API: &str = "https://pro-api.coinmarketcap.com/v1/cryptocurrency/listings/latest";

    let client = if socket5.coinmarketcap {
        let proxy = Proxy::all(format!("socks5://{}:{}", socket5.ip, socket5.port))?;
        Client::builder().proxy(proxy).build()?
    } else {
        Client::new()
    };

    let resp = client
        .get(API)
        .headers(headers)
        .query(&[
            ("start", "1"),
            ("limit", "100"),
            ("convert", "USD"),
            ("aux", "cmc_rank"),
        ])
        .send()
        .await?
        .text()
        .await?;

    Ok(resp)
}
