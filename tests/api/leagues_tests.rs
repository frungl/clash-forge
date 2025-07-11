use clash_forge::api::common::pagination::PaginationOptions;
use crate::api::utils::get_test_rest_manager;

macro_rules! encode_path {
    ($fmt:expr, $($arg:expr), *) => {
        crate::api::utils::get_mock_data_path(format!("leagues/{}.json", format!($fmt, $($arg), *)))
    };
}

macro_rules! encode_url {
    ($r#type:expr, $fmt:expr, $($arg:expr), *) => {
        format!("/{}leagues/{}", $r#type, format!($fmt, $($arg), *)).as_str()
    };
}

#[tokio::test]
async fn leagues_test() {
    let mut server = mockito::Server::new_async().await;
    let url = server.url();
    let _m = server
        .mock("GET", encode_url!("", "",))
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body_from_file(encode_path!("{}", "leagues"))
        .create_async()
        .await;
    let result = get_test_rest_manager(&url).leagues(PaginationOptions::default()).await;
    assert!(result.is_ok(), "Leagues request returned an error: {:#?}", result.err());
}

#[tokio::test]
async fn builder_base_leagues_test() {
    let mut server = mockito::Server::new_async().await;
    let url = server.url();
    let _m = server
        .mock("GET", encode_url!("builderbase", "",))
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body_from_file(encode_path!("{}", "builder_base_leagues"))
        .create_async()
        .await;
    let result = get_test_rest_manager(&url).builder_base_leagues(PaginationOptions::default()).await;
    assert!(result.is_ok(), "Builder base leagues request returned an error: {:#?}", result.err());
}

#[tokio::test]
async fn war_leagues_test() {
    let mut server = mockito::Server::new_async().await;
    let url = server.url();
    let _m = server
        .mock("GET", encode_url!("war", "",))
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body_from_file(encode_path!("{}", "war_leagues"))
        .create_async()
        .await;
    let result = get_test_rest_manager(&url).war_leagues(PaginationOptions::default()).await;
    assert!(result.is_ok(), "War leagues request returned an error: {:#?}", result.err());
}

#[tokio::test]
async fn capital_leagues_test() {
    let mut server = mockito::Server::new_async().await;
    let url = server.url();
    let _m = server
        .mock("GET", encode_url!("capital", "",))
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body_from_file(encode_path!("{}", "capital_leagues"))
        .create_async()
        .await;
    let result = get_test_rest_manager(&url).capital_leagues(PaginationOptions::default()).await;
    assert!(result.is_ok(), "Capital leagues request returned an error: {:#?}", result.err());
}

#[tokio::test]
async fn league_seasons_test() {
    let mut server = mockito::Server::new_async().await;
    let url = server.url();
    let legend_league_id = "29000022";
    let league_ids = vec![
        "29000021",
        "29000022",
    ];
    for league_id in league_ids {
        let _m = server
            .mock("GET", encode_url!("", "{}/seasons", league_id))
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body_from_file(encode_path!("league_seasons_{}", league_id))
            .create_async()
            .await;
        let result = get_test_rest_manager(&url).league_seasons(league_id, PaginationOptions::default()).await;
        if league_id == legend_league_id {
            assert!(result.is_ok(), "Legend league seasons request returned an error: {:#?}", result.err());
        } else {
            assert!(result.is_err(), "A non-legend league seasons request should return an error: {:#?}", result.err());
        }
    }
}

#[tokio::test]
async fn league_info_test() {
    let mut server = mockito::Server::new_async().await;
    let url = server.url();
    let league_ids = vec![
        "29000000",
        "29000017",
    ];
    for league_id in league_ids {
        let _m = server
            .mock("GET", encode_url!("", "{}", league_id))
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body_from_file(encode_path!("league_info_{}", league_id))
            .create_async()
            .await;
        let result = get_test_rest_manager(&url).league_info(league_id).await;
        assert!(result.is_ok(), "League info request returned an error: {:#?}", result.err());
    }
}

#[tokio::test]
async fn builder_base_league_info_test() {
    let mut server = mockito::Server::new_async().await;
    let url = server.url();
    let league_ids = vec![
        "44000034",
    ];
    for league_id in league_ids {
        let _m = server
            .mock("GET", encode_url!("builderbase", "{}", league_id))
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body_from_file(encode_path!("builder_base_league_info_{}", league_id))
            .create_async()
            .await;
        let result = get_test_rest_manager(&url).builder_base_league_info(league_id).await;
        assert!(result.is_ok(), "Builder base league Info request returned an error: {:#?}", result.err());
    }
}

#[tokio::test]
async fn war_league_info_test() {
    let mut server = mockito::Server::new_async().await;
    let url = server.url();
    let league_ids = vec![
        "48000011",
    ];
    for league_id in league_ids {
        let _m = server
            .mock("GET", encode_url!("war", "{}", league_id))
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body_from_file(encode_path!("war_league_info_{}", league_id))
            .create_async()
            .await;
        let result = get_test_rest_manager(&url).war_league_info(league_id).await;
        assert!(result.is_ok(), "War league info request returned an error: {:#?}", result.err());
    }
}

#[tokio::test]
async fn capital_league_info_test() {
    let mut server = mockito::Server::new_async().await;
    let url = server.url();
    let league_ids = vec![
        "85000010",
    ];
    for league_id in league_ids {
        let _m = server
            .mock("GET", encode_url!("capital", "{}", league_id))
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body_from_file(encode_path!("capital_league_info_{}", league_id))
            .create_async()
            .await;
        let result = get_test_rest_manager(&url).capital_league_info(league_id).await;
        assert!(result.is_ok(), "Capital league info request returned an error: {:#?}", result.err());
    }
}

#[tokio::test]
async fn league_season_rankings_test() {
    let mut server = mockito::Server::new_async().await;
    let url = server.url();
    let legend_league_id = "29000022";
    let league_season_ids = vec![
        ("29000021", "2024-12"),
        ("29000022", "2024-12"),
    ];
    for (league_id, season_id) in league_season_ids {
        let _m = server
            .mock("GET", encode_url!("", "{}/seasons/{}", league_id, season_id))
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body_from_file(encode_path!("league_season_rankings_{}_{}", league_id, season_id))
            .create_async()
            .await;
        let result = get_test_rest_manager(&url).league_season_rankings(league_id, season_id, PaginationOptions::default()).await;
        if league_id == legend_league_id {
            assert!(result.is_err(), "Currently, Legend league season rankings endpoint is disabled, so it should return an error: {:#?}", result.err());
            // assert!(result.is_ok(), "Legend league season rankings request returned an error: {:#?}", result.err());
        } else {
            assert!(result.is_err(), "A non-legend league season rankings request should return an error: {:#?}", result.err());
        }
    }
}