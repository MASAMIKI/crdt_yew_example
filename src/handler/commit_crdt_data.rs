use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        Extension, TypedHeader,
    },
    response::IntoResponse,
};
use serde_cbor::from_slice;
use std::collections::HashMap;

use crate::CrdtState;

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
        if let Some(msg) = socket.recv().await {
            if let Ok(msg) = msg {
                match msg {
                    Message::Text(t) => {
                        println!("client sent str: {:?}", t);
                    }
                    Message::Binary(b) => {
                        let hash_map: HashMap<String, String> = from_slice(&b).unwrap();
                        for (key, value) in &hash_map {
                            crdt_state
                                .write()
                                .unwrap()
                                .db
                                .insert(key.to_string(), value.to_string());
                        }
                        println!("client sent binary data: {:?}", hash_map);
                    }
                    Message::Ping(_) => {
                        println!("socket ping");
                    }
                    Message::Pong(_) => {
                        println!("socket pong");
                    }
                    Message::Close(_) => {
                        println!("client disconnected");
                        return;
                    }
                }
            } else {
                println!("client disconnected");
                return;
            }
        }
    }
}
