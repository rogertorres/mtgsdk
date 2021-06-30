use reqwest::StatusCode;
use serde::de::DeserializeOwned;

pub async fn build<T>(call: &str) -> Result<T, StatusCode> 
where T: DeserializeOwned {

    const API_URL: &str = "https://api.magicthegathering.io";
    const API_VER: &str = "v1";

    // // Build the client using the builder pattern
    // let client = reqwest::Client::builder()
    //     .build()?;

    let url = format!("{}/{}/{}", API_URL, API_VER, call);

    // // Perform the actual execution of the network request
    // let response = client
    //     .get(&url)
    //     .send()
    //     .await;

    let response = reqwest::get(url).await;

    match &response {
        Ok(r) => {
            if r.status() != StatusCode::OK {
                return Err(r.status())
            }
        },
        Err(e) => {
            if e.is_status() {
                return Err(e.status().unwrap());
            }
            else {
                return Err(StatusCode::BAD_REQUEST);
            }
        }
    }

    // Parse the response body as Json in this case
    let content= response.unwrap() 
        .json::<T>()
        .await;

    if let Ok(des) = content{
        Ok(des)
    } else { 
        Err(StatusCode::BAD_REQUEST)
    }
}

// pub async fn build_backup<T>(call: &str) -> Result<T, reqwest::Error> 
// where T: DeserializeOwned {

//     const API_URL: &str = "https://api.magicthegathering.io";
//     const API_VER: &str = "v1";

//     // Build the client using the builder pattern
//     let client = reqwest::Client::builder()
//         .build()?;

//     let url = format!("{}/{}/{}", API_URL, API_VER, call);

//     // Perform the actual execution of the network request
//     let res = client
//         .get(&url)
//         .send()
//         .await?;

//     // Parse the response body as Json in this case
//     let content: T = res
//         .json::<T>()
//         .await?;

//     Ok(content)
// }