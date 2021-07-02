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

pub struct Where<'a>{
    query: Vec<(&'a str,String)>,
}

pub fn filter<'a>() -> Where<'a>{
    Where {
        query: Vec::new(),
    }
}

impl<'a> Where<'a> {
    pub fn name(mut self, input: &'a str) -> Self {
        self.query.push(("name", String::from(input)));
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

    pub async fn all(mut self) -> Result<Vec<Set>, StatusCode>{
        let val = self.query.remove(0);
        let mut filter = format!("?{}={}",val.0,val.1);
        
        for (k,v) in self.query.into_iter(){
            filter = format!("{}&{}={}",filter,k,v);
        }

        let sets: ResultAll = query_builder::filter("sets", &filter).await;
    
        match sets {
            Ok(t) => Ok(t.sets),
            Err(e) => Err(e),
        }
    }
}
