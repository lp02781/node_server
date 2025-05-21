use core::time::Duration;
use iceoryx2::prelude::*;
use rand::Rng;
//use std::time::{SystemTime, UNIX_EPOCH};
//use reqwest::blocking::Client;
//use serde_json::json;

const CYCLE_TIME: Duration = Duration::from_secs(10);
//const REST_API_URL: &str = "http://localhost:5000/node/sm_rust_1/data";

fn main() -> Result<(), Box<dyn core::error::Error>> {
    let node = NodeBuilder::new().create::<ipc::Service>()?;
    //let client = Client::new();

    let service_temperature = node
        .service_builder(&"sensor_data/sm_rust_pub/temperature".try_into()?)
        .publish_subscribe::<f32>()
        .open_or_create()?;

    let service_humidity = node
        .service_builder(&"sensor_data/sm_rust_pub/humidity".try_into()?)
        .publish_subscribe::<i16>()
        .open_or_create()?;

    let service_current = node
        .service_builder(&"sensor_data/sm_rust_pub/current".try_into()?)
        .publish_subscribe::<f32>()
        .open_or_create()?;

    let publisher_temperature = service_temperature.publisher_builder().create()?;
    let publisher_humidity = service_humidity.publisher_builder().create()?;
    let publisher_current = service_current.publisher_builder().create()?;

    /*
    let sub_temperature = node
        .service_builder(&"sensor_data/sm_rust_2/temperature".try_into()?)
        .publish_subscribe::<f32>()
        .open_or_create()?;

    let sub_humidity = node
        .service_builder(&"sensor_data/sm_rust_2/humidity".try_into()?)
        .publish_subscribe::<i16>()
        .open_or_create()?;

    let sub_current = node
        .service_builder(&"sensor_data/sm_rust_2/current".try_into()?)
        .publish_subscribe::<f32>()
        .open_or_create()?;

    let subscriber_temperature = sub_temperature.subscriber_builder().create()?;
    let subscriber_humidity = sub_humidity.subscriber_builder().create()?;
    let subscriber_current = sub_current.subscriber_builder().create()?;
    */

    let mut rng = rand::thread_rng();

    println!("[sm_rust_pub] sm_rust_pub is ready");

    while node.wait(CYCLE_TIME).is_ok() {
        let temperature = rng.gen_range(15.0..40.0);
        let humidity = rng.gen_range(30..80);
        let current = rng.gen_range(1.0..10.0);
        
        /*
        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();

        let sensor_data = json!({
            "id": "sm_rust_1",
            "timestamp": timestamp,
            "data": {
                "temperature": temperature,
                "humidity": humidity,
                "current": current
            }
        });
        */

        publisher_temperature.loan_uninit()?.write_payload(temperature).send()?;
        publisher_humidity.loan_uninit()?.write_payload(humidity).send()?;
        publisher_current.loan_uninit()?.write_payload(current).send()?;

        println!(
            "[sm_rust_pub] Published → temp: {:.2}, hum: {}, current: {:.2}",
            temperature, humidity, current
        );

        /*

        match client.post(REST_API_URL)
            .json(&sensor_data)
            .send()
        {
            Ok(response) => {
                if response.status().is_success() {
                    println!("[sm_rust_1] Sent data to REST API successfully!");
                } else {
                    println!("[sm_rust_1] REST API error: {}", response.status());
                }
            }
            Err(err) => println!("[sm_rust_1] Failed to send data: {}", err),
        }

        if let Some(sample) = subscriber_temperature.receive()? {
            println!("[sm_rust_1] Received temp: {:.2}", *sample);
        }
        if let Some(sample) = subscriber_humidity.receive()? {
            println!("[sm_rust_1] Received hum: {}", *sample);
        }
        if let Some(sample) = subscriber_current.receive()? {
            println!("[sm_rust_1] Received current: {:.2}", *sample);
        }
        
        */
    }
    Ok(())
}
