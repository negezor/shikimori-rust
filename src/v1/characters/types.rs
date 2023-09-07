use serde::Deserialize;

use crate::utils::empty_string_as_none;

#[derive(Debug, Deserialize, Clone, Hash, PartialEq, Eq)]
pub enum CharacterRole {
    Main,
    Supporting,
    Background,
}

#[derive(Deserialize, Debug, Clone)]
pub struct CharacterCover {
    original: String,
    preview: String,
    x48: String,
    x96: String,
}

#[derive(Deserialize, Clone, Debug)]
pub struct PartialCharacter {
    id: u64,
    image: CharacterCover,
    name: String,
    #[serde(deserialize_with = "empty_string_as_none")]
    russian: Option<String>,
    url: String,
}

pub struct PartialCharacterRole {
    roles: Vec<CharacterRole>,
    roles_russian: Vec<String>,
    character: PartialCharacter,
}
