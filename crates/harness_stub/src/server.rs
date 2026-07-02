use crate::engine::StubEngine;
use axum::{
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    extract::State,
    response::IntoResponse,
    routing::get,
    Json, Router,
};
use cuecode_chp::{ChpEnvelope, CHP_VERSION, CHP_WS_SUBPROTOCOL};
use futures::StreamExt;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use tokio::net::TcpListener;

#[derive(Clone)]
struct AppState {
    engine: Arc<Mutex<StubEngine>>,
}

pub async fn run_server(addr: SocketAddr) -> anyhow::Result<()> {
    let state = AppState {
        engine: Arc::new(Mutex::new(StubEngine::new())),
    };

    let app = Router::new()
        .route("/health", get(health))
        .route("/v1/chp/connect", get(chp_connect))
        .with_state(state);

    let listener = TcpListener::bind(addr).await?;
    log::info!("harness-stub listening on http://{addr}");
    axum::Server::from_tcp(listener.into_std()?)?
        .serve(app.into_make_service())
        .await?;
    Ok(())
}

async fn health() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "status": "ok",
        "chp_version": CHP_VERSION
    }))
}

async fn chp_connect(ws: WebSocketUpgrade, State(state): State<AppState>) -> impl IntoResponse {
    ws.protocols([CHP_WS_SUBPROTOCOL])
        .on_upgrade(move |socket| handle_socket(socket, state))
}

async fn handle_socket(mut socket: WebSocket, state: AppState) {
    while let Some(message) = socket.next().await {
        let message = match message {
            Ok(Message::Text(text)) => text,
            Ok(Message::Close(_)) => break,
            Ok(_) => continue,
            Err(error) => {
                log::debug!("websocket read error: {error}");
                break;
            }
        };

        let envelope = match ChpEnvelope::from_json(&message) {
            Ok(envelope) => envelope,
            Err(error) => {
                log::warn!("invalid chp json: {error}");
                continue;
            }
        };

        let responses = {
            let mut engine = state.engine.lock().expect("engine lock");
            match engine.handle(&envelope) {
                Ok(responses) => responses,
                Err(error) => {
                    log::warn!("engine error: {error}");
                    continue;
                }
            }
        };

        for response in responses {
            let text = match response.to_json() {
                Ok(text) => text,
                Err(error) => {
                    log::warn!("serialize error: {error}");
                    continue;
                }
            };
            if socket.send(Message::Text(text)).await.is_err() {
                break;
            }
        }
    }
}
