//! Get card types (e.g.: Artifact, Land, Sorcery).
#![allow(dead_code)]
use crate::query_builder;
use reqwest::StatusCode;
use serde::Deserialize;
use std::collections::HashSet;

#[doc(hidden)]
#[derive(Clone, Debug, Deserialize)]
pub struct RootAll {
    pub types: HashSet<String>,
}

/// Function to get all types.
///
/// # Example
/// ```rust
/// use mtgsdk::types;
/// async {
///    let types = types::all().await;
///    assert!(types.unwrap().contains("Planeswalker"));
/// };
///```
///
/// # Errors
/// If the call fails, it will return a `Err(StatusCode)`.
/// To see the possible return values, check the [API docs](https://docs.magicthegathering.io/#documentationerrors).
pub async fn all() -> Result<HashSet<String>, StatusCode> {
    let types: Result<RootAll, StatusCode> = query_builder::all("types").await;

    match types {
        Ok(t) => Ok(t.types),
        Err(e) => Err(e),
    }
}
