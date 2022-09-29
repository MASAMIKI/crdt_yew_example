use axum::{
    extract::Extension,
    handler::Handler,
    http::{StatusCode, Uri},
    response::{IntoResponse, Json},
    routing::get,
    Router,
};
use std::net::{Ipv4Addr, SocketAddr};
use tower_http::trace::{DefaultMakeSpan, TraceLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

const IP: [u8; 4] = [127, 0, 0, 1];
const PORT: u16 = 3000;

mod handler;
mod layer;
use crate::layer::crdt_data::CrdtState;

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
        .route("/", get(index))
        .layer(trace_layer)
        .layer(Extension(CrdtState::default()))
        .fallback(fallback.into_service());

    let addr = SocketAddr::from((Ipv4Addr::from(IP), PORT));
    tracing::debug!("listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn index(Extension(crdt_state): Extension<CrdtState>) -> impl IntoResponse {
    Json(crdt_state.read().unwrap().db.clone())
}

async fn fallback(uri: Uri) -> impl IntoResponse {
    (StatusCode::NOT_FOUND, format!("No route for {}", uri))
}
