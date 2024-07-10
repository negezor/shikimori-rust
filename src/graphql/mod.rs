use chrono::{DateTime, NaiveDate, NaiveDateTime, Utc};
use cynic::impl_scalar;

pub mod anime;

pub mod manga;

pub mod genre;

pub mod user_rates;

pub mod contest;

pub mod external_link;

pub mod scalars;

pub mod types;

#[cynic::schema("shikimori")]
pub mod schema {}

impl_scalar!(NaiveDate, schema::ISO8601Date);
impl_scalar!(NaiveDateTime, schema::ISO8601DateTime);
impl_scalar!(DateTime<Utc>, schema::ISO8601DateTime);
impl_scalar!(u32, schema::PositiveInt);

#[cfg(feature = "register-graphql-schema")]
pub fn register_schema() {
    cynic_codegen::register_schema("shikimori")
        .from_sdl(include_str!("../../schema.graphql"))
        .unwrap()
        .as_default()
        .unwrap();
}
