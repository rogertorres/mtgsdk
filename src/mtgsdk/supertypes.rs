#![allow(dead_code)]
use std::collections::HashSet;
use reqwest::StatusCode;
use serde::Deserialize;
use crate::mtgsdk::query_builder;

#[derive(Clone, Debug, Deserialize)]
pub struct Root {
    pub supertypes: HashSet<String>,
}

pub type ResultAll = Result<Root, StatusCode>;

pub async fn all() -> Result<HashSet<String>, StatusCode>{
    let supertypes: ResultAll = query_builder::all("supertypes").await;

    match supertypes {
        Ok(t) => Ok(t.supertypes),
        Err(e) => Err(e),
    }
}