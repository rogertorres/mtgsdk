//! The `mtgsdk` crate provides a Rust wrapper (SDK) for the [Magic: The Gathering API](https://docs.magicthegathering.io/).
//!
//! It handles the calls to all the API endpoints through the use of 3 main functions: `all()`, `find()` and `filter()`.
//! - `all()` to get the unfiltered information through pagination;
//! - `find()` to return a specific information by id;
//! - `filter()` to filter the `all()` call.
//!
//! # Using the `all()` function
//! Returns the all the information available through a specific endpoint.
//!
//! # Example
//! Check the **Modules** below for examples of each function's usage.
pub mod cards;
pub mod formats;
mod query_builder;
pub mod sets;
pub mod subtypes;
pub mod supertypes;
pub mod types;

#[cfg(test)]
mod tests {
    use crate::*;

    #[tokio::test]
    //#[ignore]
    async fn get_all_formats() {
        let fmt = formats::all().await;
        assert!(fmt.unwrap().contains("Modern"));
    }

    #[tokio::test]
    //#[ignore]
    async fn get_all_types() {
        let typ = types::all().await;
        assert!(typ.unwrap().contains("Planeswalker"));
    }

    #[tokio::test]
    //#[ignore]
    async fn get_all_subtypes() {
        let sub = subtypes::all().await;
        assert!(sub.unwrap().contains("Eldrazi"));
    }

    #[tokio::test]
    //#[ignore]
    async fn get_all_supertypes() {
        let sup = supertypes::all().await;
        assert!(sup.unwrap().contains("Basic"));
    }

    #[tokio::test]
    //#[ignore]
    async fn get_all_sets() {
        let sets = sets::all().await;
        assert!(sets.unwrap().len() > 0);
    }

    #[tokio::test]
    //#[ignore]
    async fn find_set() {
        let sets = sets::find("dom").await;
        assert_eq!(sets.unwrap().name, "Dominaria");
    }

    #[tokio::test]
    //#[ignore]
    async fn filter_name() {
        let sets = sets::filter().name("mira").all().await;
        assert_eq!(sets.unwrap().pop().unwrap().name, "Mirage");
    }

    #[tokio::test]
    //#[ignore]
    async fn filter_page() {
        let sets = sets::filter().page(2).page_size(13).all().await;
        assert_eq!(sets.unwrap().len(), 13);
    }

    #[tokio::test]
    //#[ignore]
    async fn get_all_cards() {
        let cards = cards::all().await;
        assert_eq!(
            cards
                .unwrap()
                .get(0)
                .unwrap()
                .name
                .chars()
                .collect::<Vec<char>>()[0],
            'A'
        );
    }

    #[tokio::test]
    //#[ignore]
    async fn find_card() {
        let cards = cards::find(386616).await;
        assert_eq!(cards.unwrap().name, "Narset, Enlightened Master");
    }

    #[tokio::test]
    //#[ignore]
    async fn test_card_with_no_mana_cost() {
        let cards = cards::find(438608).await;
        assert_eq!(cards.unwrap().name, "Ancestral Vision");
    }

    #[tokio::test]
    //#[ignore]
    async fn filter_card() {
        let cards = cards::filter().name("Karn").all().await;
        assert!(cards
            .unwrap()
            .iter()
            .any(|card| card.name == "Karn Liberated"));
    }

    #[tokio::test]
    //#[ignore]
    async fn filter_card_two() {
        let cards = cards::filter().page(50).page_size(50).all().await;

        assert_eq!(cards.unwrap().len(), 50);
    }

    #[tokio::test]
    //#[ignore]
    async fn filter_card_three() {
        let cards = cards::filter()
            .supertypes("legendary")
            .types("creature")
            .colors("red,white")
            .all()
            .await;

        assert!(cards
            .unwrap()
            .iter()
            .any(|card| card.name == "Breya, Etherium Shaper"));
    }

    #[tokio::test]
    #[ignore]
    async fn many_pages_stress() {
        let mut page: u64 = 0;
        while page < 50 {
            page += 1;
            let cards = cards::filter().page(page).page_size(100).all().await;

            assert_eq!(cards.unwrap().len(), 100);
        }
    }
}
