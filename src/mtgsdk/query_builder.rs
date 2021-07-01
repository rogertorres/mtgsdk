use reqwest::StatusCode;
use serde::de::DeserializeOwned;

const API_URL: &str = "https://api.magicthegathering.io";
const API_VER: &str = "v1";

async fn build<T>(url: String) -> Result<T, StatusCode> 
where T: DeserializeOwned {
    // // Build the client using the builder pattern
    // let client = reqwest::Client::builder()
    //     .build()?;


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

pub async fn all<T>(call: &str) -> Result<T, StatusCode> 
where T: DeserializeOwned {
    let url = format!("{}/{}/{}", API_URL, API_VER, call);
    build(url).await
}

pub async fn find<T>(call: &str, id: &str) -> Result<T, StatusCode> 
where T: DeserializeOwned {
    let url = format!("{}/{}/{}/{}", API_URL, API_VER, call, id);
    build(url).await
}

pub async fn filter<T>(call: &str, params: &str) -> Result<T, StatusCode> 
where T: DeserializeOwned {
    let url = format!("{}/{}/{}/{}", API_URL, API_VER, call, params);
    build(url).await
}