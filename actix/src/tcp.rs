use serde::{Deserialize, Serialize};
use serde_json::json;
use std::net::TcpListener;
use std::io::{Read, Write};
use std::thread;
use std::time::Duration;

use crate::json;

pub async fn post_device_data(payload: json::NodePayload,
                            device: &str) 
-> Result<(), reqwest::Error> {
    let client = reqwest::Client::new();
    let url = format!("http://localhost:5000/node/{}/data", device);

    let response = client
        .post(url)
        .json(&payload)
        .send()
        .await?;

    Ok(())
} 

pub async fn start_tcp_actix() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:65432")?;
    println!("[tcp_actix] Listening on 127.0.0.1:65432...");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("[tcp_actix] New client connected.");
                thread::spawn(|| {
                    if let Err(e) = handle_client(stream) {
                        eprintln!("[tcp_actix] Error: {}", e);
                    }
                });
            }
            Err(e) => {
                eprintln!("[tcp_actix] Connection failed: {}", e);
            }
        }
    }

    Ok(())
}

fn handle_client(mut stream: std::net::TcpStream) -> std::io::Result<()> {
    let mut buffer = [0; 4096];

    loop {
        let bytes = stream.read(&mut buffer)?;
        if bytes == 0 {
            println!("[tcp_actix] Client disconnected.");
            break;
        }

        let received = String::from_utf8_lossy(&buffer[..bytes]);
        println!("[tcp_actix] Received: {}", received);

        match serde_json::from_slice::<json::NodePayload>(&received) {
            Ok(payload) => {
                let device_id = payload.id.clone();
                tokio::spawn(async move {
                    if let Err(e) = post_device_data(payload, &device_id).await {
                        eprintln!("[tcp_actix] HTTP post error: {}", e);
                    }
                });
            }
            Err(e) => {
                eprintln!("[tcp_actix] JSON parse error: {}", e);
            }
        }

        let response = json!({
            "status": "ok",
            "message": "Data received"
        });

        let response_string = response.to_string();
        stream.write_all(response_string.as_bytes())?;
    }

    Ok(())
}
