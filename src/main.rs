// mod error;
// mod usp_agent;
// pub use self::error::{Error, Result};
// use crate::usp_agent::{configuration::Setting, mqtt_client, telemetry, usp_msg_handle};
// use axum::{routing::get, Router};
// use env_logger;
// use mongodb::options::ClientOptions;
// use mongodb::{
//     bson::doc,
//     options::{ServerApi, ServerApiVersion},
//     Client,
// };
// use paho_mqtt::MqttVersion;
// use std::error::Error;
// use tokio::sync::mpsc;
// use tracing::{error, info, warn};
// use tracing_subscriber::filter::LevelFilter;
//use prost
mod error;
mod usp_agent;

use crate::error::{Error, Result};
use crate::usp_agent::uspa::UspAgent;

// #[tokio::main]
fn main() -> Result<()> {
    let mut rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        let mut usp = UspAgent::new("proto::DarwinTran2008".to_string());
        tokio::spawn(usp.run_dynamic_mtp_manager());
    });
    // build our application with a single route
    // let app = Router::new().route("/", get(|| async { "Hello, World!" }));
    //
    // // run our app with hyper, listening globally on port 3000
    // let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    // axum::serve(listener, app).await.unwrap();

    /*Get configuration*/
    // let web_subcriber =
    //     telemetry::get_subscriber("usp_controller".into(), LevelFilter::INFO.into());
    // telemetry::init_subscriber(web_subcriber);
    // let setting = Setting::get_setting().unwrap_or_else(|err| {
    //     panic!("Cannot read configuration {:?}", err);
    // });
    // info!("Read setting {:?}", setting);
    //
    // /*Connect to MongoDB*/
    // let mut client_options = ClientOptions::parse(setting.get_database_connect_string()).await?;
    //
    // // Set the server_api field of the client_options object to Stable API version 1
    // let server_api = ServerApi::builder().version(ServerApiVersion::V1).build();
    // client_options.server_api = Some(server_api);
    // // Create a new client and connect to the server
    // let client = Client::with_options(client_options).unwrap();
    //
    // // Print the databases in our MongoDB server
    // info!("Available databases:");
    // for db_name in client.list_database_names().await? {
    //     info!("- {}", db_name);
    // }
    // // Send a ping to confirm a successful connection
    // client
    //     .database(&setting.database.database_name)
    //     .run_command(doc! { "ping": 1 })
    //     .await
    //     .unwrap();
    // info!("Pinged your deployment. You successfully connected to MongoDB!");
    // /*Connect to MQTT Broker*/
    // let mqtt_client = mqtt_client::MQTTClient::connect(
    //     MqttVersion::V3_1,
    //     &setting.get_mqtt_url(),
    //     "darwin_tran",
    //     &setting.mqtt.username,
    //     &setting.mqtt.password,
    // )
    // .await;
    // let rx_queue = mqtt_client.client.start_consuming();
    // let _ = mqtt_client.client.subscribe("queue/Agent1", 0);
    // info!("darwin tran testing mqtt client");
    // // Publish a message
    //
    // let (mqtt_tx, mut mqtt_rx) = mpsc::channel(4096);
    // tokio::spawn(async move {
    //     for mqttmsg in rx_queue.iter() {
    //         if let Some(mqttmsg) = mqttmsg {
    //             //info!("Received: -> {}", mqttmsg.payload_str());
    //             mqtt_tx.send(mqttmsg).await.unwrap();
    //             info!("Send to channel okay");
    //         } else {
    //             warn!("Unsubscribe: connection closed");
    //             break;
    //         }
    //     }
    // });
    //
    // while let Some(msg) = mqtt_rx.recv().await {
    //     info!("Receive message {}", msg.payload_str());
    //     let record_result = usp_msg_handle::UspMsgHandle::usp_record_decode(msg.payload());
    //     match record_result {
    //         Ok(record) => {
    //             usp_msg_handle::UspMsgHandle::usp_record_debug(&record);
    //             let msg = match usp_msg_handle::UspMsgHandle::usp_record_unpack(&record) {
    //                 Ok(message) => message,
    //                 Err(_) => panic!("hello"),
    //             };
    //             // usp_msg_handle::UspMsgHandle::usp_msg_debug(&msg);
    //         }
    //         Err(e) => {
    //             error!("Error when decode result {:?}", e);
    //             error!("Error when decode");
    //         }
    //     };
    // }
    Ok(())
}
