use actix_web::{web, App, HttpServer, HttpResponse, HttpRequest, Responder};
use actix_web_actors::ws;
use sqlx::{PgPool, postgres::PgPoolOptions, FromRow};
use std::env;
use dotenv::dotenv;

mod json;
mod websocket;
mod postgresql;
//mod mqtt;
//mod tcp;

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
        .connect("postgres://admin_haha:pasword_haha@localhost:5432/db_haha")
        .await
        .expect("Failed to connect to the database");

    HttpServer::new(move || {
        App::new()
        .app_data(web::Data::new(db_pool.clone()))
        .route("/db/{device}/data", web::get().to(get_data))
        .route("/node/{device}/data", web::post().to(receive_device_data))
        .route("/ws/{device}", web::get().to(websocket_handler))
    })
    .bind(("127.0.0.1", 5000))?
    .run()
    .await
}

async fn get_data(device: web::Path<String>, db_pool: web::Data<PgPool>) -> impl Responder {
    let device_table = device.into_inner();

    let query = format!(
        "SELECT timestamp, temperature, humidity, current FROM {} ORDER BY timestamp DESC LIMIT 100",
        device_table
    );

    match sqlx::query_as::<_, json::DbRow>(&query)
        .fetch_all(db_pool.get_ref())
        .await
    {
        Ok(rows) => HttpResponse::Ok().json(rows),
        Err(e) => {
            eprintln!("Database query error for table '{}': {}", device_table, e);
            HttpResponse::InternalServerError().body("Failed to fetch data")
        }
    }
}

async fn receive_device_data(device: web::Path<String>, payload: web::Json<json::NodePayload>, db_pool: web::Data<sqlx::PgPool>,
) -> impl Responder {
    let data = payload.into_inner();
    let table_name = data.id.as_str();

    println!("\nReceived data for device: '{}'", device.as_str());

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

    HttpResponse::Ok().json(serde_json::json!({
        "status": "success",
        "message": format!("Data stored for device/table '{}'", table_name),
        "data": data
    }))
}

async fn websocket_handler(req: HttpRequest, stream: web::Payload, device: web::Path<String>,
) -> actix_web::Result<HttpResponse> {
    let device_id = device.into_inner();
    let ws = websocket::WebSocketSession { device_id };
    ws::start(ws, &req, stream)
}