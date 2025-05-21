use actix_web::{web, App, HttpServer, HttpResponse, HttpRequest, Responder};
use actix_web_actors::ws;
use sqlx::postgres::PgPoolOptions;
use std::env;
use dotenv::dotenv;

mod json;
mod websocket;
mod postgresql;
//mod mqtt;
//mod tcp;

const ALLOWED_TABLES: &[&str] = &["websocket", "tcp", "sm_cpp", "sm_rust", "mqtt"];

#[actix_web::main]
async fn main() -> std::io::Result<()> {   
    //tokio::spawn(async {mqtt::start_mqtt_1_subscriber().await;});
    //tokio::spawn(async {mqtt::start_mqtt_2_subscriber().await;});
    //tokio::spawn(async {mqtt::start_mqtt_actix_publisher().await;});
    //tokio::spawn(async {tcp::start_tcp_actix().await;});
    //tokio::spawn(async {ros2::start_ros2_publisher().await;});

    dotenv().ok();

    let db_pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&env::var("DATABASE_URL").expect("DATABASE_URL must be set"))
        .await
        .expect("Failed to connect to the database");

    HttpServer::new(move || {
        App::new()
        .app_data(web::Data::new(db_pool.clone()))
        .route("/node/{device}/data", web::post().to(receive_device_data))
        .route("/ws/{device}", web::get().to(websocket_handler))
    })
    .bind(("127.0.0.1", 5000))?
    .run()
    .await
}

async fn receive_device_data(device: web::Path<String>, payload: web::Json<json::NodePayload>, db_pool: web::Data<sqlx::PgPool>,) -> impl Responder {
    let data = payload.into_inner();
    let table_name = payload.id.as_str();
    
    println!("\n Received data for device: '{}'", device.as_str());

    match postgresql::send_database(table_name, data.clone(), db_pool.get_ref()).await {
        Ok(_) => println!("Successfully inserted data into table '{}'", table_name),
        Err(e) => {
            eprintln!("Database insert error for table '{}': {}", table_name, e);
            return HttpResponse::InternalServerError().body("Failed to store data");
        }
    }
    
    match postgresql::prune_old_rows(table_name, db_pool.get_ref()).await {
        Ok(_) => println!("Successfully pruned old rows for table '{}'", table_name),
        Err(e) => eprintln!("Prune error for table '{}': {}", table_name, e),
    }
    
    HttpResponse::Ok().json({
        serde_json::json!({
            "status": "success",
            "message": format!("Data stored for device/table '{}'", table_name),
            "data": data
        })
    })
}

async fn websocket_handler(req: HttpRequest, stream: web::Payload, device: web::Path<String>,) -> actix_web::Result<HttpResponse> {
    let device_id = device.into_inner();
    let ws = websocket::WebSocketSession { device_id };
    ws::start(ws, &req, stream)
}