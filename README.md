# Shikimori Rust

<a href="https://crates.io/crates/shikimori"><img src="https://img.shields.io/crates/v/shikimori?style=flat-square&logo=rust" alt="Crate version"></a>
<a href="https://github.com/negezor/shikimori-rust/actions/workflows/main.yml"><img src="https://img.shields.io/github/actions/workflow/status/negezor/shikimori-rust/main.yml?style=flat-square&logo=github&label=Tests" alt="Tests"></a>
<a href="https://github.com/negezor/shikimori-rust/blob/main/LICENSE"><img src="https://img.shields.io/badge/license-MIT-informational?style=flat-square" alt="License"></a>

> **Shikimori Rust** - An efficient Rust library serving as a wrapper for the Shikimori API 🦾

| 📖 [Documentation](https://docs.rs/shikimori)  |
| ------------------------------------------ |

## Installation

Install [shikimori from crates.io](https://crates.io/crates/shikimori). Add the following line to your `Cargo.toml` file's dependencies section:

```toml
shikimori = "0.4"
```

Or you can add with cargo

```sh
cargo add shikimori
```

## Usage

You should also use [querygen](https://generator.cynic-rs.dev) to simplify your life, it is available at the link. Get the schema at URL `https://shikimori.one/api/graphql`

```rs
use chrono::{DateTime, Utc};

use shikimori::client::ClientBuilder;
use shikimori::cynic::QueryBuilder;

use shikimori::graphql::anime::AnimeKind;
use shikimori::graphql::types::EntityOrder;
use shikimori::graphql::scalars::AnimeStatusString;
use shikimori::graphql::schema;

#[derive(cynic::QueryVariables, Debug)]
pub struct AnimesQueryVariables {
    pub page: i32,
    pub status: AnimeStatusString,
    pub order: EntityOrder,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(graphql_type = "Query", variables = "AnimesQueryVariables")]
pub struct AnimesQuery {
    #[arguments(censored: false, page: $page, status: $status, order: $order)]
    pub animes: Vec<Anime>,
}

#[derive(cynic::QueryFragment, Debug)]
pub struct Anime {
    pub id: cynic::Id,
    pub franchise: Option<String>,
    pub episodes: i32,
    pub kind: Option<AnimeKind>,
    pub next_episode_at: Option<DateTime<Utc>>,
    pub url: String,
}

#[tokio::main]
async fn main() {
    let client = ClientBuilder::new().build();

    let response = client
        .query(AnimesQuery::build(AnimesQueryVariables {
            page: 1,
            status: StatusString::new("ongoing"),
            order: EntityOrder::Popularity,
        }))
        .await;

    dbg!(&response);
}

// Ok(
//     GraphQlResponse {
//         data: Some(
//             AnimesQuery {
//                 animes: [
//                     Anime {
//                         id: Id(
//                             "21",
//                         ),
//                         franchise: Some(
//                             "one_piece",
//                         ),
//                         episodes: 0,
//                         kind: Some(
//                             Tv,
//                         ),
//                         next_episode_at: Some(
//                             2023-09-10T00:30:00Z,
//                         ),
//                         url: "https://shikimori.one/animes/21-one-piece",
//                     },
//                     Anime {
//                         id: Id(
//                             "51009",
//                         ),
//                         franchise: Some(
//                             "jujutsu_kaisen",
//                         ),
//                         episodes: 23,
//                         kind: Some(
//                             Tv,
//                         ),
//                         next_episode_at: Some(
//                             2023-09-07T14:56:00Z,
//                         ),
//                         url: "https://shikimori.one/animes/51009-jujutsu-kaisen-2nd-season",
//                     },
//                 ],
//             },
//         ),
//         errors: None,
//     },
// )
```
