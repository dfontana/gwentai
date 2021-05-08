use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Serialize)]
pub struct DeckListResponse {
  pub guides: Vec<DeckMetadata>,
}

#[derive(Clone, Deserialize, Debug, Serialize)]
pub struct DeckMetadata {
  pub id: usize,
  pub votes: i32,
  #[serde(rename = "leaderId")]
  pub leader: usize,
  pub faction: Faction,
  pub invalid: bool,
}

#[derive(Clone, Deserialize, Debug, Serialize)]
pub struct Faction {
  pub short: String,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct DeckResponse {
  pub deck: Deck,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct Deck {
  #[serde(rename = "srcCardTemplates")]
  pub cards: Vec<usize>,
}
