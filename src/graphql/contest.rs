use super::schema;

#[derive(cynic::Enum, Clone, Hash, PartialEq, Eq, Debug)]
#[cynic(graphql_type = "ContestMatchStateEnum", rename_all = "snake_case")]
pub enum ContestMatchState {
    Created,
    Started,
    Finished,
}

#[derive(cynic::Enum, Clone, Hash, PartialEq, Eq, Debug)]
#[cynic(graphql_type = "ContestMemberTypeEnum", rename_all = "snake_case")]
pub enum ContestMemberType {
    Anime,
    Character,
}

#[derive(cynic::Enum, Clone, Hash, PartialEq, Eq, Debug)]
#[cynic(graphql_type = "ContestRoundStateEnum", rename_all = "snake_case")]
pub enum ContestRoundState {
    Created,
    Started,
    Finished,
}

#[derive(cynic::Enum, Clone, Hash, PartialEq, Eq, Debug)]
#[cynic(graphql_type = "ContestStateEnum", rename_all = "snake_case")]
pub enum ContestState {
    Created,
    Proposing,
    Started,
    Finished,
}

#[derive(cynic::Enum, Clone, Hash, PartialEq, Eq, Debug)]
#[cynic(graphql_type = "ContestStrategyTypeEnum", rename_all = "snake_case")]
pub enum ContestStrategyType {
    DoubleElimination,
    PlayOff,
    Swiss,
}
