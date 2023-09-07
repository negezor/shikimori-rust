use super::schema;

#[derive(cynic::Enum, Clone, Hash, PartialEq, Eq, Debug)]
#[cynic(graphql_type = "ExternalLinkKindEnum", rename_all = "snake_case")]
pub enum ExternalLinkKind {
    /// Official Site
    OfficialSite,

    /// Wikipedia
    Wikipedia,

    /// Anime News Network
    AnimeNewsNetwork,

    /// MyAnimeList
    Myanimelist,

    /// AniDB
    AnimeDb,

    /// World Art
    WorldArt,

    /// KinoPoisk
    Kinopoisk,

    /// Kage Project
    KageProject,

    /// subs.com.ru
    SubsComRu,

    /// Twitter
    Twitter,

    /// Anime 365
    SmotretAnime,

    /// ReadManga
    Readmanga,

    /// MangaLib
    Mangalib,

    /// ReManga
    Remanga,

    /// Baka-Updates
    Mangaupdates,

    /// MangaDex
    Mangadex,

    /// MangaFox
    Mangafox,

    /// Mangachan
    Mangachan,

    /// Mangahub
    Mangahub,

    /// Novel.tl
    NovelTl,

    /// RuRanobe
    Ruranobe,

    /// RanobeLib
    Ranobelib,

    /// Novel Updates
    Novelupdates,
}
