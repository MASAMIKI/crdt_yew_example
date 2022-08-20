use crate::components::form::Form;
use crate::state::{Action, State};
use gloo::storage::{LocalStorage, Storage};
use std::collections::HashMap;
use yew::prelude::*;

const KEY: &str = "crdt.client.example";

#[function_component(Home)]
pub fn home() -> Html {
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

    let onadd = {
        let state = state.clone();
        Callback::from(move |(key, value): (String, String)| {
            state.dispatch(Action::Edit(key, value));
        })
    };

    html! {
        <div class="container">
            <div class="card">
                <div class="card-content">
                    <div class="content">
                        {  format!("{:?}", state.hash_map) }
                        <Form {onadd} inputs={state.hash_map.clone()} />
                    </div>
                </div>
            </div>
        </div>
    }
}
