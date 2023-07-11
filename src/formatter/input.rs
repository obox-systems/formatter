use std::cell::Cell;

use colored::Color;

use crate::traits::Config;

use super::{classes, cursor::Cursor};

#[derive(serde::Deserialize, PartialEq, Eq, Clone, Copy, Debug)]
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
    /// `r#""#`
    RawString,
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
    /// '+='
    PlusEq,
    /// `-=`
    MinusEq,
    /// `!=`
    NeEq,
    /// `-`
    Minus,
    /// `/`
    Slash,
    /// `*`
    Star,
    /// `>=`
    GreaterThan,

    /// '!='
    BangEq,

    /// `->`, `=>`
    Arrow,

    /// `:`
    Colon,

    /// `'me`
    Lifetime,

    /// `&`
    BitAnd,

    /// `&&`
    And,

    /// `%`
    Rem,

    Char,

    /// kwk
    Empty,

    /// Ident
    Ident,

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
                | Self::GreaterThan
        )
    }

    pub(crate) fn color(&self) -> Color {
        match self {
            Self::OpenDelimiter(_) => Color::BrightBlack,
            Self::CloseDelimiter(_) => Color::BrightBlack,
            Self::String => Color::BrightMagenta,
            Self::RawString => Color::Magenta,
            Self::Whitespace => Color::White,
            Self::Newline => Color::BrightRed,
            Self::Unknown => Color::Black,
            Self::Comment => Color::Cyan,
            Self::Char => Color::Blue,
            Self::Eof => todo!(),
            _ => Color::Green,
        }
    }
}

impl Token {
    pub(crate) fn skip_whitespace(&self, a: Option<Self>) -> bool {
        match *self {
            b @ (Self::OpenDelimiter(_) | Self::CloseDelimiter(_)) => Some(b) != a,
            Self::Whitespace | Self::Newline => false,
            _ => true,
        }
    }
}

#[derive(serde::Deserialize, PartialEq, Eq, Clone, Copy, Debug)]
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
        use Token::*;

        let config = Config::default();
        let mut builder = InputBuilder::new(source);
        let mut cursor = Cursor::new(source);

        while let Some(first_char) = cursor.shift() {
            let token = if let Some(token) = config.delimiters.get(&first_char) {
                *token
            } else {
                match first_char {
                    ':' => Colon,
                    '&' if cursor.shift_if('&') => And,
                    '&' => BitAnd,
                    '%' => Rem,
                    '!' if cursor.shift_if('=') => BangEq,
                    '=' if cursor.shift_if('>') => Arrow,
                    '=' if cursor.shift_if('=') => EqEq,
                    '=' if cursor.shift_if('!') => NeEq,
                    '=' => Eq,
                    '+' if cursor.shift_if('+') => PlusPlus,
                    '+' if cursor.shift_if('=') => PlusEq,
                    '+' => Plus,
                    '-' if cursor.shift_if('=') => MinusEq,
                    '-' if cursor.shift_if('>') => Arrow,
                    '-' => Minus,
                    '>' if cursor.shift_if('=') => GreaterThan,
                    '/' if cursor.shift_if('/') => {
                        cursor.shift_while(|ch| !classes::is_newline(ch));
                        Comment
                    }
                    '/' => Slash,
                    '*' => Star,
                    '"' => {
                        scan_string(first_char, &mut cursor);
                        String
                    }
                    '\'' => scan_lifetime_or_char(&mut cursor),
                    _ if classes::is_newline(first_char) => {
                        cursor.shift_while(classes::is_newline);
                        Newline
                    }
                    _ if classes::is_whitespace(first_char) => {
                        cursor.shift_while(classes::is_whitespace);
                        Whitespace
                    }
                    'r' => {
                        scan_raw_string(&mut cursor);
                        RawString
                    }
                    _ if classes::is_id_start(first_char) => {
                        cursor.shift_while(classes::is_id_continue);
                        Ident
                    }
                    _ => Unknown,
                }
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
        match self.pos.get().checked_sub(2) {
            Some(pos) => self.tokens[pos],
            None => Token::Empty,
        }
    }

    /// Returns the span of the current position in the source.
    /// The span is determined by the start offsets stored in `self.start_offsets`.
    pub(crate) fn span(&self, pos: impl Into<Option<usize>>) -> (u32, u32) {
        let pos = pos.into().unwrap_or(self.pos.get());

        let hi = self.start_offsets[pos];
        let lo = self.start_offsets[pos - 1];

        (lo, hi)
    }

    /// Returns a slice of the source string corresponding to the span.
    /// The span is determined by the `lo` and `hi` values obtained from `self.span()`.
    pub(crate) fn slice(&self) -> &str {
        let (lo, hi) = self.span(None);

        &self.source[lo as usize..hi as usize]
    }

    pub(crate) fn prev_slice(&self) -> &str {
        let (lo, hi) = self.span(self.pos.get().saturating_sub(1));

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

fn scan_lifetime_or_char(cursor: &mut Cursor) -> Token {
    if cursor.shift_if(classes::is_xid_start) {
        cursor.shift_while(classes::is_xid_continue);

        if cursor.shift_if('\'') {
            Token::Char
        } else {
            Token::Lifetime
        }
    } else {
        scan_char(cursor);
        Token::Char
    }
}

fn scan_string(c: char, cursor: &mut Cursor) {
    let quote_type = c;
    while let Some(c) = cursor.peek() {
        match c {
            '\\' => {
                cursor.shift();
                cursor.shift_if(|ch| ch == '\\' || ch == quote_type);
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

fn scan_char(cursor: &mut Cursor) {
    while let Some(c) = cursor.peek() {
        match c {
            '\\' => {
                cursor.shift();
                cursor.shift_if(|ch| ch == '\\' || ch == '\'');
            }
            '\'' => {
                cursor.shift();
                return;
            }
            '\n' => return,
            _ => {
                cursor.shift();
            }
        }
    }
}

fn scan_raw_string(cursor: &mut Cursor) {
    let mut hashes = 0;
    while cursor.shift_if('#') {
        hashes += 1;
    }

    if !cursor.shift_if('"') {
        return;
    }

    while let Some(c) = cursor.shift() {
        if c == '"' {
            let mut hashes_left = hashes;
            while cursor.peek() == '#'.into() && hashes_left > 0 {
                hashes_left -= 1;
                cursor.shift();
            }
            if hashes_left == 0 {
                return;
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
            .map(|token| format!("{token:?} at {:?}", input.span(None)))
            .join("\n");
        expect.assert_eq(&actual);
    }

    #[test]
    fn foo() {
        check("'\u{0009}'", expect!["Char at (0, 3)"]);
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
                Rem at (11, 12)
                Whitespace at (12, 13)
                Eq at (13, 14)
                Whitespace at (14, 15)
                EqEq at (15, 17)
                Whitespace at (17, 18)
                BangEq at (18, 20)
                Whitespace at (20, 21)
                Unknown at (21, 22)
                Whitespace at (22, 23)
                Unknown at (23, 24)
                Whitespace at (24, 25)
                Unknown at (25, 26)
                Eq at (26, 27)
                Whitespace at (27, 28)
                GreaterThan at (28, 30)
                Whitespace at (30, 31)
                And at (31, 33)
                Whitespace at (33, 34)
                Unknown at (34, 35)
                Unknown at (35, 36)
                Whitespace at (36, 37)
                Unknown at (37, 38)
                Whitespace at (38, 39)
                BitAnd at (39, 40)
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
                Char at (49, 56)
                Newline at (56, 57)
                Whitespace at (57, 69)"#]],
        );
    }
}
