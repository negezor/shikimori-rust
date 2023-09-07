use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct PartialStudio {
    id: u64,
    name: String,
}
