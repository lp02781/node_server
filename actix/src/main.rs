use actix_web::{web, App, HttpServer, HttpResponse, Responder};

mod mqtt;
mod json;
mod tcp;

#[actix_web::main]
async fn main() -> std::io::Result<()> {   
    tokio::spawn(async {mqtt::start_mqtt_1_subscriber().await;});
    tokio::spawn(async {mqtt::start_mqtt_2_subscriber().await;});
    tokio::spawn(async {mqtt::start_mqtt_actix_publisher().await;});
    tokio::spawn(async {tcp::start_tcp_actix().await;});
    //tokio::spawn(async {ros2::start_ros2_publisher().await;});

    HttpServer::new(move || {
        App::new()
        .route("/node/{device}/data", web::post().to(receive_device_data))
    })
    .bind(("127.0.0.1", 5000))?
    .run()
    .await
}

async fn receive_device_data( device: web::Path<String>,
                              payload: web::Json<json::NodePayload>) 
-> impl Responder {
    println!("Received data for device: {}", device);
    println!("Payload: {:?}", payload);
    
    HttpResponse::Ok().json(payload.into_inner())
}
