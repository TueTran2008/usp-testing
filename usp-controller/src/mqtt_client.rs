use paho_mqtt::{self as mqtt, AsyncClient};
use std::{process, time::Duration};

// The topics to which we subscribe.
// const TOPICS: &[&str] = &["test", "hello"];
// const QOS: &[i32] = &[1, 1];
//
#[warn(dead_code)]
pub struct ClientInfo {
    host: String,
    client_id: String,
    user_name: String,
    password: String,
}
pub struct MQTTClient {
    pub client: AsyncClient,
}

impl MQTTClient {
    pub async fn connect(
        version: mqtt::MqttVersion,
        host: &str,
        client_id: &str,
        user_name: &str,
        password: &str,
    ) -> MQTTClient {
        let option = match version {
            mqtt::MqttVersion::V5 => {
                let create_opts_default = mqtt::CreateOptionsBuilder::new()
                    .server_uri(host)
                    .client_id(client_id)
                    .finalize();
                create_opts_default
            }
            _ => {
                let create_opts_v5 = mqtt::CreateOptionsBuilder::new_v3()
                    .server_uri(host)
                    .client_id(client_id)
                    .finalize();
                create_opts_v5
            }
        };
        let cli = mqtt::AsyncClient::new(option).unwrap_or_else(|e| {
            println!("Error creating the client : {:?}", e);
            process::exit(0);
        });

        let conn_opts = match version {
            mqtt::MqttVersion::V5 => {
                let conn_opts = mqtt::ConnectOptionsBuilder::new()
                    .keep_alive_interval(Duration::from_secs(30))
                    .user_name(user_name)
                    .password(password)
                    .clean_session(false)
                    .finalize();
                conn_opts
            }
            _ => {
                let conn_opts = mqtt::ConnectOptionsBuilder::new_v3()
                    .keep_alive_interval(Duration::from_secs(30))
                    .user_name(user_name)
                    .password(password)
                    .clean_session(false)
                    .finalize();
                conn_opts
            }
        };

        cli.connect(conn_opts).await.unwrap_or_else(|e| {
            panic!("Error when connect to MQTT Broker {:?}", e);
        });

        MQTTClient { client: cli }
    }
}
