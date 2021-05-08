mod deck;
mod disk;

use anyhow::Error;
use deck::{Deck, DeckListResponse, DeckResponse};
use disk::Disk;
use futures::{future, stream, StreamExt};
use reqwest::Client;

const BASE_API: &'static str = "https://www.playgwent.com/en/decks/api/guides";
const CONCURRENT_REQUESTS: usize = 30;
const PATH: &'static str = "decks.json";

async fn get_deck_page(http: &Client, page: usize) -> Result<(usize, DeckListResponse), Error> {
  let url = format!("{}/offset/{}/limit/500", BASE_API, page * 500);
  println!("{}", &url);
  Ok((
    page,
    http
      .get(url)
      .send()
      .await?
      .json::<DeckListResponse>()
      .await?,
  ))
}

async fn get_deck(http: &Client, id: usize) -> Result<(usize, DeckResponse), Error> {
  let url = format!("{}/{}", BASE_API, id);
  println!("{}", &url);
  Ok((
    id,
    http.get(url).send().await?.json::<DeckResponse>().await?,
  ))
}

#[tokio::main]
async fn download(client: &Client) -> Disk {
  // TODO right now the upper bound on pages is 60, but you'll want a way to incrementally
  //      increase this each time this is ran.
  let range = 0..60;

  println!("Items to download: {}", range.len());

  let disk: Disk = stream::iter(range)
    .map(|page| get_deck_page(&client, page))
    .buffer_unordered(CONCURRENT_REQUESTS)
    .filter_map(|res| {
      match res {
        Ok((page, decklist)) => {
          if !decklist.guides.is_empty() {
            return future::ready(Some((page, decklist)));
          }
        }
        Err(e) => eprintln!("{:?}", e),
      }
      future::ready(None)
    })
    .fold(Disk::default(), |a, (page, res)| {
      future::ready(a.merge_meta(page, res.guides))
    })
    .await;

  println!("Items to download: {}", disk.deckmeta.len());

  let decks = stream::iter(disk.deckmeta.keys().into_iter())
    .map(|id| get_deck(&client, id.to_owned()))
    .buffer_unordered(CONCURRENT_REQUESTS)
    .filter_map(|res| match res {
      Ok((pg, d)) => future::ready(Some((pg, d.deck))),
      Err(e) => {
        eprintln!("{:?}", e);
        future::ready(None)
      }
    })
    .collect::<Vec<(usize, Deck)>>()
    .await;

  disk.merge_decks(decks)
}

fn main() -> Result<(), Error> {
  let client = Client::new();

  // TODO incrementally fetch new items starting at the last page
  let disk = Disk::load(&PATH)
    .map_err(|err| eprintln!("Failed to load Decks: {}", err))
    .or_else(|_| download(&client).save(&PATH))?;

  println!("Items: {}", disk.deckmeta.len());

  Ok(())
}
