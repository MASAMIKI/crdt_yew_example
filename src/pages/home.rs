use crate::components::crdt::Crdt;
use crate::crdt_websocket::{CrdtWebSockets, commit_socket, fetch_socket};
use yew::prelude::{function_component, html, use_state, ContextProvider};

#[function_component(Home)]
pub fn home() -> Html {
    let sockets = use_state(|| CrdtWebSockets {
        commit_socket: commit_socket(),
        fetch_socket: fetch_socket(),
    });
    html! {
        <div class="container">
            <ContextProvider<CrdtWebSockets> context={(*sockets).clone()}>
                <Crdt />
            </ContextProvider<CrdtWebSockets>>
        </div>
    }
}
