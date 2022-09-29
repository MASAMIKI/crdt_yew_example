use wasm_bindgen::{prelude::Closure, JsCast};
use web_sys::{ErrorEvent, MessageEvent, WebSocket};

#[derive(Clone, PartialEq)]
pub struct CrdtWebSockets {
    pub commit_socket: WebSocket,
    pub fetch_socket: WebSocket,
}

impl CrdtWebSockets {
    pub fn new<T: AsRef<str>>(protocol: T, ip: [u8; 4], port: u16) -> CrdtWebSockets {
        let url = format!(
            "{}://{}:{}/",
            protocol.as_ref(),
            ip.map(|i| i.to_string()).join("."),
            port
        );
        let commit_crdt_url = url.clone() + "commit_crdt";
        let fetch_crdt_url = url + "fetch_crdt";
        let cs = default_socket(commit_crdt_url);
        let fs = default_socket(fetch_crdt_url);
        CrdtWebSockets {
            commit_socket: cs,
            fetch_socket: fs,
        }
    }
}

pub fn default_socket(url: String) -> WebSocket {
    let ws = WebSocket::new(url.as_str()).unwrap();
    let onmessage_callback = Closure::<dyn FnMut(_)>::new(move |e: MessageEvent| {
        log::info!("message event: {:?}", e.data());
    });
    ws.set_onmessage(Some(onmessage_callback.as_ref().unchecked_ref()));
    onmessage_callback.forget();

    let onerror_callback = Closure::<dyn FnMut(_)>::new(move |e: ErrorEvent| {
        log::info!("error event: {:?}", e);
    });
    ws.set_onerror(Some(onerror_callback.as_ref().unchecked_ref()));
    onerror_callback.forget();

    let onopen_callback = Closure::<dyn FnMut()>::new(move || {
        log::info!("socket opened");
    });
    ws.set_onopen(Some(onopen_callback.as_ref().unchecked_ref()));
    onopen_callback.forget();

    let onclose_callback = Closure::<dyn FnMut()>::new(move || {
        log::info!("socket closed");
    });
    ws.set_onclose(Some(onclose_callback.as_ref().unchecked_ref()));
    onclose_callback.forget();
    ws
}
