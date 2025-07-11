use clash_forge::api::common::pagination::PaginationOptions;
use crate::api::utils::get_test_rest_manager;

macro_rules! encode_path {
    ($name:expr) => {
        crate::api::utils::get_mock_data_path((format!("labels/{}.json", $name)))
    };
}

#[tokio::test]
async fn player_labels_test() {
    let mut server = mockito::Server::new_async().await;
    let url = server.url();
    let _m = server
        .mock("GET", "/labels/players")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body_from_file(encode_path!("players"))
        .create_async()
        .await;
    let result = get_test_rest_manager(&url).player_labels(PaginationOptions::default()).await;
    assert!(result.is_ok(), "Player labels request returned an error: {:#?}", result.err());
}

#[tokio::test]
async fn clan_labels_test() {
    let mut server = mockito::Server::new_async().await;
    let url = server.url();
    let _m = server
        .mock("GET", "/labels/clans")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body_from_file(encode_path!("clans"))
        .create_async()
        .await;
    let result = get_test_rest_manager(&url).clan_labels(PaginationOptions::default()).await;
    assert!(result.is_ok(), "Clan labels request returned an error: {:#?}", result.err());
}