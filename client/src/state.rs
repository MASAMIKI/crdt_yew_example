use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::rc::Rc;
use wasm_bindgen_futures::spawn_local;
use web_sys::WebSocket;
use yew::prelude::*;

pub enum Action {
    Edit(WebSocket, String, String),
    Bulk(HashMap<String, String>),
}
/// state for values of input form
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct State {
    pub hash_map: HashMap<String, String>,
}

impl Reducible for State {
    type Action = Action;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        match action {
            Action::Edit(socket, key, value) => {
                let mut hash_map = self.hash_map.clone();
                hash_map.insert(key, value);
                let hash_map_clone = hash_map.clone();
                spawn_local(async move {
                    let encoded: Vec<u8> = serde_cbor::to_vec(&hash_map_clone).unwrap();
                    socket
                        .send_with_u8_array(&encoded)
                        .unwrap_or_else(|err| log::info!("error sending message: {:?}", err));
                });
                State { hash_map }.into()
            }
            Action::Bulk(hash_map) => State { hash_map }.into(),
        }
    }
}
