use super::schema;

#[derive(cynic::Enum, Clone, Hash, PartialEq, Eq, Debug)]
#[cynic(graphql_type = "UserRateTargetTypeEnum", rename_all = "PascalCase")]
pub enum UserRateTargetType {
    Anime,
    Manga,
}

#[derive(cynic::Enum, Clone, Hash, PartialEq, Eq, Debug)]
#[cynic(graphql_type = "UserRateOrderFieldEnum", rename_all = "snake_case")]
pub enum UserRateOrderField {
    /// By id
    Id,

    /// By updated_at
    UpdatedAt,
}

#[derive(cynic::Enum, Clone, Hash, PartialEq, Eq, Debug)]
#[cynic(graphql_type = "VideoKindEnum", rename_all = "snake_case")]
enum VideoKind {
    Pv,
    CharacterTrailer,
    Cm,
    Op,
    Ed,
    OpEdClip,
    Clip,
    EpisodePreview,
    Other,
}
