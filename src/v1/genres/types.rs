use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub enum GenreEntryType {
    Anime,
    Manga,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum GenreKind {
    Genre,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Genre {
    id: u64,
    kind: GenreKind,
    name: String,
    russian: String,
    entry_type: GenreEntryType,
}
