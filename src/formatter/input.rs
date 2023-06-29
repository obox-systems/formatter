use std::cell::Cell;

use super::{classes, cursor::Cursor, Delimiter, Token};

#[derive(Default)]
pub(crate) struct Input<'me> {
    pub(crate) pos: Cell<usize>,
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
                '(' => Token::OpenDelimiter(Delimiter::Paren),
                '[' => Token::OpenDelimiter(Delimiter::Bracket),
                '{' => Token::OpenDelimiter(Delimiter::Brace),
                ')' => Token::CloseDelimiter(Delimiter::Paren),
                ']' => Token::CloseDelimiter(Delimiter::Bracket),
                '}' => Token::CloseDelimiter(Delimiter::Brace),
                '+' => Token::Plus,
                '-' => Token::Minus,
                '/' => Token::Slash,
                '*' => Token::Star,
                '"' => {
                    scan_string(first_char, &mut cursor);
                    Token::String
                }
                _ if classes::is_newline(first_char) => {
                    cursor.shift_while(classes::is_newline);
                    Token::Newline
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

    pub(crate) fn iter(&'me self) -> impl Iterator<Item = Token> + 'me {
        std::iter::from_fn(|| self.next())
    }

    pub(crate) fn next(&self) -> Option<Token> {
        let peeked = self.tokens[self.pos.get()];

        match peeked {
            Token::Eof => None,
            _ => {
                self.pos.set(self.pos.get() + 1);
                Some(peeked)
            }
        }
    }

    pub(crate) fn peek(&self) -> Token {
        self.tokens[self.pos.get()]
    }

    pub(crate) fn prev(&self) -> Token {
        self.tokens[self.pos.get().saturating_sub(2)]
    }

    pub(crate) fn span(&self) -> (u32, u32) {
        let hi = self.start_offsets[self.pos.get()];
        let lo = self.start_offsets[self.pos.get() - 1];

        (lo, hi)
    }

    pub(crate) fn slice(&self) -> &str {
        let (lo, hi) = self.span();

        &self.source[lo as usize..hi as usize]
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

#[cfg(test)]
mod tests {
    use expect_test::{expect, Expect};
    use itertools::Itertools;

    use super::Input;

    #[track_caller]
    fn check(source: &str, expect: Expect) {
        let input = Input::of(source);
        let actual = input
            .iter()
            .map(|token| format!("{token:?} at {:?}", input.span()))
            .join("\n");
        expect.assert_eq(&actual);
    }

    #[test]
    fn whitespace() {
        check("    ", expect!["Whitespace at (0, 4)"]);
        check(
            "\n  \n  \n",
            expect![[r#"
                Newline at (0, 1)
                Whitespace at (1, 3)
                Newline at (3, 4)
                Whitespace at (4, 6)
                Newline at (6, 7)"#]],
        );
    }

    #[test]
    fn string() {
        check(
            r#"
            "42"
            "(42)"
            'hello'
            "#,
            expect![[r#"
                Newline at (0, 1)
                Whitespace at (1, 13)
                String at (13, 17)
                Newline at (17, 18)
                Whitespace at (18, 30)
                String at (30, 36)
                Newline at (36, 37)
                Whitespace at (37, 49)
                Unknown at (49, 50)
                Unknown at (50, 51)
                Unknown at (51, 52)
                Unknown at (52, 53)
                Unknown at (53, 54)
                Unknown at (54, 55)
                Unknown at (55, 56)
                Newline at (56, 57)
                Whitespace at (57, 69)"#]],
        );
    }
}
