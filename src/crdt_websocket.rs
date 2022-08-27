use wasm_bindgen::{prelude::Closure, JsCast};
use web_sys::{ErrorEvent, MessageEvent, WebSocket};

const PROTOCOL: &str = "ws";
const IP: [u8; 4] = [127, 0, 0, 1];
const PORT: u16 = 3000;

#[derive(Clone, PartialEq)]
pub struct CrdtWebSockets {
    pub commit_socket: WebSocket,
    pub fetch_socket: WebSocket,
}

pub fn commit_socket() -> WebSocket {
    let url = format!(
        "{}://{}:{}/{}",
        PROTOCOL,
        IP.map(|i| i.to_string()).join("."),
        PORT,
        "commit_crdt",
    );
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

pub fn fetch_socket() -> WebSocket {
    let url = format!(
        "{}://{}:{}/{}",
        PROTOCOL,
        IP.map(|i| i.to_string()).join("."),
        PORT,
        "fetch_crdt",
    );
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