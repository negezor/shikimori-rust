use std::time::Duration;

use reqwest::{
    Client as ReqwestClient, ClientBuilder as ReqwestClientBuilder, Method, Proxy, Request,
    RequestBuilder, Response,
};
use tower::{
    buffer::Buffer,
    limit::{ConcurrencyLimit, RateLimit},
    BoxError, Service, ServiceExt,
};

use cynic::{GraphQlResponse, Operation};

use crate::error::Error;

static CRATE_USER_AGENT: &str = concat!(
    env!("CARGO_PKG_NAME"),
    "/",
    env!("CARGO_PKG_VERSION"),
    " (+https://github.com/negezor/shikimori-rust)",
);

#[derive(Debug)]
pub struct ClientBuilder {
    api_key: Option<String>,
    api_url: String,
    reqwest_client_builder: ReqwestClientBuilder,
}

impl ClientBuilder {
    /// Constructs a new `ClientBuilder`
    pub fn new() -> ClientBuilder {
        ClientBuilder {
            api_key: None,
            api_url: "https://shikimori.me/api".to_owned(),
            reqwest_client_builder: ReqwestClientBuilder::new().user_agent(CRATE_USER_AGENT),
        }
    }

    /// API key (token) for Shikimori API
    ///
    /// ```
    /// use shikimori::ClientBuilder;
    ///
    /// ClientBuilder::new()
    ///   .api_key("q8p5vnf9crt7xfyzke4iwc6r5rvsurv7");
    /// ```
    pub fn api_key(mut self, api_key: impl Into<String>) -> ClientBuilder {
        self.api_key = Some(api_key.into());
        self
    }

    /// Base URL for Shikimori API
    ///
    /// Default: `https://shikimori.me/api`
    ///
    /// ```
    /// use shikimori::ClientBuilder;
    ///
    /// ClientBuilder::new()
    ///   .api_url("https://shikimori.rs/api");
    /// ```
    pub fn api_url(mut self, api_url: impl Into<String>) -> ClientBuilder {
        self.api_url = api_url.into();
        self
    }

    /// ```
    /// use shikimori::ClientBuilder;
    ///
    /// ClientBuilder::new()
    ///   .proxy(reqwest::Proxy::http("https://my.prox").unwrap());
    /// ```
    pub fn proxy(mut self, proxy: Proxy) -> ClientBuilder {
        self.reqwest_client_builder = self.reqwest_client_builder.proxy(proxy);
        self
    }

    /// ```
    /// use shikimori::ClientBuilder;
    ///
    /// ClientBuilder::new()
    ///   .custom_reqwest_builder(reqwest::ClientBuilder::new());
    /// ```
    pub fn custom_reqwest_builder(mut self, builder: ReqwestClientBuilder) -> ClientBuilder {
        self.reqwest_client_builder = builder;
        self
    }

    /// ```
    /// use shikimori::ClientBuilder;
    ///
    /// ClientBuilder::new()
    ///   .user_agent("");
    /// ```
    pub fn user_agent(mut self, builder: impl Into<String>) -> ClientBuilder {
        self.reqwest_client_builder = self.reqwest_client_builder.user_agent(builder.into());
        self
    }

    // TODO: Add handle errors
    /// # Panic
    /// If api_key is not set and if it was not possible to build http client
    ///
    /// ```
    /// use shikimori::ClientBuilder;
    ///
    /// ClientBuilder::new().api_key("q8p5vnf9crt7xfyzke4iwc6r5rvsurv7").build();
    /// ```
    pub fn build(self) -> Client {
        let http_client = self
            .reqwest_client_builder
            .build()
            .expect("failed to build reqwest client");

        Client {
            api_key: self.api_key,
            api_url: self.api_url,
            http_client: http_client.clone(),
            http_service: tower::ServiceBuilder::new()
                .buffer(10)
                .concurrency_limit(5)
                // 90rpm, here are small compensation network costs
                .rate_limit(90, Duration::from_millis(92000))
                // 5rps, here are small compensation network costs
                .rate_limit(5, Duration::from_millis(1110))
                .service(http_client),
        }
    }
}

impl Default for ClientBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// The top-level struct of the SDK, representing a client
#[derive(Debug, Clone)]
pub struct Client {
    api_key: Option<String>,
    api_url: String,
    http_client: ReqwestClient,
    http_service: Buffer<ConcurrencyLimit<RateLimit<RateLimit<ReqwestClient>>>, Request>,
}

impl Client {
    /// Create a client
    ///
    /// # Example
    ///
    /// ```
    /// # use shikimori::Client;
    ///
    /// let api_key = std::env::var("SHIKIMORI_API_KEY").expect("SHIKIMORI_API_KEY is not set");
    ///
    /// let client = Client::new(api_key);
    /// ```
    pub fn new(api_key: impl Into<String>) -> Client {
        ClientBuilder::new().api_key(api_key).build()
    }

    pub(crate) fn init_request(&self, method: Method, path: &str) -> RequestBuilder {
        let mut request: RequestBuilder = self
            .http_client
            .request(method, self.api_url.clone() + path);

        if let Some(api_key) = &self.api_key {
            request = request.bearer_auth(api_key);
        }

        request
    }

    pub(crate) async fn send(&self, request: Request) -> Result<Response, BoxError> {
        let mut svc = self.http_service.clone();

        svc.ready().await?;

        svc.call(request).await
    }

    pub async fn query<
        Vars: serde::Serialize,
        ResponseData: serde::de::DeserializeOwned + 'static,
    >(
        &self,
        operation: Operation<ResponseData, Vars>,
    ) -> Result<GraphQlResponse<ResponseData>, Error> {
        let request = self
            .init_request(Method::POST, "/graphql")
            .json(&operation)
            .build()
            .expect("failed to build request");

        let response = match self.send(request).await {
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

        let result = response
            .json::<GraphQlResponse<ResponseData>>()
            .await
            .map_err(Error::HttpError)?;

        Ok(result)
    }
}
