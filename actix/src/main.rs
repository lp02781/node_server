use actix_web::{web, App, HttpServer, HttpResponse, Responder};

mod mqtt;
mod json;

#[actix_web::main]
async fn main() -> std::io::Result<()> {   
    tokio::spawn(async {mqtt::start_mqtt_subscriber().await;});
    tokio::spawn(async {mqtt::start_mqtt_publisher().await;});

    HttpServer::new(move || {
        App::new()
            .route("/node/mqtt/data", web::post().to(manual_hello))
    })
    .bind(("127.0.0.1", 5000))?
    .run()
    .await
}

pub async fn post_mqtt_data(payload: json::MqttPayload) -> Result<(), reqwest::Error> {
    let client = reqwest::Client::new();
    let url = "http://localhost:5000/node/mqtt/data";

    let response = client
        .post(url)
        .json(&payload)
        .send()
        .await?;

    Ok(())
} 
