pub mod query_builder{
    use serde::de::DeserializeOwned;
    pub async fn build<T>(call: &str) -> Result<T, Box<dyn std::error::Error>> 
    where T: DeserializeOwned {
            // Build the client using the builder pattern
        let client = reqwest::Client::builder()
            .build()?;

        // Perform the actual execution of the network request
        let res = client
            .get("https://api.magicthegathering.io/v1/types")
            .send()
            .await?;

        // Parse the response body as Json in this case
        let types: T = res
            .json::<T>()
            .await?;

        Ok(types)
    }
}

pub mod api{
    use serde::Deserialize;
    use crate::query_builder;

    #[derive(Clone, Debug, Deserialize)]
    pub struct Types {
        pub types: Vec<String>,
    }

    impl Types{
        pub async fn all() -> Result<Vec<String>, Box<dyn std::error::Error>>{
            let types: Self = query_builder::build("types").await?;
    
            Ok(types.types)
        }
    }







}