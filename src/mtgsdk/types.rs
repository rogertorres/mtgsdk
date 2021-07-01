#![allow(dead_code)]
use std::collections::HashSet;
use reqwest::StatusCode;
use serde::Deserialize;
use crate::mtgsdk::query_builder;

#[derive(Clone, Debug, Deserialize)]
pub struct Root {
    pub types: HashSet<String>,
}

pub type ResultAll = Result<Root, StatusCode>;

pub async fn all() -> Result<HashSet<String>, StatusCode>{
    let types: ResultAll = query_builder::all("types").await;

    match types {
        Ok(t) => Ok(t.types),
        Err(e) => Err(e),
    }
}