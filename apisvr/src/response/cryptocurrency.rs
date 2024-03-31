use super::RUNTIME;
use crate::conf;
use anyhow::{anyhow, Result};
use reqwest::{
    header::{HeaderMap, ACCEPT},
    Client, Proxy,
};
use rocket::tokio::{self, sync::Mutex, time::Duration};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

lazy_static! {
    static ref LATEST: Mutex<Option<String>> = Mutex::new(None);
    static ref STATS: Mutex<Stats> = Mutex::new(Stats::default());
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Stats {
    pub greed_fear: GreedFear,
    pub global: Global,
    pub gas_fee: GasFee,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct GreedFear {
    pub data: Vec<GreedFearData>,
}
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct GreedFearData {
    pub value: String,
    pub timestamp: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Global {
    pub total_market_cap_usd: u64,
    pub total_24h_volume_usd: u64,
    pub bitcoin_percentage_of_market_cap: f64,
    pub last_updated: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct GasFee {
    pub ethereum: u64,
    pub bitcoin: (u64, u64, u64),
}

pub async fn latest_cache() -> Option<String> {
    LATEST.lock().await.clone()
}

pub async fn stats_cache() -> Result<String> {
    Ok(serde_json::to_string(&*STATS.lock().await)?)
}

pub fn init() {
    timer();
}

fn timer() {
    RUNTIME.lock().unwrap().spawn(async move {
        log::debug!("timer start...");

        let mut count = 0_u64;
        let latest_interval = u64::max(10, conf::timer().coinmarketcap_latest);

        loop {
            if count % latest_interval == 0 {
                match fetch_latest().await {
                    Ok(v) => *LATEST.lock().await = Some(v),
                    Err(e) => log::warn!("fetch_latest error: {e:?}"),
                }
            }

            if count % 60 == 0 {
                match fetch_greed_fear().await {
                    Ok(v) => STATS.lock().await.greed_fear = v,
                    Err(e) => log::warn!("fetch_greed_fear error: {e:?}"),
                }

                match fetch_global().await {
                    Ok(v) => STATS.lock().await.global = v,
                    Err(e) => log::warn!("fetch_global error: {e:?}"),
                }
            }

            if count % 30 == 0 {
                match fetch_ethereum_gas_fee().await {
                    Ok(v) => STATS.lock().await.gas_fee.ethereum = v,
                    Err(e) => log::warn!("fetch_ethereum_gas_fee error: {e:?}"),
                }

                match fetch_bitcoin_gas_fee().await {
                    Ok(v) => STATS.lock().await.gas_fee.bitcoin = v,
                    Err(e) => log::warn!("fetch_bitcoin_gas_fee error: {e:?}"),
                }
            }

            tokio::time::sleep(Duration::from_secs(1)).await;
            count += 1;
        }
    });
}

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

async fn fetch_greed_fear() -> Result<GreedFear> {
    let socket5 = conf::socket5();

    let mut headers = HeaderMap::new();
    headers.insert(ACCEPT, "application/json".parse().unwrap());

    const API: &str = "https://api.alternative.me/fng/";

    let client = if socket5.alternative {
        let proxy = Proxy::all(format!("socks5://{}:{}", socket5.ip, socket5.port))?;
        Client::builder().proxy(proxy).build()?
    } else {
        Client::new()
    };

    let resp = client
        .get(API)
        .headers(headers)
        .query(&[("limit", "2")])
        .send()
        .await?
        .json()
        .await?;

    Ok(resp)
}

async fn fetch_global() -> Result<Global> {
    let socket5 = conf::socket5();

    let mut headers = HeaderMap::new();
    headers.insert(ACCEPT, "application/json".parse().unwrap());

    const API: &str = "https://api.alternative.me/v1/global/";

    let client = if socket5.alternative {
        let proxy = Proxy::all(format!("socks5://{}:{}", socket5.ip, socket5.port))?;
        Client::builder().proxy(proxy).build()?
    } else {
        Client::new()
    };

    let resp = client
        .get(API)
        .headers(headers)
        .send()
        .await?
        .json()
        .await?;

    Ok(resp)
}

async fn fetch_ethereum_gas_fee() -> Result<u64> {
    let socket5 = conf::socket5();

    let mut headers = HeaderMap::new();
    headers.insert(ACCEPT, "application/json".parse().unwrap());

    const API: &str = "https://api.etherscan.io/api?module=proxy&action=eth_gasPrice";

    let client = if socket5.ethscan {
        let proxy = Proxy::all(format!("socks5://{}:{}", socket5.ip, socket5.port))?;
        Client::builder().proxy(proxy).build()?
    } else {
        Client::new()
    };

    let resp = client
        .get(API)
        .headers(headers)
        .send()
        .await?
        .json::<Value>()
        .await?;

    match resp.get("result") {
        Some(v) => {
            let v = v.as_str().unwrap_or_default().trim_start_matches("0x");
            match u64::from_str_radix(v, 16) {
                Ok(v) => Ok(v),
                Err(_) => Err(anyhow!("{v}")),
            }
        }
        _ => Err(anyhow!("do not find field `result`")),
    }
}

// (low, middle, high)
pub async fn fetch_bitcoin_gas_fee() -> Result<(u64, u64, u64)> {
    let socket5 = conf::socket5();
    const API: &str = "https://blockstream.info/api/fee-estimates";

    let client = if socket5.ethscan {
        let proxy = Proxy::all(format!("socks5://{}:{}", socket5.ip, socket5.port))?;
        Client::builder().proxy(proxy).build()?
    } else {
        Client::new()
    };

    let mut response = client
        .get(API)
        .send()
        .await?
        .json::<HashMap<String, f64>>()
        .await?
        .into_values()
        .map(|v| v as u64)
        .collect::<Vec<u64>>();

    response.sort_by(|a, b| a.partial_cmp(b).unwrap());
    match response.len() {
        0 => Err(anyhow!("no feerate provided")),
        1 => Ok((response[0], response[0], response[0])),
        2 => Ok((response[0], response[0], response[1])),
        _ => {
            let low = response[0];
            let middle = response[response.len() / 2];
            let high = *response.last().unwrap();
            Ok((low, middle, high))
        }
    }
}
