use config::{Config, File};
use glob::glob;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct ConfigFile {
    pub server_ip: String,
    pub server_port: u16,
    pub access_tokens: Vec<String>,
    pub mongodb_uri: String,
    pub database_name: String,
    pub collection_raffle: String,
    pub collection_ticket: String,
    pub destination_account_address: String,
}

pub fn load_config_file() -> ConfigFile {
    Config::builder()
        // File::with_name(..) is shorthand for File::from(Path::new(..))
        .add_source(File::with_name("conf/settings.toml"))
        .build()
        .unwrap()
        .try_deserialize::<ConfigFile>()
        .unwrap()
}
