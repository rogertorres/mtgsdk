mod mtgsdk;

#[cfg(test)]
mod tests {
    use reqwest::StatusCode;
    use crate::mtgsdk::*;
    
    #[tokio::test]
    #[ignore]
    async fn error_404_not_found(){
        let not: types::ResultAll = query_builder::all("forcenotfound").await;
        assert_eq!(not.unwrap_err(),StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    #[ignore]
    async fn get_all_formats(){ 
        let fmt = formats::all().await;
        assert!(fmt.unwrap().remove("Modern"));
    }

    #[tokio::test]
    #[ignore]
    async fn get_all_types(){ 
        let typ = types::all().await;
        assert!(typ.unwrap().remove("Planeswalker"));
    }

    #[tokio::test]
    #[ignore]
    async fn get_all_subtypes(){ 
        let sub = subtypes::all().await;
        assert!(sub.unwrap().remove("Eldrazi"));
    }

    #[tokio::test]
    #[ignore]
    async fn get_all_supertypes(){ 
        let sup = supertypes::all().await;
        assert!(sup.unwrap().remove("Basic"));
    }

    #[tokio::test]
    #[ignore]
    async fn get_all_sets(){ 
        let sets = sets::all().await;
        assert!(sets.unwrap().len() > 0);
    }

    #[tokio::test]
    #[ignore]
    async fn find_set(){ 
        let sets = sets::find("dom").await;
        assert_eq!(sets.unwrap().name, "Dominaria");
    }

    #[tokio::test]
    #[ignore]
    async fn filter_name(){ 
        let sets = sets::filter().name("mirag").all().await;
        assert_eq!(sets.unwrap().pop().unwrap().name, "Mirage");
    }

    #[tokio::test]
    async fn filter_page(){ 
        let sets = sets::filter().page(2).page_size(3).all().await;
        assert_eq!(sets.unwrap().len(), 3);
    }
}