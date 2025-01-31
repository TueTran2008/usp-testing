use axum::{routing::get, Router};
use env_logger;
use log::{info, warn};
use paho_mqtt::{AsyncReceiver, Message, MqttVersion};
use usp_controller::mqtt_client;
//use prost
#[tokio::main]
async fn main() {
    // build our application with a single route
    // let app = Router::new().route("/", get(|| async { "Hello, World!" }));
    //
    // // run our app with hyper, listening globally on port 3000
    // let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    // axum::serve(listener, app).await.unwrap();
    env_logger::init();
    let mqtt_client = mqtt_client::MQTTClient::connect(
        MqttVersion::V3_1,
        "mqtt://localhost:1883",
        "darwin_tran",
        "gemtek1",
        "Gemtek@123",
    )
    .await;
    let rx_queue = mqtt_client.client.start_consuming();
    let _ = mqtt_client.client.subscribe("queue/Agent1", 0);
    println!("darwin tran testing mqtt client");
    // Publish a message

    for mqttmsg in rx_queue.iter() {
        if let Some(mqttmsg) = mqttmsg {
            info!("Received: -> {}", mqttmsg.payload_str());
        } else {
            warn!("Unsubscribe: connection closed");
            break;
        }
    }
}
