use crate::lexer::Token;

// Define a public, crate-visible struct called Tokens.
// This struct holds references to an input string and a vector of Token objects.
pub(crate) struct Tokens<'input> {
    // The input string that the tokens will be extracted from.
    pub(crate) input: &'input str,
    // The vector that will store the extracted tokens.
    pub(crate) tokens: Vec<Token>,
}

impl<'input> Tokens<'input> {
    // Define a public function named `stream` that takes ownership of `self`, which is an instance of the `Tokens` struct.
    // This function will return a new `TokenStream` object, which will be used to iterate over the tokens extracted from the input string.
    pub(crate) fn stream(self) -> TokenStream<'input> {
        TokenStream {
            position: 0,
            start: 0,
            end: 0,
            lexed: self,
        }
    }
}

// Define a public, crate-visible struct named `TokenStream` parameterized by a lifetime `'input`.
// This struct is used to represent a stream of tokens extracted from an input string.
pub(crate) struct TokenStream<'input> {
    // The current position in the token stream. It represents the index of the current token being processed.
    pub(crate) position: usize,
    // The starting index of the current token in the input string.
    // This is used to identify the beginning of the current token in the original input string.
    pub(crate) start: usize,
    // The ending index of the current token in the input string.
    // This is used to identify the end of the current token in the original input string.
    pub(crate) end: usize,
    // The `Tokens` object that contains the extracted tokens.
    // It is parameterized by the same lifetime `'input` as the `TokenStream`.
    // This allows the `TokenStream` to borrow the tokens and access them during iteration.
    pub(crate) lexed: Tokens<'input>,
}

impl<'input> TokenStream<'input> {
    pub(crate) fn prev(&self) -> Option<Token> {
        self.lexed
            .tokens
            .get(self.position.checked_sub(2)?)
            .copied()
    }

    pub(crate) fn next(&mut self) -> Option<Token> {
        let token = self.lexed.tokens.get(self.position).copied();

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

    use crate::highlight::Markdown;

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
        traverse("tests/assets/lex", "md", |input, expected| {
            let input = crate::highlight::highlight(
                &std::fs::read_to_string(input).unwrap(),
                Markdown.into(),
            );

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
