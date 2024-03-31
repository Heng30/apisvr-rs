use super::data::{self, Config};
use anyhow::{anyhow, Result};
use platform_dirs::AppDirs;
use std::{fs, path::PathBuf, sync::Mutex};

const APP_NANME: &str = "apisvr";
const DB_NAME: &str = "apisvr.db";
const CONFIG_NAME: &str = "apisvr.conf";

lazy_static! {
    pub static ref CONFIG: Mutex<Config> = Mutex::new(Config::default());
}

pub fn init() {
    if let Err(e) = CONFIG.lock().unwrap().init() {
        panic!("{:?}", e);
    }
}

pub fn server() -> data::Server {
    CONFIG.lock().unwrap().server.clone()
}

pub fn socket5() -> data::Socket5 {
    CONFIG.lock().unwrap().socket5.clone()
}

pub fn api_key() -> data::ApiKey {
    CONFIG.lock().unwrap().api_key.clone()
}

pub fn timer() -> data::Timer {
    CONFIG.lock().unwrap().timer.clone()
}

pub fn db_path() -> PathBuf {
    CONFIG.lock().unwrap().db_path.clone()
}

#[allow(unused)]
pub fn save(conf: data::Config) -> Result<()> {
    let mut config = CONFIG.lock().unwrap();
    *config = conf;
    config.save()
}

impl Config {
    pub fn init(&mut self) -> Result<()> {
        self.init_config()?;
        self.load()?;
        log::debug!("{:?}", self);
        Ok(())
    }

    fn init_config(&mut self) -> Result<()> {
        let app_dirs = AppDirs::new(Some(APP_NANME), true).unwrap();
        self.config_path = app_dirs.config_dir.join(CONFIG_NAME);
        self.db_path = app_dirs.data_dir.join(DB_NAME);

        fs::create_dir_all(&app_dirs.data_dir)?;
        fs::create_dir_all(&app_dirs.config_dir)?;

        Ok(())
    }

    fn load(&mut self) -> Result<()> {
        match fs::read_to_string(&self.config_path) {
            Ok(text) => match serde_json::from_str::<Config>(&text) {
                Ok(c) => {
                    self.server = c.server;
                    self.socket5 = c.socket5;
                    self.api_key = c.api_key;
                    self.timer = c.timer;
                    Ok(())
                }
                Err(e) => Err(anyhow!("{:?}", e).into()),
            },
            Err(_) => match serde_json::to_string_pretty(self) {
                Ok(text) => Ok(fs::write(&self.config_path, text)?),
                Err(e) => Err(anyhow!("{:?}", e).into()),
            },
        }
    }

    #[allow(unused)]
    pub fn save(&self) -> Result<()> {
        match serde_json::to_string_pretty(self) {
            Ok(text) => Ok(fs::write(&self.config_path, text)?),
            Err(e) => Err(anyhow!("{:?}", e).into()),
        }
    }
}
