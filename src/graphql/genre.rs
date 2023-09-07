use super::schema;

#[derive(cynic::Enum, Clone, Hash, PartialEq, Eq, Debug)]
#[cynic(graphql_type = "GenreEntryTypeEnum", rename_all = "PascalCase")]
pub enum GenreEntryType {
    Anime,
    Manga,
}

#[derive(cynic::Enum, Clone, Hash, PartialEq, Eq, Debug)]
#[cynic(graphql_type = "GenreKindEnum", rename_all = "snake_case")]
pub enum GenreKind {
    Genre,
    Demographic,
    Theme,
}
