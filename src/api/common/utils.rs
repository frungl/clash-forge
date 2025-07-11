use crate::errors::{Error, Result};

/// Normalize a clash of clans tag for API usage
///
/// Removes # prefix and validates format
pub fn normalize_tag(tag: &str) -> Result<String> {
    let cleaned = format!("#{}", tag.trim().trim_start_matches('#').to_uppercase());

    if cleaned.is_empty() {
        return Err(Error::InvalidTag("Tag cannot be empty".to_string()));
    }

    if !cleaned.chars().skip(1).all(|c| c.is_ascii_alphanumeric()) {
        return Err(Error::InvalidTag(format!("Invalid tag format: {tag}")));
    }

    Ok(urlencoding::encode(&cleaned).to_string())
}