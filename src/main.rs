use axum::{
    handler::Handler,
    http::{StatusCode, Uri},
    routing::get,
    Router,
};
use std::net::{Ipv4Addr, SocketAddr};
use tower_http::trace::{DefaultMakeSpan, TraceLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

const IP: [u8; 4] = [127, 0, 0, 1];
const PORT: u16 = 3000;

mod handler;

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(EnvFilter::from_env("RUST_LOG").add_directive(tracing::Level::DEBUG.into()))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let trace_layer =
        TraceLayer::new_for_http().make_span_with(DefaultMakeSpan::default().include_headers(true));

    let app = Router::new()
        .route("/commit_crdt", get(handler::commit_crdt_data::handler))
        .route("/fetch_crdt", get(handler::fetch_crdt_data::handler))
        .fallback(fallback.into_service())
        .layer(trace_layer);

    let addr = SocketAddr::from((Ipv4Addr::from(IP), PORT));
    tracing::debug!("listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn fallback(uri: Uri) -> (StatusCode, String) {
    (StatusCode::NOT_FOUND, format!("No route for {}", uri))
}
