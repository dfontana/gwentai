mod deck;
mod disk;

use anyhow::Error;
use deck::{Deck, DeckListResponse, DeckResponse};
use disk::Disk;
use futures::{future, stream, StreamExt};
use indicatif::{ProgressBar, ProgressStyle};
use reqwest::Client;

const BASE_API: &'static str = "https://www.playgwent.com/en/decks/api/guides";
const CONCURRENT_REQUESTS: usize = 30;
const PATH: &'static str = "decks.json";

async fn get_deck_page(
  http: &Client,
  pb: &ProgressBar,
  page: usize,
) -> Result<(usize, DeckListResponse), Error> {
  let url = format!("{}/offset/{}/limit/500", BASE_API, page * 500);
  let res = http.get(url).send().await?;
  let json = res.json::<DeckListResponse>().await?;
  pb.inc(1);
  Ok((page, json))
}

async fn get_deck(
  http: &Client,
  pb: &ProgressBar,
  id: usize,
) -> Result<(usize, DeckResponse), Error> {
  let url = format!("{}/{}", BASE_API, id);
  let res = http.get(url).send().await?.json::<DeckResponse>().await?;
  pb.inc(1);
  Ok((id, res))
}

#[tokio::main]
async fn download(client: &Client) -> Disk {
  let sty = ProgressStyle::default_bar()
    .template("{prefix:>8} {spinner} {bar:40.cyan/blue} {pos:>5}/{len:5} ({percent}%) {msg}")
    .progress_chars("#>-");

  // TODO right now the upper bound on pages is 60, but you'll want a way to incrementally
  //      increase this each time this is ran.
  let range = 0..60;

  let pb = ProgressBar::new(range.len() as u64);
  pb.set_style(sty.clone());
  pb.set_prefix("Metadata");

  let disk: Disk = stream::iter(range)
    .map(|page| get_deck_page(&client, &pb, page))
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

  pb.finish_with_message("Done!");

  let pb2 = ProgressBar::new(disk.deckmeta.len() as u64);
  pb2.set_style(sty.clone());
  pb2.set_prefix("Decks");

  let decks = stream::iter(disk.deckmeta.keys().into_iter())
    .map(|id| get_deck(&client, &pb2, id.to_owned()))
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

  pb2.finish_with_message("Done!");

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
