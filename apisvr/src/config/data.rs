use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct Config {
    #[serde(skip)]
    pub config_path: PathBuf,

    #[serde(skip)]
    pub db_path: PathBuf,

    pub server: Server,
    pub socket5: Socket5,
    pub api_key: ApiKey,
    pub auth_token: AuthToken,
    pub timer: Timer,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Server {
    pub listen_address: String,
    pub listen_port: u16,
}

impl Default for Server {
    fn default() -> Self {
        Self {
            listen_address: "0.0.0.0".to_string(),
            listen_port: 8004,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Socket5 {
    pub ip: String,
    pub port: u16,

    pub coinmarketcap: bool,
    pub alternative: bool,
    pub blockstream: bool,
    pub ethscan: bool,
    pub awtmt: bool,
}

impl Default for Socket5 {
    fn default() -> Self {
        Self {
            ip: "127.0.0.1".to_string(),
            port: 1084,
            coinmarketcap: false,
            alternative: false,
            blockstream: false,
            ethscan: false,
            awtmt: false,
        }
    }
}

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct ApiKey {
    pub coinmarketcap: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Timer {
    pub coinmarketcap_latest: u64,
    pub awtmt_market: u64,
}

impl Default for Timer {
    fn default() -> Self {
        Self {
            coinmarketcap_latest: 1800,
            awtmt_market: 30,
        }
    }
}

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct AuthToken {
    pub rssbox_android: String,
}
