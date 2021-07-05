//! Get card super types (e.g.: Basic, Legendary, Snow).
#![allow(dead_code)]
use crate::query_builder;
use reqwest::StatusCode;
use serde::Deserialize;
use std::collections::HashSet;

#[doc(hidden)]
#[derive(Clone, Debug, Deserialize)]
pub struct RootAll {
    pub supertypes: HashSet<String>,
}

/// Function to get all super types.
///
/// # Example
/// ```rust
/// use mtgsdk::supertypes;
/// async {
///    let supertypes = supertypes::all().await;
///    assert!(supertypes.unwrap().contains("Basic"));
/// };
///```
///
/// # Errors
/// If the call fails, it will return a `Err(StatusCode)`.
/// To see the possible return values, check the [API docs](https://docs.magicthegathering.io/#documentationerrors).
pub async fn all() -> Result<HashSet<String>, StatusCode> {
    let supertypes: Result<RootAll, StatusCode> = query_builder::all("supertypes").await;

    match supertypes {
        Ok(t) => Ok(t.supertypes),
        Err(e) => Err(e),
    }
}
