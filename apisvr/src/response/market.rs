use super::RUNTIME;
use crate::conf;
use anyhow::Result;
use reqwest::{Client, Proxy};
use rocket::tokio::{self, sync::Mutex, time::Duration};
use serde::{Deserialize, Serialize};
use serde_json;
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
struct SnapshotData {
    snapshot: HashMap<String, Vec<serde_json::Value>>,
}

#[derive(Serialize, Deserialize, Debug)]
struct ResponseData {
    code: u32,
    message: String,
    data: SnapshotData,
}

#[derive(Serialize, Deserialize, Debug)]
struct MarketData {
    name: String,
    value: f64,
    precent: f64,
}

lazy_static! {
    static ref MARKET_DATA: Mutex<Option<String>> = Mutex::new(None);
}

pub async fn latest_cache() -> Option<String> {
    MARKET_DATA.lock().await.clone()
}

pub fn init() {
    timer();
}

fn timer() {
    RUNTIME.lock().unwrap().spawn(async move {
        log::debug!("latest timer start...");

        let mut count = 0_u64;
        let interval = u64::max(10, conf::timer().awtmt_market);
        loop {
            if count % interval == 0 {
                match fetch().await {
                    Ok(v) => *MARKET_DATA.lock().await = Some(v),
                    Err(e) => log::warn!("fetch awtmt market data error: {e:?}"),
                }
            }

            tokio::time::sleep(Duration::from_secs(1)).await;
            count += 1;
        }
    });
}

pub async fn fetch() -> Result<String> {
    fetch_awtmt().await
}

async fn fetch_awtmt() -> Result<String> {
    const API: &str = "https://api-ddc-wscn.awtmt.com/market/real?fields=prod_name%2Cpreclose_px%2Clast_px%2Cpx_change%2Cpx_change_rate%2Cprice_precision&prod_code=000001.SS%2CDXY.OTC%2CUS10YR.OTC%2CUSDCNH.OTC%2C399001.SZ%2C399006.SZ%2CUS500.OTC";

    let socket5 = conf::socket5();
    let client = if socket5.awtmt {
        let proxy = Proxy::all(format!("socks5://{}:{}", socket5.ip, socket5.port))?;
        Client::builder().proxy(proxy).build()?
    } else {
        Client::new()
    };

    let resp = client.get(API).send().await?.json::<ResponseData>().await?;

    let mut resp = resp.data.snapshot.into_iter().collect::<Vec<(_, _)>>();
    resp.sort_by(|a, b| a.0.cmp(&b.0));

    let resp = resp
        .into_iter()
        .filter(|(_, item)| item.len() == 7)
        .map(|(_, item)| MarketData {
            name: item[0].as_str().unwrap_or_default().to_string(),
            value: item[2].as_f64().unwrap_or_default(),
            precent: item[4].as_f64().unwrap_or_default(),
        })
        .collect::<Vec<_>>();

    Ok(serde_json::to_string(&resp)?)
}
