mod tests {
    use clash_forge::api::rest_manager::RestManager;
    use std::env;

    macro_rules! setup_client {
        () => {
            RestManager::new(env::var("BEARER_TOKEN").unwrap()).unwrap()
        };
    }

    #[tokio::test]
    async fn test_test() {
        let client = setup_client!();
        // let tmp = client.clans(
        //     ClanSearchOptions::builder().build(),
        //     PaginationOptions::builder().build(),
        // ).await;
        let tmp = client.player("#404TEST").await;
        assert!(tmp.is_err());
        println!("{}", tmp.err().unwrap());
    }
}
