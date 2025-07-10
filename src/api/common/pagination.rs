use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct PaginationOptions {
    pub limit: Option<u32>,
    pub after: Option<String>,
    pub before: Option<String>,
}

impl PaginationOptions {
    /// Creates a `PaginationOptionsBuilder` to configure `PaginationOptions`.
    ///
    /// This is the same as `PaginationOptionsBuilder::new()`.
    pub fn builder() -> PaginationOptionsBuilder {
        PaginationOptionsBuilder::new()
    }

    /// Provides `HashMap<String, String>` for existing pagination options.
    pub fn to_query_parameters(self) -> HashMap<String, String> {
        let mut parameters = HashMap::new();
        if let Some(limit) = self.limit {
            parameters.insert(String::from("limit"), limit.to_string());
        }
        if let Some(after) = self.after {
            parameters.insert(String::from("after"), after.to_string());
        }
        if let Some(before) = self.before {
            parameters.insert(String::from("before"), before.to_string());
        }
        parameters
    }
}

/// Builder for `PaginationOptions`.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct PaginationOptionsBuilder {
    limit: Option<u32>,
    after: Option<String>,
    before: Option<String>,
}

impl PaginationOptionsBuilder {
    /// Constructs a new `PaginationOptionsBuilder`.
    ///
    /// This is the same as `PaginationOptions::builder()`.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the limit for the number of items to return.
    pub fn limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }

    /// Sets the cursor to return items after a specific item.
    pub fn after(mut self, after: impl AsRef<str>) -> Self {
        self.after = Some(after.as_ref().to_owned());
        self
    }

    /// Sets the cursor to return items before a specific item.
    pub fn before(mut self, before: impl AsRef<str>) -> Self {
        self.before = Some(before.as_ref().to_owned());
        self
    }


    /// Returns the `PaginationOptions` that uses `PaginationOptionsBuilder` configuration.
    pub fn build(self) -> PaginationOptions {
        PaginationOptions {
            limit: self.limit,
            after: self.after,
            before: self.before,
        }
    }
}