use clash_forge::api::common::pagination::PaginationOptions;
use crate::api::utils::get_test_rest_manager;

macro_rules! encode_path {
    ($fmt:expr, $($arg:expr), *) => {
        crate::api::utils::get_mock_data_path(format!("locations/{}.json", format!($fmt, $($arg), *)))
    };
}

macro_rules! encode_url {
    ($fmt:expr, $($arg:expr), *) => {
        format!("/locations/{}", format!($fmt, $($arg), *)).as_str()
    };
}

#[tokio::test]
async fn locations_test() {
    let mut server = mockito::Server::new_async().await;
    let url = server.url();
    let _m = server
        .mock("GET", encode_url!("{}", ""))
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body_from_file(encode_path!("{}", "locations"))
        .create_async()
        .await;
    let result = get_test_rest_manager(&url).locations(PaginationOptions::default()).await;
    assert!(result.is_ok(), "Locations request returned an error: {:#?}", result.err());
}

#[tokio::test]
async fn location_info_test() {
    let mut server = mockito::Server::new_async().await;
    let url = server.url();
    let location_ids = vec!["32000225"];
    for location_id in location_ids {
        let _m = server
            .mock("GET", encode_url!("{}", location_id))
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body_from_file(encode_path!("location_info_{}", location_id))
            .create_async()
            .await;
        let result = get_test_rest_manager(&url).location_info(location_id).await;
        assert!(result.is_ok(), "Location info request for ID {} returned an error: {:#?}", location_id, result.err());
    }
}

#[tokio::test]
async fn players_rankings_test() {
    let mut server = mockito::Server::new_async().await;
    let url = server.url();
    let location_ids = vec!["32000225"];
    for location_id in location_ids {
        let _m = server
            .mock("GET", encode_url!("{}/rankings/players", location_id))
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body_from_file(encode_path!("players_rankings_{}", location_id))
            .create_async()
            .await;
        let result = get_test_rest_manager(&url).players_rankings(location_id, PaginationOptions::default()).await;
        assert!(result.is_ok(), "Players rankings request for location ID {} returned an error: {:#?}", location_id, result.err());
    }
}

#[tokio::test]
async fn clans_rankings_test() {
    let mut server = mockito::Server::new_async().await;
    let url = server.url();
    let location_ids = vec!["32000225"];
    for location_id in location_ids {
        let _m = server
            .mock("GET", encode_url!("{}/rankings/clans", location_id))
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body_from_file(encode_path!("clans_rankings_{}", location_id))
            .create_async()
            .await;
        let result = get_test_rest_manager(&url).clans_rankings(location_id, PaginationOptions::default()).await;
        assert!(result.is_ok(), "Clans rankings request for location ID {} returned an error: {:#?}", location_id, result.err());
    }
}

#[tokio::test]
async fn players_builder_base_rankings_test() {
    let mut server = mockito::Server::new_async().await;
    let url = server.url();
    let location_ids = vec!["32000225"];
    for location_id in location_ids {
        let _m = server
            .mock("GET", encode_url!("{}/rankings/players-builder-base", location_id))
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body_from_file(encode_path!("players_builder_base_rankings_{}", location_id))
            .create_async()
            .await;
        let result = get_test_rest_manager(&url).players_builder_base_rankings(location_id, PaginationOptions::default()).await;
        assert!(result.is_ok(), "Players builder base rankings request for location ID {} returned an error: {:#?}", location_id, result.err());
    }
}

#[tokio::test]
async fn clans_builder_base_rankings_test() {
    let mut server = mockito::Server::new_async().await;
    let url = server.url();
    let location_ids = vec!["32000225"];
    for location_id in location_ids {
        let _m = server
            .mock("GET", encode_url!("{}/rankings/clans-builder-base", location_id))
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body_from_file(encode_path!("clans_builder_base_rankings_{}", location_id))
            .create_async()
            .await;
        let result = get_test_rest_manager(&url).clans_builder_base_rankings(location_id, PaginationOptions::default()).await;
        assert!(result.is_ok(), "Clans builder base rankings request for location ID {} returned an error: {:#?}", location_id, result.err());
    }
}

#[tokio::test]
async fn capitals_rankings_test() {
    let mut server = mockito::Server::new_async().await;
    let url = server.url();
    let location_ids = vec!["32000225"];
    for location_id in location_ids {
        let _m = server
            .mock("GET", encode_url!("{}/rankings/capitals", location_id))
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body_from_file(encode_path!("capitals_rankings_{}", location_id))
            .create_async()
            .await;
        let result = get_test_rest_manager(&url).capitals_rankings(location_id, PaginationOptions::default()).await;
        assert!(result.is_ok(), "Clans capital rankings request for location ID {} returned an error: {:#?}", location_id, result.err());
    }
}