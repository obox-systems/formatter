use serde::{Deserialize, Serialize};

pub(crate) type Tokens = Vec<Token>;

#[derive(Debug, Deserialize, Serialize)]
pub struct Profile {
    pub(crate) tokens: Tokens,
    pub(crate) rules: Vec<Rule>,
}

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct Token {
    pub(crate) name: String,
    pub(crate) color: String,
    pub(crate) regex: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct Rule {
    before: String,
    action: String,
    after: String,
}
