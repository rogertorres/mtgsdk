#![allow(dead_code)]
use reqwest::StatusCode;
use serde::{Serialize,Deserialize};
use std::collections::HashSet;
use crate::mtgsdk::query_builder;

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct Rulings{
    date: String,
    text: String,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct ForeignNames{
    name: String,
    text: String,
    #[serde(rename = "type")]
    type_field: String,
    flavor: Option<String>,
    image_url: String,
    language: String,
    multiverseid: u64,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct Legalities{
    format: String,
    legality: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Card {
    pub name: String,
    pub layout: String,
    pub cmc: u8,
    pub colors: HashSet<String>,
    pub color_identity: HashSet<String>,
    #[serde(rename = "type")]
    pub type_field: String,
    pub supertypes: HashSet<String>,
    pub types: HashSet<String>,
    pub subtypes: HashSet<String>,
    pub rarity: String,
    #[serde(rename = "set")]
    pub set_field: String,
    pub set_name: String,
    pub text: String,
    // pub flavor: Option<String>,
    pub artist: String,
    pub number: String,
    pub power: Option<String>,
    pub toughness: Option<String>,
    pub loyalty: Option<String>,
    // pub language: String,
    pub id: String,
    pub multiverseid: String,
    pub names: Option<HashSet<String>>,
    pub mana_cost: String,
    pub variations: Option<Vec<String>>,
    pub image_url: String,
    pub watermark: String,
    pub border: Option<String>,
    pub release_date: Option<String>,
    pub rulings: Vec<Rulings>,
    pub foregin_names: Vec<ForeignNames>,
    pub printings: HashSet<String>,
    pub original_text: String,
    pub original_type: String,
    pub legalities: Vec<Legalities>,
}


#[derive(Clone, Debug, Deserialize)]
pub struct RootAll {
    cards: Vec<Card>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct RootFind {
    card: Card,
}

pub type ResultAll = Result<RootAll, StatusCode>;

pub type ResultFind = Result<RootFind, StatusCode>;

pub async fn all() -> Result<Vec<Card>, StatusCode>{
    let cards: ResultAll = query_builder::all("cards").await;

    match cards {
        Ok(t) => Ok(t.cards),
        Err(e) => Err(e),
    }
}

// pub async fn find(id: &str) -> Result<Set, StatusCode>{
//     let cards: ResultFind = query_builder::find("cards", id).await;

//     match cards {
//         Ok(t) => Ok(t.set),
//         Err(e) => Err(e),
//     }
// }

// pub struct Where<'a> {
//     name: &'a str,
//     page: u64,
//     page_size: u64,
// }

// impl<'a> Where<'a> {
//     pub fn name(mut self, input: &'a str) -> Self {
//         self.name = input;
//         self
//     }

//     pub fn page(mut self, input: u64) -> Self {
//         self.page = input;
//         self
//     }

//     pub fn page_size(mut self, input: u64) -> Self {
//         self.page_size = input;
//         self
//     }

//     pub async fn all(self) -> Result<Vec<Set>, StatusCode>{
//         let mut filter = String::from("?");
//         let mut and = "";

//         if self.name.len() > 0 {
//             filter = format!("{}{}{}", filter, "name=", self.name);
//             and = "&";
//         };

//         if self.page > 0 {
//             filter = format!("{}{}{}{}", filter, and, "page=", self.page);
//             and = "&";
//         };

//         if self.page_size > 0 {
//             filter = format!("{}{}{}{}", filter, and, "pageSize=", self.page_size);
//         };

//         let sets: ResultAll = query_builder::filter("sets", &filter).await;
    
//         match sets {
//             Ok(t) => Ok(t.sets),
//             Err(e) => Err(e),
//         }

//     }
// }

// pub fn filter<'a>() -> Where<'a> {
//     Where {
//         name: "",
//         page: 0,
//         page_size: 0,        
//     }
// }