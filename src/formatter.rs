use std::fmt::Write;

use self::input::Input;

mod classes;
mod cursor;
mod input;

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
#[non_exhaustive]
pub(crate) enum Token {
    OpenDelimiter,
    CloseDelimiter,
    String,
    Whitespace,
    Unknown,
    Eof,
}

pub(crate) fn format(source: &str) -> String {
    let mut output = String::new();
    let mut input = Input::of(source);

    while let Some(token) = input.next() {
        match token {
            Token::CloseDelimiter
                if !matches!(input.prev(), Token::OpenDelimiter | Token::Whitespace) =>
            {
                output.write_char(' ').unwrap()
            }
            _ => (),
        }

        output.write_str(input.slice()).unwrap();

        match token {
            Token::OpenDelimiter
                if !matches!(input.peek(), Token::CloseDelimiter | Token::Whitespace) =>
            {
                output.write_char(' ').unwrap()
            }
            _ => (),
        }
    }

    output
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use super::format;
    use expect_test::{expect, Expect};

    #[track_caller]
    fn check(lines: &[&str], expect: Expect) {
        let actual = lines.iter().map(|line| format(line)).join("\n");
        expect.assert_eq(&actual);
    }

    #[test]
    fn simple_test() {
        check(
            &[
                "#[enum_dispatch(DatabaseImpl)]",
                "#[ enum_dispatch( DatabaseImpl ) ]",
                "(1, 2, 3)",
                "( 1, 2, 3 )",
                "call(1, 2, 3)",
                ")",
                "]",
                "[]",
                "()",
                "(())",
                "[[]]",
                "'(())'",
                "\"(())\"",
            ],
            expect![[r##"
                #[ enum_dispatch( DatabaseImpl ) ]
                #[ enum_dispatch( DatabaseImpl ) ]
                ( 1, 2, 3 )
                ( 1, 2, 3 )
                call( 1, 2, 3 )
                 )
                 ]
                []
                ()
                ( () )
                [ [] ]
                '(())'
                "(())""##]],
        )
    }
}
