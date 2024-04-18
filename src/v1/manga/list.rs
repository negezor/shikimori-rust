use async_fn_stream::try_fn_stream;
use futures_util::{pin_mut, Stream, StreamExt};
use reqwest::Method;
use serde::{Deserialize, Serialize};
use time::Date;

use crate::{
    client::Client,
    error::Error,
    types::InternalResponseUnion,
    utils::{empty_string_as_none, parse_f64_from_str},
    v2::user_rates::types::UserRateStatus,
};

use super::types::{MangaCover, MangaKind, MangaStatus};

#[derive(Deserialize, Debug, Clone)]
pub struct MangaListResponse {
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
}

type MangaListResponseUnion = InternalResponseUnion<Vec<MangaListResponse>>;

#[derive(Debug, Serialize, Deserialize, Clone, Hash, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum MangaListOrder {
    Id,
    Ranked,
    Kind,
    Popularity,
    Name,
    AiredOn,
    Volumes,
    Chapters,
    Status,
    Random,
}

#[derive(Debug, Serialize, Clone)]
#[non_exhaustive]
pub struct MangaListQuery<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    page: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<u16>,

    #[serde(skip_serializing_if = "Option::is_none")]
    order: Option<MangaListOrder>,

    #[serde(skip_serializing_if = "Option::is_none")]
    kind: Option<MangaKind>,
    #[serde(skip_serializing_if = "Option::is_none")]
    status: Option<MangaStatus>,

    #[serde(skip_serializing_if = "Option::is_none")]
    season: Option<&'a [&'a str]>,

    #[serde(skip_serializing_if = "Option::is_none")]
    score: Option<f32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    genre: Option<&'a [u64]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    publisher: Option<&'a [u64]>,

    #[serde(skip_serializing_if = "Option::is_none")]
    ids: Option<&'a [u64]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    exclude_ids: Option<&'a [u64]>,

    #[serde(skip_serializing_if = "Option::is_none")]
    franchise: Option<&'a [&'a str]>,

    #[serde(skip_serializing_if = "Option::is_none")]
    censored: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    mylist: Option<UserRateStatus>,

    #[serde(skip_serializing_if = "Option::is_none")]
    search: Option<&'a str>,
}

impl<'a> MangaListQuery<'a> {
    pub fn new() -> MangaListQuery<'a> {
        MangaListQuery {
            page: None,
            limit: None,
            order: None,
            kind: None,
            status: None,
            season: None,
            score: None,
            genre: None,
            publisher: None,
            ids: None,
            exclude_ids: None,
            franchise: None,
            censored: None,
            mylist: None,
            search: None,
        }
    }

    pub fn with_page<'b>(&'b mut self, page: u16) -> &'b mut MangaListQuery<'a> {
        self.page = Some(page);
        self
    }

    pub fn with_limit<'b>(&'b mut self, limit: u16) -> &'b mut MangaListQuery<'a> {
        self.limit = Some(limit);
        self
    }

    pub fn with_order<'b>(&'b mut self, order: MangaListOrder) -> &'b mut MangaListQuery<'a> {
        self.order = Some(order);
        self
    }

    pub fn with_kind<'b>(&'b mut self, kind: MangaKind) -> &'b mut MangaListQuery<'a> {
        self.kind = Some(kind);
        self
    }

    pub fn with_status<'b>(&'b mut self, status: MangaStatus) -> &'b mut MangaListQuery<'a> {
        self.status = Some(status);
        self
    }

    pub fn season<'b>(&'b mut self, season: &'a [&'a str]) -> &'b mut MangaListQuery<'a> {
        self.season = Some(season);
        self
    }

    pub fn score<'b>(&'b mut self, score: f32) -> &'b mut MangaListQuery<'a> {
        self.score = Some(score);
        self
    }

    pub fn genre<'b>(&'b mut self, genre: &'a [u64]) -> &'b mut MangaListQuery<'a> {
        self.genre = Some(genre);
        self
    }

    pub fn publisher<'b>(&'b mut self, publisher: &'a [u64]) -> &'b mut MangaListQuery<'a> {
        self.publisher = Some(publisher);
        self
    }

    pub fn ids<'b>(&'b mut self, ids: &'a [u64]) -> &'b mut MangaListQuery<'a> {
        self.ids = Some(ids);
        self
    }

    pub fn exclude_ids<'b>(&'b mut self, exclude_ids: &'a [u64]) -> &'b mut MangaListQuery<'a> {
        self.exclude_ids = Some(exclude_ids);
        self
    }

    pub fn franchise<'b>(&'b mut self, franchise: &'a [&'a str]) -> &'b mut MangaListQuery<'a> {
        self.franchise = Some(franchise);
        self
    }

    pub fn censored<'b>(&'b mut self, censored: bool) -> &'b mut MangaListQuery<'a> {
        self.censored = Some(censored);
        self
    }

    pub fn mylist<'b>(&'b mut self, mylist: UserRateStatus) -> &'b mut MangaListQuery<'a> {
        self.mylist = Some(mylist);
        self
    }

    pub fn search<'b>(&'b mut self, search: &'a str) -> &'b mut MangaListQuery<'a> {
        self.search = Some(search);
        self
    }

    /// Execute the query and fetch the results.
    pub async fn execute<'b>(
        &'a self,
        client: &'b Client,
    ) -> Result<Vec<MangaListResponse>, Error> {
        let stream = self.stream(client);

        pin_mut!(stream);

        stream.next().await.ok_or_else(|| Error::ShikimoriError {
            message: "Empty response".to_owned(),
            code: None,
        })?
    }

    /// Stream the query
    pub fn stream(
        &self,
        client: &Client,
    ) -> impl Stream<Item = Result<Vec<MangaListResponse>, Error>> {
        let client = client.clone();
        let query =
            comma_serde_urlencoded::to_string(self).map_err(Error::UrlencodedSerializeError);

        let default_page = self.page.unwrap_or(1);

        try_fn_stream(|emitter| async move {
            let query = query?;

            let mut page = default_page;
            let mut result_per_page = 0;
            let mut has_next_page = true;

            loop {
                let request_builder = client
                    .init_request(Method::GET, &format!("/mangas?{}", query))
                    .query(&[("page", page)]);

                let request = request_builder.build().expect("failed to build request");

                let response = match client.send(request).await {
                    Ok(response) => response,
                    Err(err) => {
                        if err.is::<reqwest::Error>() {
                            err.downcast::<reqwest::Error>()
                                .map_err(Error::BoxError)
                                .and_then(|error| Err(Error::HttpError(*error)))?;

                            continue;
                        }

                        Err(Error::BoxError(err))?;

                        continue;
                    }
                };

                let result = response
                    .json::<MangaListResponseUnion>()
                    .await
                    .map_err(Error::HttpError)?;

                match result {
                    MangaListResponseUnion::Result(result) => {
                        if result_per_page == 0 && page == 1 {
                            result_per_page = result.len();
                        }

                        has_next_page = result_per_page == result.len() && !result.is_empty();

                        page += 1;

                        emitter.emit(result).await;
                    }
                    MangaListResponseUnion::Error { message, code } => {
                        Err(Error::ShikimoriError { message, code })?
                    }
                    MangaListResponseUnion::SystemError(mut errors) => {
                        Err(Error::ShikimoriError {
                            message: errors.remove(0),
                            code: None,
                        })?
                    }
                };

                if !has_next_page {
                    break;
                }
            }

            Ok(())
        })
    }
}

impl<'a> Default for MangaListQuery<'a> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use crate::client::ClientBuilder;

    use super::*;

    #[tokio::test]
    async fn testttt() {
        let client = ClientBuilder::new().build();

        let stream = MangaListQuery::new()
            .with_order(MangaListOrder::Id)
            .with_limit(100)
            .stream(&client);

        pin_mut!(stream);

        while let Some(response) = stream.next().await {
            match response {
                Ok(result) => {
                    dbg!(result.len());
                }
                Err(err) => {
                    panic!("error {:?}", err);
                }
            }
        }
    }
}
