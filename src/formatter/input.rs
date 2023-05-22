use super::{classes, cursor::Cursor, Token};

#[derive(Default)]
pub(crate) struct Input<'me> {
    pub(crate) pos: usize,
    pub(crate) source: &'me str,
    pub(crate) tokens: Vec<Token>,
    pub(crate) start_offsets: Vec<u32>,
}

impl<'me> Input<'me> {
    pub(crate) fn of(source: &'me str) -> Self {
        let mut builder = InputBuilder::new(source);
        let mut cursor = Cursor::new(source);

        while let Some(first_char) = cursor.shift() {
            let token = match first_char {
                '(' | '[' => Token::OpenDelimiter,
                ')' | ']' => Token::CloseDelimiter,
                '"' | '\'' => {
                    scan_string(first_char, &mut cursor);
                    Token::String
                }
                _ if classes::is_whitespace(first_char) => {
                    cursor.shift_while(classes::is_whitespace);
                    Token::Whitespace
                }
                _ => Token::Unknown,
            };

            let len = cursor.reset_len();
            builder.push(token, len);
        }

        builder.finish()
    }

    pub(crate) fn next(&mut self) -> Option<Token> {
        let peeked = self.tokens[self.pos];

        match peeked {
            Token::Eof => None,
            _ => {
                self.pos += 1;
                Some(peeked)
            }
        }
    }

    pub(crate) fn peek(&self) -> Token {
        self.tokens[self.pos]
    }

    pub(crate) fn prev(&self) -> Token {
        self.tokens[self.pos.saturating_sub(2)]
    }

    pub(crate) fn slice(&self) -> &str {
        let hi = self.start_offsets[self.pos] as usize;
        let lo = self.start_offsets[self.pos - 1] as usize;

        &self.source[lo..hi]
    }
}

#[derive(Default)]
struct InputBuilder<'me> {
    input: Input<'me>,
    offset: u32,
}

impl<'me> InputBuilder<'me> {
    fn new(source: &'me str) -> Self {
        Self {
            input: Input {
                source,
                ..<_>::default()
            },
            offset: 0,
        }
    }

    fn push(&mut self, kind: Token, len: u32) {
        self.input.tokens.push(kind);
        self.input.start_offsets.push(self.offset);

        self.offset += len;
    }

    fn finish(mut self) -> Input<'me> {
        self.push(Token::Eof, 0);
        self.input
    }
}

fn scan_string(c: char, cursor: &mut Cursor) {
    let quote_type = c;
    while let Some(c) = cursor.peek() {
        match c {
            '\\' => {
                cursor.shift();
                if cursor.matches('\\') || cursor.matches(quote_type) {
                    cursor.shift();
                }
            }
            c if c == quote_type => {
                cursor.shift();
                return;
            }
            _ => {
                cursor.shift();
            }
        }
    }
}
