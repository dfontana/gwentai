mod gwentone;
mod playgwent;

use askama::Template;
use askama_axum::IntoResponse;
use axum::{
  extract::{Extension, Path},
  routing::get,
  AddExtensionLayer, Json, Router,
};
use gwentone::CardData;
use reqwest::{Client, StatusCode};
use std::{net::SocketAddr, sync::Arc};
use tower_http::{
  trace::{DefaultOnResponse, TraceLayer},
  LatencyUnit,
};
use tracing::{info, Level};

use playgwent::GwentClient;

use crate::gwentone::GwentOneClient;

struct State {
  client: GwentClient,
  gwentone: GwentOneClient,
}

#[tokio::main]
async fn main() {
  // Configure Tracing
  if std::env::var_os("RUST_LOG").is_none() {
    std::env::set_var("RUST_LOG", "info,tower_http=info")
  }
  tracing_subscriber::fmt::init();

  let client = Client::new();
  let state = Arc::new(State {
    client: GwentClient::new(client.clone()),
    gwentone: GwentOneClient::new(client.clone()),
  });

  let app = Router::new()
    .route("/cards/display", get(cards_display))
    .route("/cards", get(cards))
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

async fn cards_display(Extension(state): Extension<Arc<State>>) -> CardTemplate {
  CardTemplate {
    cards: state
      .gwentone
      .get_cards()
      .await
      .values()
      .map(|c| c.clone())
      .collect(),
  }
}

async fn cards(Extension(state): Extension<Arc<State>>) -> impl IntoResponse {
  let cards = state.gwentone.get_cards().await;
  (StatusCode::OK, Json(cards))
}

#[derive(Template)]
#[template(path = "cards.html")]
struct CardTemplate {
  cards: Vec<CardData>,
}
