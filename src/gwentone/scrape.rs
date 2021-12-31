use std::collections::{HashMap, HashSet};

use anyhow::Error;
use once_cell::sync::Lazy;
use reqwest::{Client, Response};
use scraper::{ElementRef, Html, Selector};
use tracing::info;

use super::{
  model::{CardData, CardGroup, CardRarity, CardType},
  CardFaction,
};

const CARD_API: &'static str = "https://gwent.one/search/ajax";
const VERSION: &'static str = "9.6.1";

static PAGE_SELECTOR: Lazy<Selector> = Lazy::new(|| {
  Selector::parse("#pages-top a.page-link[href=\"#pages-top\"]").expect("Failed to parse selector")
});
static CARD_WRAP_SELECTOR: Lazy<Selector> =
  Lazy::new(|| Selector::parse(".card-wrap.card-data").expect("Failed to parse selector"));
static CARD_NAME_SELECTOR: Lazy<Selector> =
  Lazy::new(|| Selector::parse(".card-name > a").expect("Failed to parse selector"));
static CARD_CATEGORY_SELECTOR: Lazy<Selector> =
  Lazy::new(|| Selector::parse(".card-category").expect("Failed to parse selector"));
static CARD_KEYWORD_SELECTOR: Lazy<Selector> =
  Lazy::new(|| Selector::parse(".keyword").expect("Failed to parse selector"));

pub async fn cards(client: Client) -> Result<HashMap<usize, CardData>, Error> {
  let pages = get_pages(&client).await?;
  info!("Pages to scrape: {:?}", pages);
  let mut cards: HashMap<usize, CardData> = HashMap::new();
  for page in pages {
    let html = get_page(&client, &page).await?.text().await?;
    let doc = Html::parse_document(&html);
    for element in doc.select(&CARD_WRAP_SELECTOR) {
      let card = CardData::from(element);
      cards.insert(card.id, card);
    }
  }
  info!("Scraping complete");
  Ok(cards)
}

async fn get_pages(client: &Client) -> Result<Vec<String>, Error> {
  let html = get_page(client, "1").await?.text().await?;
  Ok(
    Html::parse_document(&html)
      .select(&PAGE_SELECTOR)
      .map(|e| e.inner_html())
      .collect(),
  )
}

async fn get_page(client: &Client, page: &str) -> Result<Response, Error> {
  Ok(
    client
      .post(CARD_API)
      .form(&[
        ("v", VERSION),
        ("total", "140"),
        ("lang", "en"),
        ("page", page),
      ])
      .send()
      .await?,
  )
}

impl<'a> From<ElementRef<'a>> for CardData {
  fn from(card: ElementRef) -> Self {
    let name = card
      .select(&CARD_NAME_SELECTOR)
      .nth(0)
      .unwrap()
      .inner_html();
    let raw_category = card
      .select(&CARD_CATEGORY_SELECTOR)
      .nth(0)
      .unwrap()
      .inner_html();
    let categories = raw_category
      .split(", ")
      .filter_map(|s| {
        if s.starts_with('&') {
          None
        } else {
          Some(s.trim().to_lowercase().to_owned())
        }
      })
      .collect::<HashSet<String>>();
    let keywords = card
      .select(&CARD_KEYWORD_SELECTOR)
      .map(|e| {
        e.value()
          .classes()
          .filter(|c| *c != "keyword")
          .collect::<String>()
      })
      .collect::<HashSet<String>>();
    CardData {
      name,
      categories,
      keywords,
      id: parse_usize(card, "id"),
      power: parse_usize(card, "power"),
      armor: parse_usize(card, "armor"),
      provisions: parse_usize(card, "provision"),
      faction: card
        .value()
        .attr("data-faction")
        .map(|v| v.parse::<CardFaction>().unwrap())
        .unwrap(),
      card_type: card
        .value()
        .attr("data-type")
        .map(|v| v.parse::<CardType>().unwrap())
        .unwrap(),
      rarity: card
        .value()
        .attr("data-rarity")
        .map(|v| v.parse::<CardRarity>().unwrap())
        .unwrap(),
      group: card
        .value()
        .attr("data-color")
        .map(|v| v.parse::<CardGroup>().unwrap())
        .unwrap(),
    }
  }
}

fn parse_usize(ele: ElementRef, field: &str) -> usize {
  ele
    .value()
    .attr(&format!("data-{}", field))
    .map(|v| {
      v.parse::<usize>()
        .expect(&format!("{} invalid type", field))
    })
    .expect(&format!("{} is missing", field))
}
