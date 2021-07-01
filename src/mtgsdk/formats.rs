#![allow(dead_code)]
use std::collections::HashSet;
use reqwest::StatusCode;
use serde::Deserialize;
use crate::mtgsdk::query_builder;

#[derive(Clone, Debug, Deserialize)]
pub struct Root {
    pub formats: HashSet<String>,
}

pub type ResultAll = Result<Root, StatusCode>;

pub async fn all() -> Result<HashSet<String>, StatusCode>{
    let formats: ResultAll = query_builder::all("formats").await;

    match formats {
        Ok(t) => Ok(t.formats),
        Err(e) => Err(e),
    }
}