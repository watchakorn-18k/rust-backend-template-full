use actix_web::{web, Error, HttpRequest, HttpResponse};
use actix_ws::{handle, Message};
use futures_util::StreamExt;
use log::info;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct WsQuery {
    pub id: String, // /ws?id=<this>
}

/// GET /ws?id=YOUR_ID
pub async fn ws_upgrade(
    req: HttpRequest,
    body: web::Payload,
    q: web::Query<WsQuery>,
) -> Result<HttpResponse, Error> {
    let client_id = q.into_inner().id;

    if client_id.trim().is_empty() {
        return Ok(HttpResponse::BadRequest().body("missing id"));
    }

    let (response, mut session, mut msg_stream) = handle(&req, body)?;

    actix_web::rt::spawn(async move {
        info!(
            "WebSocket connected: id={} from={}",
            client_id,
            req.peer_addr().map(|a| a.to_string()).unwrap_or_default()
        );

        let _ = session
            .text(format!("👋 connected (id={})", client_id))
            .await;

        while let Some(Ok(msg)) = msg_stream.next().await {
            match msg {
                Message::Ping(bytes) => {
                    let _ = session.pong(&bytes).await;
                }
                Message::Text(text) => {
                    // echo กลับ แบบติด id ไว้ให้เห็นว่าเป็นใคร
                    let _ = session.text(format!("[{}] {}", client_id, text)).await;
                    info!("[{}] : {}", client_id, text);
                    if text == "hi" {
                        let _ = session.text(format!("👋 hi (id={})", client_id)).await;
                    } else if text == "bye" {
                        let _ = session.close(None).await;
                        break;
                    }
                }
                Message::Binary(bin) => {
                    let _ = session.binary(bin).await;
                }
                Message::Close(reason) => {
                    let _ = session.close(reason).await;
                    break;
                }
                _ => {}
            }
        }

        info!("WebSocket disconnected: id={}", client_id);
    });

    Ok(response)
}
