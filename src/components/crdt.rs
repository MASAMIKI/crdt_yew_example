use crate::components::form::Form;
use crate::state::{Action, State};
use crate::crdt_websocket::CrdtWebSockets;

use gloo::storage::{LocalStorage, Storage};
use std::collections::HashMap;
use yew::use_context;
use yew::prelude::{function_component, html, use_effect_with_deps, use_reducer, Callback};
use wasm_bindgen_futures::spawn_local;

const KEY: &str = "crdt.client.example";

#[function_component(Crdt)]
pub fn crdt() -> Html {
    let ws = use_context::<CrdtWebSockets>().expect("no ctx found");
    let commit_socket = ws.commit_socket;

    let state = use_reducer(|| State {
        hash_map: LocalStorage::get(KEY).unwrap_or_else(|_| HashMap::new()),
    });

    use_effect_with_deps(
        move |state| {
            LocalStorage::set(KEY, &state.clone().hash_map).expect("failed to set");
            || ()
        },
        state.clone(),
    );

    let on_change = {
        let state = state.clone();
        Callback::from(move |(key, value): (String, String)| {
            let cloned_commit_socket = commit_socket.clone();
            state.dispatch(Action::Edit(key, value.clone()));
            spawn_local(async move {
                match cloned_commit_socket.send_with_str(value.as_str()) {
                    Ok(_) => log::info!("message successfully sent: {:?}", value.as_str()),
                    Err(err) => log::info!("error sending message: {:?}", err),
                }
            });
        })
    };

    html! {
        <div class="card">
            <div class="card-content">
                <div class="content">
                    {  format!("{:?}", state.hash_map) }
                    <Form {on_change} inputs={state.hash_map.clone()} />
                </div>
            </div>
        </div>
    }
}
