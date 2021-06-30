use serde::Deserialize;
use crate::query_builder;

#[derive(Clone, Debug, Deserialize)]
pub struct Types {
    pub types: Vec<String>,
}

pub type Type = Result<Types, reqwest::StatusCode>;

pub async fn all() -> Result<Vec<String>, reqwest::StatusCode>{
    let types: Type = query_builder::build("types").await;

    match types {
        Ok(t) => Ok(t.types),
        Err(e) => Err(e),
    }
}