mod query_builder;
mod types;

#[cfg(test)]
mod tests {
    use crate::query_builder;
    use crate::types;
    use reqwest::StatusCode;

    #[tokio::test]
    async fn error_404_not_found(){
        let tp: types::Type = query_builder::build("forcenotfound")
            .await;

        assert_eq!(tp.unwrap_err(),StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    #[ignore]
    async fn get_types(){ 
        let tp = types::all().await;
        assert_eq!(tp.unwrap().remove(0),"Artifact");
    }
}







