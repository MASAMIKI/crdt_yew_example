use crate::CrdtState;
use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        Extension, TypedHeader,
    },
    response::IntoResponse,
};

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
        let state_text = serde_json::to_string(&crdt_state.read().unwrap().db).unwrap();
        if socket.send(Message::Text(state_text)).await.is_err() {
            println!("client disconnected");
            return;
        }
        tokio::time::sleep(std::time::Duration::from_secs(3)).await;
    }
}
