use actix::prelude::*;
use actix_web_actors::ws;
use reqwest;
use crate::json; 

pub struct WebSocketSession {
    pub device_id: String,
}

impl Actor for WebSocketSession {
    type Context = ws::WebsocketContext<Self>;
}

pub async fn post_device_data(payload: json::NodePayload, device: &str) -> Result<(), reqwest::Error> {
    let client = reqwest::Client::new();
    let url = format!("http://localhost:5000/node/{}/data", device);

    let response = client
        .post(url)
        .json(&payload)
        .send()
        .await?;

    Ok(())
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WebSocketSession {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Text(text)) => {
                let device_id = self.device_id.clone(); 

                tokio::spawn(async move {
                    if let Ok(payload) = serde_json::from_str::<json::NodePayload>(&text) {
                        if let Err(e) = post_device_data(payload, &device_id).await {
                            eprintln!("Error sending data for device {}: {:?}", device_id, e);
                        }
                    } else {
                        eprintln!("Invalid JSON received: {}", text);
                    }
                });
            }
            Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
            Ok(ws::Message::Pong(_)) => {}
            Ok(ws::Message::Close(reason)) => {
                println!("WebSocket closed: {:?}", reason);
                ctx.stop();
            }
            _ => {}
        }
    }
}
