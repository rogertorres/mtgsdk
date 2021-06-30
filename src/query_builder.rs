use serde::de::DeserializeOwned;

pub async fn build<T>(call: &str) -> Result<T, Box<dyn std::error::Error>> 
where T: DeserializeOwned {

    const API_URL: &str = "https://api.magicthegathering.io";
    const API_VER: &str = "v1";

    // Build the client using the builder pattern
    let client = reqwest::Client::builder()
        .build()?;

    let url = format!("{}/{}/{}", API_URL, API_VER, call);

    // Perform the actual execution of the network request
    let res = client
        .get(&url)
        .send()
        .await?;

    // Parse the response body as Json in this case
    let types: T = res
        .json::<T>()
        .await?;

    Ok(types)
}