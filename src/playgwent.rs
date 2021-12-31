mod deck;

use anyhow::Error;
use deck::Deck;
pub use deck::*;
use reqwest::Client;

const BASE_SITE: &'static str = "https://www.playgwent.com";
const BASE_API: &'static str = "https://www.playgwent.com/en/decks/api";

/// Recreate something like this: https://www.gwentdata.com/deck
///   But would be nice to have a feature showing all cards & their classifications
///   

/// Rof16 Notes: (https://www.youtube.com/watch?v=2kCDey-pi34)
///   Classification
///     TODO I don't think this works right; "summon" sometimes means play a card, and 
//        "play" (which isn't always a keyword) can be used. And what about "spawn"?
///     - If it's tooltip has a keyword key == "summon", it's a summon
///     - If it's tooltip has a text value containing "play", it's a tutor
///     - If provsioncost > 4 && !tutor && !summon, then target
///     - else 4p

/// Validation Thoughts:
///   Display a table of card name, each condition state, and final classification
///
///   Since there's no API (known) to pull all cards from, you might scan the first N pages
///   of guides & dedupe the cards from there.

/// Discovered routes thus far:
///   Base: https://www.playgwent.com/en/decks/api
///   Deck Listings:
///     /guides
///       - Paginate: /offset/{page_int}/limit/500 -> DeckListResponse
///   Deck Details:
///     /guides/{id_int} (by internal ID number from paging) -> DeckGuideResponse
///     /decks/{id_hash} (by hash found in paging or URL) -> DeckResponse
///
/// Additionally, construction notes:
///   Imagery: (Relative to BASE_SITE)
///     Strategem: /build/img/netdecking/cardList/stratagem-gold-icon-5ecd3aea.png
///     Power:
///       - /build/img/netdecking/cardList/golden-star-6b603093.png
///       - bronze: /build/img/netdecking/cardList/bronze-star-62d174f8.png
///     RepeatCountBg: /build/img/netdecking/cardList/cards-copy-9775aeec.png
///     Cards:
///       Previews:
///         - Slot image for background
///         - Left Aligned: Strategem Logo|Special Logo|Power
///         - Left Aligned: Provisions Cost
///         - Left Aligned: Name
///         - Right Aligned: Optional<RepeatCount>
pub struct GwentClient {
  http: Client,
}

impl GwentClient {
  pub fn new(http: Client) -> GwentClient {
    GwentClient { http }
  }

  async fn get_deck(&self, hash: &str) -> Result<Deck, Error> {
    Ok(
      self
        .http
        .get(format!("{}/decks/{}", BASE_API, hash))
        .send()
        .await?
        .json::<Deck>()
        .await?,
    )
  }
}
