use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct Rule {
    pub(crate) name: String,
    pub(crate) color: String,
    pub(crate) regex: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct Profile {
    pub(crate) rules: Vec<Rule>,
}
