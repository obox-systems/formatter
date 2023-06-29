use std::cell::Cell;

use super::{classes, cursor::Cursor};

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
#[non_exhaustive]
pub(crate) enum Token {
    /// `(`, `[`, `{`
    OpenDelimiter(Delimiter),
    /// `)`, `]`, `}`
    CloseDelimiter(Delimiter),
    /// `"hello"`
    String,
    /// ` `
    Whitespace,
    /// `\n`, `\r`
    Newline,
    /// Unknown symbol
    Unknown,
    /// `// SSS`
    Comment,
    // operators
    /// `=`
    Eq,
    /// `==`
    EqEq,
    /// `+`
    Plus,
    /// `++`
    PlusPlus,
    /// `-`
    Minus,
    /// `/`
    Slash,
    /// `*`
    Star,

    /// End of file.
    Eof,
}

impl Token {
    /// Checks whether the token is an operator.
    pub(crate) fn maybe_binary_operator(&self) -> bool {
        matches!(
            self,
            Self::Plus
                | Self::PlusPlus
                | Self::Eq
                | Self::EqEq
                | Self::Minus
                | Self::Slash
                | Self::Star
        )
    }
}

impl Token {
    pub(crate) fn skip_whitespace(&self, kind: Option<Delimiter>) -> bool {
        match *self {
            Self::OpenDelimiter(delimiter) | Self::CloseDelimiter(delimiter) => {
                Some(delimiter) != kind
            }
            Self::Whitespace | Self::Newline => false,
            _ => true,
        }
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub(crate) enum Delimiter {
    /// `(`, `)`
    Paren,
    /// `{`, `}`
    Brace,
    /// `[`, `]`
    Bracket,
}

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
                '=' if cursor.shift_if_eq('=') => Token::EqEq,
                '=' => Token::Eq,
                '+' if cursor.shift_if_eq('+') => Token::PlusPlus,
                '+' => Token::Plus,
                '-' => Token::Minus,
                '/' if cursor.shift_if_eq('/') => {
                    cursor.shift_while(|ch| !classes::is_newline(ch));
                    Token::Comment
                }
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

    /// Returns an iterator over the tokens in the token stream.
    /// The iterator will iterate over the tokens starting from the current position.
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

    /// Returns the token at the current position in the token stream.
    /// The current position is determined by the value of `self.pos`.
    pub(crate) fn peek(&self) -> Token {
        self.tokens[self.pos.get()]
    }

    pub(crate) fn prev(&self) -> Token {
        self.tokens[self.pos.get().saturating_sub(2)]
    }

    /// Returns the span of the current position in the source.
    /// The span is determined by the start offsets stored in `self.start_offsets`.
    pub(crate) fn span(&self) -> (u32, u32) {
        let hi = self.start_offsets[self.pos.get()];
        let lo = self.start_offsets[self.pos.get() - 1];

        (lo, hi)
    }

    /// Returns a slice of the source string corresponding to the span.
    /// The span is determined by the `lo` and `hi` values obtained from `self.span()`.
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
    fn operators() {
        check(
            "+ ++ - * / % = == != < > <= >= && || ! & | ^ << >>",
            expect![[r#"
            Plus at (0, 1)
            Whitespace at (1, 2)
            PlusPlus at (2, 4)
            Whitespace at (4, 5)
            Minus at (5, 6)
            Whitespace at (6, 7)
            Star at (7, 8)
            Whitespace at (8, 9)
            Slash at (9, 10)
            Whitespace at (10, 11)
            Unknown at (11, 12)
            Whitespace at (12, 13)
            Eq at (13, 14)
            Whitespace at (14, 15)
            EqEq at (15, 17)
            Whitespace at (17, 18)
            Unknown at (18, 19)
            Eq at (19, 20)
            Whitespace at (20, 21)
            Unknown at (21, 22)
            Whitespace at (22, 23)
            Unknown at (23, 24)
            Whitespace at (24, 25)
            Unknown at (25, 26)
            Eq at (26, 27)
            Whitespace at (27, 28)
            Unknown at (28, 29)
            Eq at (29, 30)
            Whitespace at (30, 31)
            Unknown at (31, 32)
            Unknown at (32, 33)
            Whitespace at (33, 34)
            Unknown at (34, 35)
            Unknown at (35, 36)
            Whitespace at (36, 37)
            Unknown at (37, 38)
            Whitespace at (38, 39)
            Unknown at (39, 40)
            Whitespace at (40, 41)
            Unknown at (41, 42)
            Whitespace at (42, 43)
            Unknown at (43, 44)
            Whitespace at (44, 45)
            Unknown at (45, 46)
            Unknown at (46, 47)
            Whitespace at (47, 48)
            Unknown at (48, 49)
            Unknown at (49, 50)"#]],
        );
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
