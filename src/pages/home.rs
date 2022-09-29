use crate::components::crdt::Crdt;
use crate::crdt_websocket::CrdtWebSockets;
use yew::prelude::{function_component, html, use_state, ContextProvider};

const PROTOCOL: &str = "ws";
const IP: [u8; 4] = [127, 0, 0, 1];
const PORT: u16 = 3000;

#[function_component(Home)]
pub fn home() -> Html {
    let sockets = use_state(|| CrdtWebSockets::new(PROTOCOL, IP, PORT));
    html! {
        <div class="container">
            <ContextProvider<CrdtWebSockets> context={(*sockets).clone()}>
                <Crdt />
            </ContextProvider<CrdtWebSockets>>
        </div>
    }
}
