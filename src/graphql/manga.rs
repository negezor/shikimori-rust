use super::schema;

#[derive(cynic::Enum, Clone, Hash, PartialEq, Eq, Debug)]
#[cynic(graphql_type = "MangaKindEnum", rename_all = "snake_case")]
pub enum MangaKind {
    Manga,
    Manhwa,
    Manhua,
    LightNovel,
    Novel,
    OneShot,
    Doujin,
}

#[derive(cynic::Enum, Clone, Hash, PartialEq, Eq, Debug)]
#[cynic(graphql_type = "MangaStatusEnum", rename_all = "snake_case")]
pub enum MangaStatus {
    #[cynic(rename = "anons")]
    Announce,
    Ongoing,
    Released,
    Paused,
    Discontinued,
}
