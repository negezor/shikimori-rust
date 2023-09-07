use serde::Deserialize;

use crate::utils::empty_string_as_none;

#[derive(Debug, Deserialize, Clone, Hash, PartialEq, Eq)]
pub enum PersonRole {
    Main,
    Supporting,
    Other,
}

#[derive(Deserialize, Debug, Clone)]
pub struct PersonCover {
    original: String,
    preview: String,
    x48: String,
    x96: String,
}

#[derive(Deserialize, Clone, Debug)]
pub struct PartialPerson {
    id: u64,
    image: PersonCover,
    name: String,
    #[serde(deserialize_with = "empty_string_as_none")]
    russian: Option<String>,
    url: String,
}

pub struct PartialPersonRole {
    roles: Vec<String>,
    roles_russian: Vec<String>,
    character: PartialPerson,
}
