use anyhow::Error;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs::File};

use super::model::CardData;

#[derive(Deserialize, Serialize)]
pub struct Disk {
  pub cards: HashMap<usize, CardData>,
}

impl Disk {
  pub fn load(path: &str) -> Result<Disk, Error> {
    let file = File::open(path)?;
    let disk: Disk = serde_json::from_reader(file)?;
    Ok(disk)
  }

  pub fn save(self, path: &str) -> Result<Self, Error> {
    let file = File::create(path)?;
    serde_json::to_writer(file, &self)?;
    Ok(self)
  }

  pub fn merge(mut self, cards: HashMap<usize, CardData>) -> Disk {
    self.cards.extend(cards);
    self
  }
}

impl Default for Disk {
  fn default() -> Self {
    Disk {
      cards: HashMap::new(),
    }
  }
}
