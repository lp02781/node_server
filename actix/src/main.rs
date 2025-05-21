use actix_web::{web, App, HttpServer, HttpResponse, HttpRequest, Responder};
use actix_web_actors::ws;

mod json;
mod websocket;
//mod mqtt;
//mod tcp;

#[actix_web::main]
async fn main() -> std::io::Result<()> {   
    //tokio::spawn(async {mqtt::start_mqtt_1_subscriber().await;});
    //tokio::spawn(async {mqtt::start_mqtt_2_subscriber().await;});
    //tokio::spawn(async {mqtt::start_mqtt_actix_publisher().await;});
    //tokio::spawn(async {tcp::start_tcp_actix().await;});
    //tokio::spawn(async {ros2::start_ros2_publisher().await;});

    HttpServer::new(move || {
        App::new()
        .route("/node/{device}/data", web::post().to(receive_device_data))
        .route("/ws/{device}", web::get().to(websocket_handler))
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

async fn websocket_handler(req: HttpRequest, stream: web::Payload, device: web::Path<String>) 
-> actix_web::Result<HttpResponse> {
    let device_id = device.into_inner();
    let ws = websocket::WebSocketSession { device_id };
    ws::start(ws, &req, stream)
}
