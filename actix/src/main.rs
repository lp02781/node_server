use actix_web::{web, App, HttpServer, HttpResponse, Responder};

mod mqtt;
mod json;
mod ros2;

#[actix_web::main]
async fn main() -> std::io::Result<()> {   
    tokio::spawn(async {mqtt::start_mqtt_subscriber().await;});
    tokio::spawn(async {mqtt::start_mqtt_publisher().await;});
    tokio::spawn(async {ros2::start_ros2_publisher().await;});

    HttpServer::new(move || {
        App::new()
            .route("/node/mqtt/data", web::post().to(receive_mqtt_data))
    })
    .bind(("127.0.0.1", 5000))?
    .run()
    .await
}

async fn receive_mqtt_data(payload: web::Json<json::NodePayload>) -> impl Responder {
    println!("Received MQTT Data: {:?}", payload);
    
    HttpResponse::Ok().json(payload.into_inner())
}
