use clash_forge::api::clans::search::ClanSearchOptions;
use clash_forge::api::common::pagination::PaginationOptions;
use clash_forge::api::common::utils::normalize_tag;
use crate::api::utils::get_test_rest_manager;

macro_rules! format_path {
    ($fmt:expr, $($arg:expr), *) => {
        crate::api::utils::get_mock_data_path(format!("clans/{}.json", format!($fmt, $($arg), *)))
    };
}

macro_rules! format_url {
    ($fmt:expr, $($arg:expr), *) => {
        format!("/clans/{}", format!($fmt, $($arg), *)).as_str()
    };
}

#[tokio::test]
async fn clan_search_test() {
    let mut server = mockito::Server::new_async().await;
    let url = server.url();
    let _m = server
        .mock("GET", "/clans")
        .match_query(mockito::Matcher::Any)
        .with_status(400)
        .with_header("content-type", "application/json")
        .with_body_from_file(format_path!("search/at_least_one",))
        .create_async()
        .await;
    let result = get_test_rest_manager(&url).clans(
        ClanSearchOptions::default(),
        PaginationOptions::builder().limit(1).build()
    ).await;
    assert!(result.is_err(), "Clan search request should return an error: {:#?}", result);

    let _m = server
        .mock("GET", "/clans")
        .match_query(mockito::Matcher::Any)
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body_from_file(format_path!("search/valid_search",))
        .create_async()
        .await;

    let result = get_test_rest_manager(&url).clans(
        ClanSearchOptions::builder()
            .name("Rust")
            .min_members(15)
            .min_clan_level(4)
            .build(),
        PaginationOptions::builder()
            .limit(1)
            .after("eyJwb3MiOjN9")
            .build()
    ).await;
    assert!(result.is_ok(), "Clan search request should return a valid response: {:#?}", result);
}

#[tokio::test]
async fn clan_info_test() {
    let mut server = mockito::Server::new_async().await;
    let url = server.url();
    let tag_list = vec!["2Q0Q0JG82"];
    for tag in tag_list {
        let normalized_tag = normalize_tag(tag);
        let _m = server
            .mock("GET", format_url!("{}", normalized_tag))
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body_from_file(format_path!("clan_info/{}", tag))
            .create_async()
            .await;
        let result = get_test_rest_manager(&url).clan_info(tag).await;
        assert!(result.is_ok(), "Clan info request for tag {} returned an error: {:#?}", tag, result.err());
    }
}

#[tokio::test]
async fn clan_members_test() {
    let mut server = mockito::Server::new_async().await;
    let url = server.url();
    let tag_list = vec!["2Q0Q0JG82"];
    for tag in tag_list {
        let normalized_tag = normalize_tag(tag);
        let _m = server
            .mock("GET", format_url!("{}/members", normalized_tag))
            .match_query(mockito::Matcher::Any)
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body_from_file(format_path!("clan_members/{}", tag))
            .create_async()
            .await;
        let result = get_test_rest_manager(&url).clan_members(tag, PaginationOptions::builder().limit(1).build()).await;
        assert!(result.is_ok(), "Clan members request for tag {} returned an error: {:#?}", tag, result.err());
    }
}

#[tokio::test]
async fn war_log_test() {
    let mut server = mockito::Server::new_async().await;
    let url = server.url();
    let tag_list = vec!["2Q0Q0JG82"];
    for tag in tag_list {
        let normalized_tag = normalize_tag(tag);
        let _m = server
            .mock("GET", format_url!("{}/warlog", normalized_tag))
            .match_query(mockito::Matcher::Any)
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body_from_file(format_path!("war_log/{}", tag))
            .create_async()
            .await;
        let result = get_test_rest_manager(&url).war_log(tag, PaginationOptions::builder().limit(1).build()).await;
        assert!(result.is_ok(), "Clan war log request for tag {} returned an error: {:#?}", tag, result.err());
    }
}

#[tokio::test]
async fn current_war_test() {
    let mut server = mockito::Server::new_async().await;
    let url = server.url();
    let tag_state_list = vec![
        ("2Q0Q0JG82", "notInWar"),
        ("2GLOQ9VY2", "preparation"),
        ("QY9RQ2G2", "inWar"),
    ];
    for (tag, state) in tag_state_list {
        let normalized_tag = normalize_tag(tag);
        let _m = server
            .mock("GET", format_url!("{}/currentwar", normalized_tag))
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body_from_file(format_path!("current_war/{}_{}", tag, state))
            .create_async()
            .await;
        let result = get_test_rest_manager(&url).current_war(tag).await;
        assert!(result.is_ok(), "Current war request for tag {} and state {} returned an error: {:#?}", tag, state, result.err());
    }
}

#[tokio::test]
async fn clan_war_league_group_test() {
    let mut server = mockito::Server::new_async().await;
    let url = server.url();
    let tag_state_list = vec![
        ("2Q0Q0JG82", "inWar"),
        ("2Q0Q0JG82", "ended"),
    ];
    for (tag, state) in tag_state_list {
        let normalized_tag = normalize_tag(tag);
        let _m = server
            .mock("GET", format_url!("{}/currentwar/leaguegroup", normalized_tag))
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body_from_file(format_path!("clan_war_league_group/{}_{}", tag, state))
            .create_async()
            .await;
        let result = get_test_rest_manager(&url).clan_war_league_group(tag).await;
        assert!(result.is_ok(), "Clan war league group request for tag {} and state {} returned an error: {:#?}", tag, state, result.err());
    }
}

#[tokio::test]
async fn clan_war_league_war_test() {
    let mut server = mockito::Server::new_async().await;
    let url = server.url();
    let tag_state_list = vec![
        ("DAY1TAG", "ended"),
        ("DAY1TAG", "inWar"),
        ("DAY2TAG", "preparation"),
    ];
    for (tag, state) in tag_state_list {
        let normalized_tag = normalize_tag(tag);
        let _m = server
            .mock("GET", format!("/clanwarleagues/wars/{}", normalized_tag).as_str())
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body_from_file(format_path!("clan_war_league_war/{}_{}", tag, state))
            .create_async()
            .await;
        let result = get_test_rest_manager(&url).clan_war_league_war(tag).await;
        assert!(result.is_ok(), "Clan war league war request for tag {} and state {} returned an error: {:#?}", tag, state, result.err());
    }
}

#[tokio::test]
async fn clan_capital_raid_seasons_test() {
    let mut server = mockito::Server::new_async().await;
    let url = server.url();
    let tag_state_list = vec![
        ("2Q0Q0JG82", "new"),
        ("2Q0Q0JG82", "ongoing"),
        ("2Q0Q0JG82", "ended"),
    ];
    for (tag, state) in tag_state_list {
        let normalized_tag = normalize_tag(tag);
        let _m = server
            .mock("GET", format_url!("{}/capitalraidseasons", normalized_tag))
            .match_query(mockito::Matcher::Any)
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body_from_file(format_path!("clan_capital_raid_seasons/{}_{}", tag, state))
            .create_async()
            .await;
        let result = get_test_rest_manager(&url).clan_capital_raid_seasons(tag, PaginationOptions::builder().limit(1).build()).await;
        assert!(result.is_ok(), "Clan capital raid seasons request for tag {} and state {} returned an error: {:#?}", tag, state, result.err());
    }
}
