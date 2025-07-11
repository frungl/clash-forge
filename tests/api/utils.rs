use std::path::{Path, PathBuf};

pub fn get_mock_data_path(path: impl AsRef<Path>) -> PathBuf {
    let current_dir = std::env::current_dir().expect("Should be ok");
    current_dir.join("tests/api/mock_data").join(path.as_ref())
}

pub fn get_test_rest_manager(base_url: &str) -> clash_forge::api::rest_manager::RestManager {
    clash_forge::api::rest_manager::RestManager::with_config(
        "test_token",
        clash_forge::api::rest_manager::RestManagerConfig {
            base_url: base_url.to_string(),
            timeout: std::time::Duration::from_secs(30),
            user_agent: format!("clash-forge-tests/{}", env!("CARGO_PKG_VERSION")),
        },
    )
    .expect("Test object should be created")
}