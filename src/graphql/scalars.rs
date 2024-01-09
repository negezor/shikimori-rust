use super::schema;

#[derive(cynic::Scalar, Debug)]
pub struct AnimeKindString(String);

impl AnimeKindString {
    pub fn new(s: impl Into<String>) -> Self {
        Self(s.into())
    }

    /// Returns a reference to the value of this `AnimeKindString`
    pub fn inner(&self) -> &str {
        &self.0
    }

    /// Converts this `AnimeKindString` into its inner value
    pub fn into_inner(self) -> String {
        self.0
    }
}

#[derive(cynic::Scalar, Debug)]
pub struct DurationString(String);

impl DurationString {
    pub fn new(s: impl Into<String>) -> Self {
        Self(s.into())
    }

    /// Returns a reference to the value of this `DurationString`
    pub fn inner(&self) -> &str {
        &self.0
    }

    /// Converts this `DurationString` into its inner value
    pub fn into_inner(self) -> String {
        self.0
    }
}

#[derive(cynic::Scalar, Debug)]
pub struct MangaKindString(String);

impl MangaKindString {
    pub fn new(s: impl Into<String>) -> Self {
        Self(s.into())
    }

    /// Returns a reference to the value of this `MangaKindString`
    pub fn inner(&self) -> &str {
        &self.0
    }

    /// Converts this `MangaKindString` into its inner value
    pub fn into_inner(self) -> String {
        self.0
    }
}

#[derive(cynic::Scalar, Debug)]
pub struct MylistString(String);

impl MylistString {
    pub fn new(s: impl Into<String>) -> Self {
        Self(s.into())
    }

    /// Returns a reference to the value of this `MylistString`
    pub fn inner(&self) -> &str {
        &self.0
    }

    /// Converts this `MylistString` into its inner value
    pub fn into_inner(self) -> String {
        self.0
    }
}

#[derive(cynic::Scalar, Debug)]
pub struct RatingString(String);

impl RatingString {
    pub fn new(s: impl Into<String>) -> Self {
        Self(s.into())
    }

    /// Returns a reference to the value of this `RatingString`
    pub fn inner(&self) -> &str {
        &self.0
    }

    /// Converts this `RatingString` into its inner value
    pub fn into_inner(self) -> String {
        self.0
    }
}

#[derive(cynic::Scalar, Debug)]
pub struct SeasonString(String);

impl SeasonString {
    pub fn new(s: impl Into<String>) -> Self {
        Self(s.into())
    }

    /// Returns a reference to the value of this `SeasonString`
    pub fn inner(&self) -> &str {
        &self.0
    }

    /// Converts this `SeasonString` into its inner value
    pub fn into_inner(self) -> String {
        self.0
    }
}

#[derive(cynic::Scalar, Debug)]
pub struct AnimeStatusString(String);

impl AnimeStatusString {
    pub fn new(s: impl Into<String>) -> Self {
        Self(s.into())
    }

    /// Returns a reference to the value of this `AnimeStatusString`
    pub fn inner(&self) -> &str {
        &self.0
    }

    /// Converts this `AnimeStatusString` into its inner value
    pub fn into_inner(self) -> String {
        self.0
    }
}

#[derive(cynic::Scalar, Debug)]
pub struct MangaStatusString(String);

impl MangaStatusString {
    pub fn new(s: impl Into<String>) -> Self {
        Self(s.into())
    }

    /// Returns a reference to the value of this `MangaStatusString`
    pub fn inner(&self) -> &str {
        &self.0
    }

    /// Converts this `MangaStatusString` into its inner value
    pub fn into_inner(self) -> String {
        self.0
    }
}
