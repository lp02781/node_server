use actix_web::{web, App, HttpServer, HttpResponse, HttpRequest, Responder};
use actix_web_actors::ws;

mod mqtt;
mod json;
mod tcp;
mod websocket;

use std::panic::AssertUnwindSafe;
use futures::FutureExt;

fn spawn_with_restart<Fut>(task_fn: fn() -> Fut, name: &'static str)
where
    Fut: std::future::Future<Output = ()> + Send + 'static,
{
    tokio::spawn(async move {
        loop {
            let res = AssertUnwindSafe(task_fn()).catch_unwind().await;
            match res {
                Ok(_) => {
                    eprintln!("{} task exited normally, restarting...", name);
                }
                Err(e) => {
                    eprintln!("{} task panicked: {:?}, restarting...", name, e);
                }
            }
            tokio::time::sleep(std::time::Duration::from_secs(5)).await;
        }
    });
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    spawn_with_restart(mqtt::start_mqtt_1_subscriber, "MQTT 1 subscriber");
    spawn_with_restart(mqtt::start_mqtt_2_subscriber, "MQTT 2 subscriber");
    spawn_with_restart(mqtt::start_mqtt_actix_publisher, "MQTT publisher");
    spawn_with_restart(tcp::start_tcp_actix, "TCP");

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
