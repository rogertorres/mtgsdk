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
#[serde(rename_all = "camelCase")]
pub struct ForeignNames{
    pub name: String,
    #[serde(default)]
    pub text: String,
    #[serde(rename = "type")]
    pub type_field: Option<String>,
    pub flavor: Option<String>,
    pub image_url: Option<String>,
    pub language: String,
    pub multiverseid: Option<i64>,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct Legalities{
    pub format: String,
    pub legality: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Card {
    pub name: String,
    pub layout: String,
    pub cmc: f64,
    pub colors: Option<HashSet<String>>,
    pub color_identity: Option<HashSet<String>>,
    #[serde(rename = "type")]
    pub type_field: String,
    pub supertypes: Option<HashSet<String>>,
    pub types: HashSet<String>,
    pub subtypes: Option<HashSet<String>>,
    pub rarity: String,
    #[serde(rename = "set")]
    pub set_field: String,
    pub set_name: String,
    #[serde(default)]
    pub text: String,
    // pub flavor: Option<String>,
    pub artist: String,
    pub number: String,
    pub power: Option<String>,
    pub toughness: Option<String>,
    pub loyalty: Option<String>,
    // pub language: String,
    pub id: String,
    pub multiverseid: Option<String>,
    pub names: Option<HashSet<String>>,
    pub mana_cost: Option<String>,
    pub variations: Option<Vec<String>>,
    pub image_url: Option<String>,
    pub watermark: Option<String>,
    pub border: Option<String>,
    pub release_date: Option<String>,
    pub rulings: Option<Vec<Rulings>>,
    #[serde(default)]
    pub foreign_names: Vec<ForeignNames>,
    pub printings: HashSet<String>,
    pub original_text: Option<String>,
    pub original_type: Option<String>,
    #[serde(default)]
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

pub async fn find(id: u64) -> Result<Card, StatusCode>{
    let text_id = id.to_string();
    let cards: ResultFind = query_builder::find("cards", &text_id).await;

    match cards {
        Ok(t) => Ok(t.card),
        Err(e) => Err(e),
    }
}

// If more than one parameter is used, it has to be passed in the same field separated by | (pipe)
// E.g.: name = "nissa, worldwaker|jace|ajani, caller"
pub struct Where<'a>{
    query: Vec<(&'a str,&'a str)>,
}

pub fn filter<'a>() -> Where<'a>{
    Where {
        query: Vec::new(),
    }
}

impl<'a> Where<'a> {
    pub fn name(mut self, input: &'a str) -> Self {
        self.query.push(("name", input));
        self
    }
    pub fn layout(mut self, input: &'a str) -> Self {
        self.query.push(("layout", input));
        self
    }
    pub fn cmc(mut self, input: &'a str) -> Self {
        self.query.push(("cmc", input));
        self
    }
    pub fn colors(mut self, input: &'a str) -> Self {
        self.query.push(("colors", input));
        self
    }
    pub fn color_identity(mut self, input: &'a str) -> Self {
        self.query.push(("colorIdentity", input));
        self
    }
    pub fn type_field(mut self, input: &'a str) -> Self {
        self.query.push(("type", input));
        self
    }
    pub fn supertypes(mut self, input: &'a str) -> Self {
        self.query.push(("supertypes", input));
        self
    }
    pub fn types(mut self, input: &'a str) -> Self {
        self.query.push(("types", input));
        self
    }
    pub fn subtypes(mut self, input: &'a str) -> Self {
        self.query.push(("subtypes", input));
        self
    }
    pub fn rarity(mut self, input: &'a str) -> Self {
        self.query.push(("rarity", input));
        self
    }
    pub fn set_field(mut self, input: &'a str) -> Self {
        self.query.push(("set", input));
        self
    }
    pub fn set_name(mut self, input: &'a str) -> Self {
        self.query.push(("setName", input));
        self
    }
    pub fn text(mut self, input: &'a str) -> Self {
        self.query.push(("text", input));
        self
    }
    pub fn flavor(mut self, input: &'a str) -> Self {
        self.query.push(("flavor", input));
        self
    }
    pub fn artist(mut self, input: &'a str) -> Self {
        self.query.push(("artist", input));
        self
    }
    pub fn number(mut self, input: &'a str) -> Self {
        self.query.push(("number", input));
        self
    }
    pub fn power(mut self, input: &'a str) -> Self {
        self.query.push(("power", input));
        self
    }
    pub fn toughness(mut self, input: &'a str) -> Self {
        self.query.push(("toughness", input));
        self
    }
    pub fn loyalty(mut self, input: &'a str) -> Self {
        self.query.push(("loyalty", input));
        self
    }
    pub fn language(mut self, input: &'a str) -> Self {
        self.query.push(("language", input));
        self
    }
    pub fn game_format(mut self, input: &'a str) -> Self {
        self.query.push(("gameFormat", input));
        self
    }
    pub fn legality(mut self, input: &'a str) -> Self {
        self.query.push(("legality", input));
        self
    }
    pub fn page(mut self, input: &'a str) -> Self {
        self.query.push(("page", input));
        self
    }
    pub fn page_size(mut self, input: &'a str) -> Self {
        self.query.push(("pageSize", input));
        self
    }
    pub fn order_by(mut self, input: &'a str) -> Self {
        self.query.push(("orderBy", input));
        self
    }
    pub fn random(mut self, input: &'a str) -> Self {
        self.query.push(("random", input));
        self
    }
    pub fn contains(mut self, input: &'a str) -> Self {
        self.query.push(("contains", input));
        self
    }
    pub fn id(mut self, input: &'a str) -> Self {
        self.query.push(("id", input));
        self
    }
    pub fn multiverseid(mut self, input: &'a str) -> Self {
        self.query.push(("multiverseid", input));
        self
    }
   
    pub async fn all(mut self) -> Result<Vec<Card>, StatusCode>{
        let val = self.query.remove(0);
        let mut filter = format!("?{}={}",val.0,val.1);
        
        for (k,v) in self.query.into_iter(){
            filter = format!("{}&{}={}",filter,k,v);
        }

        let cards: ResultAll = query_builder::filter("cards", &filter).await;
    
        match cards {
            Ok(t) => Ok(t.cards),
            Err(e) => Err(e),
        }

    }
}