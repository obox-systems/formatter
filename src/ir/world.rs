use super::lexed;
use super::Profile;
use m_lexer::Lexer;
use m_lexer::LexerBuilder;
use m_lexer::TokenKind;
use std::collections::HashMap;
use std::iter::zip;

#[allow(dead_code)]
pub(crate) struct World {
    /// TODO: HashMap -> VecMap
    pub(crate) names: HashMap<u16, String>,
    pub(crate) colors: HashMap<u16, String>,
    pub(crate) lexer: Lexer,
}

impl World {
    pub(crate) fn new(profile: Profile) -> Self {
        let mut builder = LexerBuilder::new();

        let rules = profile.rules.len() + 1;
        let error_token = rules as u16;

        let mut names = HashMap::with_capacity(rules);
        let mut colors = HashMap::with_capacity(rules);

        for (rule, raw_kind) in zip(profile.rules, 0u16..) {
            names.insert(raw_kind, rule.name);
            colors.insert(raw_kind, rule.color);

            builder = builder.token(TokenKind(raw_kind), &rule.regex);
        }

        names.insert(error_token, "Error".to_owned());
        colors.insert(error_token, "black".to_owned());

        let lexer = builder.error_token(TokenKind(error_token)).build();

        Self {
            names,
            colors,
            lexer,
        }
    }

    pub(crate) fn color(&self, token: TokenKind) -> &str {
        &self.colors[&token.0]
    }

    pub(crate) fn tokenize<'input>(&self, input: &'input str) -> lexed::Lexed<'input> {
        lexed::Lexed {
            input,
            tokens: self.lexer.tokenize(input),
        }
    }
}
