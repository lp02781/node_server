use actix_web::{web, App, HttpServer, HttpResponse, HttpRequest, Responder};
use actix_web_actors::ws;
use sqlx::postgres::PgPoolOptions;
use std::env;
use dotenv::dotenv;

mod json;
mod websocket;
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

async fn send_database(table_name: &str, payload: json::NodePayload, db_pool: &sqlx::PgPool,) -> Result<(), sqlx::Error> {
    if !ALLOWED_TABLES.contains(&table_name) {
        return Err(sqlx::Error::Protocol("Invalid table name".into()));
    }

    let query = format!(
        "INSERT INTO {} (id, timestamp, temperature, humidity, current) VALUES ($1, $2, $3, $4, $5)",
        table_name
    );

    sqlx::query(&query)
        .bind(&payload.id)
        .bind(payload.timestamp)
        .bind(payload.data.temperature)
        .bind(payload.data.humidity as i32)
        .bind(payload.data.current)
        .execute(db_pool)
        .await?;

    prune_old_rows(table_name, db_pool).await?;

    Ok(())
}

async fn prune_old_rows(table_name: &str, db_pool: &sqlx::PgPool,) -> Result<(), sqlx::Error> {
    if !ALLOWED_TABLES.contains(&table_name) {
        return Err(sqlx::Error::Protocol("Invalid table name".into()));
    }

    let delete_query = format!(
        r#"
        DELETE FROM {}
        WHERE ctid IN (
            SELECT ctid FROM {}
            ORDER BY timestamp ASC
            LIMIT (
                SELECT GREATEST(count(*) - 1000, 0) FROM {}
            )
        );
        "#,
        table_name, table_name, table_name
    );

    sqlx::query(&delete_query)
        .execute(db_pool)
        .await?;

    Ok(())
}

async fn receive_device_data(device: web::Path<String>, payload: web::Json<json::NodePayload>, db_pool: web::Data<sqlx::PgPool>, ) -> impl Responder {
    let table_name = device.as_str();
    let data = payload.into_inner();

    println!("Received data for device '{}', table '{}'", device, table_name);

    if let Err(e) = send_database(table_name, data.clone(), db_pool.get_ref()).await {
        eprintln!("Database insert error for table '{}': {}", table_name, e);
        return HttpResponse::InternalServerError().body("Failed to store data");
    }

    HttpResponse::Ok().json(data)
}


async fn websocket_handler(req: HttpRequest, stream: web::Payload, device: web::Path<String>) -> actix_web::Result<HttpResponse> {
    let device_id = device.into_inner();
    let ws = websocket::WebSocketSession { device_id };
    ws::start(ws, &req, stream)
}
