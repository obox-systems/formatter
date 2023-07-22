use std::{collections::HashMap, iter::zip};

use m_lexer::{Lexer, LexerBuilder, Token, TokenKind};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
struct Rule {
    name: String,
    color: String,
    regex: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct Profile {
    rules: Vec<Rule>,
}

#[allow(dead_code)]
struct World {
    names: HashMap<u16, String>,
    colors: HashMap<u16, String>,
    lexer: Lexer,
}

impl World {
    fn new(profile: Profile) -> Self {
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

        let lexer = builder.error_token(TokenKind(error_token)).build();

        Self {
            names,
            colors,
            lexer,
        }
    }

    fn color(&self, token: TokenKind) -> &str {
        &self.colors[&token.0]
    }

    fn tokenize<'input>(&self, input: &'input str) -> Lexed<'input> {
        Lexed {
            input,
            tokens: self.lexer.tokenize(input),
        }
    }
}

struct Lexed<'input> {
    input: &'input str,
    tokens: Vec<Token>,
}

impl<'input> Lexed<'input> {
    fn reader(self) -> LexerReader<'input> {
        LexerReader {
            pos: 0,
            offset: 0,
            last_offset: 0,
            lexed: self,
        }
    }
}

struct LexerReader<'input> {
    pos: usize,
    last_offset: usize,
    offset: usize,
    lexed: Lexed<'input>,
}

impl<'input> LexerReader<'input> {
    fn next(&mut self) -> Option<&Token> {
        let token = self.lexed.tokens.get(self.pos);

        if let Some(token) = token {
            self.pos += 1;

            self.last_offset = self.offset;
            self.offset += token.len;
        }

        token
    }

    fn slice(&self) -> &str {
        &self.lexed.input[self.last_offset..self.offset]
    }
}

fn main() {
    use std::fmt::Write;

    let mut output = String::new();

    let profile = std::fs::read_to_string("rust.toml").unwrap();
    let profile: Profile = toml::from_str(&profile).unwrap();

    let state = World::new(profile);
    let mut reader = state.tokenize(" 23 23 2").reader();

    while let Some(token) = reader.next() {
        let color = state.color(token.kind);
        let slice = reader.slice();
        let slice = if slice.chars().all(|ch| ch.is_whitespace()) {
            "%20"
        } else {
            slice
        };

        writeln!(
            output,
            "![](https://img.shields.io/static/v1?label=&message={slice}&color={color})"
        )
        .unwrap();
    }

    std::fs::write("out.md", output).unwrap();
}
