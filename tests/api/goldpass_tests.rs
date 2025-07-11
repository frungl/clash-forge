use crate::api::utils::get_test_rest_manager;

macro_rules! format_path {
    ($name:expr) => {
        crate::api::utils::get_mock_data_path((format!("goldpass/{}.json", $name)))
    };
}

#[tokio::test]
async fn goldpass_test() {
    let mut server = mockito::Server::new_async().await;
    let url = server.url();
    let _m = server
        .mock("GET", "/goldpass/seasons/current")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body_from_file(format_path!("200"))
        .create_async()
        .await;
    let result = get_test_rest_manager(&url).goldpass().await;
    assert!(result.is_ok(), "Goldpass request returned an error: {:#?}", result.err());
}