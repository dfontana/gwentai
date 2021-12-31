mod model;
mod scrape;
mod storage;

use reqwest::Client;
use std::collections::HashMap;
use tracing::info;

pub use model::*;
use storage::Disk;

pub struct GwentOneClient {
  http: Client,
}

impl GwentOneClient {
  pub fn new(http: Client) -> Self {
    GwentOneClient { http }
  }

  pub async fn get_cards(&self) -> HashMap<usize, CardData> {
    // TODO cache this in memory so we don't go to disk each time
    match Disk::load("cards.json") {
      Ok(disk) => {
        info!("Using cached cards");
        disk.cards
      }
      Err(_) => {
        info!("Cache miss, loading from GwentOne");
        let cards = scrape::cards(self.http.clone())
          .await
          .expect("Failed to load cards from GwentOne");
        let storage = Disk { cards };
        let cards = storage
          .save("cards.json")
          .expect("Failed to save Card Cache")
          .cards;
        info!("Cache written");
        cards
      }
    }
  }
}
