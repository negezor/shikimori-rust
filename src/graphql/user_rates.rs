use super::schema;

#[derive(cynic::Enum, Clone, Hash, PartialEq, Eq, Debug)]
#[cynic(graphql_type = "UserRateTargetTypeEnum", rename_all = "PascalCase")]
pub enum UserRateTargetType {
    Anime,
    Manga,
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
