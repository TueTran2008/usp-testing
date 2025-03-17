use config::{Config, ConfigError};
use serde::Deserialize;
use std::collections::HashMap;
use tracing::info;

#[derive(Deserialize, Debug)]
pub struct Setting {
    pub database: DatabaseSetting,
    pub mqtt: MqttSetting,
}

#[derive(Deserialize, Debug)]
pub struct MqttSetting {
    pub host: String,
    pub port: String,
    pub username: String,
    pub password: String,
}

#[derive(Deserialize, Debug)]
pub struct DatabaseSetting {
    pub username: String,
    pub password: String,
    pub port: u16,
    pub host: String,
    pub database_name: String,
}

/*
*   The SRV URI connection scheme has the following form:
*       mongodb+srv://[username:password@]host[/[defaultauthdb][?options]]
*   Please check the https://www.mongodb.com/docs/manual/reference/connection-string/ to see more
*   document about connection string
*
* */
impl Setting {
    //mongodb:.//user:pass@sample.host:port/?Optio

    pub fn get_database_connect_string(&self) -> String {
        format!(
            "mongodb://{}:{}@{}:{}/{}",
            self.database.username,
            self.database.password,
            self.database.host,
            self.database.port,
            self.database.database_name
        )
    }
    pub fn get_mqtt_url(&self) -> String {
        format!("mqtt://{}", self.mqtt.host)
    }
    pub fn get_setting() -> Result<Self, ConfigError> {
        let setting = Config::builder()
            .add_source(config::File::with_name("configuration.yaml"))
            .build()
            .unwrap();
        setting.try_deserialize()
    }
}
