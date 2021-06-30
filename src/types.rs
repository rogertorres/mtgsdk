use serde::Deserialize;
use crate::query_builder;

#[derive(Clone, Debug, Deserialize)]
pub struct Types {
    pub types: Vec<String>,
}

// impl Types{
pub async fn all() -> Result<Vec<String>, Box<dyn std::error::Error>>{
    let types: Types = query_builder::build("types").await?;

    Ok(types.types)
}
// }
