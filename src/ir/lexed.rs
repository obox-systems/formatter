use m_lexer::Token;

pub(crate) struct Tokens<'input> {
    pub(crate) input: &'input str,
    pub(crate) tokens: Vec<Token>,
}

impl<'input> Tokens<'input> {
    pub(crate) fn reader(self) -> TokenReader<'input> {
        TokenReader {
            position: 0,
            start: 0,
            end: 0,
            lexed: self,
        }
    }
}

pub(crate) struct TokenReader<'input> {
    pub(crate) position: usize,
    pub(crate) start: usize,
    pub(crate) end: usize,
    pub(crate) lexed: Tokens<'input>,
}

impl<'input> TokenReader<'input> {
    pub(crate) fn next(&mut self) -> Option<&Token> {
        let token = self.lexed.tokens.get(self.position);

        if let Some(token) = token {
            self.position += 1;

            self.start = self.end;
            self.end += token.len;
        }

        token
    }

    pub(crate) fn slice(&self) -> &str {
        &self.lexed.input[self.start..self.end]
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

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

    fn traverse(root: &str, expected_ext: &str, f: impl Fn(PathBuf, PathBuf)) {
        for entry in std::fs::read_dir(root).unwrap() {
            let input_path = entry.unwrap().path();
            let expected_path = with_extension(&input_path, expected_ext);

            if input_path.extension().unwrap_or_default() == expected_ext {
                continue;
            };

            f(input_path, expected_path)
        }
    }

    #[test]
    fn highlight() {
        traverse("tests", "md", |input, expected| {
            let input = crate::highlight::highlight(&std::fs::read_to_string(input).unwrap());
            let expected = read_or_create(expected, &input);

            assert_eq!(input, expected);
        });
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
}
