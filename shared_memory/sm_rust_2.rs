use core::time::Duration;
use iceoryx2::prelude::*;
use rand::Rng; 

const CYCLE_TIME: Duration = Duration::from_secs(10);

fn main() -> Result<(), Box<dyn core::error::Error>> {
    let node = NodeBuilder::new().create::<ipc::Service>()?;

    let service_temperature = node
        .service_builder(&"sensor_data/sm_rust_2/temperature".try_into()?)
        .publish_subscribe::<f32>() 
        .open_or_create()?;

    let service_humidity = node
        .service_builder(&"sensor_data/sm_rust_2/humidity".try_into()?)
        .publish_subscribe::<i16>() 
        .open_or_create()?;

    let service_current = node
        .service_builder(&"sensor_data/sm_rust_2/current".try_into()?) 
        .publish_subscribe::<f32>() 
        .open_or_create()?;

    let publisher_temperature = service_temperature.publisher_builder().create()?;
    let publisher_humidity = service_humidity.publisher_builder().create()?;
    let publisher_current = service_current.publisher_builder().create()?;

    let sub_temperature = node
        .service_builder(&"sensor_data/sm_rust_1/temperature".try_into()?)
        .publish_subscribe::<f32>()
        .open_or_create()?;

    let sub_humidity = node
        .service_builder(&"sensor_data/sm_rust_1/humidity".try_into()?)
        .publish_subscribe::<i16>()
        .open_or_create()?;

    let sub_current = node
        .service_builder(&"sensor_data/sm_rust_1/current".try_into()?)
        .publish_subscribe::<f32>()
        .open_or_create()?;

    let subscriber_temperature = sub_temperature.subscriber_builder().create()?;
    let subscriber_humidity = sub_humidity.subscriber_builder().create()?;
    let subscriber_current = sub_current.subscriber_builder().create()?;

    let mut rng = rand::thread_rng(); 

    while node.wait(CYCLE_TIME).is_ok() {
        let temperature = rng.gen_range(15.0..40.0);
        let humidity = rng.gen_range(30..80); 
        let current = rng.gen_range(1.0..10.0);

        publisher_temperature.loan_uninit()?.write_payload(temperature).send()?;
        publisher_humidity.loan_uninit()?.write_payload(humidity).send()?;
        publisher_current.loan_uninit()?.write_payload(current).send()?;

        println!(
            "[sm_rust_2] Published → temp: {:.2}, hum: {}, current: {:.2}",
            temperature, humidity, current
        );

        if let Some(sample) = subscriber_temperature.receive()? {
            println!("[sm_rust_2] Received temp: {:.2}", *sample);
        }
        
        if let Some(sample) = subscriber_humidity.receive()? {
            println!("[sm_rust_2] Received hum: {:.2}", *sample);
        }
        
        if let Some(sample) = subscriber_current.receive()? {
            println!("[sm_rust_2] Received cur: {:.2}", *sample);
        }
    }
    Ok(())
}
