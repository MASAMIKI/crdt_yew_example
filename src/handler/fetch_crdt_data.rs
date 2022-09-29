use crate::CrdtState;
use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        Extension, TypedHeader,
    },
    response::IntoResponse,
};

const INTERVAL: u64 = 1;

pub async fn handler(
    ws: WebSocketUpgrade,
    user_agent: Option<TypedHeader<headers::UserAgent>>,
    Extension(crdt_state): Extension<CrdtState>,
) -> impl IntoResponse {
    if let Some(TypedHeader(user_agent)) = user_agent {
        println!("`{}` connected", user_agent.as_str());
    }

    ws.on_upgrade(|socket| handle_socket(socket, crdt_state))
}

pub async fn handle_socket(mut socket: WebSocket, crdt_state: CrdtState) {
    loop {
        let encoded: Vec<u8> = serde_cbor::to_vec(&crdt_state.read().unwrap().db).unwrap();
        if socket.send(Message::Binary(encoded)).await.is_err() {
            println!("client disconnected");
            return;
        }
        tokio::time::sleep(std::time::Duration::from_secs(INTERVAL)).await;
    }
}
