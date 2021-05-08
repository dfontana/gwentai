mod deck;
mod disk;
mod progress;

use anyhow::Error;
use deck::{Deck, DeckListResponse, DeckResponse};
use disk::Disk;
use futures::{future, stream, StreamExt};
use progress::Progress;
use reqwest::Client;

const BASE_API: &'static str = "https://www.playgwent.com/en/decks/api/guides";
const CONCURRENT_REQUESTS: usize = 30;
const PATH: &'static str = "decks.json";

async fn get_deck_page(
  http: &Client,
  progress: &Progress,
  page: usize,
) -> Result<(usize, DeckListResponse), Error> {
  let url = format!("{}/offset/{}/limit/500", BASE_API, page * 500);
  let res = http.get(url).send().await?;
  let json = res.json::<DeckListResponse>().await?;
  progress.increment(1);
  Ok((page, json))
}

async fn get_deck(
  http: &Client,
  progress: &Progress,
  id: usize,
) -> Result<(usize, DeckResponse), Error> {
  let url = format!("{}/{}", BASE_API, id);
  let res = http.get(url).send().await?.json::<DeckResponse>().await?;
  progress.increment(1);
  Ok((id, res))
}

#[tokio::main]
async fn download(client: &Client) -> Disk {
  // TODO right now the upper bound on pages is 60, but you'll want a way to incrementally
  //      increase this each time this is ran.
  let range = 0..60;
  let meta_progress = Progress::new("DL Metadata", range.len());
  let disk: Disk = stream::iter(range)
    .map(|page| get_deck_page(&client, &meta_progress, page))
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

  let deck_progress = Progress::new("DL Decks", disk.deckmeta.len());
  let decks = stream::iter(disk.deckmeta.keys().into_iter())
    .map(|id| get_deck(&client, &deck_progress, id.to_owned()))
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
