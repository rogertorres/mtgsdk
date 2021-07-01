#![allow(dead_code)]
use reqwest::StatusCode;
use serde::{Serialize,Deserialize};
use crate::mtgsdk::query_builder;

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

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(untagged)]
pub enum Booster {
    Multiple(Vec<String>),
    Single(String),
}

#[derive(Clone, Debug, Deserialize)]
pub struct RootAll {
    sets: Vec<Set>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct RootFind {
    set: Set,
}

pub type ResultAll = Result<RootAll, StatusCode>;

pub type ResultFind = Result<RootFind, StatusCode>;

pub async fn all() -> Result<Vec<Set>, StatusCode>{
    let sets: ResultAll = query_builder::all("sets").await;

    match sets {
        Ok(t) => Ok(t.sets),
        Err(e) => Err(e),
    }
}

pub async fn find(id: &str) -> Result<Set, StatusCode>{
    let sets: ResultFind = query_builder::find("sets", id).await;

    match sets {
        Ok(t) => Ok(t.set),
        Err(e) => Err(e),
    }
}

pub struct Where<'a> {
    name: &'a str,
    page: u64,
    page_size: u64,
}

impl<'a> Where<'a> {
    pub fn name(mut self, input: &'a str) -> Self {
        self.name = input;
        self
    }

    pub fn page(mut self, input: u64) -> Self {
        self.page = input;
        self
    }

    pub fn page_size(mut self, input: u64) -> Self {
        self.page_size = input;
        self
    }

    pub async fn all(self) -> Result<Vec<Set>, StatusCode>{
        let mut filter = String::from("?");
        let mut and = "";

        if self.name.len() > 0 {
            filter = format!("{}{}{}", filter, "name=", self.name);
            and = "&";
        };

        if self.page > 0 {
            filter = format!("{}{}{}{}", filter, and, "page=", self.page);
            and = "&";
        };

        if self.page_size > 0 {
            filter = format!("{}{}{}{}", filter, and, "pageSize=", self.page_size);
        };

        let sets: ResultAll = query_builder::filter("sets", &filter).await;
    
        match sets {
            Ok(t) => Ok(t.sets),
            Err(e) => Err(e),
        }

    }
}

pub fn filter<'a>() -> Where<'a> {
    Where {
        name: "",
        page: 0,
        page_size: 0,        
    }
}