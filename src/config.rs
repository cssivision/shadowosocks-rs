use std::env;
use std::path::Path;
use std::{fs, io};

use serde_derive::{Deserialize, Serialize};

static LOCAL_ADDR: &str = "0.0.0.0:6009";
static SERVER_ADDR: &str = "0.0.0.0:9006";
static PASSWORD: &str = "password";
static METHOD: &str = "aes-256-cfb";

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(default)]
pub struct Config {
    pub local_addr: String,
    pub server_addr: String,
    pub password: String,
    pub method: String,
}

impl Config {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Config, io::Error> {
        if path.as_ref().exists() {
            let contents = fs::read_to_string(path)?;
            let config = match serde_json::from_str(&contents) {
                Ok(c) => c,
                Err(e) => {
                    log::error!("{}", e);
                    return Err(io::Error::new(io::ErrorKind::Other, e));
                }
            };
            return Ok(config);
        }

        let mut config = Config {
            ..Default::default()
        };
        if config.local_addr.is_empty() {
            config.local_addr = if let Ok(addr) = env::var("SHADOWSOCKS_LOCAL_ADDR") {
                addr
            } else {
                LOCAL_ADDR.to_string()
            }
        }
        if config.server_addr.is_empty() {
            config.server_addr = if let Ok(addr) = env::var("SHADOWSOCKS_SERVER_ADDR") {
                addr
            } else {
                SERVER_ADDR.to_string()
            }
        }
        if config.password.is_empty() {
            config.password = if let Ok(addr) = env::var("SHADOWSOCKS_PASSWORD") {
                addr
            } else {
                PASSWORD.to_string()
            }
        }
        if config.method.is_empty() {
            config.method = if let Ok(addr) = env::var("SHADOWSOCKS_METHOD") {
                addr
            } else {
                METHOD.to_string()
            }
        }
        Ok(config)
    }
}
