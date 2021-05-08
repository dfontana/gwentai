use crate::deck::{Deck, DeckMetadata};
use anyhow::Error;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs::File};

#[derive(Deserialize, Serialize)]
pub struct Disk {
  pub page_start: usize,
  pub page_end: usize,
  pub deckmeta: HashMap<usize, DeckMetadata>,
  pub decks: HashMap<usize, Deck>,
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

  pub fn merge_meta(mut self, page: usize, dl: Vec<DeckMetadata>) -> Disk {
    self.page_end = page.max(self.page_end);
    for dm in dl {
      if !dm.invalid {
        self.deckmeta.insert(dm.id, dm);
      }
    }
    self
  }

  pub fn merge_decks(mut self, decks: Vec<(usize, Deck)>) -> Disk {
    self.decks.extend(decks);
    self
  }
}

impl Default for Disk {
  fn default() -> Self {
    Disk {
      page_start: 0,
      page_end: 0,
      deckmeta: HashMap::new(),
      decks: HashMap::new(),
    }
  }
}
