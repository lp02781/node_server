use actix_web::{web, App, HttpServer, HttpResponse, Responder};

mod mqtt;
mod json;

#[actix_web::main]
async fn main() -> std::io::Result<()> {   
    tokio::spawn(async {mqtt::start_mqtt_subscriber().await;});
    tokio::spawn(async {mqtt::start_mqtt_publisher().await;});

    HttpServer::new(move || {
        App::new()
            .route("/node/data", web::post().to(post_node_data))
    })
    .bind(("127.0.0.1", 5000))?
    .run()
    .await
}

pub async fn post_node_data(payload: json::NodePayload) -> Result<(), reqwest::Error> {
    let client = reqwest::Client::new();
    let url = "http://localhost:7000/database/data";

    let response = client
        .post(url)
        .json(&payload)
        .send()
        .await?;

    Ok(())
} 
