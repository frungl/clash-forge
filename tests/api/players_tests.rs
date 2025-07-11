use crate::api::utils::get_test_rest_manager;
use clash_forge::api::common::utils::normalize_tag;
use clash_forge::api::players::models::APIVerifyTokenRequest;

macro_rules! format_path {
    ($fmt:expr, $($arg:expr), *) => {
        crate::api::utils::get_mock_data_path(format!("players/{}.json", format!($fmt, $($arg), *)))
    };
}

macro_rules! format_url {
    ($fmt:expr, $($arg:expr), *) => {
        format!("/players/{}", format!($fmt, $($arg), *)).as_str()
    };
}

#[tokio::test]
async fn player_test() {
    let mut server = mockito::Server::new_async().await;
    let url = server.url();
    let tag_list = vec!["8VURQOYUJ", "9QP9LQOJ8"];
    for tag in tag_list {
        let normalized_tag = normalize_tag(tag);
        let _m = server
            .mock("GET", format_url!("{}", normalized_tag))
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body_from_file(format_path!("{}", tag))
            .create_async()
            .await;
        let result = get_test_rest_manager(&url).player(tag).await;
        assert!(result.is_ok(), "Player info request for tag {} returned an error: {:#?}", tag, result.err());
    }
}

#[tokio::test]
async fn player_verify_test() {
    let mut server = mockito::Server::new_async().await;
    let url = server.url();
    let tag = "8VURQOYUJ";
    let normalized_tag = normalize_tag(tag);
    let valid_token = "valid_token";
    let _m = server
        .mock("POST", format_url!("{}/verifytoken", normalized_tag))
        .match_request(|req| {
            let body = req.body().expect("Should have a body").as_slice();
            let request: APIVerifyTokenRequest = serde_json::from_slice(body).expect("Should be able to deserialize request body");
            request.token == valid_token.to_string()
        })
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body_from_file(format_path!("{}_valid", tag))
        .create_async()
        .await;

    let _m_invalid = server
        .mock("POST", format_url!("{}/verifytoken", normalized_tag))
        .match_request(|req| {
            let body = req.body().expect("Should have a body").as_slice();
            let request: APIVerifyTokenRequest = serde_json::from_slice(body).expect("Should be able to deserialize request body");
            request.token != valid_token.to_string()
        })
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body_from_file(format_path!("{}_invalid", tag))
        .create_async()
        .await;

    let result = get_test_rest_manager(&url).verify("8VURQOYUJ", "valid_token").await;
    assert!(result.is_ok(), "Player info request returned an error: {:#?}", result.err());
    assert!(result.unwrap(), "Expected token verification to be true, but it was false.");

    let result = get_test_rest_manager(&url).verify("8VURQOYUJ", "invalid_token").await;
    assert!(result.is_ok(), "Player info request returned an error: {:#?}", result.err());
    assert!(!result.unwrap(), "Expected token verification to be false, but it was true.");
}