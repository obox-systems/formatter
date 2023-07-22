use m_lexer::Token;

pub(crate) struct Lexed<'input> {
    pub(crate) input: &'input str,
    pub(crate) tokens: Vec<Token>,
}

impl<'input> Lexed<'input> {
    pub(crate) fn reader(self) -> LexerReader<'input> {
        LexerReader {
            position: 0,
            end: 0,
            start: 0,
            lexed: self,
        }
    }
}

pub(crate) struct LexerReader<'input> {
    pub(crate) position: usize,
    pub(crate) start: usize,
    pub(crate) end: usize,
    pub(crate) lexed: Lexed<'input>,
}

impl<'input> LexerReader<'input> {
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
