use clash_forge::api::common::pagination::PaginationOptions;

#[test]
fn pagination_correct_query_parameters_test() {
    let pagination_options = PaginationOptions::builder()
        .limit(20)
        .after("abc123".to_string())
        .before("xyz789".to_string())
        .build();
    let query_parameters = pagination_options.to_query_parameters();
    assert_eq!(query_parameters.get("limit"), Some(&"20".to_string()));
    assert_eq!(query_parameters.get("after"), Some(&"abc123".to_string()));
    assert_eq!(query_parameters.get("before"), Some(&"xyz789".to_string()));

    assert_eq!(query_parameters.len(), 3, "Expected 3 query parameters, found {}", query_parameters.len());
}

#[test]
fn pagination_empty_query_parameters_test() {
    let pagination_options = PaginationOptions::default();
    let query_parameters = pagination_options.to_query_parameters();
    assert!(query_parameters.is_empty(), "Expected empty query parameters, found {:?}", query_parameters);
}