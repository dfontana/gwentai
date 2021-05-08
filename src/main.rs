mod deck;
mod disk;

use anyhow::Error;
use deck::{Deck, DeckList};
use disk::Disk;
use futures::{future, stream, StreamExt};
use reqwest::Client;

const BASE_API: &'static str = "https://www.playgwent.com/en/decks/api/guides";
const CONCURRENT_REQUESTS: usize = 10;
const PATH: &'static str = "decks.json";

async fn get_deck_page(http: &Client, page: usize) -> Result<(usize, DeckList), Error> {
  let url = format!("{}/offset/{}/limit/500", BASE_API, page * 500);
  println!("{}", &url);
  Ok((page, http.get(url).send().await?.json::<DeckList>().await?))
}

async fn get_deck(http: &Client, id: usize) -> Result<Deck, Error> {
  let uri = format!("{}/{}", BASE_API, id);
  Ok(http.get(uri).send().await?.json::<Deck>().await?)
}

#[tokio::main]
async fn download(client: &Client) -> Disk {
  // TODO right now the upper bound on pages is 60, but you'll want a way to incrementally
  //      increase this each time this is ran.
  stream::iter(0..60)
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
    .fold(Disk::default(), |a, b| future::ready(a.merge(b)))
    .await
}

fn main() -> Result<(), Error> {
  let client = Client::new();

  let disk = Disk::load(&PATH)
    .map_err(|err| eprintln!("Failed to load Decks: {}", err))
    .or_else(|_| download(&client).save(&PATH))?;

  println!("Items: {}", disk.decks.guides.len());

  Ok(())
}
