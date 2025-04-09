use serde_json::json;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

use crate::json;

pub async fn post_device_data(payload: json::NodePayload,
                              device: &str) 
-> Result<(), reqwest::Error> {
    let client = reqwest::Client::new();
    let url = format!("http://localhost:5000/node/{}/data", device);

    let response = client.post(url).json(&payload).send().await?;

    Ok(())
} 

pub async fn start_tcp_actix() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:65432").await?;
    println!("[tcp_actix] Listening on 127.0.0.1:65432...");

    loop {
        let (mut socket, addr) = listener.accept().await?;
        println!("[tcp_actix] New client from {}", addr);

        tokio::spawn(async move {
            let mut buffer = vec![0u8; 4096];

            loop {
                let bytes_read = match socket.read(&mut buffer).await {
                    Ok(0) => {
                        println!("[tcp_actix] Client disconnected.");
                        return;
                    }
                    Ok(n) => n,
                    Err(e) => {
                        eprintln!("[tcp_actix] Read error: {}", e);
                        return;
                    }
                };

                let received = &buffer[..bytes_read];
                println!(
                    "[tcp_actix] Received: {}",
                    String::from_utf8_lossy(received)
                );

                match serde_json::from_slice::<json::NodePayload>(received) {
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

                if let Err(e) = socket
                    .write_all(response.to_string().as_bytes())
                    .await
                {
                    eprintln!("[tcp_actix] Write error: {}", e);
                    return;
                }
            }
        });
    }
}