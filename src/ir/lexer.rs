use crate::ir::profile::Tokens;
use crate::vec_map::VecMap;

use m_lexer::{Lexer as Lexer0, LexerBuilder, TokenKind};

use super::lexed;

// The `Lexer` struct definition.
#[allow(dead_code)]
pub struct Lexer {
    // `names` is a private vector-based map (VecMap) that stores strings associated with some indices.
    pub(crate) names: VecMap<String>,
    // `colors` is a private vector-based map (VecMap) that stores strings associated with some indices.
    pub(crate) colors: VecMap<String>,
    // `lexer` is a private field of type `Lexer0`, representing the core lexer functionality.
    pub(crate) lexer: Lexer0,
}
impl Lexer {
    /// Constructs a new lexer profile based on the provided tokens.
    ///
    /// # Arguments
    ///
    /// * `tokens` - A collection of `Token` objects that define the lexer's token rules.
    ///
    /// # Returns
    ///
    /// A new lexer profile with the specified tokens and associated rules.
    ///
    /// # Panics
    ///
    /// This method will panic if there are duplicate token names or if the number of tokens exceeds the capacity limit.
    /// Ensure that the `tokens` collection does not contain duplicate names, and its size does not exceed the capacity limit
    /// of the internal data structures (rules + 1).
    ///
    /// # Example
    ///
    /// ```ignore
    /// use my_lexer::{Profile, Token, Tokens};
    ///
    /// let tokens = Tokens::new(vec![
    ///     Token::new("Identifier", "green", r"[a-zA-Z_][a-zA-Z0-9_]*"),
    ///     Token::new("Number", "blue", r"\d+"),
    ///     Token::new("Operator", "purple", r"[-+*/]"),
    ///     // Add more token rules as needed.
    /// ]);
    ///
    /// let profile = Profile::new(tokens);
    /// ```
    pub(crate) fn new(tokens: Tokens) -> Self {
        let mut builder = LexerBuilder::new();

        // Calculate the number of rules, adding 1 to account for an extra rule.
        let rules = tokens.len() + 1;

        let mut names = VecMap::with_capacity(rules);
        let mut colors = VecMap::with_capacity(rules);

        for rule in tokens {
            let kind = ensure_equal(names.insert(rule.name), colors.insert(rule.color));
            builder = builder.token(kind, &rule.regex);
        }

        // Create an error token for cases where the input does not match any specified rule.
        let error_token = ensure_equal(
            names.insert("Error".to_owned()),
            colors.insert("black".to_owned()),
        );

        Self {
            names,
            colors,
            lexer: builder.error_token(error_token).build(),
        }
    }

    /// Provides access to the color associated with a specific `TokenKind`.
    ///
    /// # Arguments
    ///
    /// * `token` - A `TokenKind` representing the type of token whose color is requested.
    ///
    /// # Returns
    ///
    /// A string slice representing the color associated with the specified `TokenKind`.
    ///
    /// # Panics
    ///
    /// This function will panic if the given `TokenKind` is out of bounds for the internal colors array.
    /// The caller should ensure that `token` is a valid index for the colors array to avoid panics.
    pub(crate) fn color(&self, token: TokenKind) -> &str {
        &self.colors[token]
    }

    /// Tokenizes the input string and returns an iterator over the generated tokens.
    ///
    /// # Arguments
    ///
    /// * `input` - The input string to be tokenized.
    ///
    /// # Returns
    ///
    /// An iterator over the generated tokens along with the original input string.
    /// The iterator produces references to tokens that borrow from the input string.
    ///
    /// # Example
    ///
    /// ```ignore
    /// use my_lexer::Profile;
    ///
    /// let profile = Profile::new();
    /// let input_string = "let x = 42;";
    ///
    /// let tokens_iter = profile.tokenize(input_string);
    /// for token in tokens_iter {
    ///     println!("Token: {:?}", token);
    /// }
    /// ```
    ///
    /// # Note
    ///
    /// The returned iterator borrows from the input string (`&'input str`), so make sure the input
    /// string's lifetime is long enough to cover the usage of the tokens.
    pub(crate) fn tokenize<'input>(&self, input: &'input str) -> lexed::Tokens<'input> {
        lexed::Tokens {
            input,
            tokens: self.lexer.tokenize(input),
        }
    }
}

fn ensure_equal<T: Eq>(a: T, b: T) -> T {
    debug_assert!(a == b);
    a
}
