use crate::ir::profile::Tokens;
use crate::vec_map::VecMap;

use m_lexer::{Lexer as Lexer0, LexerBuilder, TokenKind};

use super::lexed;

#[allow(dead_code)]
pub struct Lexer {
    pub(crate) names: VecMap<String>,
    pub(crate) colors: VecMap<String>,
    pub(crate) lexer: Lexer0,
}

impl Lexer {
    pub(crate) fn new(tokens: Tokens) -> Self {
        let mut builder = LexerBuilder::new();

        let rules = tokens.len() + 1;

        let mut names = VecMap::with_capacity(rules);
        let mut colors = VecMap::with_capacity(rules);

        for rule in tokens {
            let kind = ensure_equal(names.insert(rule.name), colors.insert(rule.color));

            builder = builder.token(kind, &rule.regex);
        }

        let error_token = ensure_equal(
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

fn ensure_equal<T: Eq>(a: T, b: T) -> T {
    debug_assert!(a == b);
    a
}
