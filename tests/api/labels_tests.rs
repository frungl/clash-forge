use clash_forge::api::common::pagination::PaginationOptions;
use crate::api::utils::get_test_rest_manager;

macro_rules! format_path {
    ($name:expr) => {
        crate::api::utils::get_mock_data_path((format!("labels/{}.json", $name)))
    };
}

macro_rules! format_url {
    ($name:expr) => {
        format!("/labels/{}", $name).as_str()
    };
}

#[tokio::test]
async fn player_labels_test() {
    let mut server = mockito::Server::new_async().await;
    let url = server.url();
    let _m = server
        .mock("GET", format_url!("players"))
        .match_query(mockito::Matcher::Any)
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body_from_file(format_path!("players"))
        .create_async()
        .await;
    let result = get_test_rest_manager(&url).player_labels(PaginationOptions::builder().limit(1).build()).await;
    assert!(result.is_ok(), "Player labels request returned an error: {:#?}", result.err());
}

#[tokio::test]
async fn clan_labels_test() {
    let mut server = mockito::Server::new_async().await;
    let url = server.url();
    let _m = server
        .mock("GET", format_url!("clans"))
        .match_query(mockito::Matcher::Any)
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body_from_file(format_path!("clans"))
        .create_async()
        .await;
    let result = get_test_rest_manager(&url).clan_labels(PaginationOptions::builder().limit(1).build()).await;
    assert!(result.is_ok(), "Clan labels request returned an error: {:#?}", result.err());
}