use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

use crate::utils::empty_string_as_none;

#[derive(Serialize, Deserialize, Clone, Hash, PartialEq, Eq, Debug)]
#[serde(rename_all = "snake_case")]
pub enum UserRateStatus {
    Planned,
    Watching,
    Rewatching,
    Completed,
    OnHold,
    Dropped,
}

#[derive(Serialize, Deserialize, Clone, Hash, PartialEq, Eq, Debug)]
pub enum UserRateTargetType {
    Anime,
    Manga,
}

#[derive(Deserialize, Clone, Debug)]
pub struct UserRateScoresStats {
    name: u8,
    value: u64,
}

#[derive(Deserialize, Clone, Debug)]
pub struct UserRateStatusesStats {
    name: String,
    value: u64,
}

#[derive(Deserialize, Clone, Debug)]
pub struct PartialUserRate {
    id: u64,
    score: u32,
    status: UserRateStatus,
    chapters: u32,
    episodes: u32,
    volumes: u32,
    rewatches: u32,
    #[serde(deserialize_with = "empty_string_as_none")]
    text: Option<String>,
    #[serde(deserialize_with = "empty_string_as_none")]
    text_html: Option<String>,
    created_at: OffsetDateTime,
    updated_at: OffsetDateTime,
}
