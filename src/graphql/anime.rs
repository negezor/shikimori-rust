use super::schema;

#[derive(cynic::Enum, Clone, Hash, PartialEq, Eq, Debug)]
#[cynic(graphql_type = "AnimeKindEnum", rename_all = "snake_case")]
pub enum AnimeKind {
    Tv,
    TvSpecial,
    Movie,
    Ova,
    Ona,
    Special,
    Music,
    Pv,
    Cm,
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

#[derive(cynic::Enum, Clone, Hash, PartialEq, Eq, Debug)]
#[cynic(graphql_type = "AnimeStatusEnum", rename_all = "snake_case")]
pub enum AnimeStatus {
    #[cynic(rename = "anons")]
    Announce,
    Ongoing,
    Released,
}
