use crate::components::form::Form;
use crate::crdt_websocket::CrdtWebSockets;
use crate::state::{Action, State};

use gloo::storage::{LocalStorage, Storage};
use serde_cbor::from_slice;
use wasm_bindgen::{prelude::Closure, JsCast, JsValue};
use web_sys::MessageEvent;
use yew::prelude::{function_component, html, use_effect_with_deps, use_reducer, Callback};
use yew::use_context;
use crdts::{CmRDT, CvRDT, Map, Orswot};
use uuid::Uuid;

const CRDT_KEY: &str = "crdt.client.example";
const USER_ID_KEY: &str  =  "crdt.user";

#[function_component(Crdt)]
pub fn crdt() -> Html {
    let ws = use_context::<CrdtWebSockets>().expect("no ctx found");
    let commit_socket = ws.commit_socket;
    let fetch_socket = ws.fetch_socket;

    let state = use_reducer(|| State {
        crdt_map: LocalStorage::get(CRDT_KEY).unwrap_or_else(|_| Map::new()),
    });

    let userId: String = LocalStorage::get(USER_ID_KEY).unwrap_or_else(|_| Uuid::new_v4().to_string());
    LocalStorage::set(USER_ID_KEY, userId).expect("failed to set user id");

    use_effect_with_deps(
        move |state| {
            LocalStorage::set(CRDT_KEY, &state.clone().crdt_map).expect("failed to set data");
            || ()
        },
        state.clone(),
    );

    let state_to_commit = state.clone();
    let on_change = {
        let state = state_to_commit;
        Callback::from(move |(key, value): (String, String)| {
            state.dispatch(Action::Edit(commit_socket.clone(), key, value, userId.to_string()));
        })
    };

    let state_to_fetch = state.clone();
    let onmessage_callback = Closure::<dyn FnMut(_)>::new(move |e: MessageEvent| {
        let blob = e.data().dyn_into::<web_sys::Blob>().unwrap();
        let response = web_sys::Response::new_with_opt_blob(Some(&blob)).unwrap();
        let array_buffer = response.array_buffer().unwrap();

        let state = state_to_fetch.clone();
        let userId_clone = userId.clone();
        let callback = Closure::<dyn FnMut(_)>::new(move |ab: JsValue| {
            let ua = js_sys::Uint8Array::new(&ab);
            let hash_map: Map<String, Orswot<String, String>, String> = from_slice(&ua.to_vec()).unwrap();
            state.dispatch(Action::Bulk(hash_map, userId_clone.clone()));
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
                    <Form {on_change} inputs={state.crdt_map.clone()} />
                </div>
            </div>
        </div>
    }
}
