use serde::de::{self, Deserialize, Deserializer, IntoDeserializer};

pub fn parse_f64_from_str<'de, D>(deserializer: D) -> Result<f64, D::Error>
where
    D: Deserializer<'de>,
{
    let s: &str = Deserialize::deserialize(deserializer)?;
    let parsed = s.parse::<f64>().map_err(de::Error::custom)?;
    Ok(parsed)
}

pub fn empty_string_as_none<'de, D, T>(de: D) -> Result<Option<T>, D::Error>
where
    D: serde::Deserializer<'de>,
    T: serde::Deserialize<'de>,
{
    let opt = Option::<String>::deserialize(de)?;
    let opt = opt.as_deref();
    match opt {
        None | Some("") => Ok(None),
        Some(s) => T::deserialize(s.into_deserializer()).map(Some),
    }
}

pub fn remove_nulls_and_empty_strings<'de, D>(de: D) -> Result<Vec<String>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let values: Vec<Option<String>> = Deserialize::deserialize(de)?;

    Ok(values
        .into_iter()
        .filter_map(|val| {
            let Some(s) = val else {
                return None;
            };

            if !s.is_empty() {
                return Some(s);
            }

            None
        })
        .collect())
}
