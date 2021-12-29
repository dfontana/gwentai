mod playgwent;

use askama::Template;
use axum::{
  extract::{Extension, Path},
  routing::get,
  AddExtensionLayer, Router,
};
use std::{net::SocketAddr, sync::Arc};
use tower_http::{
  trace::{DefaultOnResponse, TraceLayer},
  LatencyUnit,
};
use tracing::{info, Level};

use playgwent::GwentClient;

struct State {
  client: GwentClient,
}

#[tokio::main]
async fn main() {
  // Configure Tracing
  if std::env::var_os("RUST_LOG").is_none() {
    std::env::set_var("RUST_LOG", "info,tower_http=info")
  }
  tracing_subscriber::fmt::init();

  let state = Arc::new(State {
    client: GwentClient::new(),
  });

  let app = Router::new()
    .route("/greet/:name", get(greet))
    .layer(
      TraceLayer::new_for_http().on_request(()).on_response(
        DefaultOnResponse::new()
          .level(Level::INFO)
          .latency_unit(LatencyUnit::Micros),
      ),
    )
    .layer(AddExtensionLayer::new(state));

  let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
  info!("listening on {}", addr);
  axum::Server::bind(&addr)
    .serve(app.into_make_service())
    .await
    .unwrap();
}

async fn greet(Extension(state): Extension<Arc<State>>, Path(name): Path<String>) -> HelloTemplate {
  HelloTemplate { name }
}

#[derive(Template)]
#[template(path = "hello.html")]
struct HelloTemplate {
  name: String,
}
