use config::Config;
use std::collections::HashMap;
use tracing::info;

pub struct DatabaseSetting {
    pub username: String,
    pub password: String,
    pub port: u16,
    pub host: String,
    pub database_name: String,
}

impl DatabaseSetting {
    //mongodb:.//user:pass@sample.host:port/?Optio
    pub fn get_database_setting() {
        let setting = Config::builder()
            .add_source(config::File::with_name("configuration.yaml"))
            .build()
            .unwrap();

        info!("{:?}", setting.try_deserialize::<HashMap<String, String>>());
        // let a = setting.try_into();
    }
}
