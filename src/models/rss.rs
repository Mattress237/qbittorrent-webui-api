use std::collections::HashMap;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum RssFeed {
    Item(String),
    Folder(HashMap<String, String>),
}

#[derive(Debug, Deserialize)]
pub struct RssFeedCollection {
    pub feeds: HashMap<String, RssFeed>,
}
