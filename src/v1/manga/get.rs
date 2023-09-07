use std::marker::PhantomData;

use reqwest::{Method, StatusCode};
use serde::{Deserialize, Serialize};
use time::Date;

use crate::{
    client::Client,
    error::Error,
    types::InternalResponseUnion,
    utils::{empty_string_as_none, parse_f64_from_str, remove_nulls_and_empty_strings},
    v1::{genres::types::Genre, studios::types::PartialStudio},
    v2::user_rates::types::{PartialUserRate, UserRateScoresStats, UserRateStatusesStats},
};

use super::types::{MangaCover, MangaKind, MangaStatus};

#[derive(Deserialize, Debug, Clone)]
pub struct MangaGetResponse {
    id: u64,
    name: String,
    #[serde(deserialize_with = "empty_string_as_none")]
    russian: Option<String>,
    image: MangaCover,
    url: String,
    kind: MangaKind,
    #[serde(deserialize_with = "parse_f64_from_str")]
    score: f64,
    status: MangaStatus,
    volumes: u32,
    chapters: u32,
    aired_on: Option<Date>,
    released_on: Option<Date>,
    #[serde(deserialize_with = "remove_nulls_and_empty_strings")]
    synonyms: Vec<String>,
    #[serde(deserialize_with = "remove_nulls_and_empty_strings")]
    english: Vec<String>,
    #[serde(deserialize_with = "remove_nulls_and_empty_strings")]
    japanese: Vec<String>,
    #[serde(deserialize_with = "empty_string_as_none")]
    license_name_ru: Option<String>,
    #[serde(deserialize_with = "empty_string_as_none")]
    description: Option<String>,
    description_html: Option<String>,
    description_source: Option<String>,
    #[serde(deserialize_with = "empty_string_as_none")]
    franchise: Option<String>,
    anons: bool,
    favoured: bool,
    ongoing: bool,
    thread_id: Option<u64>,
    topic_id: Option<u64>,
    myanimelist_id: u64,
    genres: Vec<Genre>,
    #[serde(deserialize_with = "remove_nulls_and_empty_strings")]
    licensors: Vec<String>,
    publishers: Vec<PartialStudio>,
    rates_scores_stats: Vec<UserRateScoresStats>,
    rates_statuses_stats: Vec<UserRateStatusesStats>,
    user_rate: Option<PartialUserRate>,
}

type MangaGetResponseUnion = InternalResponseUnion<MangaGetResponse>;

#[derive(Debug, Serialize, Clone)]
#[non_exhaustive]
pub struct MangaGetQuery<'a> {
    slug: String,
    _phantom: PhantomData<&'a u8>,
}

impl<'a> MangaGetQuery<'a> {
    pub fn new(slug: impl ToString) -> MangaGetQuery<'a> {
        MangaGetQuery {
            slug: slug.to_string(),
            _phantom: PhantomData,
        }
    }

    /// Execute the query and fetch the results.
    pub async fn execute<'b>(
        &'a self,
        client: &'b Client,
    ) -> Result<Option<MangaGetResponse>, Error> {
        let request_builder = client.init_request(Method::GET, &format!("/mangas/{}", self.slug));

        let request = request_builder.build().expect("failed to build request");

        let response = match client.send(request).await {
            Ok(response) => response,
            Err(err) => {
                if err.is::<reqwest::Error>() {
                    return err
                        .downcast::<reqwest::Error>()
                        .map_err(Error::BoxError)
                        .and_then(|error| Err(Error::HttpError(*error)));
                }

                return Err(Error::BoxError(err))?;
            }
        };

        if response.status() == StatusCode::NOT_FOUND {
            return Ok(None);
        }

        let result = response
            .json::<MangaGetResponseUnion>()
            .await
            .map_err(Error::HttpError)?;

        result.simplify_result().map(Some)
    }
}

#[cfg(test)]
mod tests {
    use crate::client::ClientBuilder;

    use super::*;

    #[tokio::test]
    async fn testttt() {
        let client = ClientBuilder::new().build();

        let response = MangaGetQuery::new(8908).execute(&client).await;

        dbg!(response);
    }
}
