use std::{env, sync::OnceLock};

static CONFIG: OnceLock<Config> = OnceLock::new();

#[derive(Clone)]
pub struct Config {
    pub listen_port: u16,
    pub default_destination: Option<String>,
}

impl Config {
    pub fn from_env() -> Self {
        let listen_port = env::var("PROXY_PORT")
            .unwrap_or_else(|_| "8081".to_string())
            .parse()
            .expect("Invalid PROXY_PORT");

        let default_destination = env::var("DESTINATION").ok();

        Self {
            listen_port,
            default_destination,
        }
    }
}

pub fn get_config() -> &'static Config {
    return CONFIG.get_or_init(|| Config::from_env());
}
