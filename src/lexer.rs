use fancy_regex::Regex;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct TokenKind(pub u16);

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub(crate) struct Token {
    pub(crate) kind: TokenKind,
    pub(crate) len: usize,
}

pub(crate) struct Lexer {
    error_token: TokenKind,
    rules: Vec<Rule>,
}

pub(crate) struct Rule {
    kind: TokenKind,
    regex: Regex,
}

#[derive(Default)]
pub(crate) struct LexerBuilder {
    error_token: Option<TokenKind>,
    rules: Vec<Rule>,
}

impl Lexer {
    pub(crate) fn next_token(&self, input: &str) -> Token {
        self.valid_token(input)
            .unwrap_or_else(|| self.invalid_token(input))
    }

    pub(crate) fn tokenize(&self, input: &str) -> Vec<Token> {
        let mut tokens = Vec::new();
        let mut remaining_input = input;

        while !remaining_input.is_empty() {
            let token = self.next_token(remaining_input);
            tokens.push(token);

            remaining_input = &remaining_input[token.len..];
        }

        tokens
    }

    fn valid_token(&self, input: &str) -> Option<Token> {
        let mut longest_match: Option<(usize, &Rule)> = None;

        for rule in self.rules.iter().rev() {
            if let Some(matches) = rule.regex.find(input).ok()? {
                let len = matches.end();
                longest_match = Some((len, rule));
                break;
            }
        }

        longest_match.map(|(len, rule)| Token {
            kind: rule.kind,
            len,
        })
    }

    fn invalid_token(&self, input: &str) -> Token {
        let mut len = 0;

        for ch in input.chars() {
            len += ch.len_utf8();

            if self.valid_token(&input[len..]).is_some() {
                break;
            }
        }

        Token {
            kind: self.error_token,
            len,
        }
    }
}

impl LexerBuilder {
    pub(crate) fn token(mut self, kind: TokenKind, re: &str) -> Self {
        let rule = Rule {
            kind,
            regex: parse_regex(re),
        };
        self.rules.push(rule);
        self
    }

    pub(crate) fn error_token(mut self, kind: TokenKind) -> Self {
        self.error_token = Some(kind);
        self
    }

    pub(crate) fn build(self) -> Lexer {
        Lexer {
            error_token: self.error_token.expect("Error: `error_token` not set."),
            rules: self.rules,
        }
    }
}

fn parse_regex(re: &str) -> Regex {
    let combined_regex = format!("^({})", re);
    Regex::new(&combined_regex).unwrap()
}
