use super::schema;

#[derive(cynic::Enum, Clone, Hash, PartialEq, Eq, Debug)]
#[cynic(graphql_type = "OrderEnum", rename_all = "snake_case")]
pub enum EntityOrder {
    /// By ID
    Id,
    /// id_desc
    IdDesc,
    /// By rank
    Ranked,
    /// By type
    Kind,
    /// By popularity
    Popularity,
    /// In alphabetical order
    Name,
    /// By release date
    AiredOn,
    /// By number of episodes
    Episodes,
    /// By status
    Status,
    /// By random
    Random,
    /// By random
    RankedRandom,
    /// By Shikimori ranking
    RankedShiki,
    /// created_at
    CreatedAt,
    /// created_at_desc
    CreatedAtDesc,
}

#[derive(cynic::Enum, Clone, Hash, PartialEq, Eq, Debug)]
#[cynic(graphql_type = "SortOrderEnum", rename_all = "snake_case")]
pub enum EntitySort {
    Asc,
    Desc,
}

#[derive(cynic::Enum, Clone, Hash, PartialEq, Eq, Debug)]
#[cynic(graphql_type = "RelationKindEnum", rename_all = "snake_case")]
pub enum RelationKind {
    /// Adaptation
    Adaptation,

    /// Alternative Setting
    AlternativeSetting,

    /// Alternative Version
    AlternativeVersion,

    /// Character
    Character,

    /// Full Story
    FullStory,

    /// Parent Story
    ParentStory,

    /// Prequel
    Prequel,

    /// Sequel
    Sequel,

    /// Side Story
    SideStory,

    /// Spin-off
    SpinOff,

    /// Summary
    Summary,

    /// Other
    Other,
}
