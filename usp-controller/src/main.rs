use tokio::sync::mpsc;

use axum::{routing::get, Router};
use env_logger;
use paho_mqtt::MqttVersion;
use tracing::{error, info, warn};
use tracing_subscriber::filter::LevelFilter;
use usp_controller::{mqtt_client, telemetry, usp_msg_handle};
//use prost
#[tokio::main]
async fn main() {
    // build our application with a single route
    // let app = Router::new().route("/", get(|| async { "Hello, World!" }));
    //
    // // run our app with hyper, listening globally on port 3000
    // let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    // axum::serve(listener, app).await.unwrap();
    let web_subcriber = telemetry::get_subscriber("zero2prod_app".into(), LevelFilter::INFO.into());
    telemetry::init_subscriber(web_subcriber);
    let mqtt_client = mqtt_client::MQTTClient::connect(
        MqttVersion::V3_1,
        "mqtt://localhost:1883",
        "darwin_tran",
        "gemtek",
        "Gemtek@123",
    )
    .await;
    let rx_queue = mqtt_client.client.start_consuming();
    let _ = mqtt_client.client.subscribe("queue/Agent1", 0);
    info!("darwin tran testing mqtt client");
    // Publish a message

    let (mqtt_tx, mut mqtt_rx) = mpsc::channel(4096);
    tokio::spawn(async move {
        for mqttmsg in rx_queue.iter() {
            if let Some(mqttmsg) = mqttmsg {
                //info!("Received: -> {}", mqttmsg.payload_str());
                mqtt_tx.send(mqttmsg).await.unwrap();
                info!("Send to channel okay");
            } else {
                warn!("Unsubscribe: connection closed");
                break;
            }
        }
    });

    while let Some(msg) = mqtt_rx.recv().await {
        info!("Receive message {}", msg.payload_str());
        let record_result = usp_msg_handle::UspMsgHandle::usp_record_decode(msg.payload());
        match record_result {
            Ok(record) => {
                usp_msg_handle::UspMsgHandle::usp_record_debug(&record);
                let msg = match usp_msg_handle::UspMsgHandle::usp_record_unpack(&record) {
                    Ok(message) => message,
                    Err(_) => panic!("hello"),
                };
                usp_msg_handle::UspMsgHandle::usp_msg_debug(&msg);
            }
            Err(e) => {
                error!("Error when decode result {:?}", e);
                error!("Error when decode");
            }
        };
    }
}
