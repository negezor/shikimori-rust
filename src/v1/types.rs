use serde::Deserialize;

use super::{characters::types::PartialCharacter, people::types::PartialPerson};

#[derive(Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum EntryStaffRole {
    Character(PartialCharacter),
    Person(PartialPerson),
}
