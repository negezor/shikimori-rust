use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Hash, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum MangaKind {
    Manga,
    Manhwa,
    Manhua,
    LightNovel,
    Novel,
    OneShot,
    Doujin,
}

#[derive(Debug, Serialize, Deserialize, Clone, Hash, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum MangaStatus {
    #[serde(rename = "anons")]
    Announce,
    Ongoing,
    Released,
    Paused,
    Discontinued,
}

#[derive(Deserialize, Debug, Clone)]
pub struct MangaCover {
    original: String,
    preview: String,
    x48: String,
    x96: String,
}
