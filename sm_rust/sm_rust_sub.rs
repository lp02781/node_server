use core::time::Duration;
use iceoryx2::prelude::*;
use std::time::{SystemTime, UNIX_EPOCH};
use reqwest::blocking::Client;
use serde_json::json;

const CYCLE_TIME: Duration = Duration::from_secs(10);
const REST_API_URL: &str = "http://localhost:5000/node/sm_rust/data";

fn main() -> Result<(), Box<dyn core::error::Error>> {
    let node = NodeBuilder::new().create::<ipc::Service>()?;
    let client = Client::new();

    let sub_temperature = node
        .service_builder(&"sensor_data/sm_rust_pub/temperature".try_into()?)
        .publish_subscribe::<f32>()
        .open_or_create()?;

    let sub_humidity = node
        .service_builder(&"sensor_data/sm_rust_pub/humidity".try_into()?)
        .publish_subscribe::<i16>()
        .open_or_create()?;

    let sub_current = node
        .service_builder(&"sensor_data/sm_rust_pub/current".try_into()?)
        .publish_subscribe::<f32>()
        .open_or_create()?;

    let subscriber_temperature = sub_temperature.subscriber_builder().create()?;
    let subscriber_humidity = sub_humidity.subscriber_builder().create()?;
    let subscriber_current = sub_current.subscriber_builder().create()?;

    println!("[sm_rust_sub] sm_rust_sub subscriber + REST API relay is ready");

    while node.wait(CYCLE_TIME).is_ok() {
        let temp = subscriber_temperature.receive()?.map(|s| *s);
        let hum = subscriber_humidity.receive()?.map(|s| *s);
        let curr = subscriber_current.receive()?.map(|s| *s);

        if let (Some(temperature), Some(humidity), Some(current)) = (temp, hum, curr) {
            let timestamp = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();
        
            println!(
                "[sm_rust_sub] Forwarding → temp: {:.2}, hum: {}, current: {:.2}, timestamp: {}",
                temperature, humidity, current, timestamp
            );
        
            let sensor_data = json!({
                "id": "sm_rust",
                "timestamp": timestamp,
                "data": {
                    "temperature": temperature,
                    "humidity": humidity,
                    "current": current
                }
            });
        
            match client.post(REST_API_URL)
                .json(&sensor_data)
                .send()
            {
                Ok(response) => {
                    if response.status().is_success() {
                        println!("[sm_rust_sub] Sent data to REST API successfully!");
                    } else {
                        println!("[sm_rust_sub] REST API error: {}", response.status());
                    }
                }
                Err(err) => println!("[sm_rust_sub] Failed to send data: {}", err),
            }
        }
    }

    Ok(())
}
