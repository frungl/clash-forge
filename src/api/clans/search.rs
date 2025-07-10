use crate::api::clans::models::APIWarFrequency;
use std::collections::HashMap;


/// Represents search options for clans.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ClanSearchOptions {
    pub name: Option<String>,
    pub war_frequency: Option<APIWarFrequency>,
    pub location_id: Option<i64>,
    pub min_members: Option<i64>,
    pub max_members: Option<i64>,
    pub min_clan_level: Option<i64>,
    pub label_ids: Option<Vec<String>>,
}

impl ClanSearchOptions {
    /// Creates a `PaginationOptionsBuilder` to configure `ClanSearchOptions`.
    ///
    /// This is the same as `ClanSearchOptionsBuilder::new()`.
    pub fn builder() -> ClanSearchOptionsBuilder {
        ClanSearchOptionsBuilder::new()
    }
    
    /// Provides `HashMap<String, String>` for existing search options.
    pub fn to_query_parameters(self) -> HashMap<String, String> {
        let mut parameters = HashMap::new();
        if let Some(name) = self.name {
            parameters.insert(String::from("name"), name);
        }
        if let Some(war_frequency) = self.war_frequency {
            parameters.insert(String::from("warFrequency"), serde_json::to_string(&war_frequency).unwrap());
        }
        if let Some(location_id) = self.location_id {
            parameters.insert(String::from("locationId"), location_id.to_string());
        }
        if let Some(min_members) = self.min_members {
            parameters.insert(String::from("minMembers"), min_members.to_string());
        }
        if let Some(max_members) = self.max_members {
            parameters.insert(String::from("maxMembers"), max_members.to_string());
        }
        if let Some(min_clan_level) = self.min_clan_level {
            parameters.insert(String::from("minClanLevel"), min_clan_level.to_string());
        }
        if let Some(label_ids) = self.label_ids {
            parameters.insert(String::from("labelIds"), label_ids.join(","));
        }
        parameters
    }
}

/// Builder for `ClanSearchOptions`
#[derive(Default, Debug)]
pub struct ClanSearchOptionsBuilder {
    name: Option<String>,
    war_frequency: Option<APIWarFrequency>,
    location_id: Option<i64>,
    min_members: Option<i64>,
    max_members: Option<i64>,
    min_clan_level: Option<i64>,
    label_ids: Option<Vec<String>>,
}

impl ClanSearchOptionsBuilder {
    /// Constructs a new `ClanSearchOptionsBuilder`.
    /// 
    /// This is the same as `ClanSearchOptions::builder()`.
    pub fn new() -> Self {
        ClanSearchOptionsBuilder::default()
    }

    /// Sets the name filter for the clan search.
    pub fn name(mut self, name: impl AsRef<str>) -> Self {
        self.name = Some(name.as_ref().to_owned());
        self
    }

    /// Sets the war frequency filter for the clan search.
    pub fn war_frequency(mut self, war_frequency: APIWarFrequency) -> Self {
        self.war_frequency = Some(war_frequency);
        self
    }

    /// Sets the location ID filter for the clan search.
    pub fn location_id(mut self, location_id: i64) -> Self {
        self.location_id = Some(location_id);
        self
    }


    /// Sets the minimum number of clan members for the search.
    pub fn min_members(mut self, min_members: i64) -> Self {
        self.min_members = Some(min_members);
        self
    }


    /// Sets the maximum number of clan members for the search.
    pub fn max_members(mut self, max_members: i64) -> Self {
        self.max_members = Some(max_members);
        self
    }

    /// Sets the minimum clan level for the search.
    pub fn min_clan_level(mut self, min_clan_level: i64) -> Self {
        self.min_clan_level = Some(min_clan_level);
        self
    }


    /// Sets the label IDs for filtering results.
    pub fn label_ids(mut self, label_ids: Vec<String>) -> Self {
        self.label_ids = Some(label_ids);
        self
    }

    /// Returns the 'ClanSearchOptions' that uses 'ClanSearchOptionsBuilder' configuration.
    pub fn build(self) -> ClanSearchOptions {
        ClanSearchOptions {
            name: self.name,
            war_frequency: self.war_frequency,
            location_id: self.location_id,
            min_members: self.min_members,
            max_members: self.max_members,
            min_clan_level: self.min_clan_level,
            label_ids: self.label_ids,
        }
    }
}