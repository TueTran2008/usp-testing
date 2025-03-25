use crate::mqtt_client::MQTTClient;
use async_trait::async_trait;
use std::error::Error;

struct UspAgentMtpMQTT {
    client: MQTTClient,
    response_topic: String,
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
    async fn new_mtp(&self) -> Result<(), Box<dyn Error>>;
    // async fn connect(&mut self, uri: String) -> Result<(), Box<dyn Error>>;
}

#[async_trait]
impl MTPConnection for UspAgentMtpMQTT {
    async fn new_mtp(&self) -> Result<(), Box<dyn Error>> {}
}
