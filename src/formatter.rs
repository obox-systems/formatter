use self::input::{Delimiter, Input, Token};

mod classes;
mod cursor;
pub(crate) mod input;

#[derive(Default)]
struct Emitter {
    output: String,
    lvl: usize,
}

impl Emitter {
    #[allow(dead_code)]
    fn newline(&mut self) {
        self.output.push('\n');
    }

    fn raw(&mut self, string: &str) {
        self.output.push_str(string);
    }

    fn whitespace(&mut self) {
        self.output.push(' ');
    }

    fn indent(&mut self, lvl: Option<usize>) {
        let lvl = lvl.unwrap_or(self.lvl);
        let s = "  ".repeat(lvl);

        self.output.push_str(&s);
    }

    fn finish(self) -> String {
        self.output
    }
}

impl Emitter {
    fn before(&mut self, current: Token, input: &Input) {
        match current {
            Token::CloseDelimiter(delimiter) => {
                if input
                    .prev()
                    .skip_whitespace(Token::OpenDelimiter(delimiter).into())
                {
                    match delimiter {
                        Delimiter::Brace => {}
                        _ => self.whitespace(),
                    }
                }
            }
            Token::OpenDelimiter(Delimiter::Brace) if input.prev() == Token::Colon => {}
            Token::OpenDelimiter(Delimiter::Brace)
                if !matches!(input.prev(), Token::Newline | Token::Empty) =>
            {
                // self.newline();
                //self.indent(None);
            }
            _ if current.maybe_binary_operator() && input.prev() != Token::Whitespace => {
                self.whitespace()
            }
            _ => {}
        }
    }

    fn after(&mut self, current: Token, input: &Input) {
        match current {
            Token::OpenDelimiter(delimiter) => match delimiter {
                Delimiter::Paren | Delimiter::Bracket
                    if input
                        .peek()
                        .skip_whitespace(Token::CloseDelimiter(delimiter).into()) =>
                {
                    self.whitespace();
                }
                _ => {}
            },
            _ if current.maybe_binary_operator() && input.peek() != Token::Whitespace => {
                self.whitespace()
            }
            _ => (),
        }
    }
}

pub(crate) fn format(source: &str) -> String {
    let mut emitter = Emitter::default();
    let input = Input::of(source);

    for token in input.iter() {
        match token {
            Token::OpenDelimiter(Delimiter::Brace) => emitter.lvl += 1,
            Token::CloseDelimiter(Delimiter::Brace) => emitter.lvl -= 1,
            _ => {}
        };

        match token {
            Token::Newline => {}
            Token::Whitespace
                if matches!(input.peek(), Token::CloseDelimiter(Delimiter::Brace)) =>
            {
                emitter.indent(Some(emitter.lvl - 1));
                continue;
            }
            Token::Whitespace if input.prev() == Token::Newline => {
                emitter.indent(None);
                continue;
            }
            _ => {}
        }

        emitter.before(token, &input);
        emitter.raw(input.slice());
        emitter.after(token, &input);
    }

    emitter.finish()
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::{format, input::Input};
    use itertools::Itertools;
    use pretty_assertions::assert_eq;

    fn update_expect() -> bool {
        std::env::var("UPDATE_EXPECT").is_ok()
    }

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
    fn with_extension_test() {
        let path = PathBuf::from("file.txt");
        let expected = PathBuf::from("file.txt.expected");
        assert_eq!(with_extension(&path, "expected"), expected);

        let path = PathBuf::from("dir/file");
        let expected = PathBuf::from("dir/file.expected");
        assert_eq!(with_extension(&path, "expected"), expected);

        let path = PathBuf::from("file");
        let expected = PathBuf::from("file.expected");
        assert_eq!(with_extension(&path, "expected"), expected);
    }

    fn read_or_create(path: PathBuf, fallback: &str) -> String {
        let fallback = || {
            println!("\x1b[1m\x1b[92mupdating\x1b[0m: {}", path.display());
            std::fs::write(&path, fallback).unwrap();
            fallback.to_owned()
        };

        match std::fs::read_to_string(&path) {
            Ok(value) => {
                if update_expect() {
                    return fallback();
                }

                value
            }
            Err(err) if err.kind() == std::io::ErrorKind::NotFound => fallback(),
            Err(err) => panic!("{err:?}"),
        }
    }

    #[test]
    fn formatter() {
        traverse("tests/assets/formatter", |input, expected| {
            let input = format(&std::fs::read_to_string(input).unwrap());
            let expected = read_or_create(expected, &input);

            assert_eq!(input, expected);

            let expected = format(&input);
            assert_eq!(input, expected);
        });
    }

    #[test]
    fn highlight() {
        traverse("tests/assets/highlight", |input, expected| {
            let input = crate::highlight::highlight(&std::fs::read_to_string(input).unwrap());
            let expected = read_or_create(expected, &input);

            assert_eq!(input, expected);
        });
    }

    #[test]
    fn lex() {
        traverse("tests/assets/lex", |input, expected| {
            let input = std::fs::read_to_string(input).unwrap();
            let input = Input::of(&input);

            let input = input
                .iter()
                .map(|token| format!("{token:?} at {:?}", input.span()))
                .join("\n");

            let expected = read_or_create(expected, &input);
            assert_eq!(input, expected);
        });
    }

    #[test]
    fn pg() {
        println!("{}", format("fn name() {}"));
    }
}
