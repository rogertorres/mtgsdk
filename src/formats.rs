//! Get game formats (e.g.: Standard, Modern, Onslaught Block).
#![allow(dead_code)]
use crate::query_builder;
use reqwest::StatusCode;
use serde::Deserialize;
use std::collections::HashSet;

#[doc(hidden)]
#[derive(Clone, Debug, Deserialize)]
pub struct RootAll {
    pub formats: HashSet<String>,
}

/// Function to get all formats.
///
/// # Example
/// ```rust
/// use mtgsdk::formats;
/// async {
///    let formats = formats::all().await;
///    assert!(formats.unwrap().contains("Modern"));
/// };
///```
///
/// # Errors
/// If the call fails, it will return a `Err(StatusCode)`.
/// To see the possible return values, check the [API docs](https://docs.magicthegathering.io/#documentationerrors).
pub async fn all() -> Result<HashSet<String>, StatusCode> {
    let formats: Result<RootAll, StatusCode> = query_builder::all("formats").await;

    match formats {
        Ok(t) => Ok(t.formats),
        Err(e) => Err(e),
    }
}
