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
        let sets = sets::filter()
            .name("mirag")
            .all()
            .await;
        assert_eq!(sets.unwrap().pop().unwrap().name, "Mirage");
    }
    
    #[tokio::test]
    #[ignore]
    async fn filter_page(){ 
        let sets = sets::filter()
            .page(2)
            .page_size(13)
            .all()
            .await;
        assert_eq!(sets.unwrap().len(), 13);
    }
 
    #[tokio::test]
    #[ignore]
    async fn get_all_cards(){ 
        let cards = cards::all().await;
        assert_eq!(cards.unwrap().get(0).unwrap().name.chars().collect::<Vec<char>>()[0], 'A');
    }
 
    #[tokio::test]
    #[ignore]
    async fn find_card(){ 
        let cards = cards::find(386616).await;
        assert_eq!(cards.unwrap().name, "Narset, Enlightened Master");
    }
 
    #[tokio::test]
    #[ignore]
    async fn find_card_no_mana_cost(){ 
        let cards = cards::find(438608).await;
        println!("{}",cards.clone().unwrap().name);
        assert_eq!(cards.unwrap().name, "Ancestral Vision");
    }
    
    #[tokio::test]
    #[ignore]
    async fn filter_card(){ 
        let cards = cards::filter()
        .name("Karn")
        .all()
        .await;
        // assert!(cards.unwrap().get(0).unwrap().name.contains("Karn"));
        assert!(cards.unwrap().iter().any(|card| card.name == "Karn Liberated"));
    }
    
    #[tokio::test]
    #[ignore]
    async fn filter_card_two(){ 
        let cards = cards::filter()
            .page("50")
            .page_size("50")
            .all()
            .await;
        
        assert_eq!(cards.unwrap().len(), 50);
    }
    
    #[tokio::test]
    #[ignore]
    async fn filter_card_three(){ 
        let cards = cards::filter()
            .supertypes("legendary")
            .types("creature")
            .colors("red,white")
            .all()
            .await;
        
        assert!(cards.unwrap().iter().any(|card| card.name == "Breya, Etherium Shaper"));
    }
    
    #[tokio::test]
    #[ignore]
    async fn many_pages_stress(){
        let mut page: u64 = 0;
        while page < 50{
            page += 1;
            let cards = cards::filter()
                .page(&page.to_string())
                .page_size("100")
                .all()
                .await;
            
            assert_eq!(cards.unwrap().len(),100);
        }
    }
}