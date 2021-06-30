mod query_builder;
mod types;

#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn get_types() -> Result<(), Box<dyn std::error::Error>> {
        use crate::types;
        let mut tp: Vec<String> = types::all().await?;
        assert_eq!(tp.remove(0),"Artifact");
        Ok(())
    }
}