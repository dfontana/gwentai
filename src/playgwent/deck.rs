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
pub struct DeckGuideResponse {
  pub deck: Deck,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct Deck {
  /// Provision Cost of this deck
  #[serde(rename = "provisionsCost")]
  pub provisions_cost: usize,
  /// Scrap cost of this deck
  #[serde(rename = "craftingCost")]
  pub crafting_cost: usize,
  /// Leader chosen
  pub leader: Card,
  /// Strategm
  pub strategem: Card,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct Card {
  pub provisionsCost: usize,
  pub craftingCost: usize,
  pub repeatCount: usize,
  pub name: String,
  pub power: usize,
  pub slotImg: Image,
  /// One of "bronze", "gold".
  pub cardGroup: String,
  /// Each index is one line. Empty vecs are blank lines.
  pub tooltip: Vec<Vec<Text>>
}

#[derive(Deserialize, Debug, Serialize)]
pub struct Image {
  pub small: String,
  pub big: String
}

#[derive(Deserialize, Debug, Serialize)]
pub struct Text {
  /// "keyword" or "text"
  #[serde(rename = "type")]
  pub text_type: String,
  pub value: String,
  /// only when text_type == keyword.
  /// Example: "summon", "melee", etc
  pub key: String,
}
