//! Get game cards (e.g.: "Kokusho, the Evening Star", "Island", "Black Lotus").
//!
//! Alongside `sets`, `cards' is one of the calls that allow the `find()` method as well as specific filters.
//! For a complete list of the paremeters available for the filters, check de [API docs](https://docs.magicthegathering.io/#api_v1cards_list).
#![allow(dead_code)]
use crate::query_builder;
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

/// Structure to deserialize rulings inside the cards' JSON.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct Rulings {
    pub date: String,
    pub text: String,
}
/// Structure to deserialize legalities inside the cards' JSON.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ForeignNames {
    pub name: String,
    #[serde(default)]
    pub text: String,
    #[serde(rename = "type")]
    pub type_field: Option<String>,
    pub flavor: Option<String>,
    pub image_url: Option<String>,
    pub language: String,
    pub multiverseid: Option<i64>,
}

/// Structure to deserialize legalities inside the cards' JSON.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct Legalities {
    pub format: String,
    pub legality: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]

/// Structure to deserialize cards' JSON.
///
/// Values inside `Option` are optional, and you should check if there is `Some` or `None` before using it.
pub struct Card {
    pub name: String,
    pub layout: String,
    pub cmc: f64,
    pub colors: Option<HashSet<String>>,
    pub color_identity: Option<HashSet<String>>,
    #[serde(rename = "type")]
    pub type_field: String,
    pub supertypes: Option<HashSet<String>>,
    pub types: HashSet<String>,
    pub subtypes: Option<HashSet<String>>,
    pub rarity: String,
    #[serde(rename = "set")]
    pub set_field: String,
    pub set_name: String,
    #[serde(default)]
    pub text: String,
    // pub flavor: Option<String>,
    pub artist: String,
    pub number: String,
    pub power: Option<String>,
    pub toughness: Option<String>,
    pub loyalty: Option<String>,
    // pub language: String,
    pub id: String,
    pub multiverseid: Option<String>,
    pub names: Option<HashSet<String>>,
    pub mana_cost: Option<String>,
    pub variations: Option<Vec<String>>,
    pub image_url: Option<String>,
    pub watermark: Option<String>,
    pub border: Option<String>,
    pub release_date: Option<String>,
    pub rulings: Option<Vec<Rulings>>,
    #[serde(default)]
    pub foreign_names: Vec<ForeignNames>,
    pub printings: HashSet<String>,
    pub original_text: Option<String>,
    pub original_type: Option<String>,
    #[serde(default)]
    pub legalities: Vec<Legalities>,
}

#[doc(hidden)]
#[derive(Clone, Debug, Deserialize)]
pub struct RootAll {
    cards: Vec<Card>,
}

#[doc(hidden)]
#[derive(Clone, Debug, Deserialize)]
pub struct RootFind {
    card: Card,
}

/// Function to get all cards.
///
/// The call will return a maximum of 100 cards. To get more, it is necessary to use the `page` filter.
///
/// To use filters (either pagination or other queries, see `filter()`).
///
/// For more information on the available filterms, check the [API docs](https://docs.magicthegathering.io/#api_v1sets_list).
///
/// # Example
/// ```rust
/// use mtgsdk::cards;
/// async {
///    let cards = cards::all().await;
///    assert_eq!(cards.unwrap().get(0).unwrap().name.chars().collect::<Vec<char>>()[0], 'A');
/// };
///```
///
/// # Errors
/// If the call fails, it will return a `Err(StatusCode)`.
/// To see the possible return values, check the [API docs](https://docs.magicthegathering.io/#documentationerrors).
pub async fn all() -> Result<Vec<Card>, StatusCode> {
    let cards: Result<RootAll, StatusCode> = query_builder::all("cards").await;

    match cards {
        Ok(t) => Ok(t.cards),
        Err(e) => Err(e),
    }
}

/// Function to get a single card.
///
/// # Example
/// ```rust
/// use mtgsdk::cards;
/// async {
///    let cards = cards::find(386616).await;
///    assert_eq!(cards.unwrap().name, "Narset, Enlightened Master");
/// };
///```
///
/// # Errors
/// If the call fails, it will return a `Err(StatusCode)`.
/// To see the possible return values, check the [API docs](https://docs.magicthegathering.io/#documentationerrors).
pub async fn find(id: u64) -> Result<Card, StatusCode> {
    let text_id = id.to_string();
    let cards: Result<RootFind, StatusCode> = query_builder::find("cards", &text_id).await;

    match cards {
        Ok(t) => Ok(t.card),
        Err(e) => Err(e),
    }
}

#[doc(hidden)]
pub struct Where<'a> {
    query: Vec<(&'a str, String)>,
}

/// Function to get all card matching the query filters.
///
/// To use it, call `filter()` followed by the desired filters and then close with `all()`.
///
/// # Example
/// This call will get 25 cards from page 50.
/// ```rust
/// use mtgsdk::cards;
/// async {
///     let cards = cards::filter()
///         .page(50)
///         .page_size(25)
///         .all()
///         .await;
///     assert_eq!(cards.unwrap().len(), 25);
/// };
///```
///
/// To query, you may stack multiple filters.
/// ```rust
/// use mtgsdk::cards;
/// async {
///     let cards = cards::filter()
///         .supertypes("legendary")
///         .types("creature")
///         .colors("red,white")
///         .all()
///         .await;
///     assert!(cards.unwrap().iter().any(|card| card.name == "Breya, Etherium Shaper"));
/// };
///```
///
/// # Errors
/// If the call fails, it will return a `Err(StatusCode)`.
/// To see the possible return values, check the [API docs](https://docs.magicthegathering.io/#documentationerrors).
pub fn filter<'a>() -> Where<'a> {
    Where { query: Vec::new() }
}

impl<'a> Where<'a> {
    pub fn name(mut self, input: &'a str) -> Self {
        self.query.push(("name", String::from(input)));
        self
    }
    pub fn layout(mut self, input: &'a str) -> Self {
        self.query.push(("layout", String::from(input)));
        self
    }
    pub fn cmc(mut self, input: u64) -> Self {
        self.query.push(("cmc", input.to_string()));
        self
    }
    pub fn colors(mut self, input: &'a str) -> Self {
        self.query.push(("colors", String::from(input)));
        self
    }
    pub fn color_identity(mut self, input: &'a str) -> Self {
        self.query.push(("colorIdentity", String::from(input)));
        self
    }
    pub fn type_field(mut self, input: &'a str) -> Self {
        self.query.push(("type", String::from(input)));
        self
    }
    pub fn supertypes(mut self, input: &'a str) -> Self {
        self.query.push(("supertypes", String::from(input)));
        self
    }
    pub fn types(mut self, input: &'a str) -> Self {
        self.query.push(("types", String::from(input)));
        self
    }
    pub fn subtypes(mut self, input: &'a str) -> Self {
        self.query.push(("subtypes", String::from(input)));
        self
    }
    pub fn rarity(mut self, input: &'a str) -> Self {
        self.query.push(("rarity", String::from(input)));
        self
    }
    pub fn set_field(mut self, input: &'a str) -> Self {
        self.query.push(("set", String::from(input)));
        self
    }
    pub fn set_name(mut self, input: &'a str) -> Self {
        self.query.push(("setName", String::from(input)));
        self
    }
    pub fn text(mut self, input: &'a str) -> Self {
        self.query.push(("text", String::from(input)));
        self
    }
    pub fn flavor(mut self, input: &'a str) -> Self {
        self.query.push(("flavor", String::from(input)));
        self
    }
    pub fn artist(mut self, input: &'a str) -> Self {
        self.query.push(("artist", String::from(input)));
        self
    }
    pub fn number(mut self, input: &'a str) -> Self {
        self.query.push(("number", String::from(input)));
        self
    }
    pub fn power(mut self, input: &'a str) -> Self {
        self.query.push(("power", String::from(input)));
        self
    }
    pub fn toughness(mut self, input: &'a str) -> Self {
        self.query.push(("toughness", String::from(input)));
        self
    }
    pub fn loyalty(mut self, input: &'a str) -> Self {
        self.query.push(("loyalty", String::from(input)));
        self
    }
    pub fn language(mut self, input: &'a str) -> Self {
        self.query.push(("language", String::from(input)));
        self
    }
    pub fn game_format(mut self, input: &'a str) -> Self {
        self.query.push(("gameFormat", String::from(input)));
        self
    }
    pub fn legality(mut self, input: &'a str) -> Self {
        self.query.push(("legality", String::from(input)));
        self
    }
    pub fn page(mut self, input: u64) -> Self {
        self.query.push(("page", input.to_string()));
        self
    }
    pub fn page_size(mut self, input: u64) -> Self {
        self.query.push(("pageSize", input.to_string()));
        self
    }
    pub fn order_by(mut self, input: &'a str) -> Self {
        self.query.push(("orderBy", String::from(input)));
        self
    }
    pub fn random(mut self, input: &'a str) -> Self {
        self.query.push(("random", String::from(input)));
        self
    }
    pub fn contains(mut self, input: &'a str) -> Self {
        self.query.push(("contains", String::from(input)));
        self
    }
    pub fn id(mut self, input: &'a str) -> Self {
        self.query.push(("id", String::from(input)));
        self
    }
    pub fn multiverseid(mut self, input: u64) -> Self {
        self.query.push(("multiverseid", input.to_string()));
        self
    }

    pub async fn all(mut self) -> Result<Vec<Card>, StatusCode> {
        let val = self.query.remove(0);
        let mut filter = format!("?{}={}", val.0, val.1);

        for (k, v) in self.query.into_iter() {
            filter = format!("{}&{}={}", filter, k, v);
        }

        let cards: Result<RootAll, StatusCode> = query_builder::filter("cards", &filter).await;

        match cards {
            Ok(t) => Ok(t.cards),
            Err(e) => Err(e),
        }
    }
}
