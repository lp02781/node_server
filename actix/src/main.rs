use actix_web::{web, App, HttpServer, HttpResponse, HttpRequest, Responder};
use actix_web_actors::ws;

mod mqtt;
mod json;
mod tcp;
mod websocket;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let _ = tokio::spawn(mqtt::start_mqtt_1_subscriber());
    let _ = tokio::spawn(mqtt::start_mqtt_2_subscriber());
    let _ = tokio::spawn(mqtt::start_mqtt_actix_publisher());
    let _ = tokio::spawn(tcp::start_tcp_actix());
    // let _ = tokio::spawn(ros2::start_ros2_publisher()); 

    HttpServer::new(move || {
        App::new()
            .route("/node/{device}/data", web::post().to(receive_device_data))
            .route("/ws/{device}", web::get().to(websocket_handler))
    })
    .bind(("127.0.0.1", 5000))?
    .run()
    .await
}

async fn receive_device_data(
    device: web::Path<String>,
    payload: web::Json<json::NodePayload>,
) -> impl Responder {
    println!("Received data for device: {}", device);
    println!("Payload: {:?}", payload);

    HttpResponse::Ok().json(payload.into_inner())
}

async fn websocket_handler(
    req: HttpRequest,
    stream: web::Payload,
    device: web::Path<String>,
) -> actix_web::Result<HttpResponse> {
    let device_id = device.into_inner();
    let ws = websocket::WebSocketSession { device_id };
    ws::start(ws, &req, stream)
}
