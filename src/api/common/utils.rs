/// Normalize a clash of clans tag for API usage
///
/// Removes # prefix and validates format
pub fn normalize_tag(tag: &str) -> String {
    let cleaned = format!("#{}", tag.trim().trim_start_matches('#').to_uppercase());
    urlencoding::encode(&cleaned).to_string()
}