use config::{ConfigError, Config, File};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Author {
    pub title: String,
    pub description: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub title: String,
    pub organization: String,
    pub record_number: String,
    pub app_port: u16,
    pub log_level: String,
    pub author: Author,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let conf = Config::builder()
            .add_source(File::with_name("config"))
            .build()?;

        println!("Port: {:?}", conf.get::<u16>("app_port"));

        conf.try_deserialize()
    }
}