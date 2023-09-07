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
