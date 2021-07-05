//! Get game sets (e.g.: Urza's Saga, Darksteel, Dragons of Tarkir).
//!
//! Alongside `cards`, `sets` is one of the calls that allow the `find()` method as well as specific filters.
//! For a complete list of the paremeters available for the filters, check [API docs](https://docs.magicthegathering.io/#api_v1sets_list).
#![allow(dead_code)]
use crate::query_builder;
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};

/// Structure to deserialize sets' JSON.
///
/// Values inside `Option` are optional, and you should check if there is `Some` or `None` before using it.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Set {
    pub code: String,
    pub name: String,
    #[serde(rename = "type")]
    pub type_field: String,
    #[serde(default)]
    pub booster: Vec<Booster>,
    pub release_date: String,
    pub block: Option<String>,
    pub online_only: Option<bool>,
    pub gatherer_code: Option<String>,
    pub old_code: Option<String>,
    pub magic_cards_info_code: Option<String>,
    pub border: Option<String>,
    pub expansion: Option<String>,
    pub mkm_name: Option<String>,
    pub mkm_id: Option<u32>,
}

#[doc(hidden)]
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(untagged)]
pub enum Booster {
    Multiple(Vec<String>),
    Single(String),
}

#[doc(hidden)]
#[derive(Clone, Debug, Deserialize)]
pub struct RootAll {
    sets: Vec<Set>,
}

#[doc(hidden)]
#[derive(Clone, Debug, Deserialize)]
pub struct RootFind {
    set: Set,
}

/// Function to get all sets.
///
/// To use filters (either pagination or other queries, see `filter()`).
///
/// # Example
/// ```rust
/// use mtgsdk::sets;
/// async {
///    let sets = sets::all().await;
///    assert_eq!(sets.unwrap().get(0).unwrap().type_field, "core");
/// };
///```
///
/// # Errors
/// If the call fails, it will return a `Err(StatusCode)`.
/// To see the possible return values, check the [API docs](https://docs.magicthegathering.io/#documentationerrors).
pub async fn all() -> Result<Vec<Set>, StatusCode> {
    let sets: Result<RootAll, StatusCode> = query_builder::all("sets").await;

    match sets {
        Ok(t) => Ok(t.sets),
        Err(e) => Err(e),
    }
}

/// Function to get a single set.
///
/// # Example
/// ```rust
/// use mtgsdk::sets;
/// async {
///    let sets = sets::find("dom").await;
///    assert_eq!(sets.unwrap().name, "Dominaria");
/// };
///```
///
/// # Errors
/// If the call fails, it will return a `Err(StatusCode)`.
/// To see the possible return values, check the [API docs](https://docs.magicthegathering.io/#documentationerrors).
pub async fn find(id: &str) -> Result<Set, StatusCode> {
    let sets: Result<RootFind, StatusCode> = query_builder::find("sets", id).await;

    match sets {
        Ok(t) => Ok(t.set),
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
/// For more information on the available filterms, check the [API docs](https://docs.magicthegathering.io/#api_v1sets_list).
///
/// # Example
/// This call will get 13 sets from page 2.
/// ```rust
/// use mtgsdk::sets;
/// async {
///    let sets = sets::filter()
///       .page(2)
///       .page_size(13)
///       .all()
///       .await;
///    assert_eq!(sets.unwrap().len(), 13);
/// };
///```
///
/// To query, you may stack filters.
/// ```rust
/// use mtgsdk::sets;
/// async {
///    let sets = sets::filter()
///       .block("kamigawa")
///       .name("bet")
///       .all()
///       .await;
///    assert!(sets.unwrap().iter().any(|set| set.name == "Betrayers of Kamigawa"));
/// };
///```
///
/// # Errors
/// If the call fails, it will return a `Err(StatusCode)`.
/// To see the possible return values, check https://docs.magicthegathering.io/#documentationerrors.
pub fn filter<'a>() -> Where<'a> {
    Where { query: Vec::new() }
}

impl<'a> Where<'a> {
    pub fn name(mut self, input: &'a str) -> Self {
        self.query.push(("name", String::from(input)));
        self
    }

    pub fn block(mut self, input: &'a str) -> Self {
        self.query.push(("block", String::from(input)));
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

    pub async fn all(mut self) -> Result<Vec<Set>, StatusCode> {
        let val = self.query.remove(0);
        let mut filter = format!("?{}={}", val.0, val.1);

        for (k, v) in self.query.into_iter() {
            filter = format!("{}&{}={}", filter, k, v);
        }

        let sets: Result<RootAll, StatusCode> = query_builder::filter("sets", &filter).await;

        match sets {
            Ok(t) => Ok(t.sets),
            Err(e) => Err(e),
        }
    }
}
