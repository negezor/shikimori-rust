use super::schema;

#[derive(cynic::Enum, Clone, Hash, PartialEq, Eq, Debug)]
#[cynic(graphql_type = "AnimeKindEnum", rename_all = "snake_case")]
pub enum AnimeKind {
    Tv,
    Movie,
    Ova,
    Ona,
    Special,
    Music,
}

#[derive(cynic::Enum, Clone, Hash, PartialEq, Eq, Debug)]
#[cynic(graphql_type = "AnimeRatingEnum", rename_all = "snake_case")]
pub enum AnimeRating {
    None,
    G,
    Pg,
    #[cynic(rename = "pg_13")]
    Pg13,
    R,
    #[cynic(rename = "r_plus")]
    RPlus,
    Rx,
}
