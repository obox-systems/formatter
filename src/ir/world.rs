use crate::vec_map::VecMap;

use super::{lexed, Profile};
use m_lexer::{Lexer, LexerBuilder, TokenKind};

#[allow(dead_code)]
pub(crate) struct World {
    pub(crate) names: VecMap<String>,
    pub(crate) colors: VecMap<String>,
    pub(crate) lexer: Lexer,
}

impl World {
    pub(crate) fn new(profile: Profile) -> Self {
        let mut builder = LexerBuilder::new();

        let rules = profile.rules.len() + 1;

        let mut names = VecMap::with_capacity(rules);
        let mut colors = VecMap::with_capacity(rules);

        for rule in profile.rules {
            let kind = eq(names.insert(rule.name), colors.insert(rule.color));

            builder = builder.token(kind, &rule.regex);
        }

        let error_token = eq(
            names.insert("Error".to_owned()),
            colors.insert("black".to_owned()),
        );

        Self {
            names,
            colors,
            lexer: builder.error_token(error_token).build(),
        }
    }

    pub(crate) fn color(&self, token: TokenKind) -> &str {
        &self.colors[token]
    }

    pub(crate) fn tokenize<'input>(&self, input: &'input str) -> lexed::Tokens<'input> {
        lexed::Tokens {
            input,
            tokens: self.lexer.tokenize(input),
        }
    }
}

fn eq<T: Eq>(a: T, b: T) -> T {
    debug_assert!(a == b);
    a
}
