use config::{Config, File};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct ConfigFile {
    pub database_name: String,
    pub collection_raffle: String,
    pub collection_ticket: String,
}

pub fn load_config_file() -> ConfigFile {
    Config::builder()
        .add_source(File::with_name("conf/settings.toml"))
        .build()
        .unwrap()
        .try_deserialize::<ConfigFile>()
        .unwrap()
}
