#![allow(dead_code)]
use std::collections::HashSet;
use reqwest::StatusCode;
use serde::Deserialize;
use crate::mtgsdk::query_builder;

#[derive(Clone, Debug, Deserialize)]
pub struct Root {
    pub subtypes: HashSet<String>,
}

type ResultAll = Result<Root, StatusCode>;

pub async fn all() -> Result<HashSet<String>, StatusCode>{
    let subtypes: ResultAll = query_builder::all("subtypes").await;

    match subtypes {
        Ok(t) => Ok(t.subtypes),
        Err(e) => Err(e),
    }
}