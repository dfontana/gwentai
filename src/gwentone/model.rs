use std::{collections::HashSet, fmt::Display, str::FromStr};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum CardType {
  UNIT,
  ABILITY,
  SPELL,
  STRATEGEM,
  ARTIFACT,
  SPECIAL,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum CardRarity {
  LEGENDARY,
  EPIC,
  RARE,
  COMMON,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum CardGroup {
  GOLD,
  BRONZE,
  NONE,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum CardFaction {
  NEUTRAL,
  SKELLIGE,
  SYNDICATE,
  SCOIATAEL,
  NILFGAARD,
  NORTHERNREALMS,
  MONSTER,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CardData {
  pub id: usize,
  pub name: String,
  // TODO would be nice to have categories & keywords as enums
  pub categories: HashSet<String>,
  pub keywords: HashSet<String>,
  pub power: usize,
  pub provisions: usize,
  pub armor: usize,
  pub faction: CardFaction,
  pub card_type: CardType,
  pub rarity: CardRarity,
  pub group: CardGroup,
}

impl Display for CardData {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(
      f,
      "[{}]({:?}) {}\n\t{:?}\n\t{:?}\n\t{}-{}-{}\n\t{:?} {:?} {:?}",
      self.id,
      self.faction,
      self.name,
      self.categories,
      self.keywords,
      self.power,
      self.armor,
      self.provisions,
      self.card_type,
      self.rarity,
      self.group,
    )
  }
}

impl FromStr for CardType {
  type Err = String;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s.trim().to_lowercase().as_ref() {
      "unit" => Ok(CardType::UNIT),
      "ability" => Ok(CardType::ABILITY),
      "spell" => Ok(CardType::SPELL),
      "stratagem" => Ok(CardType::STRATEGEM),
      "artifact" => Ok(CardType::ARTIFACT),
      "special" => Ok(CardType::SPECIAL),
      _ => Err(format!("Unknown CardType: {:?}", s)),
    }
  }
}

impl FromStr for CardFaction {
  type Err = String;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s.trim().to_lowercase().as_ref() {
      "monster" => Ok(CardFaction::MONSTER),
      "neutral" => Ok(CardFaction::NEUTRAL),
      "skellige" => Ok(CardFaction::SKELLIGE),
      "syndicate" => Ok(CardFaction::SYNDICATE),
      "scoiatael" => Ok(CardFaction::SCOIATAEL),
      "nilfgaard" => Ok(CardFaction::NILFGAARD),
      "northern_realms" => Ok(CardFaction::NORTHERNREALMS),
      _ => Err(format!("Unknown CardFaction: {:?}", s)),
    }
  }
}

impl FromStr for CardRarity {
  type Err = String;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s.trim().to_lowercase().as_ref() {
      "legendary" => Ok(CardRarity::LEGENDARY),
      "epic" => Ok(CardRarity::EPIC),
      "rare" => Ok(CardRarity::RARE),
      "common" => Ok(CardRarity::COMMON),
      _ => Err(format!("Unknown CardRarity: {:?}", s)),
    }
  }
}

impl FromStr for CardGroup {
  type Err = String;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s.trim().to_lowercase().as_ref() {
      "gold" => Ok(CardGroup::GOLD),
      "bronze" => Ok(CardGroup::BRONZE),
      "leader" => Ok(CardGroup::NONE),
      _ => Err(format!("Unknown CardGroup: {:?}", s)),
    }
  }
}
