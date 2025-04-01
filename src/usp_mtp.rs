use crate::mqtt_client::MQTTClient;
use async_trait::async_trait;
use paho_mqtt::{Message, MqttVersion, QoS};
use std::error::Error;
use tokio::sync::mpsc;
// use tracing::{error, info};

pub struct MQTTConfig {
    broker: String,
    client_id: String,
    publish_topic: String,
    username: Option<String>,
    password: Option<String>,
    subscribe_topic: Vec<(String, u32)>,
    version: MqttVersion,
}

pub struct UspAgentMtpMQTT {
    client: Option<MQTTClient>,
    config: MQTTConfig,
    receiver: Option<mpsc::Receiver<Message>>,
}

pub enum MtpData {
    MQTT(UspAgentMtpMQTT),
}

pub struct UspAgentMtpInstance {
    mtp_protocol: MtpData,
    name: String,
}

#[async_trait]
pub trait MTPConnection {
    async fn connect(&mut self) -> Result<(), Box<dyn Error>>;
    async fn send(&self, msg: &[u8]) -> Result<(), Box<dyn Error>>;
    async fn receive(&mut self) -> Result<Vec<u8>, Box<dyn Error>>;
    // async fn connect(&mut self, uri: String) -> Result<(), Box<dyn Error>>;
}

impl UspAgentMtpMQTT {
    pub fn new(mqtt_config: MQTTConfig) -> UspAgentMtpMQTT {
        UspAgentMtpMQTT {
            client: None,
            config: mqtt_config,
            receiver: None,
        }
    }
}

#[async_trait]
impl MTPConnection for UspAgentMtpMQTT {
    async fn connect(&mut self) -> Result<(), Box<dyn Error>> {
        let username = self
            .config
            .username
            .as_ref()
            .ok_or("MQTT username is empty")?;
        let password = self
            .config
            .password
            .as_ref()
            .ok_or("MQTT password is empty")?;
        let mqtt_client = MQTTClient::connect(
            self.config.version,
            self.config.broker.as_ref(),
            self.config.client_id.as_ref(),
            username,
            password,
        )
        .await;

        let (mqtt_tx, mut mqtt_rx) = mpsc::channel(4096);
        mqtt_client
            .client
            .set_message_callback(move |_client, msg| {
                if let Some(message) = msg {
                    let _ = mqtt_tx.blocking_send(message);
                }
            });
        for (topic, qos) in &self.config.subscribe_topic {
            let _sub_token = mqtt_client.client.subscribe(topic, *qos).await?;

            // subscribe_token.properties()
        }

        // Save AsyncClinet DataStructure and the Receive mpsc
        self.client = Some(mqtt_client);
        self.receiver = Some(mqtt_rx);
        Ok(())
    }
    async fn send(&self, msg: &[u8]) -> Result<(), Box<dyn Error>> {
        let mqtt_client = self
            .client
            .as_ref()
            .ok_or(paho_mqtt::errors::Error::General(
                "MQTT Client is not connected",
            ))?;
        let pub_msg = Message::new(
            self.config.publish_topic.clone(),
            msg.to_vec(),
            QoS::AtMostOnce,
        );
        let _pub_token = mqtt_client.client.publish(pub_msg).await?;
        Ok(())
    }
    async fn receive(&mut self) -> Result<Vec<u8>, Box<dyn Error>> {
        let mqtt_rx = self
            .receiver
            .as_mut()
            .ok_or(paho_mqtt::errors::Error::General(
                "MQTT Client is not connected",
            ))?;
        let msg = mqtt_rx
            .recv()
            .await
            .ok_or(paho_mqtt::errors::Error::General("Message channel closed"))?;
        Ok(msg.payload().to_vec())
    }
}
