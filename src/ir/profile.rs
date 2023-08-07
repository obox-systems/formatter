use serde::{Deserialize, Serialize};

pub type Tokens = Vec<Token>;

/// Represents a Profile containing tokens and rules.
#[derive(Debug, Deserialize, Serialize)]
pub struct Profile {
    /// The tokens associated with this profile.
    pub tokens: Tokens,
    /// The rules associated with this profile.
    pub rules: Vec<Rule>,
}

/// Represents a Token with its name, color, and regex pattern.
#[derive(Debug, Deserialize, Serialize)]
pub struct Token {
    /// The name of the token.
    pub(crate) name: String,
    /// The color associated with the token.
    pub(crate) color: String,
    /// The regular expression pattern that matches this token.
    pub(crate) regex: String,
}

/// Represents a Rule that defines an action to be taken before and after a specific event.
#[derive(Debug, Deserialize, Serialize)]
pub struct Rule {
    /// The condition or event that occurs before the action is triggered.
    pub(crate) before: String,
    /// The action to be taken when the specified condition or event is met.
    pub(crate) action: String,
    /// The condition or event that occurs after the action is triggered.
    pub(crate) after: String,
}
