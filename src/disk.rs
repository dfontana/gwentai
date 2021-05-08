use crate::deck::DeckList;
use anyhow::Error;
use serde::{Deserialize, Serialize};
use std::fs::File;

#[derive(Deserialize, Serialize)]
pub struct Disk {
  pub page_start: usize,
  pub page_end: usize,
  pub decks: DeckList,
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

  pub fn merge(mut self, (page, other): (usize, DeckList)) -> Disk {
    self.page_end = page.max(self.page_end);
    self.decks = self.decks.merge(&other);
    self
  }
}

impl Default for Disk {
  fn default() -> Self {
    Disk {
      page_start: 0,
      page_end: 0,
      decks: DeckList::default(),
    }
  }
}
