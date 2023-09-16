use config::{ConfigError, Config, File};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub title: String,
    pub organization: String,
    #[serde(rename="backup-number")]
    pub backup_number: String,
    #[serde(rename="app-port")]
    pub app_port: u16,
    #[serde(rename="log-level")]
    pub log_level: String,

}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let conf = Config::builder()
            .add_source(File::with_name("config.yml"))
            .build()?;

        println!("Port: {:?}", conf.get::<u16>("app-port"));

        conf.try_deserialize()
    }
}