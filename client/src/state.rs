use crdts::{CmRDT, CvRDT, Map, Orswot};
use serde::{Deserialize, Serialize};
use std::rc::Rc;
use wasm_bindgen_futures::spawn_local;
use web_sys::WebSocket;
use yew::prelude::*;

pub enum Action {
    Edit(WebSocket, String, String, String),
    Bulk(Map<String, Orswot<String, String>, String>, String),
}
/// state for values of input form
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct State {
    pub crdt_map: Map<String, Orswot<String, String>, String>,
}

impl Reducible for State {
    type Action = Action;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        match action {
            Action::Edit(socket, key, value, userKey) => {
                let read_ctx = self.crdt_map.len();
                self.crdt_map.apply(self.crdt_map.update(
                    key,
                    read_ctx.derive_add_ctx(userKey),
                    |set, ctx| set.add(value, ctx),
                ));
                let hash_map_clone = self.crdt_map.clone();
                spawn_local(async move {
                    let encoded: Vec<u8> = serde_cbor::to_vec(&hash_map_clone).unwrap();
                    socket
                        .send_with_u8_array(&encoded)
                        .unwrap_or_else(|err| log::info!("error sending message: {:?}", err));
                });
                State {
                    crdt_map: self.crdt_map.clone(),
                }
                .into()
            }
            Action::Bulk(hash_map, userKey) => {
                self.crdt_map.merge(hash_map);
                State {
                    crdt_map: self.crdt_map.clone(),
                }
                .into()
            }
        }
    }
}
