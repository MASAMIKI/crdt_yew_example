use crate::components::form::Form;
use crate::crdt_websocket::CrdtWebSockets;
use crate::state::{Action, State};

use gloo::storage::{LocalStorage, Storage};
use serde_cbor::from_slice;
use std::collections::HashMap;
use wasm_bindgen::{prelude::Closure, JsCast, JsValue};
use web_sys::MessageEvent;
use yew::prelude::{function_component, html, use_effect_with_deps, use_reducer, Callback};
use yew::use_context;

const KEY: &str = "crdt.client.example";

#[function_component(Crdt)]
pub fn crdt() -> Html {
    let ws = use_context::<CrdtWebSockets>().expect("no ctx found");
    let commit_socket = ws.commit_socket;
    let fetch_socket = ws.fetch_socket;

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

    let state_to_commit = state.clone();
    let on_change = {
        let state = state_to_commit;
        Callback::from(move |(key, value): (String, String)| {
            state.dispatch(Action::Edit(commit_socket.clone(), key, value));
        })
    };

    let state_to_fetch = state.clone();
    let onmessage_callback = Closure::<dyn FnMut(_)>::new(move |e: MessageEvent| {
        let blob = e.data().dyn_into::<web_sys::Blob>().unwrap();
        let response = web_sys::Response::new_with_opt_blob(Some(&blob)).unwrap();
        let array_buffer = response.array_buffer().unwrap();

        let state = state_to_fetch.clone();
        let callback = Closure::<dyn FnMut(_)>::new(move |ab: JsValue| {
            let ua = js_sys::Uint8Array::new(&ab);
            let hash_map: HashMap<String, String> = from_slice(&ua.to_vec()).unwrap();
            state.dispatch(Action::Bulk(hash_map));
        });
        array_buffer.then(&callback);
        callback.forget();
    });
    fetch_socket.set_onmessage(Some(onmessage_callback.as_ref().unchecked_ref()));
    onmessage_callback.forget();

    html! {
        <div class="card">
            <div class="card-content">
                <div class="content">
                    <Form {on_change} inputs={state.hash_map.clone()} />
                </div>
            </div>
        </div>
    }
}
