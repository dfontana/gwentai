use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Serialize)]
pub struct DeckList {
  pub guides: Vec<DeckListItem>,
}

#[derive(Clone, Deserialize, Debug, Serialize)]
pub struct DeckListItem {
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

#[derive(Deserialize, Debug)]
pub struct Deck {
  #[serde(rename = "srcCardTemplates")]
  pub cards: Vec<usize>,
}

impl DeckList {
  pub fn merge(mut self, other: &DeckList) -> DeckList {
    self.guides.extend(other.guides.clone());
    self
  }
}

impl Default for DeckList {
  fn default() -> Self {
    DeckList { guides: Vec::new() }
  }
}
