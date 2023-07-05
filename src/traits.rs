use std::collections::HashMap;

use wca::Context;

use crate::formatter::input::{Delimiter, Token};

#[derive(Clone)]
pub(crate) struct Config {
    pub(crate) delimiters: HashMap<char, Token>,
}

impl Default for Config {
    fn default() -> Self {
        let mut delimiters = HashMap::new();

        delimiters.insert('(', Token::OpenDelimiter(Delimiter::Paren));
        delimiters.insert(')', Token::CloseDelimiter(Delimiter::Paren));
        delimiters.insert('{', Token::OpenDelimiter(Delimiter::Brace));
        delimiters.insert('}', Token::CloseDelimiter(Delimiter::Brace));
        delimiters.insert('[', Token::OpenDelimiter(Delimiter::Bracket));
        delimiters.insert(']', Token::CloseDelimiter(Delimiter::Bracket));

        Self { delimiters }
    }
}

pub(crate) trait HasConfig {
    fn config(&self) -> &Config;
}

impl HasConfig for Context {
    fn config(&self) -> &Config {
        // ! Unsafe !
        match self.get_ref() {
            Some(config) => config,
            None => {
                self.insert(load_config());
                self.get_ref().unwrap()
            }
        }
    }
}

fn load_config() -> Config {
    todo!()
}
