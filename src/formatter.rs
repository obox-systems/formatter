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
    let input = Input::of(source);

    for token in input.iter() {
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
    use super::format;
    use pretty_assertions::assert_eq;

    fn check(lines: &[&str], expect: &[&str]) {
        let actual: Vec<String> = lines.iter().map(|line| format(line)).collect();
        assert_eq!(actual, expect);
    }

    #[test]
    fn empty() {
        check(&[""], &[""]);
    }

    #[test]
    fn single() {
        check(&[")", "(", "[", "]"], &[" )", "( ", "[ ", " ]"]);
    }

    #[test]
    fn attribute() {
        check(
            &[
                "#[enum_dispatch(DatabaseImpl)]",
                "#[ enum_dispatch( DatabaseImpl ) ]",
            ],
            &[
                "#[ enum_dispatch( DatabaseImpl ) ]",
                "#[ enum_dispatch( DatabaseImpl ) ]",
            ],
        );
    }

    #[test]
    fn call() {
        check(&["add(40, 2)"], &["add( 40, 2 )"]);
    }

    #[test]
    fn parentheses() {
        check(
            &["()", "(40, 2)", "( 40, 2 )"],
            &["()", "( 40, 2 )", "( 40, 2 )"],
        );
    }

    #[test]
    fn string() {
        check(&["\"(hello)\""], &["\"(hello)\""]);
    }
}
