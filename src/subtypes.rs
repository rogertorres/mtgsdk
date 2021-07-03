//! Get card sub types (e.g.: Adventure, Elemental, Trap, Wizard).
#![allow(dead_code)]
use std::collections::HashSet;
use reqwest::StatusCode;
use serde::Deserialize;
use crate::query_builder;

#[doc(hidden)]
#[derive(Clone, Debug, Deserialize)]
pub struct RootAll {
    pub subtypes: HashSet<String>,
}

/// Function to get all sub types. 
///
/// # Example
/// ```rust
/// use mtgsdk::subtypes;
/// async { 
///    let subtypes = subtypes::all().await;
///    assert!(subtypes.unwrap().contains("Planeswalker"));
/// };
///```
///
/// # Errors
/// If the call fails, it will return a `Err(StatusCode)`. 
/// To see the possible return values, check https://docs.magicthegathering.io/#documentationerrors.
pub async fn all() -> Result<HashSet<String>, StatusCode>{
    let subtypes: Result<RootAll, StatusCode> = query_builder::all("subtypes").await;

    match subtypes {
        Ok(t) => Ok(t.subtypes),
        Err(e) => Err(e),
    }
}