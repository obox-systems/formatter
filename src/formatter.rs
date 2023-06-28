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
    use std::path::PathBuf;

    use super::format;
    use pretty_assertions::assert_eq;

    fn with_extension(path: &PathBuf, extension: &str) -> PathBuf {
        match path.extension() {
            Some(raw_extension) => {
                let mut raw_extension = raw_extension.to_os_string();
                raw_extension.push(".");
                raw_extension.push(extension);
                path.with_extension(raw_extension)
            }
            None => path.with_extension(extension),
        }
    }

    fn traverse(root: &str, f: impl Fn(PathBuf, PathBuf)) {
        for entry in std::fs::read_dir(root).unwrap() {
            let input_path = entry.unwrap().path();
            let expected_path = with_extension(&input_path, "expected");

            if input_path.extension().unwrap_or_default() == "expected" {
                continue;
            };

            f(input_path, expected_path)
        }
    }

    #[test]
    fn tests() {
        traverse("tests/assets", |input, expected| {
            let input = format(&std::fs::read_to_string(input).unwrap());
            let expected = std::fs::read_to_string(expected).unwrap();

            assert_eq!(input, expected);
        });
    }

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
