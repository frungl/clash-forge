use clash_forge::api::clans::models::APIWarFrequency;
use clash_forge::api::clans::search::ClanSearchOptions;

#[test]
fn search_correct_query_parameters_test() {
    let search_options = ClanSearchOptions::builder()
        .name("Test Clan".to_string())
        .war_frequency(APIWarFrequency::Always)
        .location_id(12345)
        .min_members(10)
        .max_members(50)
        .min_clan_level(5)
        .label_ids(vec!["label1".to_string(), "label2".to_string()])
        .build();
    let query_parameters = search_options.to_query_parameters();
    assert_eq!(query_parameters.get("name"), Some(&"Test Clan".to_string()));
    assert_eq!(query_parameters.get("warFrequency"), Some(&serde_json::to_string(&APIWarFrequency::Always).unwrap()));
    assert_eq!(query_parameters.get("locationId"), Some(&"12345".to_string()));
    assert_eq!(query_parameters.get("minMembers"), Some(&"10".to_string()));
    assert_eq!(query_parameters.get("maxMembers"), Some(&"50".to_string()));
    assert_eq!(query_parameters.get("minClanLevel"), Some(&"5".to_string()));
    assert_eq!(query_parameters.get("labelIds"), Some(&"label1,label2".to_string()));

    assert_eq!(query_parameters.len(), 7, "Expected 7 query parameters, found {}", query_parameters.len());
}

#[test]
fn search_empty_query_parameters_test() {
    let search_options = ClanSearchOptions::default();
    let query_parameters = search_options.to_query_parameters();
    assert!(query_parameters.is_empty(), "Expected empty query parameters, found {:?}", query_parameters);
}